## Unreleased

### Changed
- Sibling `AGENTS.md` Change Protocol: read `../mkd/AGENTS.md` first; canonical `platform/standards/agent/change-protocol.md` (SemVer bump for code, `tea` PR, leave open unless Nate asks to merge). No local merge-to-main copy.

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
