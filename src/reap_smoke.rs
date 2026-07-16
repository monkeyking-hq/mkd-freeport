// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: LicenseRef-FFM-1.0

//! `mkd-freeport reap-smoke` — operator entry point for the FS7 docker reaper.
//!
//! This module is a thin Rust wrapper that locates the bash script
//! `mkd-docker-reap-smoke` (shipped in the `mkd` repo at
//! `deploy/docker/scripts/mkd-docker-reap-smoke.sh`) and forwards a
//! whitelisted set of flags to it.
//!
//! Why a wrapper, not a port? See
//! `platform/docs/specs/qa-gate-hygiene-contract-spec.md` §3 Non-goals:
//! the bash script is the auditable source of behaviour for the
//! docker-inspect / docker-rm / docker-compose-down sequence, age parsing,
//! and the deny-list regex. A full Rust port is the explicit follow-on
//! (`mkd-freeport` will grow an own implementation once the spec
//! accumulates enough elapsed failure data to justify the maintenance
//! surface); for now, the Rust binary is the operator's *preferred*
//! entry point — same flags, same output, but a single binary with
//! no runtime deps (no Python, no bash 4+, just the bash script it
//! shells out to).
//!
//! Spec: platform/docs/specs/qa-gate-hygiene-contract-spec.md §3 (reaper
//! design — operator-owned; H2 follow-on is to harden this wrapper and
//! eventually replace the bash implementation).

use clap::Args;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Arguments for the `reap-smoke` subcommand. Intentionally a thin mirror of
/// the bash script's flags — Rust <-> bash contract is round-trippable.
#[derive(Args, Debug)]
pub struct ReapSmokeArgs {
    /// Override the prefix list (repeatable). Default = gate + dryrun + palace-smoke + qa-smoke.
    #[arg(long, value_name = "GLOB")]
    pub prefix: Vec<String>,

    /// Override age thresholds (in hours) for all prefixes.
    #[arg(long, value_name = "HOURS")]
    pub min_age_hours: Option<u64>,

    /// Actually reap (default is dry-run — list candidates, no changes).
    #[arg(long)]
    pub yes: bool,

    /// Default; never use `down -v`. Listed for explicitness only (deny-by-default).
    #[arg(long, conflicts_with = "no_keep_volumes")]
    pub keep_volumes: bool,

    /// DANGER: also remove named volumes. Not recommended.
    #[arg(long)]
    pub no_keep_volumes: bool,

    /// Override log path (default: ${XDG_STATE_HOME:-$HOME/.local/state}/mkd/reap-smoke.log).
    #[arg(long, value_name = "PATH")]
    pub log_file: Option<PathBuf>,

    /// Override path to the bash reaper script. Default: search PATH first,
    /// then `MKD_REAP_SCRIPT` env, then `<mkd-repo>/deploy/docker/scripts/mkd-docker-reap-smoke.sh`.
    #[arg(long, value_name = "PATH")]
    pub reaper_script: Option<PathBuf>,

    /// Show help for the underlying bash reaper instead of running it.
    #[arg(long)]
    pub help_reaper: bool,
}

/// Exit codes returned to the operator (mirrors bash script: 0 = clean /
/// dry-run done, 1 = partial failure, 2 = preflight failure).
pub fn run(args: &ReapSmokeArgs) -> Result<u8, String> {
    let script = locate_reaper_script(args.reaper_script.as_ref())?;
    let mut cmd = Command::new(&script);
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let forwarded = build_argv(args, args.help_reaper).map_err(|e| e.to_string())?;
    for a in forwarded {
        cmd.arg(a);
    }

    let status = cmd.status().map_err(|e| {
        format!(
            "failed to execute {}: {e}\n  hint: ensure mkd-docker-reap-smoke is on PATH or pass --reaper-script",
            script.display()
        )
    })?;

    // Bash exits 0..255; map any out-of-range to 1 on saturate.
    let code = status.code().unwrap_or(1);
    let code = u8::try_from(code).unwrap_or(1);
    Ok(code)
}

