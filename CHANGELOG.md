# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-07-16

### Added (FS7 H2 — operator entry point for the docker reaper)

- **New `reap-smoke` subcommand**: `mkd-freeport reap-smoke [--prefix <glob>...] [--min-age-hours <N>] [--yes] [--keep-volumes|--no-keep-volumes] [--log-file <path>] [--reaper-script <path>] [--help-reaper]`. Thin Rust wrapper around the bash reaper `mkd-docker-reap-smoke.sh` shipped in `mkd`. **Default dry-run** (lists candidates, no reaping); same flags round-trip to bash. Exit codes mirror the bash script (0 = clean / dry-run done, 1 = partial failure, 2 = preflight). Spec: `platform/docs/specs/qa-gate-hygiene-contract-spec.md` §3.
- **Script location**: looks up `mkd-docker-reap-smoke` via (1) explicit `--reaper-script`, (2) `$MKD_REAP_SCRIPT`, (3) `PATH`, (4) well-known `mkd/deploy/docker/scripts/` fallback. No runtime deps for the operator beyond `PATH` access.

### Changed (breaking — subcommand-only CLI)

- Top-level `Args` refactored into `clap::Subcommand`. `mkd-freeport` with no args still allocates a port (backward-compatible), but new syntax is `mkd-freeport find [--first] [--plain] [--human|--pretty|--format compact|json|human]`. Default args unchanged.

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
