#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
