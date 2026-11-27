# Fedora & EPEL RPM Packaging (`mkd-freeport`)

## Direct `.rpm` from GitHub Releases

Prebuilt `x86_64` and `aarch64` `.rpm` packages (using static musl binaries) are attached to every GitHub Release and tested on `fedora:latest` and `almalinux:9`.

## Building SRPM / COPR for Fedora & EPEL 9/10

`mkd-freeport.spec` performs an offline vendored build against the target distro's system `cargo`/`rustc` and glibc:

```bash
VERSION=0.4.0
git archive --format=tar.gz --prefix="mkd-freeport-${VERSION}/" "v${VERSION}" > "mkd-freeport-${VERSION}.tar.gz"
cargo vendor vendor
tar -cJf "mkd-freeport-${VERSION}-vendor.tar.xz" vendor
rm -rf vendor
rpmbuild -bs packaging/rpm/mkd-freeport.spec \
  --define "_sourcedir $(pwd)" \
  --define "_srcrpmdir $(pwd)/dist"
```

Submit the resulting `.src.rpm` to Fedora COPR with chroots `fedora-41-x86_64`, `fedora-42-x86_64`, `fedora-rawhide-x86_64`, `epel-9-x86_64`, `epel-9-aarch64`, `epel-10-x86_64`, and `epel-10-aarch64`.
