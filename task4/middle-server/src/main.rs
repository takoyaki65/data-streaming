use axum::{Router, extract::DefaultBodyLimit, routing::get};
use error::ws::WebSocketError;
use handlers::{count_window::count_handler, time_window::time_handler};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

pub mod error;
pub mod handlers;
pub mod middleware;
pub mod model;
pub mod stat;
pub mod utils;
pub mod window;

// Domain
const IPADRESS: &str = "127.0.0.1";
const PORT: u16 = 7000;

#[tokio::main]
async fn main() -> Result<(), WebSocketError> {
    // shared object
    let shared_state = Arc::new(RwLock::new(0));
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // cors
    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    // router
    let app: Router<()> = Router::new()
        .route("/time", get(time_handler))
        .route("/count", get(count_handler))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)) //100MB
        .with_state(Arc::clone(&shared_state));

    // server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", IPADRESS, PORT)).await?;

    //* start server *//
    tracing::info!("listening on ws://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
