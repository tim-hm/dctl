mod application;
mod theme;

use std::error::Error;

use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    utils::env::init();
    utils::log::init();
    utils::env::print_env_if_dev();

    tokio::select! {
        reason = application::start() => {
            info!("Server finished: {:?}", reason)
        },
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl+c received")
        },
    }
    info!("Goodbye 👋");

    Ok(())
}
