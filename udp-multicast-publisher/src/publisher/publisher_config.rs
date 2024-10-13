use std::net::Ipv4Addr;
use udp_data_pipeline::MULTICAST_ADDR;

const MULTICAST_PORT: u16 = 1900;

/// Networking options.
#[derive(argh::FromArgs)]
pub struct PublisherConfig {
    /// multicast address that the socket must join
    #[argh(option, short = 'a', default = "MULTICAST_ADDR")]
    pub address: Ipv4Addr,
    /// specific port to bind the socket to
    #[argh(option, short = 'p', default = "MULTICAST_PORT")]
    pub port: u16,
    /// delay in milliseconds between published messages
    #[argh(option, short = 'd', default = "1000")]
    pub delay: u64,
}
