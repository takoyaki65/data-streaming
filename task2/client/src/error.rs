#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    ParseTimeStampError(#[from] chrono::ParseError),
    #[error("StockEnumParseError: {0} is a invalid value.")]
    StockEnumParseError(String),
    #[error("MaxValueNotFoundError: max value is not found.")]
    MaxValueNotFoundError,
    #[error("MinValueNotFoundError: min value is not found.")]
    MinValueNotFoundError,
}
