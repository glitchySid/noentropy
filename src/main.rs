use clap::Parser;
use noentropy::cli::{Args, handle_organization, handle_undo};
use noentropy::files::duplicate::execute_delete;
use noentropy::settings::config::change_and_prompt_api_key;
use noentropy::settings::{get_or_prompt_config, get_or_prompt_download_folder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.undo {
        let download_path = get_or_prompt_download_folder()?;
        handle_undo(args, download_path).await?;
        return Ok(());
    } else if args.change_key {
        let api_key = change_and_prompt_api_key();
        match api_key {
            Ok(_key) => println!("Key saved"),
            Err(e) => {
                eprintln!("{e}")
            }
        }
    } else if args.duplicate {
        execute_delete(args.recursive);
    } else {
        let config = get_or_prompt_config()?;
        handle_organization(args, config).await?;
    }

    Ok(())
}
