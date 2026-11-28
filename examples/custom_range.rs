// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: MIT

//! Custom range, bind address, and ascending (`Strategy::First`) port discovery.

use mkd_freeport::{find_ports, AllocationOptions, Protocol, Strategy};

fn main() -> Result<(), mkd_freeport::Error> {
    let options = AllocationOptions {
        protocol: Protocol::Tcp,
        bind_address: "127.0.0.1".parse().expect("valid loopback IP"),
        range: 49152..=50000,
        strategy: Strategy::First,
        count: 2,
    };

    let ports = find_ports(&options)?;
    println!("First 2 free TCP ports on 127.0.0.1 in 49152..=50000: {ports:?}");

    Ok(())
}
