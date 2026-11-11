use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, UdpSocket},
    process::{Child, Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};
fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mkd-freeport"))
}
struct Holder(Child);
impl Drop for Holder {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn holder(extra: &[&str]) -> (Holder, serde_json::Value) {
    let child = command()
        .args(["--reserve", "--bind", "127.0.0.1", "--count", "2"])
        .args(extra)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut holder = Holder(child);
    let stdout = holder.0.stdout.take().unwrap();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut line = String::new();
        let result = BufReader::new(stdout).read_line(&mut line);
        let _ = tx.send((result, line));
    });
    let (result, line) = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("holder must flush stdout promptly");
    result.unwrap();
    let value: serde_json::Value = serde_json::from_str(&line).expect("success JSON");
    assert_eq!(value["result"], "OK", "holder failed: {value}");
    (holder, value)
}
fn wait(holder: &mut Holder) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(status) = holder.0.try_wait().unwrap() {
            assert!(status.success());
            return;
        }
        assert!(Instant::now() < deadline, "holder did not exit");
        std::thread::sleep(Duration::from_millis(20));
    }
}
#[test]
fn timeout_releases_tcp_and_udp() {
    for protocol in ["tcp", "udp"] {
        let (mut holder, value) = holder(&["--protocol", protocol, "--timeout", "1"]);
        assert_eq!(value["data"]["reserved"], true);
        let ports: Vec<_> = value["data"]["ports"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as u16)
            .collect();
        for p in &ports {
            if protocol == "tcp" {
                assert!(TcpListener::bind(("127.0.0.1", *p)).is_err());
            } else {
                assert!(UdpSocket::bind(("127.0.0.1", *p)).is_err());
            }
        }
        wait(&mut holder);
        for p in ports {
            if protocol == "tcp" {
                assert!(TcpListener::bind(("127.0.0.1", p)).is_ok());
            } else {
                assert!(UdpSocket::bind(("127.0.0.1", p)).is_ok());
            }
        }
    }
}
#[cfg(unix)]
#[test]
fn signals_release_successfully() {
    for signal in ["-INT", "-TERM"] {
        let (mut holder, value) = holder(&[]);
        let ports: Vec<_> = value["data"]["ports"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as u16)
            .collect();
        assert!(Command::new("kill")
            .args([signal, &holder.0.id().to_string()])
            .status()
            .unwrap()
            .success());
        wait(&mut holder);
        for p in ports {
            assert!(TcpListener::bind(("127.0.0.1", p)).is_ok());
        }
    }
}
#[test]
fn validates_cli_options() {
    for args in [
        vec!["--range", "0-10"],
        vec!["--range", "20-10"],
        vec!["--count", "0"],
        vec!["--timeout", "1"],
        vec!["--reserve", "--timeout", "0"],
        vec!["--ipv6", "--bind", "127.0.0.1"],
        vec!["--bind", "localhost"],
        vec!["--range", "50000-50001", "--count", "3"],
    ] {
        assert!(
            !command().args(&args).output().unwrap().status.success(),
            "{args:?}"
        );
    }
}
#[test]
fn multiple_output_formats() {
    for extra in [vec![], vec!["--pretty"], vec!["--human"], vec!["--plain"]] {
        let out = command()
            .args(["--count", "2", "--protocol", "udp", "--bind", "127.0.0.1"])
            .args(&extra)
            .output()
            .unwrap();
        assert!(out.status.success());
        let text = String::from_utf8(out.stdout).unwrap();
        if extra.contains(&"--plain") {
            assert_eq!(text.lines().count(), 2);
            for l in text.lines() {
                l.parse::<u16>().unwrap();
            }
        } else if extra.contains(&"--human") {
            assert_eq!(text.lines().filter(|l| l.starts_with("Port ")).count(), 2);
        } else {
            let value: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert!(value["data"].get("port").is_none());
            assert_eq!(value["data"]["ports"].as_array().unwrap().len(), 2);
            assert_eq!(value["data"]["protocol"], "udp");
        }
    }
}

#[test]
fn cli_singleton_range_and_ipv6_inference() {
    // Allocate the custom range at runtime; release immediately before CLI discovery.
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let out = command()
        .args([
            "--bind",
            "127.0.0.1",
            "--first",
            "--range",
            &format!("{port}-{port}"),
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["data"]["port"], port);
    assert_eq!(value["data"]["strategy"], "first");
    assert_eq!(value["data"]["reserved"], false);

    match TcpListener::bind(("::1", 0)) {
        Ok(probe) => drop(probe),
        Err(e)
            if matches!(
                e.kind(),
                std::io::ErrorKind::AddrNotAvailable | std::io::ErrorKind::Unsupported
            ) || e.raw_os_error() == Some(libc::EAFNOSUPPORT) =>
        {
            eprintln!("IPv6 unavailable: {e}");
            return;
        }
        Err(e) => panic!("IPv6 capability probe failed unexpectedly: {e}"),
    }
    for args in [vec!["--bind", "::1"], vec!["--ipv6"]] {
        let out = command().args(args).output().unwrap();
        assert!(out.status.success());
        let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        assert!(value["data"]["bind_address"]
            .as_str()
            .unwrap()
            .contains(':'));
    }
}
