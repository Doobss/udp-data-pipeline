use std::net::{Ipv4Addr, SocketAddr};
use udp_data_pipeline::{logging, MULTICAST_ADDR};
use udp_multicast_subscriber::{ABSubscriber, SubscriberResult};

const MULTICAST_PORT: u16 = 1900;

/// Networking options.
#[derive(argh::FromArgs)]
struct SubscriberArgs {
    /// multicast address that the socket must join
    #[argh(option, short = 'a', default = "MULTICAST_ADDR")]
    address: Ipv4Addr,
    /// specific port to bind the socket to
    #[argh(option, short = 'p', default = "MULTICAST_PORT")]
    port: u16,
}

#[tokio::main]
async fn main() -> SubscriberResult<()> {
    logging::init();
    tracing::info!("Starting udp-multicast-subscriber");

    let SubscriberArgs { address, port } = argh::from_env();
    let multicast_address = SocketAddr::new(std::net::IpAddr::V4(address), port);

    tracing::info!("joining multicast at address: {:?}", &multicast_address);
    let ab_socket = ABSubscriber::new(multicast_address)?;
    ab_socket.listen().await
}
