use udp_data_pipeline::logging;
use udp_data_pipeline::socket;
use udp_multicast_publisher::PublisherResult;

use std::net::{Ipv4Addr, SocketAddr};

const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 1);
// pub static ref IPV4: IpAddr = Ipv4Addr::new(224, 0, 0, 123).into();
// pub static ref IPV6: IpAddr = Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123).into();
const MULTICAST_PORT: u16 = 7645;

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

    // let socket = tokio::net::UdpSocket::from_std(publisher_socket)?;
    // let buffer = [0; 1024];
    let mut len = 18;
    let message = b"Hello from client!";
    loop {
        tracing::info!("sending {len} bytes to {:?}", multicast_address);
        len = publisher_socket.send_to(message, multicast_address)?;
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await
    }

    // Ok(())
}
