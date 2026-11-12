# mkd-freeport

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Discover or reserve TCP/UDP ports over IPv4 and IPv6 for scripts, AI agents, and CI pipelines. Includes a CLI and a public Rust API.

**Version 0.3.1**

## Install

Install the published CLI from [crates.io](https://crates.io/crates/mkd-freeport):

```bash
cargo install mkd-freeport --locked
```

Or install from a local checkout of this repository:

```bash
cargo install --path .
```

`cargo install` builds in release mode by default.

## Usage

```bash
# Find one TCP/IPv4 port; compact single-line JSON is the default output
mkd-freeport

# First available port above 1024
mkd-freeport --first

# Find three distinct UDP ports on IPv6
mkd-freeport --protocol udp --ipv6 --count 3

# Search an inclusive custom range on IPv4 loopback, in ascending order
mkd-freeport --bind 127.0.0.1 --range 49152-50000 --first --count 2

# An IPv6 bind address selects IPv6 without needing --ipv6
mkd-freeport --bind ::1

# Alternative output formats
mkd-freeport --human
mkd-freeport --pretty           # same as --format json
mkd-freeport --plain            # bare port numbers, one per line

mkd-freeport --help
mkd-freeport --version
```

Defaults are TCP, bind address `0.0.0.0`, one port, and random selection from `49152–65535`. `--first` searches `1025–65535` in ascending order unless `--range START-END` overrides the range. Random selection tries candidates without repetition.

`--protocol tcp|udp` selects the transport. `--ipv6` changes the default bind address to `::`; `--bind` accepts a literal IPv4 or IPv6 address. An IPv4 `--bind` cannot be combined with `--ipv6`. IPv6 sockets are IPv6-only.

Ranges are inclusive and must satisfy `1 <= START <= END <= 65535`. The requested `--count` must be positive and fit within the range; allocation succeeds only if all requested ports can be acquired.

## Foreground reservations

```bash
# Hold two TCP ports for up to 30 seconds
mkd-freeport --reserve --bind 127.0.0.1 --count 2 --timeout 30

# Hold one UDP port until Ctrl-C or SIGTERM
mkd-freeport --reserve --protocol udp
```

`--reserve` keeps sockets open after printing and flushing the result. Ctrl-C (SIGINT), SIGTERM, or the optional timeout releases them and exits. `--timeout` requires `--reserve` and a positive number of seconds.

Read reservation output from another process. Shell command substitution such as `PORT=$(mkd-freeport --reserve)` waits for the command to exit, by which time the reservation has been released.

Discovery releases sockets before returning, so another process can claim a reported port. Reservations cover only the selected transport, bind address, and IP family. Release a reservation before your service binds; the handoff can still race, so callers must handle bind failures.

## Output

Default output is a compact JSON envelope for scripts and automation. Example discovery result (the actual port varies):

```json
{"result":"OK","message":"Free ports found","data":{"port":49152,"strategy":"random","protocol":"tcp","bind_address":"0.0.0.0","reserved":false}}
```

A single port uses `data.port`; multiple ports use `data.ports` instead. For example, a reservation can report:

```json
{"result":"OK","message":"Ports reserved","data":{"ports":[49152,49153],"strategy":"random","protocol":"udp","bind_address":"::1","reserved":true}}
```

Parse discovery output with `jq`:

```bash
PORT=$(mkd-freeport | jq -r '.data.port')
PORTS=$(mkd-freeport --count 3 | jq -r '.data.ports[]')

# Bare port output for shell scripts
PORT=$(mkd-freeport --plain)
```

`--format compact|json|human` selects compact JSON, pretty JSON, or readable text. `--plain` prints one port per line. Allocation errors exit with a nonzero status; JSON errors include `result: "ERROR"`, `level`, `message`, and `error_code`. Human-readable errors go to stderr.

## Rust API

Add the published library as a dependency:

```toml
[dependencies]
mkd-freeport = "0.3.1"
```

```rust
use mkd_freeport::{
    get_free_port, get_free_ports, reserve_ports, AllocationOptions, Protocol,
};

fn main() -> Result<(), mkd_freeport::Error> {
    let port = get_free_port()?;
    let ports = get_free_ports(3)?;
    println!("Discovered TCP ports: {port}, {ports:?}");

    let options = AllocationOptions {
        protocol: Protocol::Udp,
        bind_address: "127.0.0.1".parse().unwrap(),
        count: 2,
        ..Default::default()
    };
    let reservation = reserve_ports(&options)?;
    println!("Reserved UDP addresses: {:?}", reservation.addresses());
    drop(reservation); // Release before another service binds.
    Ok(())
}
```

`get_free_port()` and `get_free_ports(count)` discover TCP/IPv4 ports with default options. `find_ports(&options)` accepts a custom policy and releases sockets before returning. `reserve_ports(&options)` returns an owned `Reservation`; its `ports()` and `addresses()` expose the allocation, and dropping it releases every socket.

`AllocationOptions` controls `protocol`, `bind_address`, inclusive `range`, `strategy` (`Strategy::Random` or `Strategy::First`), and `count`. Its default range is always `49152..=65535`; set `range: 1025..=65535` explicitly to match the CLI's `--first` default. Errors distinguish invalid options, exhausted ranges, and bind failures with an underlying I/O cause.

## See Also

- [CHANGELOG.md](./CHANGELOG.md)

## Universal Code

This project follows the [Universal Code v1.0.0](https://github.com/monkeyking-hq/universal-code).

## License

Released under the [MIT License](LICENSE).
