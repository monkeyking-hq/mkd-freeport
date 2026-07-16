// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: LicenseRef-FFM-1.0

//! Machine-first default output contract (mkd#81).
//!
//! Since 0.3.0 the free-port command lives under the `find` subcommand.
//! For backward compatibility, `mkd-freeport` with no args still
//! allocates a port and emits the same JSON envelope.

use std::process::Command;

fn mkd_freeport() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mkd-freeport"))
}

fn mkd_freeport_find() -> Command {
    let mut cmd = mkd_freeport();
    cmd.arg("find");
    cmd
}

#[test]
fn help_documents_machine_first_defaults() {
    let out = mkd_freeport()
        .arg("--help")
        .output()
        .expect("mkd-freeport --help");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    // 0.3.0 surfaces `find` and `reap-smoke` subcommands.
    assert!(
        stdout.contains("find"),
        "help should mention find subcommand: {stdout}"
    );
    assert!(
        stdout.contains("reap-smoke"),
        "help should mention reap-smoke subcommand: {stdout}"
    );
}

#[test]
fn default_stdout_is_compact_json_envelope() {
    let out = mkd_freeport().output().expect("mkd-freeport");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert!(
        stdout.starts_with('{') && stdout.ends_with('}') && !stdout.contains('\n'),
        "expected single-line JSON, got: {stdout}"
    );
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(v.get("result").and_then(|r| r.as_str()), Some("OK"));
    let port = v
        .pointer("/data/port")
        .and_then(|p| p.as_u64())
        .expect("data.port");
    assert!(port > 1024);
}

#[test]
fn human_mode_is_readable_text() {
    let out = mkd_freeport_find()
        .arg("--human")
        .output()
        .expect("mkd-freeport find --human");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Port "));
    assert!(!stdout.trim_start().starts_with('{'));
}

#[test]
fn plain_mode_emits_bare_port() {
    let out = mkd_freeport_find()
        .arg("--plain")
        .output()
        .expect("mkd-freeport find --plain");
    assert!(out.status.success());
    let stdout_buf = String::from_utf8_lossy(&out.stdout);
    let stdout = stdout_buf.trim();
    assert!(stdout.chars().all(|c| c.is_ascii_digit()));
    let port: u16 = stdout.parse().expect("numeric port");
    assert!(port > 1024);
}

#[test]
fn pretty_json_is_multiline() {
    let out = mkd_freeport_find()
        .arg("--pretty")
        .output()
        .expect("mkd-freeport find --pretty");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains('\n'));
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(v.get("result").and_then(|r| r.as_str()), Some("OK"));
}

#[test]
fn reap_smoke_help_subcommand() {
    // Just make sure the subcommand is registered and its --help works
    // (we don't actually invoke the bash script from a unit test).
    let out = mkd_freeport()
        .args(["reap-smoke", "--help"])
        .output()
        .expect("mkd-freeport reap-smoke --help");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("--prefix"),
        "reap-smoke help should mention --prefix: {stdout}"
    );
    assert!(
        stdout.contains("--min-age-hours"),
        "reap-smoke help should mention --min-age-hours: {stdout}"
    );
    assert!(
        stdout.contains("--yes"),
        "reap-smoke help should mention --yes: {stdout}"
    );
}
