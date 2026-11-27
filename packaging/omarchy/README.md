# Omarchy Packaging for `mkd-freeport`

## Immediate Install via AUR

Once `packaging/arch/PKGBUILD` is published to the AUR as `mkd-freeport`:

```bash
omarchy pkg aur add mkd-freeport
```

## Upstream `omacom/omarchy-pkgs` Recipe

To ship `mkd-freeport` in the curated Omarchy binary repository (`omarchy pkg add mkd-freeport`):

1. Fork/clone `omacom/omarchy-pkgs`.
2. Copy `../arch/PKGBUILD` into `pkgbuilds/mkd-freeport/PKGBUILD` and run `updpkgsums` to pin the release tarball checksum.
3. Copy `./package.json` into `pkgbuilds/mkd-freeport/.omarchy/package.json` so new GitHub releases are tracked automatically.
4. Open a pull request against `omacom/omarchy-pkgs`.
