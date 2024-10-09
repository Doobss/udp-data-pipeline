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

fn parse_message(data: &[u8]) -> Option<messages::SimpleMessage> {
    match messages::SimpleMessage::from_bytes(data) {
        Ok(parsed) => Some(parsed),
        Err(error) => {
            tracing::error!("Error parsing message: {error}");
            None
        }
    }
}

#[tokio::main]
async fn main() -> SubscriberResult<()> {
    logging::init();
    tracing::info!("Starting udp-multicast-subscriber");

    let SubscriberArgs { address, port } = argh::from_env();
    let multicast_address = SocketAddr::new(std::net::IpAddr::V4(address), port);

    tracing::info!("joining multicast at address: {:?}", &multicast_address);
    let ABSubscriber { socket_a, socket_b } = ABSubscriber::new(multicast_address)?;
    let mut buffer_a = [0u8; 1028];
    let mut buffer_b = [0u8; 1028];

    loop {
        match tokio::try_join!(
            socket_a.recv_from(&mut buffer_a),
            socket_b.recv_from(&mut buffer_b)
        ) {
            Ok((a_response, b_response)) => {
                let (a_len, a_address) = a_response;
                let (b_len, b_address) = b_response;
                let a_message = parse_message(&buffer_a[..a_len]);
                let b_message = parse_message(&buffer_b[..b_len]);
                tracing::info!(
                    "A socket recieved bytes.len {a_len} from address: {:?} message: {:?}",
                    &a_address,
                    &a_message
                );
                tracing::info!(
                    "B socket recieved bytes.len {b_len} from address: {:?} message: {:?}",
                    &b_address,
                    &b_message
                );
            }
            Err(error) => {
                tracing::error!("Error parsing message: {error}");
            }
        }
    }
}
