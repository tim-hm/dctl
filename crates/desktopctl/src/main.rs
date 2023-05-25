use std::error::Error;

use clap::Parser;
use log::debug;

mod arg_list;
mod infrastructure;
mod theme;

use arg_list::{ArgsList, Commands, ThemeCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    utils::log::init();
    utils::env::init();

    let args = ArgsList::parse();
    debug!("args: {:?}", args);

    match args.command {
        Commands::Theme(command) => match command {
            ThemeCommand::Toggle => theme::toggle_theme().await?,
        },
        _ => {
            debug!("Unknown command: {:?}", &args.command)
        }
    }

    Ok(())
}
