pub type SubscriberResult<T> = Result<T, SubscriberError>;

#[derive(Debug, thiserror::Error)]
pub enum SubscriberError {
    #[error(transparent)]
    TokioError(#[from] tokio::io::Error),
}
