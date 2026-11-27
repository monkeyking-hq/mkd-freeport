// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: MIT

//! Shared CLI definition for `mkd-freeport` and `xtask` asset generation.

use clap::{CommandFactory, Parser};
use std::net::IpAddr;

/// Output format for command results (default: compact single-line JSON envelope).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    /// Canonical single-line JSON envelope (default). Best for agents and scripting with `jq`.
    #[default]
    Compact,
    /// Pretty-printed JSON envelope.
    Json,
    /// Human-readable text (no JSON).
    Human,
}

/// Discover or reserve TCP/UDP ports over IPv4 and IPv6.
///
/// Defaults: one TCP/IPv4 port on 0.0.0.0, selected randomly from 49152-65535,
/// with compact single-line JSON output. Discovery releases sockets before returning.
#[derive(Parser, Debug)]
#[command(
    name = "mkd-freeport",
    version,
    after_help = "Examples:\n  mkd-freeport --protocol udp --ipv6 --count 2\n  mkd-freeport --bind 127.0.0.1 --range 49152-50000 --first\n  mkd-freeport --reserve --count 2 --timeout 30\n\nDefault random range: 49152-65535. Discovery releases sockets before returning.\nReservations run in the foreground. See --help for output and handoff details.",
    after_long_help = r#"Examples:
  mkd-freeport --protocol udp --ipv6 --count 2
  mkd-freeport --bind 127.0.0.1 --range 49152-50000 --first
  mkd-freeport --reserve --count 2 --timeout 30
  mkd-freeport --plain
  mkd-freeport | jq -r '.data.port'

Output:
  Example compact JSON discovery result (the actual port varies):
  {"result":"OK","message":"Free ports found","data":{"port":49152,"strategy":"random","protocol":"tcp","bind_address":"0.0.0.0","reserved":false}}
  One port uses data.port; multiple ports use data.ports (an array) instead.
  Reservations report message "Ports reserved" and data.reserved true.
  Allocation errors exit nonzero; JSON errors include result "ERROR", level,
  message and error_code. Human-readable errors go to stderr.

Discovery and reservations:
  Discovery releases sockets before returning; a reported port can be claimed by
  another process. Callers must handle bind failures.
  Reservations cover only the selected transport, bind address and IP family.
  IPv6 sockets are IPv6-only; an IPv6 --bind selects IPv6 without --ipv6.
  Reservations print and flush output, then run in the foreground until Ctrl-C
  (SIGINT), SIGTERM or timeout. Read stdout from another process: shell command
  substitution waits for exit, when the reservation has already been released.
  Release before the service binds; that handoff can still race."#
)]
pub struct Args {
    /// Search in ascending order (default range: 1025-65535; --range overrides)
    #[arg(long)]
    pub first: bool,
    /// Transport: tcp or udp
    #[arg(long, value_parser = ["tcp", "udp"], default_value = "tcp")]
    pub protocol: String,
    /// Use IPv6 (default bind: ::); cannot combine with IPv4 --bind
    #[arg(long)]
    pub ipv6: bool,
    /// Literal bind IP address; defaults to 0.0.0.0 (or :: with --ipv6)
    #[arg(long)]
    pub bind: Option<IpAddr>,
    /// Inclusive range START-END (1 <= START <= END <= 65535); overrides defaults
    ///
    /// Default: 49152-65535 with random selection, or 1025-65535 with --first.
    /// Random selection tries candidates without repetition.
    #[arg(long, value_parser = parse_range)]
    pub range: Option<std::ops::RangeInclusive<u16>>,
    /// Positive number of distinct ports; must fit within the range
    ///
    /// All requested ports must be acquired or allocation fails and releases any
    /// partial reservations.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u16).range(1..))]
    pub count: u16,
    /// Hold sockets until Ctrl-C, SIGTERM, or timeout
    #[arg(long)]
    pub reserve: bool,
    /// Release after this positive number of seconds; requires --reserve
    #[arg(long, requires = "reserve", value_parser = clap::value_parser!(u64).range(1..))]
    pub timeout: Option<u64>,
    /// Output format (default: compact single-line JSON)
    #[arg(long, short = 'F', value_enum, default_value_t = OutputFormat::Compact)]
    pub format: OutputFormat,
    /// Human-readable output (same as --format human)
    #[arg(long, conflicts_with_all = ["format", "pretty"])]
    pub human: bool,
    /// Pretty-printed JSON output (same as --format json)
    #[arg(long, conflicts_with = "format")]
    pub pretty: bool,
    /// Bare port numbers, one per line; overrides successful output formatting
    #[arg(long)]
    pub plain: bool,
}

/// Build the `clap::Command` definition for `mkd-freeport`.
#[allow(dead_code)]
pub fn cli_command() -> clap::Command {
    Args::command()
}

pub fn parse_range(value: &str) -> Result<std::ops::RangeInclusive<u16>, String> {
    let (start, end) = value.split_once('-').ok_or("Use START-END")?;
    let start: u16 = start.parse().map_err(|_| "Invalid range start")?;
    let end: u16 = end.parse().map_err(|_| "Invalid range end")?;
    if start == 0 || start > end {
        return Err("Range must satisfy 1 <= START <= END <= 65535".into());
    }
    Ok(start..=end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_validation() {
        assert_eq!(parse_range("1-65535").unwrap(), 1..=65535);
        for value in ["0-1", "65536-65537", "2-1", "1", "-2", "one-two"] {
            assert!(parse_range(value).is_err());
        }
    }

    #[test]
    fn defaults_and_argument_validation() {
        let args = Args::try_parse_from(["mkd-freeport"]).unwrap();
        assert_eq!(args.protocol, "tcp");
        assert_eq!(args.count, 1);
        assert!(!args.reserve);
        for args in [
            vec!["mkd-freeport", "--count", "0"],
            vec!["mkd-freeport", "--timeout", "1"],
            vec!["mkd-freeport", "--reserve", "--timeout", "0"],
            vec!["mkd-freeport", "--protocol", "sctp"],
            vec!["mkd-freeport", "--bind", "localhost"],
            vec!["mkd-freeport", "--bind", "fe80::1%eth0"],
        ] {
            assert!(Args::try_parse_from(args).is_err());
        }
    }
}
