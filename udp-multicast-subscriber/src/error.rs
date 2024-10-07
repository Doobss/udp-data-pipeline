pub type SubscriberResult<T> = Result<T, SubscriberError>;

#[derive(Debug, thiserror::Error)]
pub enum SubscriberError {
    #[error(transparent)]
    TokioError(#[from] tokio::io::Error),
    #[error(transparent)]
    UdpPipelineError(#[from] udp_data_pipeline::UdpPipelineError),
    #[error(transparent)]
    UdpMessageError(#[from] udp_data_pipeline::messages::MessageError),
}
