use clap::Parser;
use noentropy::cli::{
    Args,
    orchestrator::{handle_organization, handle_undo},
};
use noentropy::settings::{get_or_prompt_api_key, get_or_prompt_download_folder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.undo {
        let download_path = get_or_prompt_download_folder()?;
        handle_undo(args, download_path).await?;
        return Ok(());
    }

    let api_key = get_or_prompt_api_key()?;
    let download_path = get_or_prompt_download_folder()?;

    handle_organization(args, api_key, download_path).await?;

    Ok(())
}
