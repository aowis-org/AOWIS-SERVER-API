use std::net::SocketAddr;

use axum::{Router, routing::get};
use tracing::info;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app = Router::new().route("/", get(async || "Hello, world!"));

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Started listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}
