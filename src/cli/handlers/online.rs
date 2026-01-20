use crate::cli::Command;
use crate::cli::errors::handle_gemini_error;
use crate::files::{FileBatch, execute_move, is_text_file, read_file_sample};
use crate::gemini::GeminiClient;
use crate::models::OrganizationPlan;
use crate::settings::Config;
use crate::storage::{Cache, UndoLog};
use colored::*;
use futures::future::join_all;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Handles the online (AI-powered) organization of files.
///
/// This function uses the Gemini API to intelligently categorize files based on
/// their names and content. It supports deep inspection for text files, where the
/// AI will read file contents to suggest sub-categories.
///
/// # Arguments
/// * `command` - Command enum containing organize-specific arguments
/// * `config` - Configuration containing API key and categories
/// * `batch` - The batch of files to organize
/// * `target_path` - The target directory for organized files
/// * `cache` - Cache for storing/retrieving AI responses
/// * `undo_log` - Log for tracking file moves (for undo functionality)
///
/// # Returns
/// * `Ok(None)` - Organization completed (result printed to console)
/// * `Err(_)` - An error occurred during organization
pub async fn handle_online_organization(
    command: &Command,
    config: &Config,
    batch: FileBatch,
    target_path: &Path,
    cache: &mut Cache,
    undo_log: &mut UndoLog,
) -> Result<Option<OrganizationPlan>, Box<dyn std::error::Error>> {
    let (max_concurrent, dry_run) = match command {
        Command::Organize {
            max_concurrent,
            dry_run,
            ..
        } => (*max_concurrent, *dry_run),
        _ => unreachable!(),
    };

    let client = GeminiClient::new(&config.api_key, &config.categories);

    println!("Asking Gemini to organize...");

    let mut plan: OrganizationPlan = match client
        .organize_files_in_batches(batch.filenames, Some(cache), Some(target_path))
        .await
    {
        Ok(plan) => plan,
        Err(e) => {
            handle_gemini_error(e);
            return Ok(None);
        }
    };

    println!(
        "{}",
        "Gemini Plan received! Performing deep inspection...".green()
    );

    let client_arc: Arc<GeminiClient> = Arc::new(client);
    let semaphore: Arc<tokio::sync::Semaphore> =
        Arc::new(tokio::sync::Semaphore::new(max_concurrent));

    let tasks: Vec<_> = plan
        .files
        .iter_mut()
        .zip(batch.paths.iter())
        .map(
            |(file_category, path): (&mut crate::models::FileCategory, &PathBuf)| {
                let client: Arc<GeminiClient> = Arc::clone(&client_arc);
                let filename: String = file_category.filename.clone();
                let category: String = file_category.category.clone();
                let path: PathBuf = path.clone();
                let semaphore: Arc<tokio::sync::Semaphore> = Arc::clone(&semaphore);

                async move {
                    if is_text_file(&path) {
                        let _permit = semaphore.acquire().await.unwrap();
                        if let Some(content) = read_file_sample(&path, 5000) {
                            println!("Reading content of {}...", filename.green());
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
            },
        )
        .collect();

    let sub_categories: Vec<String> = join_all(tasks).await;

    for (file_category, sub_category) in plan.files.iter_mut().zip(sub_categories) {
        file_category.sub_category = sub_category;
    }

    println!("{}", "Deep inspection complete! Moving Files.....".green());

    if dry_run {
        println!("{} Dry run mode - skipping file moves.", "INFO:".cyan());
    } else {
        execute_move(target_path, plan, Some(undo_log));
    }
    println!("{}", "Done!".green().bold());

    Ok(None)
}
