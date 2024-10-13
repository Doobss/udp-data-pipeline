use udp_data_pipeline::{messages, socket};

use crate::PublisherResult;

use super::PublisherConfig;

pub struct Publisher<T>
where
    T: messages::PublishedMessage + Clone + Send + Sync,
{
    pub message_producer: std::sync::Arc<messages::MessageProducer<T>>,
    pub config: PublisherConfig,
}

impl<T> Publisher<T>
where
    T: messages::PublishedMessage + Clone + Send + Sync,
{
    pub fn from_config(config: PublisherConfig) -> PublisherResult<Self> {
        // let PublisherConfig { address, port } = config;
        // let multicast_address = std::net::SocketAddr::new(std::net::IpAddr::V4(address), port);
        // tracing::info!(
        //     "Creating new multicast publisher at {:?} is multicast: {}",
        //     &multicast_address,
        //     &address.is_multicast()
        // );
        // let publisher_socket =
        //     tokio::net::UdpSocket::from_std(socket::multicast::new_publisher(&multicast_address)?)?;

        let message_producer = std::sync::Arc::new(messages::MessageProducer::<T>::default());

        Ok(Self {
            message_producer,
            config,
        })
    }

    // pub fn
}
