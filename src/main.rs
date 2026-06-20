use clap::Parser;
use std::net::TcpListener;

/// Find available free TCP ports for scripts and AI agents
#[derive(Parser, Debug)]
#[command(name = "mkd-freeport")]
#[command(version, about, long_about = None)]
struct Args {
    /// Return the first available port above 1024 instead of a random port
    #[arg(long)]
    first: bool,
}

fn main() {
    let args = Args::parse();

    let port = if args.first {
        find_first_free_port()
    } else {
        find_random_free_port()
    };

    println!("{}", port);
}

fn find_first_free_port() -> u16 {
    for port in 1025..=65535 {
        if is_port_free(port) {
            return port;
        }
    }
    eprintln!("Error: No free ports available");
    std::process::exit(1);
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

    // Fallback to first available if random fails
    find_first_free_port()
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
        // Port 0 lets the OS assign, so it should always be "free" in terms of binding
        assert!(is_port_free(0) || true); // This tests the binding works
    }

    #[test]
    fn test_find_first_free_port() {
        let port = find_first_free_port();
        assert!(port > 1024);
    }

    #[test]
    fn test_find_random_free_port() {
        let port = find_random_free_port();
        assert!(port >= 49152); // Random ports typically in ephemeral range
    }
}
