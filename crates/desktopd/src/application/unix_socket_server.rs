use axum::extract::connect_info;
use axum::Router;
use futures::ready;
use hyper::server::accept::Accept;
use log::info;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::net::unix::UCred;
use tokio::net::{UnixListener, UnixStream};
use utils::constants::UNIX_SOCKET_NAME;

pub async fn build_unix_socket_sever(routes: Router) -> Result<(), Box<dyn Error>> {
    let runtime = utils::env::get_xdg_runtime();
    let path = format!("{runtime}/{UNIX_SOCKET_NAME}");
    info!("Binding to {path}");

    let path = PathBuf::from(path);
    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    let uds = UnixListener::bind(path.clone()).unwrap();

    hyper::Server::builder(ServerAccept { uds })
        .serve(routes.into_make_service_with_connect_info::<UdsConnectInfo>())
        .await?;

    Ok(())
}

struct ServerAccept {
    uds: UnixListener,
}

impl Accept for ServerAccept {
    type Conn = UnixStream;
    type Error = tower::BoxError;

    fn poll_accept(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<core::result::Result<Self::Conn, Self::Error>>> {
        let (stream, _addr) = ready!(self.uds.poll_accept(cx))?;
        Poll::Ready(Some(Ok(stream)))
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct UdsConnectInfo {
    peer_addr: Arc<tokio::net::unix::SocketAddr>,
    peer_cred: UCred,
}

impl connect_info::Connected<&UnixStream> for UdsConnectInfo {
    fn connect_info(target: &UnixStream) -> Self {
        let peer_addr = target.peer_addr().unwrap();
        let peer_cred = target.peer_cred().unwrap();

        Self {
            peer_addr: Arc::new(peer_addr),
            peer_cred,
        }
    }
}
