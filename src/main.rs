use clap::Parser;
use noentropy::cli::{Args, Command, handle_organization, handle_undo};
use noentropy::error::Result;
use noentropy::files::duplicate::execute_delete;
use noentropy::settings::config::change_and_prompt_api_key;
use noentropy::settings::{get_or_prompt_config, get_or_prompt_download_folder};
use noentropy::tui::run_app;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting noentropy application");
    let args = Args::parse();

    match &args.command {
        Some(Command::Organize { .. }) => {
            let config = get_or_prompt_config()?;
            handle_organization(args, config).await?;
        }
        Some(Command::Undo { .. }) => {
            let download_path = get_or_prompt_download_folder()?;
            handle_undo(args.command.as_ref().unwrap(), download_path).await?;
        }
        Some(Command::ChangeKey) => {
            change_and_prompt_api_key()?;
        }
        Some(Command::Duplicates { recursive }) => {
            execute_delete(*recursive);
        }
        None => {
            // Default: Launch TUI
            let config = get_or_prompt_config()?;
            run_app(
                config,
                args.path,
                args.recursive,
                args.dry_run,
                args.offline,
            )
            .await?;
        }
    }
    Ok(())
}
