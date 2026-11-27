# Winget Packaging (`mkd-freeport`)

`mkd-freeport` is published to `microsoft/winget-pkgs` as **`MonkeyKingDev.mkd-freeport`** using the portable `x86_64-pc-windows-msvc.zip` release asset.

## Automated Updates

`.github/workflows/release.yml` runs `vedantmgoyal9/winget-releaser@v2` on `v*` tag releases when the repository secret `WINGET_TOKEN` (a GitHub PAT with `public_repo` scope for opening PRs to `microsoft/winget-pkgs`) is configured.
