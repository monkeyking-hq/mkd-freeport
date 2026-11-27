// SPDX-FileCopyrightText: 2026 Nathan A Chadwick
//
// SPDX-License-Identifier: MIT

//! Workspace build helper for generating man pages and shell completions.

#[allow(dead_code)]
#[path = "../../src/cli.rs"]
mod cli;

use clap::{Arg, Command};
use clap_complete::{generate_to, shells};
use clap_mangen::Man;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

const ROOT_CARGO_TOML: &str = include_str!("../../Cargo.toml");

fn package_version() -> &'static str {
    let mut in_package = false;
    for line in ROOT_CARGO_TOML.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
            continue;
        }
        if in_package {
            if let Some(rest) = trimmed.strip_prefix("version") {
                let rest = rest.trim_start();
                if let Some(rest) = rest.strip_prefix('=') {
                    let val = rest.trim().trim_matches('"');
                    if !val.is_empty() {
                        return val;
                    }
                }
            }
        }
    }
    panic!("Could not determine [package] version from root Cargo.toml");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("xtask")
        .about("Build and packaging helper for mkd-freeport")
        .subcommand_required(true)
        .subcommand(
            Command::new("dist-assets")
                .about("Generate man pages, plain-text help, and shell completions")
                .arg(
                    Arg::new("out-dir")
                        .long("out-dir")
                        .value_name("DIR")
                        .default_value("dist/assets")
                        .help("Output directory for generated assets"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("dist-assets", sub)) => {
            let out_dir = PathBuf::from(
                sub.get_one::<String>("out-dir")
                    .expect("out-dir has default"),
            );
            generate_dist_assets(&out_dir)?;
        }
        _ => unreachable!("subcommand_required is true"),
    }

    Ok(())
}

fn generate_dist_assets(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let man_dir = out_dir.join("man");
    let comp_dir = out_dir.join("completions");
    fs::create_dir_all(&man_dir)?;
    fs::create_dir_all(&comp_dir)?;

    let version = package_version();
    let mut cmd = cli::cli_command().version(version);

    // Render troff man page
    let man = Man::new(cmd.clone());
    let mut man_buf = Vec::new();
    man.render_title(&mut man_buf)?;
    man.render_name_section(&mut man_buf)?;
    man.render_synopsis_section(&mut man_buf)?;
    man.render_description_section(&mut man_buf)?;
    man.render_options_section(&mut man_buf)?;
    write_extra_man_sections(&mut man_buf)?;
    fs::write(man_dir.join("mkd-freeport.1"), man_buf)?;

    // Render plain-text help for platforms without troff/man (e.g. Windows archives)
    let long_help = cmd.render_long_help().to_string();
    let mut txt_buf = String::new();
    txt_buf.push_str(&format!("mkd-freeport {version}\n"));
    txt_buf.push_str("==================\n\n");
    txt_buf.push_str(&long_help);
    if !long_help.ends_with('\n') {
        txt_buf.push('\n');
    }
    txt_buf.push_str("\nExit Status:\n  0  Success.\n  1  Invalid arguments, signal error, exhausted range, or socket bind failure.\n");
    fs::write(man_dir.join("mkd-freeport.1.txt"), txt_buf)?;

    // Generate shell completions
    generate_to(shells::Bash, &mut cmd, "mkd-freeport", &comp_dir)?;
    generate_to(shells::Zsh, &mut cmd, "mkd-freeport", &comp_dir)?;
    generate_to(shells::Fish, &mut cmd, "mkd-freeport", &comp_dir)?;
    generate_to(shells::PowerShell, &mut cmd, "mkd-freeport", &comp_dir)?;

    Ok(())
}

fn write_extra_man_sections(buf: &mut Vec<u8>) -> std::io::Result<()> {
    writeln!(
        buf,
        r#".SH EXAMPLES
Find one TCP/IPv4 port (compact JSON output):
.PP
.nf
mkd-freeport
.fi
.PP
Find the first available port above 1024:
.PP
.nf
mkd-freeport --first
.fi
.PP
Find three distinct UDP ports on IPv6:
.PP
.nf
mkd-freeport --protocol udp --ipv6 --count 3
.fi
.PP
Search an inclusive custom range on IPv4 loopback in ascending order:
.PP
.nf
mkd-freeport --bind 127.0.0.1 --range 49152-50000 --first --count 2
.fi
.PP
Hold two TCP ports in the foreground for 30 seconds:
.PP
.nf
mkd-freeport --reserve --bind 127.0.0.1 --count 2 --timeout 30
.fi
.PP
Print bare port numbers for shell scripts:
.PP
.nf
PORT=$(mkd-freeport --plain)
.fi
.SH OUTPUT
Default output is a single-line compact JSON envelope:
.PP
.nf
{{"result":"OK","message":"Free ports found","data":{{"port":49152,"strategy":"random","protocol":"tcp","bind_address":"0.0.0.0","reserved":false}}}}
.fi
.PP
One port uses \fBdata.port\fR; multiple ports use \fBdata.ports\fR instead.
Reservations report message \fB"Ports reserved"\fR and \fBdata.reserved\fR set to \fBtrue\fR.
JSON errors include \fBresult: "ERROR"\fR, \fBlevel\fR, \fBmessage\fR, and \fBerror_code\fR.
Human-readable errors (\fB--human\fR / \fB--format human\fR) are written to stderr.
.SH "EXIT STATUS"
.TP
.B 0
Success.
.TP
.B 1
Invalid arguments, signal handler failure, exhausted port range, or socket bind error.
.SH "SEE ALSO"
Project homepage and source: https://github.com/monkeyking-hq/mkd-freeport
.SH LICENSE
MIT License. Copyright (c) 2026 Nathan A Chadwick."#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_man_page_help_and_completions() {
        let dir = std::env::temp_dir().join(format!("mkd-freeport-xtask-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        generate_dist_assets(&dir).expect("dist-assets generation");

        let man_path = dir.join("man/mkd-freeport.1");
        let txt_path = dir.join("man/mkd-freeport.1.txt");
        let man_src = fs::read_to_string(&man_path).expect("read man page");
        let txt_src = fs::read_to_string(&txt_path).expect("read plain help");

        let version = package_version();
        assert!(man_src.contains(".TH mkd-freeport 1"));
        assert!(man_src.contains(version));
        assert!(man_src.contains("EXAMPLES"));
        assert!(man_src.contains("EXIT STATUS"));
        assert!(man_src.contains("--reserve"));
        assert!(man_src.contains("--plain"));
        assert!(txt_src.contains(version));
        assert!(txt_src.contains("--reserve"));

        for file in [
            "completions/mkd-freeport.bash",
            "completions/_mkd-freeport",
            "completions/mkd-freeport.fish",
            "completions/_mkd-freeport.ps1",
        ] {
            let content = fs::read_to_string(dir.join(file)).expect(file);
            assert!(!content.trim().is_empty(), "{file} should not be empty");
        }

        let _ = fs::remove_dir_all(&dir);
    }
}
