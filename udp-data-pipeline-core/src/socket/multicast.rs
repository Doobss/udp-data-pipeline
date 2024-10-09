use std::{io, net::SocketAddr};

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

pub fn new_socket(address: &SocketAddr) -> io::Result<Socket> {
    let domain = if address.is_ipv4() {
        Domain::IPV4
    } else {
        Domain::IPV6
    };

    let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;

    socket.set_read_timeout(Some(std::time::Duration::from_millis(100)))?;
    socket.set_reuse_address(true)?;
    // socket.set_multicast_all_v4(false)?;
    // socket.set_multicast_all_v6(false)?;
    Ok(socket)
}

pub fn new_subscriber(address: SocketAddr) -> io::Result<std::net::UdpSocket> {
    let ip_address = address.ip();
    tracing::debug!("new_subscriber: ip_address: {:?}", &ip_address);

    let socket = new_socket(&address)?;
    bind_multicast(&socket, &address)?;
    let local_address = socket.local_addr()?;
    if local_address.is_ipv4() {
        tracing::debug!(
            "new_subscriber: local_address: {:?}",
            &local_address.as_socket_ipv4()
        );
    } else {
        tracing::debug!(
            "new_subscriber: local_address: {:?}",
            &local_address.as_socket_ipv6()
        );
    }
    match ip_address {
        std::net::IpAddr::V4(ref mdns_v4) => {
            let interface = std::net::Ipv4Addr::UNSPECIFIED;
            tracing::debug!(
                "new_subscriber: join_multicast_v4 mdns_v4: {:?} on interface: {:?}",
                &mdns_v4,
                &interface
            );
            socket.join_multicast_v4(mdns_v4, &interface)?;
        }
        std::net::IpAddr::V6(ref mdns_v6) => {
            let interface = 0;
            tracing::debug!(
                "new_subscriber: join_multicast_v6 mdns_v6: {:?} on interface: {interface}",
                &mdns_v6
            );
            socket.join_multicast_v6(mdns_v6, interface)?;
            socket.set_only_v6(true)?;
        }
    };
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

#[cfg(unix)]
fn bind_multicast(socket: &Socket, addr: &std::net::SocketAddr) -> io::Result<()> {
    let addr = match *addr {
        std::net::SocketAddr::V4(addr) => {
            std::net::SocketAddr::new(std::net::Ipv4Addr::UNSPECIFIED.into(), addr.port())
        }
        std::net::SocketAddr::V6(addr) => {
            std::net::SocketAddr::new(std::net::Ipv6Addr::UNSPECIFIED.into(), addr.port())
        }
    };
    tracing::debug!("bind_multicast to address : {:?}", &addr);
    socket.bind(&socket2::SockAddr::from(addr))
}
