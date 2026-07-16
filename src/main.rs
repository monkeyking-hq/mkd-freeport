// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: LicenseRef-FFM-1.0

//! mkd-freeport — find free TCP ports and operate the FS7 docker reaper
//! (`mkd-docker-reap-smoke`).
//!
//! Default mode (no subcommand): allocate and return a free random TCP port
//! for agents and scripts (see mkd#81).
//!
//! `reap-smoke` subcommand: thin Rust wrapper around the bash reaper
//! `mkd-docker-reap-smoke.sh` shipped in `mkd` (deploy/docker/scripts/).
//! Default action is dry-run; pass `--yes` to actually reap. The bash
//! script remains the canonical engine for docker-inspect / docker-rm /
//! docker-compose interactions and remains the auditable source of
//! behaviour — see `qa-gate-hygiene-contract-spec.md` §3 Non-goals.

mod output;
mod reap_smoke;

use clap::{Parser, Subcommand};
use output::{print_error, print_plain_port, print_success, OutputFormat, PortStrategy};
use std::net::TcpListener;
use std::process::ExitCode;

/// Free-port finder + FS7 docker reaper operator surface.
#[derive(Parser, Debug)]
#[command(name = "mkd-freeport")]
#[command(
    version,
    about = "Find free TCP ports and run the FS7 docker reaper — default subcommand is `find`",
    long_about = None,
    arg_required_else_help = false
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Find and return a free TCP port (default when no subcommand given).
    Find(FindArgs),

    /// Reap stale QA-gate and cutover dry-run Compose projects (FS7).
    ///
    /// Thin wrapper around `mkd-docker-reap-smoke.sh`. All safety properties
    /// (deny list, `--keep-volumes` default, age thresholds, dry-run by default)
    /// are owned by the bash script; this binary is the operator entry point
    /// that's always available, has no runtime deps, and survives even when
    /// `mkd-cli` is not installed.
    ReapSmoke(reap_smoke::ReapSmokeArgs),
}

#[derive(Parser, Debug)]
struct FindArgs {
    /// Return the first available port above 1024 instead of a random port.
    #[arg(long)]
    first: bool,

    /// Output format for results (default: compact single-line JSON envelope).
    #[arg(long, short = 'F', value_enum, default_value_t = OutputFormat::Compact)]
    format: OutputFormat,

    /// Human-readable text output (shorthand for `--format human`).
    #[arg(long, conflicts_with = "format")]
    human: bool,

    /// Pretty-printed JSON envelope (shorthand for `--format json`).
    #[arg(long, conflicts_with = "format")]
    pretty: bool,

    /// Legacy bare port number only (pre-0.2.0 behavior for shell capture).
    #[arg(long)]
    plain: bool,
}

impl FindArgs {
    fn output_format(&self) -> OutputFormat {
        if self.human {
            OutputFormat::Human
        } else if self.pretty {
            OutputFormat::Json
        } else {
            self.format
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        // Backward-compatible: `mkd-freeport` (no subcommand) defaults to `find`.
        None => {
            // Built once with clap's Default for FindArgs.
            let find_args = FindArgs {
                first: false,
                format: OutputFormat::Compact,
                human: false,
                pretty: false,
                plain: false,
            };
            run_find(find_args)
        }
        Some(Command::Find(args)) => run_find(args),
        Some(Command::ReapSmoke(args)) => match reap_smoke::run(&args) {
            Ok(code) => ExitCode::from(code),
            Err(msg) => {
                eprintln!("reap-smoke error: {msg}");
                ExitCode::from(2)
            }
        },
    }
}

fn run_find(args: FindArgs) -> ExitCode {
    let format = args.output_format();
    let strategy = if args.first {
        PortStrategy::First
    } else {
        PortStrategy::Random
    };

    match allocate_port(args.first) {
        Ok(port) => {
            if args.plain {
                if let Err(e) = print_plain_port(port) {
                    eprintln!("Error: failed to write output: {e}");
                    return ExitCode::from(1);
                }
            } else if let Err(e) = print_success(port, strategy, format) {
                eprintln!("Error: failed to write output: {e}");
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
        Err(code) => {
            if let Err(e) = print_error("No free ports available", code, format) {
                eprintln!("Error: failed to write output: {e}");
            }
            ExitCode::from(1)
        }
    }
}

fn allocate_port(first: bool) -> Result<u16, &'static str> {
    if first {
        find_first_free_port().ok_or("NO_FREE_PORT")
    } else {
        Ok(find_random_free_port())
    }
}

fn find_first_free_port() -> Option<u16> {
    (1025..=65535).find(|&port| is_port_free(port))
}

fn find_random_free_port() -> u16 {
    let mut attempts = 0;
    let max_attempts = 1000;

    while attempts < max_attempts {
        let port = rand_port();
        if is_port_free(port) {
            return port;
        }
        attempts += 1;
    }

    find_first_free_port().unwrap_or_else(|| {
        // Extremely unlikely if the host has any free port.
        std::process::abort();
    })
}

fn rand_port() -> u16 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(49152..=65535)
}

fn is_port_free(port: u16) -> bool {
    TcpListener::bind(("0.0.0.0", port)).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_port_free() {
        assert!(is_port_free(0));
    }

    #[test]
    fn test_find_first_free_port() {
        let port = find_first_free_port().expect("expected a free port");
        assert!(port > 1024);
    }

    #[test]
    fn test_find_random_free_port() {
        let port = find_random_free_port();
        assert!(port >= 49152);
    }
}
