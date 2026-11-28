# Protocols & Bind Addresses

Test availability by actually binding (and listening on TCP) for the exact transport, IP family, and interface address your service will use.

<video autoplay loop muted playsinline controls style="max-width: 100%; border-radius: 8px;">
  <source src="../assets/protocols-and-bind.webm" type="video/webm">
  <img src="../assets/protocols-and-bind.gif" alt="mkd-freeport protocols and bind address demo">
</video>

<details>
<summary>Terminal screenshot</summary>

![mkd-freeport protocols and bind address screenshot](../assets/protocols-and-bind.png)

</details>

## Flags

| Flag | Default | Behavior |
|---|---|---|
| `--protocol tcp\|udp` | `tcp` | Transport protocol (`tcp` binds and listens; `udp` binds datagram socket) |
| `--ipv6` | `false` | Use IPv6 (`::` default bind); cannot combine with an IPv4 `--bind` |
| `--bind IP` | `0.0.0.0` (or `::`) | Literal IPv4 or IPv6 bind address; an IPv6 address implies IPv6 without `--ipv6` |

IPv6 sockets are IPv6-only (`IPV6_V6ONLY`), keeping IPv4 and IPv6 port spaces independent.

## Executable Example

```bash
{{#include ../../../examples/cli/04-protocols-and-bind.sh}}
```
