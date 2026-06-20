# mkd-freeport

Find available free TCP ports for scripts and AI agents.

**Version 0.1.0**

Simple CLI utility that returns an available TCP port on the system. Useful for:
- Scripts that need to start services on random ports
- AI agents programmatically discovering free ports
- CI/CD pipelines requiring dynamic port allocation

## Usage

```bash
# Get a random free ephemeral port (49152-65535)
mkd-freeport

# Get the first available port above 1024
mkd-freeport --first

# Show help
mkd-freeport --help

# Show version
mkd-freeport --version
```

## Install

```bash
cargo install --path .
```

## Output

The utility outputs only the port number to stdout, making it easy to capture in scripts:

```bash
PORT=$(mkd-freeport)
echo "Using port $PORT"
```

## See Also

- [CHANGELOG.md](./CHANGELOG.md)
