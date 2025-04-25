#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
