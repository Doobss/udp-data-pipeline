pub type UdpPipelineResult<T> = Result<T, UdpPipelineError>;

#[derive(Debug, thiserror::Error)]
pub enum UdpPipelineError {
    #[error(transparent)]
    MessageError(#[from] crate::messages::MessageError),
}
