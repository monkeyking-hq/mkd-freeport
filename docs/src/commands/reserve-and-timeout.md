# Foreground Reservations

Hold bound sockets open until `Ctrl-C` (`SIGINT`), `SIGTERM`, or an optional `--timeout` expires.

<video autoplay loop muted playsinline controls style="max-width: 100%; border-radius: 8px;">
  <source src="../assets/foreground-reserve.webm" type="video/webm">
  <img src="../assets/foreground-reserve.gif" alt="mkd-freeport foreground reservations demo">
</video>

<details>
<summary>Terminal screenshot</summary>

![mkd-freeport foreground reservations screenshot](../assets/foreground-reserve.png)

</details>

## Flags

| Flag | Default | Behavior |
|---|---|---|
| `--reserve` | `false` | Print and flush stdout, then hold sockets in the foreground until signal or timeout |
| `--timeout SECONDS` | none | Release and exit after a positive number of seconds; requires `--reserve` |

> **Shell handoff note:** Read reservation output from another process or FIFO. Shell command substitution (`PORT=$(mkd-freeport --reserve)`) waits for the process to exit—by which time the reservation has already been released.

## Executable Example

```bash
{{#include ../../../examples/cli/05-foreground-reserve.sh}}
```
