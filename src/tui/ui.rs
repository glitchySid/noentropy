use crate::tui::app::{App, AppState, Tab};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs, Wrap},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Tabs
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Status bar
            Constraint::Length(3), // Help bar
        ])
        .split(frame.area());

    draw_title(frame, chunks[0]);
    draw_tabs(frame, app, chunks[1]);
    draw_main_content(frame, app, chunks[2]);
    draw_status_bar(frame, app, chunks[3]);
    draw_help_bar(frame, app, chunks[4]);
}

fn draw_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("NoEntropy - AI File Organizer")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn draw_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["Files", "Plan", "Progress"];
    let selected = match app.tab {
        Tab::Files => 0,
        Tab::Plan => 1,
        Tab::Progress => 2,
    };

    let tabs = Tabs::new(titles)
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider("|")
        .block(Block::default().borders(Borders::ALL).title("View"));

    frame.render_widget(tabs, area);
}

fn draw_main_content(frame: &mut Frame, app: &App, area: Rect) {
    match app.tab {
        Tab::Files => draw_files_tab(frame, app, area),
        Tab::Plan => draw_plan_tab(frame, app, area),
        Tab::Progress => draw_progress_tab(frame, app, area),
    }
}

fn draw_files_tab(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // File list
    let items: Vec<ListItem> = match &app.batch {
        Some(batch) => batch
            .filenames
            .iter()
            .enumerate()
            .map(|(i, filename)| {
                let style = if i == app.file_list_state {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                let prefix = if i == app.file_list_state { "> " } else { "  " };
                ListItem::new(format!("{}{}", prefix, filename)).style(style)
            })
            .collect(),
        None => vec![ListItem::new("No files loaded").style(Style::default().fg(Color::DarkGray))],
    };

    let file_list = List::new(items).block(Block::default().borders(Borders::ALL).title(format!(
        "Files ({}/{})",
        app.file_list_state + 1,
        app.total_files
    )));
    frame.render_widget(file_list, chunks[0]);

    // File details
    let details = match app.get_selected_file() {
        Some((filename, path)) => {
            let size = std::fs::metadata(path)
                .map(|m| format_size(m.len()))
                .unwrap_or_else(|_| "Unknown".to_string());

            let extension = path
                .extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_else(|| "None".to_string());

            vec![
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::Cyan)),
                    Span::raw(filename),
                ]),
                Line::from(vec![
                    Span::styled("Size: ", Style::default().fg(Color::Cyan)),
                    Span::raw(size),
                ]),
                Line::from(vec![
                    Span::styled("Extension: ", Style::default().fg(Color::Cyan)),
                    Span::raw(extension),
                ]),
                Line::from(vec![
                    Span::styled("Path: ", Style::default().fg(Color::Cyan)),
                    Span::raw(path.display().to_string()),
                ]),
            ]
        }
        None => vec![Line::from("No file selected")],
    };

    let details_widget = Paragraph::new(details)
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .wrap(Wrap { trim: true });
    frame.render_widget(details_widget, chunks[1]);
}

fn draw_plan_tab(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Plan list
    let items: Vec<ListItem> = match &app.plan {
        Some(plan) => plan
            .files
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == app.plan_list_state {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                let prefix = if i == app.plan_list_state { "> " } else { "  " };
                let target = if item.sub_category.is_empty() {
                    item.category.clone()
                } else {
                    format!("{}/{}", item.category, item.sub_category)
                };
                ListItem::new(format!("{}{} -> {}", prefix, item.filename, target)).style(style)
            })
            .collect(),
        None => vec![
            ListItem::new("No plan available. Press 'o' to organize.")
                .style(Style::default().fg(Color::DarkGray)),
        ],
    };

    let plan_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Organization Plan"),
    );
    frame.render_widget(plan_list, chunks[0]);

    // Plan item details
    let details = match app.get_selected_plan_item() {
        Some(item) => {
            let target = if item.sub_category.is_empty() {
                item.category.clone()
            } else {
                format!("{}/{}", item.category, item.sub_category)
            };
            vec![
                Line::from(vec![
                    Span::styled("File: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&item.filename),
                ]),
                Line::from(vec![
                    Span::styled("Category: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&item.category),
                ]),
                Line::from(vec![
                    Span::styled("Sub-category: ", Style::default().fg(Color::Cyan)),
                    Span::raw(if item.sub_category.is_empty() {
                        "None"
                    } else {
                        &item.sub_category
                    }),
                ]),
                Line::from(vec![
                    Span::styled("Target: ", Style::default().fg(Color::Green)),
                    Span::raw(target),
                ]),
            ]
        }
        None => vec![Line::from("No item selected")],
    };

    let details_widget = Paragraph::new(details)
        .block(Block::default().borders(Borders::ALL).title("Move Details"))
        .wrap(Wrap { trim: true });
    frame.render_widget(details_widget, chunks[1]);
}

fn draw_progress_tab(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Progress bar
            Constraint::Min(5),    // Stats
        ])
        .split(area);

    // Progress bar
    let progress_percent = if app.total_files > 0 {
        (app.progress as f64 / app.total_files as f64 * 100.0) as u16
    } else {
        0
    };

    let progress_label = format!("{}/{} files processed", app.progress, app.total_files);
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(progress_percent)
        .label(progress_label);
    frame.render_widget(gauge, chunks[0]);

    // Stats
    let stats = vec![
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:?}", app.state),
                match app.state {
                    AppState::Done => Style::default().fg(Color::Green),
                    AppState::Error(_) => Style::default().fg(Color::Red),
                    AppState::Moving => Style::default().fg(Color::Yellow),
                    _ => Style::default().fg(Color::White),
                },
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Moved: ", Style::default().fg(Color::Green)),
            Span::raw(app.moved_count.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Errors: ", Style::default().fg(Color::Red)),
            Span::raw(app.error_count.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Remaining: ", Style::default().fg(Color::Yellow)),
            Span::raw((app.total_files.saturating_sub(app.progress)).to_string()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Dry Run: ", Style::default().fg(Color::Cyan)),
            Span::raw(if app.dry_run {
                "Yes (no files will be moved)"
            } else {
                "No"
            }),
        ]),
        Line::from(vec![
            Span::styled("Mode: ", Style::default().fg(Color::Cyan)),
            Span::raw(if app.offline {
                "Offline"
            } else {
                "Online (AI)"
            }),
        ]),
    ];

    let stats_widget = Paragraph::new(stats)
        .block(Block::default().borders(Borders::ALL).title("Statistics"))
        .wrap(Wrap { trim: true });
    frame.render_widget(stats_widget, chunks[1]);
}

fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status_style = match app.state {
        AppState::Error(_) => Style::default().fg(Color::Red),
        AppState::Done => Style::default().fg(Color::Green),
        _ => Style::default().fg(Color::Yellow),
    };

    let status = Paragraph::new(app.status_message.clone())
        .style(status_style)
        .block(Block::default().borders(Borders::ALL).title("Status"));
    frame.render_widget(status, area);
}

fn draw_help_bar(frame: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.state {
        AppState::FileList => "[o] Organize  [Tab] Switch view  [j/k] Navigate  [q] Quit",
        AppState::Fetching => "Fetching... Please wait",
        AppState::PlanReview => "[c] Confirm  [Tab] Switch view  [j/k] Navigate  [q] Quit",
        AppState::Moving => "Moving files... Please wait",
        AppState::Done => "[q] Quit  [r] Restart",
        AppState::Error(_) => "[q] Quit  [r] Retry",
        AppState::Scanning => "Scanning files...",
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    frame.render_widget(help, area);
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
