## Unreleased

### Changed (2026-09-20 change protocol)
- Change Protocol alignment (2026-09-20): monitor PR CI, fix failures, **auto-merge on the forge when green**, auto-advance to the next task; coding tasks delegated to The Smith (`mkd-agent/agents/thesmith.md`); compact summary after each finished task. Canonical: `mkd/platform/standards/agent/change-protocol.md`.

### Changed
- Sibling `AGENTS.md` Change Protocol: read `../mkd/AGENTS.md` first; canonical `platform/standards/agent/change-protocol.md` (SemVer bump for code, `tea` PR, leave open unless Nate asks to merge). No local merge-to-main copy.

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-11-11
### Added
- Public Rust APIs for single/multiple-port discovery and owned socket reservations.
- TCP/UDP over IPv4 and IPv6, custom bind addresses and inclusive ranges, and distinct multi-port selection.
- Foreground `--reserve` with SIGINT/SIGTERM release and optional timeout.
- Protocol, bind address and reservation metadata in JSON output; multiple ports use `data.ports`.

### Changed
- Random allocation searches its full requested range without repetition; allocation failures return errors instead of aborting.
- IPv6 reservations explicitly use IPv6-only sockets; discovery remains best-effort after sockets are released.

## [0.2.1] - 2026-07-17
### Added
- `rust-toolchain.toml` pinning workspace to Rust 1.97.1 (harmonized across sibling mkd-* repos).

### Removed
- Duplicate `serde_json` dev-dependency in Cargo.toml (was already listed under `[dependencies]`).

## [0.2.0] - 2026-06-29

### Changed
- **Breaking:** default stdout is now a compact single-line JSON envelope (mkd-cli machine-first contract, mkd#81 / mkd#47).
- Errors use the same envelope shape with `result: ERROR` (or human text on stderr when `--human`).

### Added
- Global `--format compact|json|human` (default: `compact`).
- `--human` and `--pretty` shorthands.
- `--plain` for legacy bare port output (shell `PORT=$(mkd-freeport --plain)`).
- Integration tests for the output contract (`tests/cli_output_contract.rs`).
- `src/output.rs` shared envelope formatting.

## [0.1.1] - 2026-06-20

### Added
- `AGENTS.md` following the canonical template from `mkd` (LAWS-EMBED summary + enforcement + contextual corollaries for port allocation truthfulness and stewardship).
- CHANGELOG entry for the documentation + governance addition.

## [0.1.0] - 2026-06-20

### Added
- Initial release of `mkd-freeport` CLI utility
- `--version` flag to display version
- `--help` flag to display usage information
- `--first` flag to return the first available port above 1024 (default: random ephemeral port)
- Random port selection from ephemeral range (49152-65535)
- Unit tests for core functionality

## Unreleased — 2026-09-20 — spec 785 cutover

- **Changed**: AGENTS.md front matter `wing=mkd-freeport` → `group=mkd.projects.mkd-freeport`; `- Wing:` → `- Group:`. Closes issue #20.
