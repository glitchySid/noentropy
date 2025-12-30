use crate::cli::Args;
use crate::files::{FileBatch, execute_move, is_text_file, read_file_sample};
use crate::gemini::GeminiClient;
use crate::models::OrganizationPlan;
use crate::settings::Config;
use crate::storage::{Cache, UndoLog};
use colored::*;
use futures::future::join_all;
use std::path::PathBuf;
use std::sync::Arc;

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
    api_key: String,
    download_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let client: GeminiClient = GeminiClient::new(api_key);

    let mut cache_path = std::env::var("HOME")
        .map(PathBuf::from)
        .expect("No Home found");
    cache_path.push(".config/noentropy/data/.noentropy_cache.json");
    let mut cache = Cache::load_or_create(cache_path.as_path());

    cache.cleanup_old_entries(7 * 24 * 60 * 60);

    let undo_log_path = Config::get_undo_log_path()?;
    let mut undo_log = UndoLog::load_or_create(&undo_log_path);
    undo_log.cleanup_old_entries(30 * 24 * 60 * 60);

    let batch = FileBatch::from_path(download_path.clone(), args.recursive);

    if batch.filenames.is_empty() {
        println!("{}", "No files found to organize!".yellow());
        return Ok(());
    }

    println!(
        "Found {} files. Asking Gemini to organize...",
        batch.count()
    );

    let mut plan: OrganizationPlan = match client
        .organize_files_in_batches(batch.filenames, Some(&mut cache), Some(&download_path))
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
        execute_move(&download_path, plan, Some(&mut undo_log));
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

    crate::files::undo_moves(&download_path, &mut undo_log, args.dry_run)?;

    if let Err(e) = undo_log.save(&undo_log_path) {
        eprintln!("Warning: Failed to save undo log: {}", e);
    }

    Ok(())
}
