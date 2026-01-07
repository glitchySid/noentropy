use crate::cli::Args;
use crate::files::{FileBatch, execute_move, is_text_file, read_file_sample};
use crate::gemini::GeminiClient;
use crate::models::OrganizationPlan;
use crate::settings::Config;
use crate::storage::{Cache, UndoLog};
use colored::*;
use futures::future::join_all;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

/// Validates that a path exists and is a readable directory
/// Returns the canonicalized path if validation succeeds
fn validate_and_normalize_path(path: &PathBuf) -> Result<PathBuf, String> {
    if !path.exists() {
        return Err(format!("Path '{}' does not exist", path.display()));
    }

    if !path.is_dir() {
        return Err(format!("Path '{}' is not a directory", path.display()));
    }

    // Check if we can read the directory
    match fs::read_dir(path) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!(
                "Cannot access directory '{}': {}",
                path.display(),
                e
            ));
        }
    }

    // Normalize the path to resolve ., .., and symlinks
    match path.canonicalize() {
        Ok(canonical) => Ok(canonical),
        Err(e) => Err(format!(
            "Failed to normalize path '{}': {}",
            path.display(),
            e
        )),
    }
}

pub fn handle_gemini_error(error: crate::gemini::GeminiError) {
    use colored::*;

    match error {
        crate::gemini::GeminiError::RateLimitExceeded { retry_after } => {
            println!(
                "{} API rate limit exceeded. Please wait {} seconds before trying again.",
                "ERROR:".red(),
                retry_after
            );
        }
        crate::gemini::GeminiError::QuotaExceeded { limit } => {
            println!(
                "{} Quota exceeded: {}. Please check your Gemini API usage.",
                "ERROR:".red(),
                limit
            );
        }
        crate::gemini::GeminiError::ModelNotFound { model } => {
            println!(
                "{} Model '{}' not found. Please check the model name in the configuration.",
                "ERROR:".red(),
                model
            );
        }
        crate::gemini::GeminiError::InvalidApiKey => {
            println!(
                "{} Invalid API key. Please check your GEMINI_API_KEY environment variable.",
                "ERROR:".red()
            );
        }
        crate::gemini::GeminiError::ContentPolicyViolation { reason } => {
            println!("{} Content policy violation: {}", "ERROR:".red(), reason);
        }
        crate::gemini::GeminiError::ServiceUnavailable { reason } => {
            println!(
                "{} Gemini service is temporarily unavailable: {}",
                "ERROR:".red(),
                reason
            );
        }
        crate::gemini::GeminiError::NetworkError(e) => {
            println!("{} Network error: {}", "ERROR:".red(), e);
        }
        crate::gemini::GeminiError::Timeout { seconds } => {
            println!(
                "{} Request timed out after {} seconds.",
                "ERROR:".red(),
                seconds
            );
        }
        crate::gemini::GeminiError::InvalidRequest { details } => {
            println!("{} Invalid request: {}", "ERROR:".red(), details);
        }
        crate::gemini::GeminiError::ApiError { status, message } => {
            println!(
                "{} API error (HTTP {}): {}",
                "ERROR:".red(),
                status,
                message
            );
        }
        crate::gemini::GeminiError::InvalidResponse(msg) => {
            println!("{} Invalid response from Gemini: {}", "ERROR:".red(), msg);
        }
        crate::gemini::GeminiError::InternalError { details } => {
            println!("{} Internal server error: {}", "ERROR:".red(), details);
        }
        crate::gemini::GeminiError::SerializationError(e) => {
            println!("{} JSON serialization error: {}", "ERROR:".red(), e);
        }
    }

    println!("\n{} Check the following:", "HINT:".yellow());
    println!("  • Your GEMINI_API_KEY is correctly set");
    println!("  • Your internet connection is working");
    println!("  • Gemini API service is available");
    println!("  • You haven't exceeded your API quota");
}

