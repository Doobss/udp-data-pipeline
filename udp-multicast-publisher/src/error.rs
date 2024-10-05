pub type PublisherResult<T> = Result<T, PublisherError>;

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error(transparent)]
    TokioError(#[from] tokio::io::Error),
}
