use std::borrow::BorrowMut;

use udp_data_pipeline::{messages, socket};

use crate::PublisherResult;

pub async fn task<'a, T>(
    address: std::net::Ipv4Addr,
    port: u16,
    mut message_producer: std::sync::Arc<messages::MessageProducer<T>>,
) -> PublisherResult<tokio::task::JoinHandle<()>>
where
    T: messages::PublishedMessage + Clone + messages::ToString + Send + Sync + 'static,
{
    let multicast_address = std::net::SocketAddr::new(std::net::IpAddr::V4(address), port);
    tracing::info!(
        "Creating new multicast publisher at {:?} is multicast: {}",
        &multicast_address,
        &address.is_multicast()
    );
    let publisher_socket =
        tokio::net::UdpSocket::from_std(socket::multicast::new_publisher(&multicast_address)?)?;
    // let message_producer = message_producer.clone();

    Ok(tokio::task::spawn(async move {
        let message_producer = message_producer.borrow_mut();

        loop {
            // let message = message_producer.next_message();
            // match message.to_string() {
            //     Ok(message) => {
            //         let message = message.as_bytes();
            //         let len = message.len();
            //         tracing::info!("sending {len} bytes to {:?}", multicast_address);
            //         // match publisher_socket.send_to(message, multicast_address).await {
            //         //     Ok(_) => {}
            //         //     Err(error) => {
            //         //         tracing::error!("Error publishing message {:?}", error)
            //         //     }
            //         // };
            //     }
            //     Err(error) => tracing::error!("Error serializing message {:?}", error),
            // };

            // tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }))
}
