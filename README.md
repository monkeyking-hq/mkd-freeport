# mkd-freeport

Find available free TCP ports for scripts and AI agents.

**Version 0.2.0**

Simple CLI utility that returns an available TCP port on the system. Useful for:
- Scripts that need to start services on random ports
- AI agents programmatically discovering free ports
- CI/CD pipelines requiring dynamic port allocation

## Usage

```bash
# Default: compact single-line JSON envelope (machine-first / agent-friendly)
mkd-freeport

# Get the first available port above 1024
mkd-freeport --first

# Human-readable text
mkd-freeport --human

# Pretty-printed JSON envelope
mkd-freeport --pretty
# or: mkd-freeport --format json

# Legacy bare port number (pre-0.2.0 shell capture)
mkd-freeport --plain

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

**Default (0.2.0+):** a compact JSON envelope aligned with mkd-cli machine-first output:

```json
{"result":"OK","message":"Free TCP port allocated","data":{"port":49152,"strategy":"random"}}
```

Parse with `jq`:

```bash
PORT=$(mkd-freeport | jq -r '.data.port')
echo "Using port $PORT"
```

For legacy scripts that expect only the port on stdout, use `--plain`:

```bash
PORT=$(mkd-freeport --plain)
```

Refs: mkd#47 (mkd-cli contract), mkd#81 (this tool).

## See Also

- [CHANGELOG.md](./CHANGELOG.md)
