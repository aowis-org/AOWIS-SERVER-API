mod app;
mod config;
mod dto;
mod error;
mod handlers;
mod middleware;
mod routes;
mod state;

use std::process::exit;

use anyhow::Context;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

use crate::{app::app_router, config::Config};

/// Entry point to catch errors and correctly log them
#[tokio::main]
async fn main() {
    // Setup .env loading
    if let Err(err) = dotenv() {
        eprintln!("WARN: Failed loading .env file: {err}.\nThis is not an error");
    }

    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    if let Err(err) = run().await {
        error!("fatal server error: {err:?}");
        exit(1);
    }
}

/// Actual application entry point
async fn run() -> anyhow::Result<()> {
    let config = Config::from_env().context("Failed to load config")?;

    // Create listener
    let addr = config.addr;
    let listener = TcpListener::bind(addr)
        .await
        .context("Failed to bind listener")?;

    info!("Started listening on {addr}");

    // Get router and serve it with the created listener
    axum::serve(listener, app_router())
        .await
        .context("Failed to start web server")?;

    Ok(())
}
