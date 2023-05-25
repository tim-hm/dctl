use crate::infrastructure;
use log::{debug, info};
use std::error::Error;

pub async fn toggle_theme() -> Result<(), Box<dyn Error>> {
    debug!("Toggling theme!");

    let client = infrastructure::get_client()?;
    let response = client.await?;

    info!("Response: {:?}", response);

    Ok(())
}