/// Build the argv vector to forward to the bash script. Exported for tests.
fn build_argv(args: &ReapSmokeArgs, help_reaper: bool) -> io::Result<Vec<OsString>> {
    let mut v: Vec<OsString> = Vec::new();

    if help_reaper {
        v.push(OsString::from("--help"));
        return Ok(v);
    }

    if args.yes {
        v.push(OsString::from("--yes"));
    }

    for p in &args.prefix {
        v.push(OsString::from("--prefix"));
        v.push(OsString::from(p));
    }

    if let Some(h) = args.min_age_hours {
        v.push(OsString::from("--min-age-hours"));
        v.push(OsString::from(h.to_string()));
    }

    match (args.keep_volumes, args.no_keep_volumes) {
        (true, false) => {
            v.push(OsString::from("--keep-volumes"));
        }
        (false, true) => {
            v.push(OsString::from("--no-keep-volumes"));
        }
        (false, false) => {
            // Default: --keep-volumes. The bash script default is also keep;
            // emit explicitly so the operator's logs are self-documenting.
            v.push(OsString::from("--keep-volumes"));
        }
        (true, true) => unreachable!("clap conflicts_with prevents this"),
    }

    if let Some(ref path) = args.log_file {
        v.push(OsString::from("--log-file"));
        v.push(OsString::from(path));
    }

    Ok(v)
}

/// Resolve the bash reaper script: explicit > MKD_REAP_SCRIPT > PATH >
/// well-known mkd repo location.
fn locate_reaper_script(explicit: Option<&PathBuf>) -> Result<PathBuf, String> {
    if let Some(p) = explicit {
        if !p.exists() {
            return Err(format!("--reaper-script does not exist: {}", p.display()));
        }
        return Ok(p.clone());
    }

    if let Ok(env) = std::env::var("MKD_REAP_SCRIPT") {
        let p = PathBuf::from(env);
        if p.exists() {
            return Ok(p);
        }
    }

    // PATH lookup (slowly; no `which` crate to keep deps minimal).
    if let Ok(paths) = std::env::var("PATH") {
        for dir in paths.split(':') {
            if dir.is_empty() {
                continue;
            }
            let candidate = PathBuf::from(dir).join("mkd-docker-reap-smoke");
            if candidate.exists() {
                return Ok(candidate);
            }
        }
    }

    // Last-ditch: well-known location inside the mkd workspace.
    let fallback = PathBuf::from(
        "/home/nate/workspaces/mkd-workspace/mkd/deploy/docker/scripts/mkd-docker-reap-smoke.sh",
    );
    if fallback.exists() {
        return Ok(fallback);
    }

    Err(
        "could not locate mkd-docker-reap-smoke (PATH lookup failed and MKD_REAP_SCRIPT unset)\n  \
         hint: install the script or pass --reaper-script <path>"
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_args() -> ReapSmokeArgs {
        ReapSmokeArgs {
            prefix: vec![],
            min_age_hours: None,
            yes: false,
            keep_volumes: false,
            no_keep_volumes: false,
            log_file: None,
            reaper_script: None,
            help_reaper: false,
        }
    }

    #[test]
    fn build_argv_dry_run_by_default_keeps_volumes() {
        let v = build_argv(&minimal_args(), false).unwrap();
        assert_eq!(
            v.iter().map(|s| s.to_string_lossy().into_owned()).collect::<Vec<_>>(),
            vec!["--keep-volumes"]
        );
    }

    #[test]
    fn build_argv_yes_mode() {
        let mut a = minimal_args();
        a.yes = true;
        let v = build_argv(&a, false).unwrap();
        assert_eq!(
            v.iter().map(|s| s.to_string_lossy().into_owned()).collect::<Vec<_>>(),
            vec!["--yes", "--keep-volumes"]
        );
    }

    #[test]
    fn build_argv_prefix_repeats() {
        let mut a = minimal_args();
        a.prefix = vec!["mkd-fs*-smoke".into(), "mkd-palace-dryrun".into()];
        let v = build_argv(&a, false).unwrap();
        assert_eq!(
            v.iter().map(|s| s.to_string_lossy().into_owned()).collect::<Vec<_>>(),
            vec![
                "--prefix",
                "mkd-fs*-smoke",
                "--prefix",
                "mkd-palace-dryrun",
                "--keep-volumes",
            ]
        );
    }

    #[test]
    fn build_argv_min_age_hours() {
        let mut a = minimal_args();
        a.min_age_hours = Some(48);
        let v = build_argv(&a, false).unwrap();
        assert_eq!(
            v.iter().map(|s| s.to_string_lossy().into_owned()).collect::<Vec<_>>(),
            vec!["--min-age-hours", "48", "--keep-volumes"]
        );
    }

    #[test]
    fn build_argv_log_file() {
        let mut a = minimal_args();
        a.log_file = Some(PathBuf::from("/tmp/reap.log"));
        let v = build_argv(&a, false).unwrap();
        assert!(v.contains(&OsString::from("--log-file")));
        assert!(v.contains(&OsString::from("/tmp/reap.log")));
    }

    #[test]
    fn build_argv_help_short_circuits() {
        let v = build_argv(&minimal_args(), true).unwrap();
        assert_eq!(v, vec![OsString::from("--help")]);
    }
}
