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
}
