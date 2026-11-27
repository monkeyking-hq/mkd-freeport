Name:           mkd-freeport
Version:        0.4.0
Release:        1%{?dist}
Summary:        Discover and reserve TCP/UDP ports over IPv4 and IPv6 for scripts and AI agents

License:        MIT
URL:            https://github.com/monkeyking-hq/mkd-freeport
Source0:        %{url}/archive/refs/tags/v%{version}/%{name}-%{version}.tar.gz
Source1:        %{name}-%{version}-vendor.tar.xz

BuildRequires:  cargo
BuildRequires:  rust >= 1.85

%description
Discover or reserve TCP/UDP ports over IPv4 and IPv6 for scripts, AI agents,
and CI pipelines. Emits compact single-line JSON by default with optional
pretty JSON, human-readable, and bare-port output formats.

%prep
%autosetup -n %{name}-%{version} -a1
mkdir -p .cargo
cat > .cargo/config.toml <<'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

%build
cargo build --release --offline --bin mkd-freeport
cargo run --release --offline -p xtask -- dist-assets --out-dir dist/assets

%check
cargo test --release --offline --workspace

%install
install -Dm755 target/release/mkd-freeport %{buildroot}%{_bindir}/mkd-freeport
install -Dm644 dist/assets/man/mkd-freeport.1 %{buildroot}%{_mandir}/man1/mkd-freeport.1
install -Dm644 dist/assets/completions/mkd-freeport.bash %{buildroot}%{_datadir}/bash-completion/completions/mkd-freeport
install -Dm644 dist/assets/completions/_mkd-freeport %{buildroot}%{_datadir}/zsh/site-functions/_mkd-freeport
install -Dm644 dist/assets/completions/mkd-freeport.fish %{buildroot}%{_datadir}/fish/vendor_completions.d/mkd-freeport.fish

%files
%license LICENSE
%doc README.md
%{_bindir}/mkd-freeport
%{_mandir}/man1/mkd-freeport.1*
%{_datadir}/bash-completion/completions/mkd-freeport
%{_datadir}/zsh/site-functions/_mkd-freeport
%{_datadir}/fish/vendor_completions.d/mkd-freeport.fish

%changelog
* Fri Nov 27 2026 Nathan A Chadwick <nate@monkeyking.dev> - 0.4.0-1
- Initial RPM package for Fedora and EPEL 9/10 with man page and shell completions.
