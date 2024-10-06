use udp_data_pipeline::{logging, socket, MULTICAST_ADDR};

use udp_multicast_subscriber::SubscriberResult;

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
    let subscriber_socket = socket::multicast::new_subscriber(multicast_address)?;

    // let subscriber_socket = tokio::net::UdpSocket::from_std(subscriber_socket)?;
    let mut buffer = [0u8; 1028];
    subscriber_socket.set_nonblocking(false)?;

    loop {
        match subscriber_socket.recv_from(&mut buffer) {
            Ok((len, address)) => {
                let data = &buffer[..len];
                let response = String::from_utf8_lossy(data);

                tracing::info!(
                    "Recieved bytes.len :{len} response: {response} from address: {:?}",
                    &address
                );
            }
            Err(_error) => {
                // tracing::debug!("subscriber_socket.recv_from error: {:?}", error)
            }
        }
        // let (len, address) = subscriber_socket.recv_from(&mut buffer);
        // let data = &buffer[..len];
        // let response = String::from_utf8_lossy(data);

        // tracing::info!(
        //     "Recieved bytes.len :{len} response: {response} from address: {:?}",
        //     &address
        // );
    }
}
