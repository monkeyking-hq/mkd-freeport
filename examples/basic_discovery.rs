// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: MIT

//! Basic port discovery using the `mkd-freeport` library API.

use mkd_freeport::{get_free_port, get_free_ports};

fn main() -> Result<(), mkd_freeport::Error> {
    let port = get_free_port()?;
    println!("Discovered single TCP port: {port}");

    let ports = get_free_ports(3)?;
    println!("Discovered 3 distinct TCP ports: {ports:?}");

    Ok(())
}
