# Output Formats

`mkd-freeport` is machine-first by default: stdout stays parseable with zero progress chatter, while human-readable and bare-port modes are one flag away.

<video autoplay loop muted playsinline controls style="max-width: 100%; border-radius: 8px;">
  <source src="../assets/output-formats.webm" type="video/webm">
  <img src="../assets/output-formats.gif" alt="mkd-freeport output formats demo">
</video>

<details>
<summary>Terminal screenshot</summary>

![mkd-freeport output formats screenshot](../assets/output-formats.png)

</details>

## Flags

| Flag | Alias | Output |
|---|---|---|
| `--format compact` | *(default)* | Single-line JSON envelope |
| `--format json` | `--pretty` | Pretty-printed multi-line JSON envelope |
| `--format human` | `--human` | Human-readable text (errors go to stderr) |
| `--plain` | — | Bare port numbers, one per line (overrides `--format`) |

## JSON Envelope Shape

- **Single port**: `data.port` (integer)
- **Multiple ports (`--count N > 1`)**: `data.ports` (array of integers)
- **Errors**: nonzero exit code; JSON errors emit `result: "ERROR"`, `level`, `message`, and `error_code`.

## Executable Example

```bash
{{#include ../../../examples/cli/01-basic-json.sh}}
```

```bash
{{#include ../../../examples/cli/02-output-formats.sh}}
```
