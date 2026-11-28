# Man Page & Shell Completions

OS packages (`.deb`, `.rpm`, Arch/AUR, Homebrew) install `mkd-freeport(1)` and shell completions automatically:

```bash
man mkd-freeport
```

## Generating Assets Locally

`xtask` renders `man/mkd-freeport.1`, `man/mkd-freeport.1.txt` (plain-text help), and completions for Bash, Zsh, Fish, and PowerShell directly from `src/cli.rs`:

```bash
cargo run --release -p xtask -- dist-assets --out-dir dist/assets
```
