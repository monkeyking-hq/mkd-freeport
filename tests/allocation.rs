use mkd_freeport::{
    find_ports, get_free_port, get_free_ports, reserve_ports, AllocationOptions, Error, Protocol,
    Strategy,
};
use std::net::{IpAddr, TcpListener, UdpSocket};

fn options(protocol: Protocol, address: &str) -> AllocationOptions {
    AllocationOptions {
        protocol,
        bind_address: address.parse().unwrap(),
        ..Default::default()
    }
}
fn can_bind(protocol: Protocol, ip: IpAddr, port: u16) -> bool {
    match protocol {
        Protocol::Tcp => TcpListener::bind((ip, port)).is_ok(),
        Protocol::Udp => UdpSocket::bind((ip, port)).is_ok(),
    }
}
fn ipv6_available(protocol: Protocol) -> bool {
    let result = match protocol {
        Protocol::Tcp => TcpListener::bind(("::1", 0)).map(|_| ()),
        Protocol::Udp => UdpSocket::bind(("::1", 0)).map(|_| ()),
    };
    match result {
        Ok(()) => true,
        Err(e)
            if matches!(
                e.kind(),
                std::io::ErrorKind::AddrNotAvailable | std::io::ErrorKind::Unsupported
            ) || e.raw_os_error() == Some(libc::EAFNOSUPPORT) =>
        {
            eprintln!("IPv6 unavailable: {e}");
            false
        }
        Err(e) => panic!("IPv6 capability probe failed unexpectedly: {e}"),
    }
}
#[test]
fn protocols_families_and_release() {
    for protocol in [Protocol::Tcp, Protocol::Udp] {
        for ip in ["127.0.0.1", "0.0.0.0", "::1", "::"] {
            if ip.contains(':') && !ipv6_available(protocol) {
                continue;
            }
            let opts = AllocationOptions {
                count: 3,
                ..options(protocol, ip)
            };
            let reservation = reserve_ports(&opts).unwrap();
            let ports = reservation.ports();
            let unique: std::collections::HashSet<_> = ports.iter().collect();
            assert_eq!(unique.len(), 3);
            for port in &ports {
                assert!(!can_bind(protocol, opts.bind_address, *port));
            }
            drop(reservation);
            for port in ports {
                assert!(can_bind(protocol, opts.bind_address, port));
            }
        }
    }
}
#[test]
fn ipv6_is_separate_from_ipv4() {
    if !ipv6_available(Protocol::Tcp) {
        return;
    }
    let held = reserve_ports(&options(Protocol::Tcp, "::")).unwrap();
    assert!(TcpListener::bind(("0.0.0.0", held.ports()[0])).is_ok());
}
#[test]
fn defaults_and_discovery() {
    assert!(get_free_port().unwrap() >= 49152);
    assert_eq!(get_free_ports(4).unwrap().len(), 4);
    let opts = options(Protocol::Udp, "127.0.0.1");
    for port in find_ports(&opts).unwrap() {
        assert!(can_bind(opts.protocol, opts.bind_address, port));
    }
}
#[test]
fn singleton_range_exhaustion_and_io_cause() {
    let opts = options(Protocol::Tcp, "127.0.0.1");
    let held = reserve_ports(&opts).unwrap();
    let port = held.ports()[0];
    let singleton = AllocationOptions {
        range: port..=port,
        ..opts.clone()
    };
    assert!(matches!(reserve_ports(&singleton), Err(Error::Exhausted)));
    drop(held);
    assert_eq!(find_ports(&singleton).unwrap(), vec![port]);
    let invalid_address = AllocationOptions {
        bind_address: "203.0.113.1".parse().unwrap(),
        ..opts
    };
    let err = reserve_ports(&invalid_address).unwrap_err();
    assert!(matches!(err, Error::Bind { .. }));
    assert!(std::error::Error::source(&err).is_some());
}
#[test]
fn invalid_options() {
    for opts in [
        AllocationOptions {
            count: 0,
            ..Default::default()
        },
        AllocationOptions {
            range: 0..=1,
            ..Default::default()
        },
        AllocationOptions {
            range: std::ops::RangeInclusive::new(2, 1),
            ..Default::default()
        },
        AllocationOptions {
            range: 50000..=50001,
            count: 3,
            ..Default::default()
        },
    ] {
        assert!(matches!(
            reserve_ports(&opts),
            Err(Error::InvalidOptions(_))
        ));
    }
}
#[test]
fn first_order_and_partial_cleanup() {
    // Acquire a dynamically chosen span; only use it if its adjacent ports are free.
    for _ in 0..100 {
        let seed = reserve_ports(&options(Protocol::Tcp, "127.0.0.1")).unwrap();
        let start = seed.ports()[0];
        if start > 65532 {
            continue;
        }
        drop(seed);
        let opts = AllocationOptions {
            range: start..=start + 2,
            count: 3,
            strategy: Strategy::First,
            ..options(Protocol::Tcp, "127.0.0.1")
        };
        let Ok(all) = reserve_ports(&opts) else {
            continue;
        };
        assert_eq!(all.ports(), vec![start, start + 1, start + 2]);
        drop(all);
        let blocker = TcpListener::bind(("127.0.0.1", start + 2)).unwrap();
        assert!(matches!(reserve_ports(&opts), Err(Error::Exhausted)));
        assert!(TcpListener::bind(("127.0.0.1", start)).is_ok());
        assert!(TcpListener::bind(("127.0.0.1", start + 1)).is_ok());
        drop(blocker);
        return;
    }
    panic!("Could not acquire dynamic contiguous range");
}
