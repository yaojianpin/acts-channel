use std::sync::atomic::{AtomicU16, Ordering};

use tokio::sync::oneshot::Receiver;
use tonic::transport::Server;

use crate::acts_service_server::ActsServiceServer;

mod act;
mod server;
mod workflow;

#[cfg(feature = "docker_test")]
pub const SERVER_ADDR: &str = "172.17.0.1";

#[cfg(not(feature = "docker_test"))]
pub const SERVER_ADDR: &str = "127.0.0.1";

async fn start_server(port: u16, rx: Receiver<()>) {
    let server = server::GrpcServer::new();
    server.init().await;
    let grpc = ActsServiceServer::new(server);
    let addr = format!("{SERVER_ADDR}:{port}").parse().unwrap();
    tokio::spawn(async move {
        Server::builder()
            .add_service(grpc)
            .serve_with_shutdown(addr, async { drop(rx.await) })
            .await
    });
}

static PORT: AtomicU16 = AtomicU16::new(10080);
fn pick_port() -> u16 {
    let port = PORT.fetch_add(1, Ordering::Relaxed);
    port + 1
}
