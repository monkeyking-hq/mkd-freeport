# Installation

## Cargo & `cargo-binstall`

```bash
# Build from crates.io
cargo install mkd-freeport --locked

# Or download a prebuilt binary
cargo binstall mkd-freeport
```

## Arch Linux & Omarchy

```bash
# Omarchy (via AUR)
omarchy pkg aur add mkd-freeport

# Arch Linux
paru -S mkd-freeport
```

## Debian / Ubuntu (`.deb`)

Download the `amd64` or `arm64` `.deb` from [GitHub Releases](https://github.com/monkeyking-hq/mkd-freeport/releases/latest):

```bash
sudo apt install ./mkd-freeport_*_amd64.deb
```

## Fedora & EPEL (`.rpm`)

Download the `x86_64` or `aarch64` `.rpm` from [GitHub Releases](https://github.com/monkeyking-hq/mkd-freeport/releases/latest):

```bash
sudo dnf install ./mkd-freeport-*.x86_64.rpm
```

## macOS (Homebrew)

```bash
brew install monkeyking-hq/tap/mkd-freeport
```

## Windows (Winget & Scoop)

```powershell
# Scoop
scoop bucket add monkeyking.dev https://github.com/monkeyking-hq/scoop-bucket
scoop install mkd-freeport

# Winget
winget install MonkeyKingDev.mkd-freeport
```
