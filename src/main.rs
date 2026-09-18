mod output;

use clap::Parser;
use output::{print_error, print_plain_port, print_success, OutputFormat, PortStrategy};
use std::net::TcpListener;

/// Find available free TCP ports for scripts and AI agents.
///
/// Default output is a compact single-line JSON envelope for agents and automation.
/// Use `--human` or `--format human` for readable text, or `--plain` for legacy bare port output.
#[derive(Parser, Debug)]
#[command(name = "mkd-freeport")]
#[command(
    version,
    about = "Find free TCP ports — default output is compact JSON for agents; use --format human or --human for readable text",
    long_about = None
)]
struct Args {
    /// Return the first available port above 1024 instead of a random port
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

impl Args {
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

fn main() {
    let args = Args::parse();
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
                    std::process::exit(1);
                }
            } else if let Err(e) = print_success(port, strategy, format) {
                eprintln!("Error: failed to write output: {e}");
                std::process::exit(1);
            }
        }
        Err(code) => {
            if let Err(e) = print_error("No free ports available", code, format) {
                eprintln!("Error: failed to write output: {e}");
            }
            std::process::exit(1);
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
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random_range(49152..=65535)
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
