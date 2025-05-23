use crate::{
    error::app::AppError,
    model::shared_state::RwLockSharedState,
    utils::websocket::{self},
};
use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};

// handler
pub async fn websocket_handler(
    State(shared_state): State<RwLockSharedState>,
    web_socket: WebSocketUpgrade,
) -> Result<impl IntoResponse, AppError> {
    let shared_state = shared_state.read().await;
    let response = web_socket.on_upgrade(|socket| async move {
        if let Err(error) = websocket::websocket_processing(socket).await {
            tracing::error!("WebSocket error: {:?}", error);
        }
    });
    drop(shared_state);
    Ok(response)
}
