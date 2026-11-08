// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: MIT

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

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PortStrategy {
    First,
    Random,
}

#[derive(Debug, Clone, Serialize)]
pub struct PortData {
    pub port: u16,
    pub strategy: PortStrategy,
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

pub fn print_success(port: u16, strategy: PortStrategy, format: OutputFormat) -> io::Result<()> {
    match format {
        OutputFormat::Human => {
            let label = match strategy {
                PortStrategy::First => "first available above 1024",
                PortStrategy::Random => "random ephemeral",
            };
            writeln!(io::stdout(), "Port {port} ({label}, bind-checked)")
        }
        OutputFormat::Compact | OutputFormat::Json => {
            let response = CliResponse {
                result: ResultCode::Ok,
                message: "Free TCP port allocated".to_string(),
                level: None,
                data: Some(PortData { port, strategy }),
                error_code: None,
            };
            emit_json(&response, format)
        }
    }
}

pub fn print_plain_port(port: u16) -> io::Result<()> {
    writeln!(io::stdout(), "{port}")
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
