use std::{io, net::SocketAddr};

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

// this will be common for all our sockets
pub fn new_socket(address: &SocketAddr) -> io::Result<Socket> {
    let domain = if address.is_ipv4() {
        Domain::IPV4
    } else {
        Domain::IPV6
    };

    let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;

    // socket.set_nonblocking(false)?;
    // we're going to use read timeouts so that we don't hang waiting for packets
    socket.set_read_timeout(Some(std::time::Duration::from_millis(100)))?;

    Ok(socket)
}

pub fn new_subscriber(address: SocketAddr) -> io::Result<std::net::UdpSocket> {
    let ip_address = address.ip();
    tracing::debug!("new_subscriber: ip_address: {:?}", &ip_address);

    let socket = new_socket(&address)?;

    match ip_address {
        std::net::IpAddr::V4(ref mdns_v4) => {
            tracing::debug!("new_subscriber: join_multicast_v4 mdns_v4: {:?}", &mdns_v4);
            socket.join_multicast_v4(mdns_v4, &std::net::Ipv4Addr::new(0, 0, 0, 0))?;
        }
        std::net::IpAddr::V6(ref mdns_v6) => {
            tracing::debug!("new_subscriber: join_multicast_v6 mdns_v6: {:?}", &mdns_v6);
            socket.join_multicast_v6(mdns_v6, 0)?;
            socket.set_only_v6(true)?;
        }
    };
    bind_multicast(&socket, &address)?;
    Ok(socket.into())
}

pub fn new_publisher(addr: &SocketAddr) -> io::Result<std::net::UdpSocket> {
    let socket = new_socket(addr)?;

    let bound_address = if addr.is_ipv4() {
        SockAddr::from(SocketAddr::new(
            std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
            0,
        ))
    } else {
        SockAddr::from(SocketAddr::new(
            std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(),
            0,
        ))
    };

    tracing::debug!(
        "new_publisher: binding to address: {:?}",
        &bound_address.as_socket()
    );
    socket.bind(&bound_address)?;

    Ok(socket.into())
}

#[cfg(windows)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    let addr = match *addr {
        SocketAddr::V4(addr) => SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), addr.port()),
        SocketAddr::V6(addr) => {
            SocketAddr::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(), addr.port())
        }
    };
    socket.bind(&socket2::SockAddr::from(addr))
}

/// On unixes we bind to the multicast address, which causes multicast packets to be filtered
#[cfg(unix)]
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    let addr = match *addr {
        SocketAddr::V4(addr) => {
            SocketAddr::new(std::net::Ipv4Addr::new(0, 0, 0, 0).into(), addr.port())
        }
        SocketAddr::V6(addr) => SocketAddr::new(
            std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(),
            addr.port(),
        ),
    };
    socket.bind(&socket2::SockAddr::from(addr))
}
