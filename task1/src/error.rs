#[derive(Debug, thiserror::Error)]
pub enum GenerateMockError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("OrderingError: Failed to sort the array.")]
    OrderingError,
}
