use axum::http::StatusCode;

use super::app::AppError;

#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    #[error("{0}")]
    UnexpectedMessagetypeError(String),
    #[error(transparent)]
    SetGlobalDefaultError(#[from] tracing::subscriber::SetGlobalDefaultError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
}

impl From<WebSocketError> for AppError {
    fn from(error: WebSocketError) -> Self {
        match error {
            WebSocketError::UnexpectedMessagetypeError(e) => AppError {
                status_code: StatusCode::BAD_REQUEST,
                message: format!("UnexpectedMessagetypeError: {}", e),
            },
            WebSocketError::SetGlobalDefaultError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("SetGlobalDefaultError: {}", e),
            },
            WebSocketError::IoError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("IoError: {}", e),
            },
            WebSocketError::AxumError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("AxumError: {}", e),
            },
        }
    }
}
