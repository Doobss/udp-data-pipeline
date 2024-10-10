use udp_data_pipeline::socket;

pub struct ABSubscriber {
    pub socket_a: tokio::net::UdpSocket,
    pub socket_b: tokio::net::UdpSocket,
}

impl ABSubscriber {
    pub fn new(address: std::net::SocketAddr) -> crate::SubscriberResult<Self> {
        Ok(Self {
            socket_a: tokio::net::UdpSocket::from_std(socket::multicast::new_subscriber(address)?)?,
            socket_b: tokio::net::UdpSocket::from_std(socket::multicast::new_subscriber(address)?)?,
        })
    }

    pub async fn listen(&self) -> crate::SubscriberResult<()> {
        let ABSubscriber { socket_a, socket_b } = self;
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

        // Ok(())
    }
}

fn parse_message(data: &[u8]) -> Option<udp_data_pipeline::messages::SimpleMessage> {
    match <udp_data_pipeline::messages::SimpleMessage as udp_data_pipeline::messages::FromBytes>::from_bytes(data) {
        Ok(parsed) => Some(parsed),
        Err(error) => {
            tracing::error!("Error parsing message: {error}");
            None
        }
    }
}
