use crate::cli::Args;
use crate::cli::Command;
use crate::cli::handlers::{handle_offline_organization, handle_online_organization};
use crate::cli::path_utils::validate_and_normalize_path;
use crate::error::Result;
use crate::files::FileBatch;
use crate::gemini::GeminiClient;
use crate::settings::{Config, Prompter};
use crate::storage::{Cache, UndoLog};
use colored::*;

fn initialize_cache() -> Result<(Cache, std::path::PathBuf)> {
    const CACHE_RETENTION_SECONDS: u64 = 7 * 24 * 60 * 60;
    let data_dir = Config::get_data_dir()?;
    let cache_path = data_dir.join(".noentropy_cache.json");
    let mut cache = Cache::load_or_create(&cache_path, false);
    cache.cleanup_old_entries(CACHE_RETENTION_SECONDS);
    Ok((cache, cache_path))
}

fn initialize_undo_log() -> Result<(UndoLog, std::path::PathBuf)> {
    const UNDO_LOG_RETENTION_SECONDS: u64 = 30 * 24 * 60 * 60;
    let undo_log_path = Config::get_undo_log_path()?;
    let mut undo_log = UndoLog::load_or_create(&undo_log_path, false);
    undo_log.cleanup_old_entries(UNDO_LOG_RETENTION_SECONDS);
    Ok((undo_log, undo_log_path))
}

async fn resolve_target_path(args: &Args, config: &Config) -> Option<std::path::PathBuf> {
    let target_path = match &args.command {
        Some(Command::Organize { path, .. }) => path
            .as_ref()
            .cloned()
            .unwrap_or_else(|| config.download_folder.clone()),
        Some(Command::Undo { path, .. }) => path
            .as_ref()
            .cloned()
            .unwrap_or_else(|| config.download_folder.clone()),
        _ => config.download_folder.clone(),
    };

    match validate_and_normalize_path(&target_path).await {
        Ok(normalized) => Some(normalized),
        Err(e) => {
            println!("{}", format!("ERROR: {}", e).red());
            None
        }
    }
}

async fn determine_offline_mode(args: &Args, config: &Config) -> Option<bool> {
    // Check CLI flags first
    let cli_online = match &args.command {
        Some(Command::Organize { online, .. }) => *online,
        _ => args.online,
    };

    let cli_offline = match &args.command {
        Some(Command::Organize { offline, .. }) => *offline,
        _ => args.offline,
    };

    // Determine mode preference
    let prefer_online = config.should_use_online_mode(cli_online, cli_offline);

    if cli_offline {
        println!("{}", "Using offline mode (--offline flag).".cyan());
        return Some(true);
    }

    // Offline-first: default to offline unless online explicitly requested
    if !prefer_online {
        println!(
            "{}",
            "Using offline mode (offline-first). Use --online to enable AI categorization.".cyan()
        );
        return Some(true);
    }

    // Online mode requested - validate API connectivity
    println!("{}", "Checking online mode availability...".cyan());
    let client = GeminiClient::new(&config.api_key, &config.categories);
    match client.check_connectivity().await {
        Ok(()) => {
            println!("{}", "Online mode enabled.".green());
            Some(false)
        }
        Err(e) => {
            println!("{}", format!("Online mode unavailable: {}", e).yellow());
            if Prompter::prompt_offline_mode(&e.to_string()) {
                Some(true)
            } else {
                println!("{}", "Exiting.".yellow());
                None
            }
        }
    }
}

pub async fn handle_organization(args: Args, config: Config) -> Result<()> {
    let (mut cache, cache_path) = initialize_cache()?;
    let (mut undo_log, undo_log_path) = initialize_undo_log()?;

    let Some(target_path) = resolve_target_path(&args, &config).await else {
        return Ok(());
    };

    let (batch, dry_run) = match &args.command {
        Some(Command::Organize {
            recursive, dry_run, ..
        }) => (FileBatch::from_path(&target_path, *recursive), *dry_run),
        _ => unreachable!(),
    };

    if batch.filenames.is_empty() {
        println!("{}", "No files found to organize!".yellow());
        return Ok(());
    }

    println!("Found {} files to organize.", batch.count());

    let Some(use_offline) = determine_offline_mode(&args, &config).await else {
        return Ok(());
    };

    let plan = if use_offline {
        handle_offline_organization(batch, &target_path, dry_run, &mut undo_log)?
    } else {
        handle_online_organization(
            args.command.as_ref().unwrap(),
            &config,
            batch,
            &target_path,
            &mut cache,
            &mut undo_log,
        )
        .await?
    };

    if let Err(e) = cache.save(&cache_path)
        && plan.is_none()
    {
        eprintln!("Warning: Failed to save cache: {}", e);
    }

    if let Err(e) = undo_log.save(&undo_log_path) {
        eprintln!("Warning: Failed to save undo log: {}", e);
    }

    Ok(())
}
