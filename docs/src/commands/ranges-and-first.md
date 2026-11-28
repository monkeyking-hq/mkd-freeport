# Ranges, Ordering & Multi-Port

Control the candidate port pool, search order, and number of distinct ports allocated atomically.

<video autoplay loop muted playsinline controls style="max-width: 100%; border-radius: 8px;">
  <source src="../assets/ranges-and-first.webm" type="video/webm">
  <img src="../assets/ranges-and-first.gif" alt="mkd-freeport ranges and --first demo">
</video>

<details>
<summary>Terminal screenshot</summary>

![mkd-freeport ranges and --first screenshot](../assets/ranges-and-first.png)

</details>

## Flags

| Flag | Default | Behavior |
|---|---|---|
| `--first` | `false` (random) | Search in ascending order from `1025-65535` (unless `--range` is set) |
| `--range START-END` | `49152-65535` (or `1025-65535` with `--first`) | Inclusive range satisfying `1 <= START <= END <= 65535` |
| `--count N` | `1` | Positive count of distinct ports; fails cleanly if all `N` cannot be acquired |

## Executable Example

```bash
{{#include ../../../examples/cli/03-ranges-and-first.sh}}
```
