// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: MIT

//! Machine-first default output contract (mkd#81).

use std::process::Command;

fn mkd_freeport() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mkd-freeport"))
}

#[test]
fn help_documents_machine_first_defaults() {
    let out = mkd_freeport()
        .arg("--help")
        .output()
        .expect("mkd-freeport --help");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("compact"),
        "help should document compact default: {stdout}"
    );
    assert!(
        stdout.contains("human"),
        "help should document human mode: {stdout}"
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
    let out = mkd_freeport()
        .arg("--human")
        .output()
        .expect("mkd-freeport --human");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Port "));
    assert!(!stdout.trim_start().starts_with('{'));
}

#[test]
fn plain_mode_emits_bare_port() {
    let out = mkd_freeport()
        .arg("--plain")
        .output()
        .expect("mkd-freeport --plain");
    assert!(out.status.success());
    let stdout_buf = String::from_utf8_lossy(&out.stdout);
    let stdout = stdout_buf.trim();
    assert!(stdout.chars().all(|c| c.is_ascii_digit()));
    let port: u16 = stdout.parse().expect("numeric port");
    assert!(port > 1024);
}

#[test]
fn pretty_json_is_multiline() {
    let out = mkd_freeport()
        .arg("--pretty")
        .output()
        .expect("mkd-freeport --pretty");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains('\n'));
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(v.get("result").and_then(|r| r.as_str()), Some("OK"));
}
