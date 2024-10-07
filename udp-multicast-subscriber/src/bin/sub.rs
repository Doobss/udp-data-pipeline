use udp_data_pipeline::{
    logging,
    messages::{self, FromBytes},
    MULTICAST_ADDR,
};

use udp_multicast_subscriber::{ABSubscriber, SubscriberResult};

use std::net::{Ipv4Addr, SocketAddr};

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
    let ABSubscriber { socket_a } = ABSubscriber::new(multicast_address)?;
    let mut buffer = [0u8; 1028];

    loop {
        let (len, address) = socket_a.recv_from(&mut buffer).await?;
        let data = &buffer[..len];
        let response = match messages::SimpleMessage::from_bytes(data) {
            Ok(parsed) => Some(parsed),
            Err(error) => {
                tracing::error!("Error parsing message: {error}");
                None
            }
        };

        tracing::info!(
            "Recieved bytes.len {len} from address: {:?} response: {:?}",
            &address,
            &response
        );
    }
}
