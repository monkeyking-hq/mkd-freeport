//! Discover or reserve TCP/UDP ports on IPv4 and IPv6.
//!
//! Discovery releases sockets before returning: callers must handle a competing bind.
//! Reservations hold all sockets until dropped and cover only the chosen protocol,
//! address and family. IPv6 sockets are IPv6-only.
//!
//! ```
//! use mkd_freeport::{reserve_ports, AllocationOptions, Protocol};
//! let options = AllocationOptions { protocol: Protocol::Udp, count: 2,
//!     ..Default::default() };
//! let reservation = reserve_ports(&options)?;
//! assert_eq!(reservation.addresses().len(), 2);
//! drop(reservation); // release before another service binds
//! # Ok::<(), mkd_freeport::Error>(())
//! ```

use rand::seq::SliceRandom;
use socket2::{Domain, Socket, Type};
use std::{
    fmt, io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    ops::RangeInclusive,
};

/// Transport whose port space is checked or reserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Protocol {
    #[default]
    Tcp,
    Udp,
}
impl Protocol {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
        }
    }
}
/// Order in which candidate ports are tried.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Strategy {
    #[default]
    Random,
    First,
}
impl Strategy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Random => "random",
            Self::First => "first",
        }
    }
}
/// Allocation policy. Use `1025..=65535` with `First` to match CLI `--first`.
#[derive(Debug, Clone)]
pub struct AllocationOptions {
    pub protocol: Protocol,
    pub bind_address: IpAddr,
    pub range: RangeInclusive<u16>,
    pub strategy: Strategy,
    pub count: usize,
}
impl Default for AllocationOptions {
    fn default() -> Self {
        Self {
            protocol: Protocol::Tcp,
            bind_address: Ipv4Addr::UNSPECIFIED.into(),
            range: 49152..=65535,
            strategy: Strategy::Random,
            count: 1,
        }
    }
}
/// Allocation failure, retaining operating-system causes when applicable.
#[derive(Debug)]
pub enum Error {
    InvalidOptions(&'static str),
    Exhausted,
    Bind {
        address: SocketAddr,
        source: io::Error,
    },
}
impl Error {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidOptions(_) => "INVALID_OPTIONS",
            Self::Exhausted => "NO_FREE_PORT",
            Self::Bind { .. } => "BIND_FAILED",
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOptions(message) => write!(f, "{message}"),
            Self::Exhausted => write!(f, "Not enough free ports in the requested range"),
            Self::Bind { address, source } => write!(f, "Cannot bind {address}: {source}"),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Bind { source, .. } => Some(source),
            _ => None,
        }
    }
}
/// Owns bound sockets; dropping this value releases every reservation.
#[derive(Debug)]
pub struct Reservation {
    _sockets: Vec<Socket>,
    addresses: Vec<SocketAddr>,
}
impl Reservation {
    pub fn addresses(&self) -> &[SocketAddr] {
        &self.addresses
    }
    pub fn ports(&self) -> Vec<u16> {
        self.addresses.iter().map(SocketAddr::port).collect()
    }
}
/// Find one TCP/IPv4 port using default options. The port is released before return.
pub fn get_free_port() -> Result<u16, Error> {
    Ok(get_free_ports(1)?[0])
}
/// Find distinct TCP/IPv4 ports using default options. Released before return.
pub fn get_free_ports(count: usize) -> Result<Vec<u16>, Error> {
    find_ports(&AllocationOptions {
        count,
        ..Default::default()
    })
}
/// Find ports under a custom policy. Released before return; this is not a lease.
pub fn find_ports(options: &AllocationOptions) -> Result<Vec<u16>, Error> {
    Ok(reserve_ports(options)?.ports())
}
/// Acquire the complete requested set or release all partial allocations.
pub fn reserve_ports(options: &AllocationOptions) -> Result<Reservation, Error> {
    let start = *options.range.start();
    let end = *options.range.end();
    if start == 0 || start > end {
        return Err(Error::InvalidOptions(
            "Range must be within 1–65535 with start <= end",
        ));
    }
    let capacity = usize::from(end) - usize::from(start) + 1;
    if options.count == 0 || options.count > capacity {
        return Err(Error::InvalidOptions(
            "Count must be positive and fit within the range",
        ));
    }
    let mut candidates: Vec<_> = options.range.clone().collect();
    if options.strategy == Strategy::Random {
        candidates.shuffle(&mut rand::rng());
    }
    let mut reservation = Reservation {
        _sockets: Vec::with_capacity(options.count),
        addresses: Vec::with_capacity(options.count),
    };
    let mut bind_error = None;
    for port in candidates {
        let address = SocketAddr::new(options.bind_address, port);
        match bind_socket(address, options.protocol) {
            Ok(socket) => {
                reservation._sockets.push(socket);
                reservation.addresses.push(address);
                if reservation.addresses.len() == options.count {
                    return Ok(reservation);
                }
            }
            Err(source) if source.kind() == io::ErrorKind::AddrInUse => {}
            // Permission may vary per port (privileged ports); keep searching.
            Err(source) if source.kind() == io::ErrorKind::PermissionDenied => {
                bind_error = Some(Error::Bind { address, source });
            }
            Err(source) => return Err(Error::Bind { address, source }),
        }
    }
    Err(bind_error.unwrap_or(Error::Exhausted))
}
fn bind_socket(address: SocketAddr, protocol: Protocol) -> io::Result<Socket> {
    let domain = if address.is_ipv6() {
        Domain::IPV6
    } else {
        Domain::IPV4
    };
    let kind = match protocol {
        Protocol::Tcp => Type::STREAM,
        Protocol::Udp => Type::DGRAM,
    };
    let socket = Socket::new(domain, kind, None)?;
    if address.is_ipv6() {
        socket.set_only_v6(true)?;
    }
    socket.set_reuse_address(false)?;
    socket.bind(&address.into())?;
    if protocol == Protocol::Tcp {
        socket.listen(128)?;
    }
    Ok(socket)
}
