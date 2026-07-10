# Marxist Quote

Marxist Quote is the GTK/libadwaita settings UI for the Marxist Quote desktop quote system. It owns its application settings, author weights, and desktop display behavior, and uses `wikiquote-fetcher` for reusable quote fetching.

The Arch package installs two application binaries:

- `marxist-quote-gui`: graphical settings UI
- `marxist-quote-background`: desktop quote display, plus `--fetch` for quote refreshes

## How It Fits Together

At runtime, Marxist Quote provides the user-facing settings UI and a user service for the desktop overlay. The background binary uses the reusable `wikiquote-fetcher` library at build time for quote fetching.

```text
User session
  |
  +-- marxist_quote
  |     edits settings
  |
  +-- marxist-quote-background
        displays and refreshes quotes
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
- `/usr/bin/marxist-quote-gui`
- `/usr/bin/marxist-quote-background`
- `/usr/share/applications/marxist_quote.desktop`
- `/usr/lib/systemd/user/desktop-quote.service`
- `/usr/lib/systemd/user/marxist-quote-fetch.service`
- `/usr/lib/systemd/user/marxist-quote-fetch.timer`
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
