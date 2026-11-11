mod output;

use clap::Parser;
use mkd_freeport::{reserve_ports, AllocationOptions, Protocol, Strategy};
use output::{print_error, print_success, OutputFormat};
use std::{
    io::{self, Write},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

/// Discover or reserve TCP/UDP ports over IPv4 and IPv6.
///
/// Defaults: one TCP/IPv4 port on 0.0.0.0, selected randomly from 49152-65535,
/// with compact single-line JSON output. Discovery releases sockets before returning.
#[derive(Parser, Debug)]
#[command(
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
  Reservations cover only the chosen transport, bind address and IP family.
  IPv6 sockets are IPv6-only; an IPv6 --bind selects IPv6 without --ipv6.
  Reservations print and flush output, then run in the foreground until Ctrl-C
  (SIGINT), SIGTERM or timeout. Read stdout from another process: shell command
  substitution waits for exit, when the reservation has already been released.
  Release before the service binds; that handoff can still race."#
)]
struct Args {
    /// Search in ascending order (default range: 1025-65535; --range overrides)
    #[arg(long)]
    first: bool,
    /// Transport: tcp or udp
    #[arg(long, value_parser = ["tcp", "udp"], default_value = "tcp")]
    protocol: String,
    /// Use IPv6 (default bind: ::); cannot combine with IPv4 --bind
    #[arg(long)]
    ipv6: bool,
    /// Literal bind IP address; defaults to 0.0.0.0 (or :: with --ipv6)
    #[arg(long)]
    bind: Option<IpAddr>,
    /// Inclusive range START-END (1 <= START <= END <= 65535); overrides defaults
    ///
    /// Default: 49152-65535 with random selection, or 1025-65535 with --first.
    /// Random selection tries candidates without repetition.
    #[arg(long, value_parser = parse_range)]
    range: Option<std::ops::RangeInclusive<u16>>,
    /// Positive number of distinct ports; must fit within the range
    ///
    /// All requested ports must be acquired or allocation fails and releases any
    /// partial reservations.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u16).range(1..))]
    count: u16,
    /// Hold sockets until Ctrl-C, SIGTERM, or timeout
    #[arg(long)]
    reserve: bool,
    /// Release after this positive number of seconds; requires --reserve
    #[arg(long, requires = "reserve", value_parser = clap::value_parser!(u64).range(1..))]
    timeout: Option<u64>,
    /// Output format (default: compact single-line JSON)
    #[arg(long, short = 'F', value_enum, default_value_t = OutputFormat::Compact)]
    format: OutputFormat,
    /// Human-readable output (same as --format human)
    #[arg(long, conflicts_with_all = ["format", "pretty"])]
    human: bool,
    /// Pretty-printed JSON output (same as --format json)
    #[arg(long, conflicts_with = "format")]
    pretty: bool,
    /// Bare port numbers, one per line; overrides successful output formatting
    #[arg(long)]
    plain: bool,
}
fn parse_range(value: &str) -> Result<std::ops::RangeInclusive<u16>, String> {
    let (start, end) = value.split_once('-').ok_or("Use START-END")?;
    let start: u16 = start.parse().map_err(|_| "Invalid range start")?;
    let end: u16 = end.parse().map_err(|_| "Invalid range end")?;
    if start == 0 || start > end {
        return Err("Range must satisfy 1 <= START <= END <= 65535".into());
    }
    Ok(start..=end)
}
fn main() {
    let args = Args::parse();
    let format = if args.human {
        OutputFormat::Human
    } else if args.pretty {
        OutputFormat::Json
    } else {
        args.format
    };
    if let Err((code, message)) = run(&args, format) {
        if let Err(error) = print_error(message, code, format) {
            eprintln!("Cannot write error: {error}");
        }
        std::process::exit(1);
    }
}
fn run(args: &Args, format: OutputFormat) -> Result<(), (&'static str, String)> {
    let bind_address = args.bind.unwrap_or_else(|| {
        if args.ipv6 {
            Ipv6Addr::UNSPECIFIED.into()
        } else {
            Ipv4Addr::UNSPECIFIED.into()
        }
    });
    if args.ipv6 && bind_address.is_ipv4() {
        return Err((
            "INVALID_OPTIONS",
            "--ipv6 conflicts with an IPv4 --bind address".into(),
        ));
    }
    let options = AllocationOptions {
        protocol: if args.protocol == "udp" {
            Protocol::Udp
        } else {
            Protocol::Tcp
        },
        bind_address,
        range: args.range.clone().unwrap_or(if args.first {
            1025..=65535
        } else {
            49152..=65535
        }),
        strategy: if args.first {
            Strategy::First
        } else {
            Strategy::Random
        },
        count: usize::from(args.count),
    };
    let stopped = Arc::new(AtomicBool::new(false));
    if args.reserve {
        for signal in [libc::SIGINT, libc::SIGTERM] {
            let flag = Arc::clone(&stopped);
            // SAFETY: the callback only performs a lock-free atomic store. No allocation,
            // locks or other non-async-signal-safe operations run in signal context.
            unsafe {
                signal_hook_registry::register(signal, move || flag.store(true, Ordering::SeqCst))
            }
            .map_err(|e| ("SIGNAL_FAILED", e.to_string()))?;
        }
    }
    let reservation = reserve_ports(&options).map_err(|e| (e.code(), e.to_string()))?;
    let ports = reservation.ports();
    // Discovery releases before announcing availability; reservations retain ownership.
    let held = if args.reserve {
        Some(reservation)
    } else {
        drop(reservation);
        None
    };
    let started = Instant::now();
    let output = || -> io::Result<()> {
        if args.plain {
            for port in &ports {
                writeln!(io::stdout(), "{port}")?;
            }
        } else {
            print_success(&ports, &options, args.reserve, format)?;
        }
        io::stdout().flush()
    };
    output().map_err(|e| ("OUTPUT_FAILED", e.to_string()))?;
    if held.is_some() {
        while !stopped.load(Ordering::SeqCst) {
            if args
                .timeout
                .is_some_and(|seconds| started.elapsed() >= Duration::from_secs(seconds))
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    Ok(())
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
