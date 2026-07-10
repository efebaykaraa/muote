# AUR package directories

This directory contains two separate AUR package submissions:

- `engyls`: shared Rust configuration/source library package
- `marxist-quote`: desktop overlay, settings GUI, fetcher, position tool, services, and desktop entry

Before publishing a release:

1. Commit and push the release source.
2. Create and push a matching Git tag, for example `v0.1.0`.
3. In each package directory, replace `sha256sums=('SKIP')` with the real archive checksum using `updpkgsums`.
4. Regenerate `.SRCINFO` with `makepkg --printsrcinfo > .SRCINFO`.
5. Publish each directory to its matching AUR Git repository.
