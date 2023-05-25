use axum::Router;
use core::result::Result;
use std::error::Error;

use crate::application::http_server::build_http_server;
use crate::application::unix_socket_server::build_unix_socket_sever;
use crate::theme;
use utils::env::EnvName;

pub async fn start() -> Result<(), Box<dyn Error>> {
    build_server().await?;

    Ok(())
}

async fn build_server() -> Result<(), Box<dyn Error>> {
    let env_name = utils::env::get_env_name();
    let theme_router = theme::build_routes();
    let routes = Router::new().merge(theme_router);

    // TODO I'm not sure how to return the server from these builders so that
    // I can apply the graceful shutdown logic once
    // https://github.com/programatik29/axum-server/blob/master/examples/graceful_shutdown.rs
    match env_name {
        EnvName::Dev => {
            build_http_server(routes).await?;
        }
        _ => {
            build_unix_socket_sever(routes).await?;
        }
    }

    Ok(())
}
