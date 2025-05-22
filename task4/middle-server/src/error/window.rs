use super::app::AppError;
use axum::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("{0}")]
    ParseArgsError(String),
    #[error("cannot get WindowType::Time(f64) value from get_window_count_value method.")]
    GetWindowTypeCountValueError,
    #[error("cannot get WindowType::Count(u64) value from get_window_time_value method.")]
    GetWindowTypeTimeValueError,
    #[error("cannot get CountType::Time(f64) value from get_slide_type_count_value method.")]
    GetSlideTypeCountValueError,
    #[error("cannot get SlideType::Count(u64) value from get_slide_time_value method.")]
    GetSlideTypeTimeValueError,
    #[error(transparent)]
    ParseTimeStampError(#[from] chrono::ParseError),
    #[error("{0} is a invalid value.")]
    StockEnumParseError(String),
    #[error("max value is not found.")]
    MaxValueNotFoundError,
    #[error("min value is not found.")]
    MinValueNotFoundError,
    #[error("push failed.")]
    PushFailedError,
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

impl From<WindowError> for AppError {
    fn from(error: WindowError) -> Self {
        match error {
            WindowError::IoError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("IoError: {}", e),
            },
            WindowError::Infallible(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Infallible: {}", e),
            },
            WindowError::ParseIntError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("ParseIntError: {}", e),
            },
            WindowError::ParseFloatError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("ParseFloatError: {}", e),
            },
            WindowError::ParseArgsError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("ParseArgsError: {}", e),
            },
            WindowError::GetWindowTypeCountValueError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "GetWindowTypeCountValueError: cannot get WindowType::Time(f64) value from get_window_count_value method.".to_string(),
            },
            WindowError::GetWindowTypeTimeValueError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "GetWindowTypeTimeValueError: cannot get WindowType::Count(u64) value from get_window_time_value method.".to_string(),
            },
            WindowError::GetSlideTypeCountValueError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "GetSlideTypeCountValueError: cannot get CountType::Time(f64) value from get_slide_type_count_value method.".to_string(),
            },
            WindowError::GetSlideTypeTimeValueError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "GetSlideTypeTimeValueError: cannot get SlideType::Count(u64) value from get_slide_time_value method.".to_string(),
            },
            WindowError::ParseTimeStampError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("ParseTimeStampError: {}", e),
            },
            WindowError::StockEnumParseError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("StockEnumParseError: {} is a invalid value.", e),
            },
            WindowError::MaxValueNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "MaxValueNotFoundError: max value is not found.".to_string(),
            },
            WindowError::MinValueNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "MinValueNotFoundError: min value is not found.".to_string(),
            },
            WindowError::PushFailedError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "PushFailedError: push failed.".to_string(),
            },
            WindowError::AxumError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("AxumError: {}", e),
            },
            WindowError::SerdeJsonError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("SerdeJsonError: {}", e),
            },
        }
    }
}
