use std::path::PathBuf;

use noentropy::files::FileBatch;
use noentropy::files::OrganizationPlan;
use noentropy::files::execute_move;
use noentropy::gemini::GeminiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GEMINI_API_KEY").expect("KEY not set");
    let download_path_var = std::env::var("DOWNLOAD_FOLDER").expect("Set DOWNLOAD_FOLDER={path}");

    // 1. Setup
    let download_path: PathBuf = PathBuf::from(download_path_var.to_string());
    let client: GeminiClient = GeminiClient::new(api_key);

    // 2. Get Files (Using your previous FileBatch logic)
    // Assuming FileBatch::from_path returns a struct with .filenames
    let batch = FileBatch::from_path(download_path.clone());

    if batch.filenames.is_empty() {
        println!("No files found to organize!");
        return Ok(());
    }

    println!(
        "Found {} files. Asking Gemini to organize...",
        batch.filenames.len()
    );

    // 3. Call Gemini
    let plan: OrganizationPlan = client.organize_files(batch.filenames).await?;

    println!("Gemini Plan received! Moving files...");

    // 4. Execute
    execute_move(&download_path, plan);

    println!("Done!");
    Ok(())
}
