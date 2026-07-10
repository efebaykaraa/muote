# Marxist Quote

Marxist Quote is the GTK/libadwaita settings UI for the Marxist Quote desktop quote system. It owns its application settings, author weights, and desktop display behavior, and uses `wikiquote-fetcher` for reusable quote fetching.

The Arch package installs one application binary:

- `marxist_quote`: graphical settings UI

## How It Fits Together

At runtime, Marxist Quote provides the user-facing settings UI. `wikiquote-fetcher` is kept as a package dependency so AUR helpers such as `paru` and `yay` install it automatically when it is missing.

```text
User session
  |
  +-- marxist_quote
  |     edits settings
  |
  +-- wikiquote-fetcher
        installed as a dependency for quote data
```

## Building

Build the project with Cargo:

```sh
cargo build --release --locked
```

Build an Arch package from the repository root:

```sh
makepkg -si
```

## Package Contents

The `PKGBUILD` installs:

- `/usr/bin/marxist_quote`
- `/usr/share/applications/marxist_quote.desktop`
- `/usr/share/icons/hicolor/.../apps/marxist-quote.png`
- `/usr/share/licenses/marxist-quote/LICENSE`

## Repository Layout

```text
.
|-- assets/              desktop entry and icon assets
|-- gui/                 GTK/libadwaita settings UI
|-- aur/                 AUR package files
|-- PKGBUILD             local Arch package recipe
`-- LICENSE
```
