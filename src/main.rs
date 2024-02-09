use std::sync::Arc;

use axum::{routing::get, Router};
use state::LocalChatState;
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::services::ServeDir;
use tracing::{info, Level};
use tracing_subscriber::{filter::Targets, prelude::*};

mod handlers {
    pub mod pages;
    pub mod rest;
    pub mod ws;
}
mod chat_message;
mod state;

#[derive(Debug)]
enum AppError {
    InvalidBindAddress(std::io::Error),
    ServerNotStarting(std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::registry()
        .with(
            Targets::new()
                .with_default(Level::ERROR)
                .with_target("axum_htmx", Level::DEBUG),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (tx, _) = broadcast::channel(100);
    let app_state = Arc::new(LocalChatState::new(Vec::new(), tx));

    let app = Router::new()
        .route("/", get(handlers::pages::index))
        .route("/chat", get(handlers::ws::upgrade_handler))
        .route("/history", get(handlers::rest::chat_history))
        .with_state(app_state)
        .nest_service("/assets", ServeDir::new("build"));

    let host = std::env::var("APP_HOST").unwrap_or("0.0.0.0".into());
    let port = std::env::var("APP_PORT").unwrap_or("3000".into());
    let addr = format!("{}:{}", host, port);
    info!("listening on {}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .map_err(AppError::InvalidBindAddress)?;

    axum::serve(listener, app)
        .await
        .map_err(AppError::ServerNotStarting)
}
