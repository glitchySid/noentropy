use crate::cli::path_utils::validate_and_normalize_path;
use crate::error::Result;
use crate::files::{execute_move_silent, is_text_file, read_file_sample};
use crate::gemini::GeminiClient;
use crate::models::OrganizationPlan;
use crate::settings::Config;
use crate::storage::{Cache, UndoLog};
use crate::tui::app::{App, AppState, Tab};
use crate::tui::ui::draw;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::future::join_all;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

const CACHE_RETENTION_SECONDS: u64 = 7 * 24 * 60 * 60;
const UNDO_LOG_RETENTION_SECONDS: u64 = 30 * 24 * 60 * 60;

pub async fn run_app(
    config: Config,
    target_path: Option<PathBuf>,
    recursive: bool,
    dry_run: bool,
    offline: bool,
) -> Result<()> {
    // Validate and normalize the target path
    let target_path = match target_path {
        Some(p) => validate_and_normalize_path(&p).await?,
        None => validate_and_normalize_path(&config.download_folder).await?,
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(
        config.clone(),
        target_path.clone(),
        recursive,
        dry_run,
        offline,
    );

    // Scan files initially
    app.scan_files();

    // Initialize cache and undo log
    let data_dir = Config::get_data_dir()?;
    let cache_path = data_dir.join(".noentropy_cache.json");
    let mut cache = Cache::load_or_create(&cache_path, true);
    cache.cleanup_old_entries(CACHE_RETENTION_SECONDS);

    let undo_log_path = Config::get_undo_log_path()?;
    let mut undo_log = UndoLog::load_or_create(&undo_log_path, true);
    undo_log.cleanup_old_entries(UNDO_LOG_RETENTION_SECONDS);

    // Main event loop
    let result = run_event_loop(&mut terminal, &mut app, &config, &mut cache, &mut undo_log).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Save cache and undo log
    if let Err(e) = cache.save(&cache_path) {
        eprintln!("Warning: Failed to save cache: {}", e);
    }
    if let Err(e) = undo_log.save(&undo_log_path) {
        eprintln!("Warning: Failed to save undo log: {}", e);
    }

    result
}

async fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    config: &Config,
    cache: &mut Cache,
    undo_log: &mut UndoLog,
) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, app))?;

        // Non-blocking event poll with timeout
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    app.should_quit = true;
                }
                KeyCode::Tab => {
                    app.next_tab();
                }
                KeyCode::BackTab => {
                    app.previous_tab();
                }
                KeyCode::Down | KeyCode::Char('j') => match app.tab {
                    Tab::Files => app.next_file(),
                    Tab::Plan => app.next_plan_item(),
                    Tab::Progress => {}
                },
                KeyCode::Up | KeyCode::Char('k') => match app.tab {
                    Tab::Files => app.previous_file(),
                    Tab::Plan => app.previous_plan_item(),
                    Tab::Progress => {}
                },
                KeyCode::Char('o') => {
                    if matches!(app.state, AppState::FileList) {
                        // Start organization
                        app.start_fetching();
                        terminal.draw(|frame| draw(frame, app))?;

                        match fetch_organization_plan(app, config, cache).await {
                            Ok(plan) => {
                                app.set_plan(plan);
                            }
                            Err(e) => {
                                app.set_error(e.to_string());
                            }
                        }
                    }
                }
                KeyCode::Char('c') => {
                    if matches!(app.state, AppState::PlanReview) {
                        // Confirm and execute
                        app.start_moving();
                        terminal.draw(|frame| draw(frame, app))?;

                        execute_organization(app, undo_log);
                        app.finish();
                    }
                }
                KeyCode::Char('r') => {
                    if matches!(app.state, AppState::Done | AppState::Error(_)) {
                        // Restart
                        *app = App::new(
                            config.clone(),
                            app.target_path.clone(),
                            app.recursive,
                            app.dry_run,
                            app.offline,
                        );
                        app.scan_files();
                    }
                }
                KeyCode::Char('t') => {
                    // Toggle offline mode
                    app.offline = !app.offline;
                    let mode_text = if app.offline { "ON" } else { "OFF" };
                    app.status_message = format!("Offline mode: {}", mode_text);
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

async fn fetch_organization_plan(
    app: &App,
    config: &Config,
    cache: &mut Cache,
) -> Result<OrganizationPlan> {
    let batch = app.batch.as_ref().ok_or("No files to organize")?;

    if app.offline {
        // Use offline categorization
        use crate::files::categorize_files_offline;
        let result = categorize_files_offline(batch.filenames.clone());
        return Ok(result.plan);
    }

    // Online AI categorization
    let client = GeminiClient::new(&config.api_key, &config.categories);

    // Check connectivity first
    client.check_connectivity().await?;

    // Get initial plan
    let mut plan = client
        .organize_files_in_batches(batch.filenames.clone(), Some(cache), Some(&app.target_path))
        .await?;

    // Deep inspection for text files
    let client_arc = Arc::new(client);
    let semaphore = Arc::new(tokio::sync::Semaphore::new(5));

    let tasks: Vec<_> = plan
        .files
        .iter_mut()
        .zip(batch.paths.iter())
        .map(|(file_category, path)| {
            let client = Arc::clone(&client_arc);
            let filename = file_category.filename.clone();
            let category = file_category.category.clone();
            let path = path.clone();
            let semaphore = Arc::clone(&semaphore);

            async move {
                if is_text_file(&path) {
                    let _permit = semaphore.acquire().await.unwrap();
                    if let Some(content) = read_file_sample(&path, 5000) {
                        client
                            .get_ai_sub_category(&filename, &category, &content)
                            .await
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
        })
        .collect();

    let sub_categories: Vec<String> = join_all(tasks).await;

    for (file_category, sub_category) in plan.files.iter_mut().zip(sub_categories) {
        file_category.sub_category = sub_category;
    }

    Ok(plan)
}

fn execute_organization(app: &mut App, undo_log: &mut UndoLog) {
    let Some(plan) = app.plan.take() else {
        app.set_error("No plan to execute".to_string());
        return;
    };

    if app.dry_run {
        // Dry run - just simulate
        app.moved_count = plan.files.len();
        app.progress = plan.files.len();
        app.plan = Some(plan);
        return;
    }

    // Execute the move using silent version (no console output)
    match execute_move_silent(&app.target_path, plan.clone(), Some(undo_log)) {
        Ok(summary) => {
            app.moved_count = summary.moved_count();
            app.error_count = summary.error_count();
            app.progress = app.moved_count + app.error_count;
        }
        Err(e) => {
            app.set_error(format!("Move failed: {}", e));
        }
    }
    app.plan = Some(plan);
}
