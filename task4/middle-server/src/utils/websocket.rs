use axum::extract::ws::{Message, WebSocket};

use crate::{
    error::{app::AppError, ws::WebSocketError},
    middleware::socket,
    utils::args,
};

pub async fn websocket_processing(mut socket: WebSocket) -> Result<(), AppError> {
    while let Some(message) = socket.recv().await {
        // Receive a message from the client
        match message {
            Ok(message) => {
                match message {
                    Message::Text(text) => {
                        // create args_set
                        let args_set = args::create_args_set(text)?;
                        //* send stat result to client *//
                        socket::socket(args_set, &mut socket).await?;
                    }
                    Message::Binary(binary) => {
                        tracing::error!("Received binary: {:?}", binary);
                        return Err(WebSocketError::UnexpectedMessagetypeError(
                            "Bynary".to_string(),
                        )
                        .into());
                    }
                    Message::Ping(ping) => {
                        tracing::error!("Received ping: {:?}", ping);
                        return Err(
                            WebSocketError::UnexpectedMessagetypeError("Ping".to_string()).into(),
                        );
                    }
                    Message::Pong(pong) => {
                        tracing::error!("Received pong: {:?}", pong);
                        return Err(
                            WebSocketError::UnexpectedMessagetypeError("Pong".to_string()).into(),
                        );
                    }
                    Message::Close(close) => {
                        tracing::info!("Client disconnected: {:?}", close);
                        return Ok(());
                    }
                }
            }
            Err(errer) => {
                tracing::error!("Error receiving message: {}", errer);
                return Err(WebSocketError::from(errer).into());
            }
        }
    }
    Ok(())
}
