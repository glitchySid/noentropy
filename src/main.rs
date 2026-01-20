use clap::Parser;
use noentropy::cli::{Args, Command, handle_organization, handle_undo};
use noentropy::files::duplicate::execute_delete;
use noentropy::settings::config::change_and_prompt_api_key;
use noentropy::settings::{get_or_prompt_config, get_or_prompt_download_folder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match &args.command {
        Command::Organize { .. } => {
            let config = get_or_prompt_config()?;
            handle_organization(args, config).await?;
        }
        Command::Undo { .. } => {
            let download_path = get_or_prompt_download_folder()?;
            handle_undo(&args.command, download_path).await?;
        }
        Command::ChangeKey => {
            change_and_prompt_api_key()?;
        }
        Command::Duplicates { recursive } => {
            execute_delete(*recursive);
        }
    } else if args.duplicate {
        execute_delete(args.recursive);
    } else {
        let config = get_or_prompt_config()?;
        handle_organization(args, config).await?;
    }

    Ok(())
}
