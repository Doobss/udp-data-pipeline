pub type PublisherResult<T> = Result<T, PublisherError>;

#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error(transparent)]
    TokioError(#[from] tokio::io::Error),
    #[error(transparent)]
    UdpPipelineError(#[from] udp_data_pipeline::UdpPipelineError),
    #[error(transparent)]
    UdpMessageError(#[from] udp_data_pipeline::messages::MessageError),
}
