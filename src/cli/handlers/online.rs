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

fn get_deep_inspect_flags(command: &Command) -> (bool, bool) {
    match command {
        Command::Organize {
            skip_deep_inspect,
            no_skip_deep_inspect,
            ..
        } => (*skip_deep_inspect, *no_skip_deep_inspect),
        _ => (false, false),
    }
}

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
    
    let (skip_flag, no_skip_flag) = get_deep_inspect_flags(command);
    let should_deep_inspect = config.should_deep_inspect(skip_flag, no_skip_flag);
    
    let paths = batch.paths.clone();

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

    if should_deep_inspect {
        perform_deep_inspection(&mut plan, &paths, &client, max_concurrent).await;
    }

    println!("{}", "Moving Files.....".green());

    if dry_run {
        println!("{} Dry run mode - skipping file moves.", "INFO:".cyan());
    } else {
        execute_move(target_path, plan, Some(undo_log));
    }
    println!("{}", "Done!".green().bold());

    Ok(None)
}

async fn perform_deep_inspection(
    plan: &mut OrganizationPlan,
    paths: &[PathBuf],
    client: &GeminiClient,
    max_concurrent: usize,
) {
    println!("{}", "Gemini Plan received! Performing deep inspection...".green());

    let client_arc = Arc::new(client.clone());
    let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));

    let tasks: Vec<_> = plan
        .files
        .iter_mut()
        .zip(paths.iter())
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
        })
        .collect();

    let sub_categories: Vec<String> = join_all(tasks).await;

    for (file_category, sub_category) in plan.files.iter_mut().zip(sub_categories) {
        file_category.sub_category = sub_category;
    }

    println!("{}", "Deep inspection complete!".green());
}
