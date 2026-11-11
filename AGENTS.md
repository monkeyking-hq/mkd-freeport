<!--
SPDX-FileCopyrightText: 2026 Nathan A Chadwick
SPDX-License-Identifier: MIT
-->

# Agent guide: mkd-freeport

## Read first

In the MKD multi-repository workspace, read these before continuing:

1. Workspace `../AGENTS.md` and `../AGENTS.local.md`.
2. Company/platform instructions: [`../mkd/AGENTS.md`](../mkd/AGENTS.md).
3. Canonical [Change Protocol](../mkd/platform/standards/agent/change-protocol.md).

Follow the [Universal Code](../universal-code/UC-v1.0.0.md) and company rules through those canonical sources rather than duplicating them here. Read local products from disk first.

These sibling paths describe the MKD workspace layout. In a standalone checkout, do not invent missing policy contents or assume access to the private forge; report material missing context and follow the instructions available in that checkout.

## Purpose and source map

`mkd-freeport` is a Rust CLI and library for discovering and reserving TCP/UDP ports over IPv4 and IPv6. Keep it small, dependency-conscious, and useful to scripts, agents, and CI pipelines.

| Path | Responsibility |
| --- | --- |
| `src/lib.rs` | Allocation policy, port discovery, owned reservations, socket binding, and errors |
| `src/main.rs` | CLI arguments/help, validation, signal handling, reservation timeout, and exit status |
| `src/output.rs` | Compact/pretty JSON envelopes and human-readable output |
| `tests/allocation.rs` | Library allocation, exhaustion, cleanup, protocols, and address families |
| `tests/reservation_cli.rs` | CLI reservation lifetime, signals, timeouts, and option validation |
| `tests/cli_output_contract.rs` | CLI output contract |
| `.gitea/workflows/ci.yml` | Required CI commands |
| `Cargo.toml`, `rust-toolchain.toml` | Package version/dependencies and pinned compiler/components |

Use the source, README, and rendered `--help` as references. Keep README examples, CLI help, and library documentation aligned when behavior changes.

## Allocation invariants

- Verify availability by actually binding the selected transport/address/family. A successful TCP check also listens. Process listings alone do not prove availability.
- Discovery releases sockets before returning. It is best-effort; callers must handle a competing bind. Never promise a race-free handoff.
- `Reservation` owns its sockets until dropped. CLI `--reserve` flushes output and holds sockets in the foreground until SIGINT, SIGTERM, or an optional positive timeout.
- Reservations cover only the selected protocol, bind address, and IP family. IPv6 sockets are IPv6-only. Release before another service binds; shell command substitution waits for the reservation process to exit.
- Multi-port requests return distinct ports and succeed as a complete set or release all partial allocations. Preserve cleanup on errors and exit paths.
- Default allocation: one TCP port on `0.0.0.0`, random selection from `49152..=65535`. Random candidates are tried without repetition.
- CLI `--first` searches `1025..=65535` in ascending order unless `--range` overrides it. Library `Strategy::First` changes ordering only; `AllocationOptions::default()` still uses `49152..=65535`.
- Validate positive counts, nonzero ordered inclusive ranges, and count fitting the range before opening sockets. Preserve useful I/O causes for bind failures.

## CLI output contract

- Default: compact single-line JSON; `--pretty` / `--format json` emits pretty JSON.
- Success: `result`, `message`, and `data` containing `strategy`, `protocol`, `bind_address`, and `reserved`. One port uses `data.port`; multiple ports use `data.ports` instead.
- `--human` / `--format human` emits readable output. `--plain` overrides successful output formatting and prints one bare port per line.
- Allocation/runtime errors exit nonzero. JSON errors include `result: "ERROR"`, `level`, `message`, and `error_code`; human errors go to stderr. Clap argument errors use its own diagnostics.
- Keep stdout parseable. Do not add progress messages or diagnostics to successful machine output.

## Build and verification

All Cargo compile, check, lint, test, and run commands **must use `--release`**. See [release-only rule](../mkd/platform/standards/agents/rules/002-cargo-release-only.md). Do not create `target/debug`; remove it immediately if accidentally created. `cargo install --path .` already uses release mode.

Run checks appropriate to the change. For code changes, match CI:

```bash
cargo fmt --all -- --check
cargo clippy --release --workspace --all-features --all-targets -- -D warnings
cargo check --release --workspace --all-targets
cargo test --release --workspace -- --quiet
```

For help changes, also build with `cargo build --release` and inspect `target/release/mkd-freeport -h`, `--help`, and `--version`. For documentation-only changes, verify links/examples and `git diff --check`; avoid unnecessary builds.

Socket tests need permission to create and bind local TCP/UDP sockets. Allocate test ports dynamically with this library, `mkd-freeport`, or OS-selected port `0`; avoid hardcoded service ports and privileged ports below 1024. Reclaim sockets/processes after each test. Skip IPv6 only for confirmed lack of IPv6 support, not unrelated errors.

If socket tests or forge commands fail because of execution restrictions, request the execution permission supported by the session and retry when approved. Do not change allocation semantics, weaken tests, or claim tests passed to work around a restricted environment.

## Delivery in the MKD workspace

- Working forge: **Gitea**, `MonkeyKing.dev/mkd-freeport`; SSH `mkd-git:37374`, HTTP `stonemonkey:37373`. Use **`tea`** (or installed `tea-cli`), **never `gh`**, for this forge. A public GitHub copy does not change the workspace delivery remote.
- Follow the canonical [Change Protocol](../mkd/platform/standards/agent/change-protocol.md): feature branch, commit, push, Gitea PR against `main`; monitor CI, fix failures, resolve review comments, and auto-merge on the forge when green. Never merge into `main` locally. An operator request to leave a PR open overrides auto-merge for that PR.
- Delegate new coding tasks to [The Smith](../mkd-agent/agents/thesmith.md) as a subagent. Keep changes scoped; preserve unrelated work.
- Code-producing changes require a SemVer bump in `Cargo.toml`, a matching `Cargo.lock` update, and a versioned `CHANGELOG.md` entry. Docs-only changes require a dated changelog entry without a version bump.
- After each finished task, post a summary of at most six lines: result/PR, verification, version/changelog, and next task or blocker. Advance to the next unambiguous task in the active backlog when one exists.
- Do not infer publication from a version number or a request for GitHub metadata. Follow the company lifecycle default until an explicit first-public-release declaration updates it.

## Documentation and secrets

Maintain repository-facing documentation here: README, CLI help, Rust API docs, changelog, and this guide. Durable MKD specs, plans, architecture decisions, and verification reports belong in their canonical central MKD document homes per the company guide; do not create a local `platform/docs/` tree.

Never print, commit, or place credentials in URLs, PRs, examples, or logs. Follow company handling for redaction incidents. MKD subject/group: `mkd.projects.mkd-freeport`.
