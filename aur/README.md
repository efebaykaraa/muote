# AUR package directory

This directory contains the `marxist-quote` AUR package submission.
The separate `engyls` and `wikiquote-fetcher` packages live in sibling repositories.

Before publishing a release:

1. Commit and push the release source.
2. Create and push a matching Git tag, for example `v0.1.0`.
3. Replace `sha256sums=('SKIP')` with the real archive checksums using `updpkgsums`.
4. Regenerate `.SRCINFO` with `makepkg --printsrcinfo > .SRCINFO`.
5. Publish `aur/marxist-quote` to the `marxist-quote` AUR Git repository.
