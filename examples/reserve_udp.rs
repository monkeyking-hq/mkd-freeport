// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: MIT

//! Owned UDP port reservation: holds sockets until dropped.

use mkd_freeport::{reserve_ports, AllocationOptions, Protocol};
use std::net::UdpSocket;

fn main() -> Result<(), mkd_freeport::Error> {
    let options = AllocationOptions {
        protocol: Protocol::Udp,
        bind_address: "127.0.0.1".parse().expect("valid loopback IP"),
        count: 2,
        ..Default::default()
    };

    let reservation = reserve_ports(&options)?;
    let addrs = reservation.addresses().to_vec();
    println!("Reserved UDP sockets: {addrs:?}");

    // While `reservation` is alive, competing binds fail.
    for addr in &addrs {
        assert!(UdpSocket::bind(addr).is_err(), "{addr} should be held");
    }

    // Release before starting the target service.
    drop(reservation);
    println!("Released reservation; ports are now bindable.");

    for addr in &addrs {
        let sock = UdpSocket::bind(addr).expect("port should be free after drop");
        drop(sock);
    }

    Ok(())
}
