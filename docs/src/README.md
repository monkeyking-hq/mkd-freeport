# mkd-freeport

Discover or reserve TCP/UDP ports over IPv4 and IPv6 for scripts, AI agents, and CI pipelines. Includes a CLI and a public Rust library API.

<video autoplay loop muted playsinline controls style="max-width: 100%; border-radius: 8px;">
  <source src="assets/overview.webm" type="video/webm">
  <img src="assets/overview.gif" alt="mkd-freeport overview demo">
</video>

<details>
<summary>Terminal screenshot</summary>

![mkd-freeport overview screenshot](assets/overview.png)

</details>

## Defaults

- **Protocol**: `tcp`
- **Bind address**: `0.0.0.0` (or `::` with `--ipv6`)
- **Selection**: one random port from `49152-65535` (without repetition)
- **Output**: compact single-line JSON envelope (`{"result":"OK",...}`)
- **Lifetime**: discovery releases sockets before returning; `--reserve` holds sockets in the foreground until `SIGINT`, `SIGTERM`, or `--timeout`

## Explore by Option Group

- [Output Formats](commands/output-formats.md) — `compact`, `--pretty` (`--format json`), `--human`, `--plain`, and `jq`
- [Ranges, Ordering & Multi-Port](commands/ranges-and-first.md) — `--first`, `--range START-END`, `--count N`
- [Protocols & Bind Addresses](commands/protocols-and-bind.md) — `--protocol tcp|udp`, `--ipv6`, `--bind`
- [Foreground Reservations](commands/reserve-and-timeout.md) — `--reserve`, `--timeout`, `SIGINT`/`SIGTERM`, and handoff
- [Rust Library API](rust-api.md) — `get_free_port`, `get_free_ports`, `find_ports`, `reserve_ports`
- [Installation](install.md) & [Man Page & Completions](man-page.md)

All examples in this guide are executable and verified in CI under [`examples/`](https://github.com/monkeyking-hq/mkd-freeport/tree/main/examples).