pub async fn handle_organization(
    args: Args,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let client: GeminiClient = GeminiClient::new(config.api_key, config.categories.clone());

    let data_dir = Config::get_data_dir()?;
    let cache_path = data_dir.join(".noentropy_cache.json");
    let mut cache = Cache::load_or_create(&cache_path);

    const CACHE_RETENTION_SECONDS: u64 = 7 * 24 * 60 * 60; // 7 days
    const UNDO_LOG_RETENTION_SECONDS: u64 = 30 * 24 * 60 * 60; // 30 days

    cache.cleanup_old_entries(CACHE_RETENTION_SECONDS);

    let undo_log_path = Config::get_undo_log_path()?;
    let mut undo_log = UndoLog::load_or_create(&undo_log_path);
    undo_log.cleanup_old_entries(UNDO_LOG_RETENTION_SECONDS);

    // Use custom path if provided, otherwise fall back to configured download folder
    let target_path = args.path.unwrap_or(config.download_folder);

    // Validate and normalize the target path early
    let target_path = match validate_and_normalize_path(&target_path) {
        Ok(normalized) => normalized,
        Err(e) => {
            println!("{}", format!("ERROR: {}", e).red());
            return Ok(());
        }
    };

    let batch = FileBatch::from_path(target_path.clone(), args.recursive);

    if batch.filenames.is_empty() {
        println!("{}", "No files found to organize!".yellow());
        return Ok(());
    }

    println!(
        "Found {} files. Asking Gemini to organize...",
        batch.count()
    );

    let mut plan: OrganizationPlan = match client
        .organize_files_in_batches(batch.filenames, Some(&mut cache), Some(&target_path))
        .await
    {
        Ok(plan) => plan,
        Err(e) => {
            handle_gemini_error(e);
            return Ok(());
        }
    };

    println!(
        "{}",
        "Gemini Plan received! Performing deep inspection...".green()
    );

    let client_arc: Arc<GeminiClient> = Arc::new(client);
    let semaphore: Arc<tokio::sync::Semaphore> =
        Arc::new(tokio::sync::Semaphore::new(args.max_concurrent));

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

    if args.dry_run {
        println!("{} Dry run mode - skipping file moves.", "INFO:".cyan());
    } else {
        execute_move(&target_path, plan, Some(&mut undo_log));
    }
    println!("{}", "Done!".green().bold());

    if let Err(e) = cache.save(cache_path.as_path()) {
        eprintln!("Warning: Failed to save cache: {}", e);
    }

    if let Err(e) = undo_log.save(&undo_log_path) {
        eprintln!("Warning: Failed to save undo log: {}", e);
    }

    Ok(())
}

pub async fn handle_undo(
    args: Args,
    download_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let undo_log_path = Config::get_undo_log_path()?;

    if !undo_log_path.exists() {
        println!("{}", "No undo log found. Nothing to undo.".yellow());
        return Ok(());
    }

    let mut undo_log = UndoLog::load_or_create(&undo_log_path);

    if !undo_log.has_completed_moves() {
        println!("{}", "No completed moves to undo.".yellow());
        return Ok(());
    }

    // Use custom path if provided, otherwise use the configured download path
    let target_path = args.path.unwrap_or(download_path);

    // Validate and normalize the target path early
    let target_path = match validate_and_normalize_path(&target_path) {
        Ok(normalized) => normalized,
        Err(e) => {
            println!("{}", format!("ERROR: {}", e).red());
            return Ok(());
        }
    };

    crate::files::undo_moves(&target_path, &mut undo_log, args.dry_run)?;

    if let Err(e) = undo_log.save(&undo_log_path) {
        eprintln!(
            "{}", 
            format!(
                "WARNING: Failed to save undo log to '{}': {}. Your undo history may be incomplete.",
                undo_log_path.display(),
                e
            ).yellow()
        );
    }

    Ok(())
}
