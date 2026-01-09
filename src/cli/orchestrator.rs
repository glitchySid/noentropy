use crate::cli::Args;
use crate::cli::handlers::{handle_offline_organization, handle_online_organization};
use crate::cli::path_utils::validate_and_normalize_path;
use crate::files::FileBatch;
use crate::gemini::GeminiClient;
use crate::settings::{Config, Prompter};
use crate::storage::{Cache, UndoLog};
use colored::*;

/// Main entry point for file organization.
/// Coordinates cache, undo log, and delegates to online/offline handlers.
pub async fn handle_organization(
    args: Args,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let target_path = args
        .path
        .as_ref()
        .cloned()
        .unwrap_or_else(|| config.download_folder.clone());

    // Validate and normalize the target path early
    let target_path = match validate_and_normalize_path(&target_path).await {
        Ok(normalized) => normalized,
        Err(e) => {
            println!("{}", format!("ERROR: {}", e).red());
            return Ok(());
        }
    };

    let batch = FileBatch::from_path(&target_path, args.recursive);

    if batch.filenames.is_empty() {
        println!("{}", "No files found to organize!".yellow());
        return Ok(());
    }

    println!("Found {} files to organize.", batch.count());

    // Determine if we should use offline mode
    let use_offline = if args.offline {
        println!("{}", "Using offline mode (--offline flag).".cyan());
        true
    } else {
        let client = GeminiClient::new(&config.api_key, &config.categories);
        match client.check_connectivity().await {
            Ok(()) => false,
            Err(e) => {
                if Prompter::prompt_offline_mode(&e.to_string()) {
                    true
                } else {
                    println!("{}", "Exiting.".yellow());
                    return Ok(());
                }
            }
        }
    };

    let plan = if use_offline {
        handle_offline_organization(batch, &target_path, args.dry_run, &mut undo_log)?
    } else {
        handle_online_organization(
            &args,
            &config,
            batch,
            &target_path,
            &mut cache,
            &mut undo_log,
        )
        .await?
    };

    // Only save if we have a plan (online mode returns None after moving)
    if plan.is_none()
        && let Err(e) = cache.save(cache_path.as_path())
    {
        eprintln!("Warning: Failed to save cache: {}", e);
    }

    if let Err(e) = undo_log.save(&undo_log_path) {
        eprintln!("Warning: Failed to save undo log: {}", e);
    }

    Ok(())
}
