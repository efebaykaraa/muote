# Muote

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/9938e8cf-a750-406f-a19f-3bc29a497e79" />

Muote is the GTK/libadwaita settings UI for the Muote desktop quote system. It owns its application settings, author weights, and desktop display behavior, and uses `wikiquote-fetcher` for reusable quote fetching.

## AUR

> [!TIP]
> **muote** is available on the Arch User Repository: [`muote`](https://aur.archlinux.org/packages/muote)
>
> ```sh
> yay -S muote
> ```

The Arch package installs two application binaries:

- `muote-gui`: graphical settings UI
- `muote-background`: desktop quote display, plus `--fetch` for quote refreshes

## How It Fits Together

At runtime, Muote provides the user-facing settings UI and a user service for the desktop overlay. The background binary uses the reusable `wikiquote-fetcher` library at build time for quote fetching.

```text
User session
  |
  +-- muote
  |     edits settings
  |
  +-- muote-background
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

- `/usr/bin/muote`
- `/usr/bin/muote-gui`
- `/usr/bin/muote-background`
- `/usr/share/applications/muote.desktop`
- `/usr/lib/systemd/user/desktop-quote.service`
- `/usr/lib/systemd/user/muote-fetch.service`
- `/usr/lib/systemd/user/muote-fetch.timer`
- `/usr/share/icons/hicolor/.../apps/muote.png`
- `/usr/share/licenses/muote/LICENSE`

## Repository Layout

```text
.
|-- assets/              desktop entry and icon assets
|-- gui/                 GTK/libadwaita settings UI
|-- aur/                 AUR package files
|-- PKGBUILD             local Arch package recipe
`-- LICENSE
```
