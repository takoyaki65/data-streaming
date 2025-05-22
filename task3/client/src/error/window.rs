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
    #[error("ParseArgsError: {0}")]
    ParseArgsError(String),
    #[error(
        "GetWindowTypeCountValueError: cannot get WindowType::Time(f64) value from get_window_count_value method."
    )]
    GetWindowTypeCountValueError,
    #[error(
        "GetWindowTypeTimeValueError: cannot get WindowType::Count(u64) value from get_window_time_value method."
    )]
    GetWindowTypeTimeValueError,
    #[error(
        "GetSlideTypeCountValueError: cannot get CountType::Time(f64) value from get_slide_type_count_value method."
    )]
    GetSlideTypeCountValueError,
    #[error(
        "GetSlideTypeTimeValueError: cannot get SlideType::Count(u64) value from get_slide_time_value method."
    )]
    GetSlideTypeTimeValueError,
    #[error(transparent)]
    ParseTimeStampError(#[from] chrono::ParseError),
    #[error("StockEnumParseError: {0} is a invalid value.")]
    StockEnumParseError(String),
    #[error("MaxValueNotFoundError: max value is not found.")]
    MaxValueNotFoundError,
    #[error("MinValueNotFoundError: min value is not found.")]
    MinValueNotFoundError,
    #[error("PushFailedError: push failed.")]
    PushFailedError,
}
