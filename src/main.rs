use colored::*;
use futures::future::join_all;
use noentropy::cache::Cache;
use noentropy::files::{FileBatch, OrganizationPlan, execute_move};
use noentropy::gemini::GeminiClient;
use noentropy::gemini_errors::GeminiError;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GEMINI_API_KEY")
        .map_err(|_| "GEMINI_API_KEY environment variable not set. Please set it in your .env file.")?;
    let download_path_var = std::env::var("DOWNLOAD_FOLDER")
        .map_err(|_| "DOWNLOAD_FOLDER environment variable not set. Please set it in your .env file.")?;

    // 1. Setup
    let download_path: PathBuf = PathBuf::from(download_path_var.to_string());
    let client: GeminiClient = GeminiClient::new(api_key);
    
    // Initialize cache
    let cache_path = Path::new(".noentropy_cache.json");
    let mut cache = Cache::load_or_create(cache_path);
    
    // Clean up old cache entries (older than 7 days)
    cache.cleanup_old_entries(7 * 24 * 60 * 60);

    // 2. Get Files
    let batch = FileBatch::from_path(download_path.clone());

    if batch.filenames.is_empty() {
        println!("No files found to organize!");
        return Ok(());
    }

    println!(
        "Found {} files. Asking Gemini to organize...",
        batch.count()
    );

    // 3. Call Gemini for Initial Categorization
    let mut plan: OrganizationPlan = match client
        .organize_files_with_cache(batch.filenames, Some(&mut cache), Some(&download_path))
        .await
    {
        Ok(plan) => plan,
        Err(e) => {
            handle_gemini_error(e);
            return Ok(());
        }
    };

    println!("Gemini Plan received! Performing deep inspection...");

    // 4. Deep Inspection - Process files concurrently
    let client = Arc::new(client);
    
    let tasks: Vec<_> = plan.files.iter_mut()
        .zip(batch.paths.iter())
        .map(|(file_category, path)| {
            let client = Arc::clone(&client);
            let filename = file_category.filename.clone();
            let category = file_category.category.clone();
            let path = path.clone();
            
            async move {
                if noentropy::files::is_text_file(&path) {
                    if let Some(content) = noentropy::files::read_file_sample(&path, 2000) {
                        println!("Reading content of {}...", filename.green());
                        client.get_ai_sub_category(&filename, &category, &content).await
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
        })
        .collect();

    // Wait for all concurrent tasks to complete
    let sub_categories = join_all(tasks).await;
    
    // Apply the results back to the plan
    for (file_category, sub_category) in plan.files.iter_mut().zip(sub_categories) {
        file_category.sub_category = sub_category;
    }

    println!("Deep inspection complete! Moving Files.....");
    // 5. Execute
    execute_move(&download_path, plan);
    println!("Done!");
    
    // Save cache before exiting
    if let Err(e) = cache.save(cache_path) {
        println!("Warning: Failed to save cache: {}", e);
    }

    Ok(())
}

fn handle_gemini_error(error: GeminiError) {
    use colored::*;
    
    match error {
        GeminiError::RateLimitExceeded { retry_after } => {
            println!("{} API rate limit exceeded. Please wait {} seconds before trying again.", 
                "ERROR:".red(), retry_after);
        }
        GeminiError::QuotaExceeded { limit } => {
            println!("{} Quota exceeded: {}. Please check your Gemini API usage.", 
                "ERROR:".red(), limit);
        }
        GeminiError::ModelNotFound { model } => {
            println!("{} Model '{}' not found. Please check the model name in the configuration.", 
                "ERROR:".red(), model);
        }
        GeminiError::InvalidApiKey => {
            println!("{} Invalid API key. Please check your GEMINI_API_KEY environment variable.", 
                "ERROR:".red());
        }
        GeminiError::ContentPolicyViolation { reason } => {
            println!("{} Content policy violation: {}", 
                "ERROR:".red(), reason);
        }
        GeminiError::ServiceUnavailable { reason } => {
            println!("{} Gemini service is temporarily unavailable: {}", 
                "ERROR:".red(), reason);
        }
        GeminiError::NetworkError(e) => {
            println!("{} Network error: {}", 
                "ERROR:".red(), e);
        }
        GeminiError::Timeout { seconds } => {
            println!("{} Request timed out after {} seconds.", 
                "ERROR:".red(), seconds);
        }
        GeminiError::InvalidRequest { details } => {
            println!("{} Invalid request: {}", 
                "ERROR:".red(), details);
        }
        GeminiError::ApiError { status, message } => {
            println!("{} API error (HTTP {}): {}", 
                "ERROR:".red(), status, message);
        }
        GeminiError::InvalidResponse(msg) => {
            println!("{} Invalid response from Gemini: {}", 
                "ERROR:".red(), msg);
        }
        GeminiError::InternalError { details } => {
            println!("{} Internal server error: {}", 
                "ERROR:".red(), details);
        }
        GeminiError::SerializationError(e) => {
            println!("{} JSON serialization error: {}", 
                "ERROR:".red(), e);
        }
    }
    
    println!("\n{} Check the following:", "HINT:".yellow());
    println!("  • Your GEMINI_API_KEY is correctly set");
    println!("  • Your internet connection is working");
    println!("  • Gemini API service is available");
    println!("  • You haven't exceeded your API quota");
}
