use udp_data_pipeline::{logging, socket, MULTICAST_ADDR};
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
    let publisher_socket = socket::multicast::new_publisher(&multicast_address)?;

    let publisher_socket = tokio::net::UdpSocket::from_std(publisher_socket)?;
    // let buffer = [0; 1024];
    let mut len = 18;
    let message = b"Hello from publisher!";
    loop {
        tracing::info!("sending {len} bytes to {:?}", multicast_address);
        len = publisher_socket.send_to(message, multicast_address).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await
    }

    // Ok(())
}
