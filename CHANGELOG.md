## Unreleased

## [0.4.0] - 2026-11-27

### Added
- Cross-platform signal handling via `ctrlc` (supporting Windows `CTRL_C`/`CTRL_BREAK`/`CTRL_CLOSE` alongside Unix `SIGINT`/`SIGTERM`) and cross-platform IPv6 test probes, removing `libc` from runtime dependencies.
- `xtask` workspace helper generating `man/mkd-freeport.1`, plain-text `man/mkd-freeport.1.txt`, and Bash, Zsh, Fish, and PowerShell completions from the shared `clap` CLI definition (`src/cli.rs`).
- Multi-platform GitHub Actions CI matrix (`.github/workflows/ci.yml` for Linux, macOS, Windows) and automated release pipeline (`.github/workflows/release.yml` for static musl Linux, macOS, Windows, `.deb`, `.rpm`, Arch `makepkg` verification, and downstream Homebrew/Scoop/Winget hooks).
- Packaging metadata (`[package.metadata.binstall]`, `[package.metadata.deb]`, `[package.metadata.generate-rpm]`) and distro recipes under `packaging/` for Arch Linux (AUR), Omarchy, Fedora/EPEL (`.spec`), Homebrew, Scoop, and Winget.
- Per-platform installation, man page, and shell completion documentation in `README.md`.

### Changed
- Declared `rust-version = "1.85"` in `Cargo.toml` and enabled `strip = true` in `[profile.release]`.
- Wired Tier 2 HashSeal instruction integrity gate (`.hashseal.json`, `.gitea/workflows/hashseal.yml`, and digest seal on `AGENTS.md`).

### Changed (2026-11-11)
- Documented crates.io CLI installation and the published library dependency.
- Set package repository metadata to the public GitHub mirror.
- Rewrote AGENTS.md with current CLI/library contracts, source map, release-only verification, and socket-test guidance.
- Removed private infrastructure references from package metadata, documentation, comments, and shared agent instructions; kept local agent rules untracked.
- Updated README for 0.3.0 CLI options, output metadata, foreground reservations, and public Rust APIs.

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2026-11-11
### Fixed
- Completed CLI help with output-format descriptions, default ranges, allocation and timeout constraints, JSON examples, and discovery/reservation lifetime and handoff details.

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
- **Breaking:** default stdout is now a compact single-line JSON envelope for scripting and automation.
- Errors use the same envelope shape with `result: ERROR` (or human text on stderr when `--human`).

### Added
- Global `--format compact|json|human` (default: `compact`).
- `--human` and `--pretty` shorthands.
- `--plain` for legacy bare port output (shell `PORT=$(mkd-freeport --plain)`).
- Integration tests for the output contract (`tests/cli_output_contract.rs`).
- `src/output.rs` shared envelope formatting.

## [0.1.1] - 2026-06-20

### Added
- `AGENTS.md` with development instructions and port allocation correctness guidance.
- Changelog entry for the documentation and agent guidance.

## [0.1.0] - 2026-06-20

### Added
- Initial release of `mkd-freeport` CLI utility
- `--version` flag to display version
- `--help` flag to display usage information
- `--first` flag to return the first available port above 1024 (default: random ephemeral port)
- Random port selection from ephemeral range (49152-65535)
- Unit tests for core functionality
