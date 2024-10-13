use udp_data_pipeline::messages;

use crate::{PublisherConfig, PublisherResult};

pub async fn task<'a, T>(
    config: &PublisherConfig,
    message_producer: std::sync::Arc<tokio::sync::Mutex<messages::MessageProducer<T>>>,
) -> PublisherResult<tokio::task::JoinHandle<()>>
where
    T: messages::PublishedMessage + Clone + messages::ToString + Send + Sync + 'static,
{
    let PublisherConfig { address, port, .. } = &config;

    Ok(tokio::task::spawn(async move {}))
}
