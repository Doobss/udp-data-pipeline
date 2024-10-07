pub type MessageResult<T> = Result<T, MessageError>;

#[derive(Debug, thiserror::Error)]
pub enum MessageError {
    #[error(transparent)]
    SerializeError(#[from] serde_json::Error),
}
