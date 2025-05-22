use crate::{
    error::app::AppError,
    model::{args::SlidingWindowEnumType, shared_state::RwLockSharedState},
    utils::web_socket::web_socket_handler,
};
use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};

// handler
pub async fn time_handler(
    State(shared_state): State<RwLockSharedState>,
    web_socket: WebSocketUpgrade,
) -> Result<impl IntoResponse, AppError> {
    let shared_state = shared_state.read().await;
    let response = web_socket.on_upgrade(|socket| async move {
        if let Err(error) = web_socket_handler(socket, SlidingWindowEnumType::Time).await {
            tracing::error!("WebSocket error: {:?}", error);
        }
    });
    drop(shared_state);
    Ok(response)
}
