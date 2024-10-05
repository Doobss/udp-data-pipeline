use udp_data_pipeline::logging;
use udp_data_pipeline::socket;
use udp_multicast_subscriber::SubscriberResult;

use std::net::{Ipv4Addr, SocketAddr};

const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 1);
const MULTICAST_PORT: u16 = 7644;

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
    let subscriber_socket = socket::multicast::new_subscriber(multicast_address)?;

    let subscribed_socket = tokio::net::UdpSocket::from_std(subscriber_socket)?;
    let mut buffer = [0u8; 1028];

    loop {
        let (len, address) = subscribed_socket.recv_from(&mut buffer).await?;
        let data = &buffer[..len];
        let response = String::from_utf8_lossy(data);

        tracing::info!(
            "Recieved bytes.len :{len} response: {response} from address: {:?}",
            &address
        );
    }
}
