mod adapters;
mod application;
mod domain;
mod ports;

use std::{net::SocketAddr, sync::Arc};

use adapters::inbound::http::{router, AppState};
use adapters::outbound::memory::InMemoryGreetingRepository;
use application::greeting::GreetingService;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let host = std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("APP_PORT").unwrap_or_else(|_| "3000".into());
    let address: SocketAddr = format!("{host}:{port}").parse()?;

    let repository = InMemoryGreetingRepository::default();
    let service = Arc::new(GreetingService::new(repository));
    let app = router(AppState { service });
    let listener = TcpListener::bind(address).await?;

    info!(%address, "example app listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

