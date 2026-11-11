// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: LicenseRef-FFM-1.0

//! Machine-first CLI output aligned with mkd-cli compact JSON envelope (mkd#47 / mkd#81).

use serde::Serialize;
use std::io::{self, Write};

/// Output format for command results (default: compact single-line JSON envelope).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    /// Canonical single-line JSON envelope (default). Best for agents and scripting with `jq`.
    #[default]
    Compact,
    /// Pretty-printed JSON envelope.
    Json,
    /// Human-readable text (no JSON).
    Human,
}

#[derive(Debug, Clone, Serialize)]
pub struct PortData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<u16>>,
    pub strategy: &'static str,
    pub protocol: &'static str,
    pub bind_address: std::net::IpAddr,
    pub reserved: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum ResultCode {
    Ok,
    Error,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
enum ErrorLevel {
    Error,
}

#[derive(Debug, Clone, Serialize)]
struct CliResponse {
    result: ResultCode,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<ErrorLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PortData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<String>,
}

pub fn print_success(
    ports: &[u16],
    options: &mkd_freeport::AllocationOptions,
    reserved: bool,
    format: OutputFormat,
) -> io::Result<()> {
    match format {
        OutputFormat::Human => {
            let label = if reserved { "reserved" } else { "bind-checked" };
            for port in ports {
                writeln!(
                    io::stdout(),
                    "Port {port} ({}, {}, {}, {label})",
                    options.protocol.as_str(),
                    options.bind_address,
                    options.strategy.as_str()
                )?;
            }
            Ok(())
        }
        OutputFormat::Compact | OutputFormat::Json => {
            let response = CliResponse {
                result: ResultCode::Ok,
                message: if reserved {
                    "Ports reserved"
                } else {
                    "Free ports found"
                }
                .into(),
                level: None,
                data: Some(PortData {
                    port: (ports.len() == 1).then(|| ports[0]),
                    ports: (ports.len() != 1).then(|| ports.to_vec()),
                    strategy: options.strategy.as_str(),
                    protocol: options.protocol.as_str(),
                    bind_address: options.bind_address,
                    reserved,
                }),
                error_code: None,
            };
            emit_json(&response, format)
        }
    }
}

pub fn print_error(
    message: impl Into<String>,
    error_code: &str,
    format: OutputFormat,
) -> io::Result<()> {
    let message = message.into();
    match format {
        OutputFormat::Human => writeln!(io::stderr(), "Error: {message}"),
        OutputFormat::Compact | OutputFormat::Json => {
            let response = CliResponse {
                result: ResultCode::Error,
                message,
                level: Some(ErrorLevel::Error),
                data: None,
                error_code: Some(error_code.to_string()),
            };
            emit_json(&response, format)
        }
    }
}

fn emit_json<T: Serialize>(value: &T, format: OutputFormat) -> io::Result<()> {
    let line = match format {
        OutputFormat::Compact => {
            serde_json::to_string(value).map_err(|e| io::Error::other(e.to_string()))?
        }
        OutputFormat::Json => {
            serde_json::to_string_pretty(value).map_err(|e| io::Error::other(e.to_string()))?
        }
        OutputFormat::Human => unreachable!("human handled separately"),
    };
    writeln!(io::stdout(), "{line}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_and_multiple_shapes() {
        for ports in [vec![50000], vec![50000, 50001]] {
            let data = PortData {
                port: (ports.len() == 1).then(|| ports[0]),
                ports: (ports.len() > 1).then(|| ports.clone()),
                strategy: "random",
                protocol: "udp",
                bind_address: "::1".parse().unwrap(),
                reserved: true,
            };
            let value = serde_json::to_value(data).unwrap();
            assert_eq!(value["reserved"], true);
            assert_eq!(value["bind_address"], "::1");
            assert_eq!(value.get("port").is_some(), ports.len() == 1);
            assert_eq!(value.get("ports").is_some(), ports.len() > 1);
        }
    }
}
