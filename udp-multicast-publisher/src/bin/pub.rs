use udp_data_pipeline::{
    logging,
    messages::{self, ToString},
    socket, MULTICAST_ADDR,
};
use udp_multicast_publisher::PublisherResult;

use std::net::{Ipv4Addr, SocketAddr};

const MULTICAST_PORT: u16 = 1900;

/// Networking options.
#[derive(argh::FromArgs)]
struct PublisherArgs {
    /// multicast address that the socket must join
    #[argh(option, short = 'a', default = "MULTICAST_ADDR")]
    address: Ipv4Addr,
    /// specific port to bind the socket to
    #[argh(option, short = 'p', default = "MULTICAST_PORT")]
    port: u16,
}

#[tokio::main]
async fn main() -> PublisherResult<()> {
    logging::init();
    tracing::info!("Starting udp-multicast-publisher");

    let PublisherArgs { address, port } = argh::from_env();
    let multicast_address = SocketAddr::new(std::net::IpAddr::V4(address), port);
    tracing::info!(
        "Creating new multicast publisher at {:?} is multicast: {}",
        &multicast_address,
        &address.is_multicast()
    );
    let publisher_socket =
        tokio::net::UdpSocket::from_std(socket::multicast::new_publisher(&multicast_address)?)?;

    let mut message_producer = messages::MessageProducer::<messages::SimpleMessage>::default();
    loop {
        let message = message_producer.next_message();
        let message = message.to_string()?;
        let message = message.as_bytes();
        let len = message.len();
        tracing::info!("sending {len} bytes to {:?}", multicast_address);
        _ = publisher_socket.send_to(message, multicast_address).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await
    }
}
