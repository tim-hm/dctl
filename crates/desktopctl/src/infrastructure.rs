use hyper::{client::ResponseFuture, Client, Uri as HttpUri};
use hyperlocal::{UnixClientExt, Uri as SocketUri};
use std::{env, error::Error, str::FromStr};
use utils::env::EnvName;

pub fn get_client() -> Result<ResponseFuture, Box<dyn Error>> {
    let env_name = utils::env::get_env_name();

    match env_name {
        EnvName::Dev => {
            let client = Client::new();
            let port = utils::env::get_port();
            let host = utils::constants::DEV_ADDRESS;
            let uri = format!("http://{host}:{port}/theme/toggle");
            let uri = HttpUri::from_str(&uri)?;

            Ok(client.get(uri))
        }
        _ => {
            let client = Client::unix();
            let runtime = env::var(utils::constants::XDG_RUNTIME_DIR)?;
            let socket = utils::constants::UNIX_SOCKET_NAME;

            let uri = format!("{runtime}/{socket}");
            let uri = uri.as_str();
            let uri = SocketUri::new(uri, "/theme/toggle").into();

            Ok(client.get(uri))
        }
    }
}
