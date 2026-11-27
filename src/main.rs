// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: MIT

mod cli;
mod output;

use clap::Parser;
use cli::{Args, OutputFormat};
use mkd_freeport::{reserve_ports, AllocationOptions, Protocol, Strategy};
use output::{print_error, print_success};
use std::{
    io::{self, Write},
    net::{Ipv4Addr, Ipv6Addr},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

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
        let flag = Arc::clone(&stopped);
        ctrlc::set_handler(move || {
            flag.store(true, Ordering::SeqCst);
        })
        .map_err(|e| ("SIGNAL_FAILED", e.to_string()))?;
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
