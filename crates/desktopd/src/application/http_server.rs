use axum::Router;
use axum::Server;
use core::result::Result;
use log::info;
use std::error::Error;
use std::net::SocketAddr;

use utils::constants;

pub async fn build_http_server(routes: Router) -> Result<(), Box<dyn Error>> {
    let port = utils::env::get_port();
    let url = constants::DEV_ADDRESS;
    let addr = SocketAddr::new(url.parse()?, port);
    info!("Listening on {addr:?}");

    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await?;

    Ok(())
}
