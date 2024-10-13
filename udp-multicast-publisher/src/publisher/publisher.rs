use udp_data_pipeline::messages::{self, ToString};

use crate::{publisher, PublisherResult};

use super::PublisherConfig;

pub struct Publisher<T>
where
    T: messages::PublishedMessage + Clone + Send + Sync + ToString + 'static,
{
    pub message_producer: std::sync::Arc<tokio::sync::Mutex<messages::MessageProducer<T>>>,
    pub config: PublisherConfig,
}

impl<T> Publisher<T>
where
    T: messages::PublishedMessage + Clone + Send + Sync + ToString + 'static,
{
    pub fn from_config(config: PublisherConfig) -> Self {
        let message_producer = std::sync::Arc::new(tokio::sync::Mutex::new(
            messages::MessageProducer::<T>::default(),
        ));
        Self {
            message_producer,
            config,
        }
    }

    pub async fn run(self) -> PublisherResult<()> {
        let Self {
            message_producer,
            config,
        } = self;
        let multicast_publish_task =
            publisher::tasks::socket::task(&config, message_producer.clone()).await?;
        let server_task = publisher::tasks::server::task(&config, message_producer).await?;
        match tokio::try_join!(multicast_publish_task, server_task) {
            Ok(_res) => {
                tracing::info!("Publisher finished")
            }
            Err(error) => {
                tracing::error!("Error running publisher: {:?}", error)
            }
        }
        Ok(())
    }
}
