# Marxist Quote

```text
AI image generation prompt:
Create a wide README hero banner for a Linux desktop app named "Marxist Quote", 16:9 aspect ratio. Show a quiet GNOME/libadwaita-style desktop scene with a soft wallpaper, a small translucent quote overlay near the lower third, and a compact settings window blurred slightly in the background. Use refined line work, subtle shadows, and a restrained palette of graphite, warm cream, deep red accents, and near-black text. The mood should feel thoughtful, literary, and political-theory adjacent without showing real political symbols, faces, flags, books with readable copyrighted covers, or propaganda imagery. Leave clean negative space on the left side for the project title and keep all UI text either absent or intentionally unreadable except for the word "Marxist Quote" if needed.
```

Marxist Quote is a small Rust desktop quote system. It fetches quotes, stores the current quote state, and provides a settings UI with a placement tool for positioning the quote text.

The Arch package builds four binaries:

- `marxist_quote`: graphical settings UI
- `wikiquote-fetcher`: quote fetch/update helper for WikiQuote
- `desktop-quote`: desktop quote display
- `position-containers`: position containers tool used by the settings UI

## How It Fits Together

```text
AI image generation prompt:
Create a clean technical architecture diagram for a Rust Linux desktop application named "Marxist Quote", suitable for a GitHub README. Use a flat vector style on a light background with four clearly labeled component boxes: "marxist_quote settings UI", "wikiquote-fetcher", "desktop quote display", and "position containers tool". Add two small data boxes labeled "configuration" and "quote state". Draw readable arrows showing: settings UI writes configuration; settings UI launches engyls-place; engyls-place saves appearance settings back to configuration; wikiquote-fetcher updates quote state; quote state feeds desktop quote display. Include a small user-session boundary around the runtime components and a subtle Rust/Linux visual language through simple icons, not logos. Keep labels large, horizontal, and high contrast; avoid decorative complexity, tiny text, 3D effects, and busy backgrounds.
```

At runtime, Marxist Quote separates the user-facing settings UI, the WikiQuote fetcher, the display process, and the position containers tool. `wikiquote-fetcher` refreshes the cached quote state, `desktop-quote` is the desktop quote display, and `position-containers` is launched from the settings flow to save appearance values back to configuration. The package also installs user-level systemd units so quote updates can run automatically.

```text
User session
  |
  +-- marxist_quote
  |     edits settings
  |
  +-- wikiquote-fetcher
  |     refreshes quote data from WikiQuote
  |
  +-- marxist-quote-fetch.timer
  |     schedules quote refreshes
  |
  +-- desktop-quote
  |     desktop quote display
  |
  +-- position-containers
        position containers for settings
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

```text
AI image generation prompt:
Create a precise installation map for an Arch Linux package named "engyls", suitable for package documentation. Use a clean two-column diagram: the left column is labeled "package files" and the right column is labeled "install destination". Show the four binaries flowing into "/usr/bin": "marxist_quote", "wikiquote-fetcher", "desktop-quote", and "position-containers". Show the user systemd files flowing into "/usr/lib/systemd/user": "desktop-quote.service", "marxist-quote-fetch.service", and "marxist-quote-fetch.timer". Show "engyls.ico" flowing into "/usr/share/pixmaps". Use crisp monochrome line art with one muted accent color, folder/file icons, straight connector lines, and large readable labels. Avoid cute metaphors, shelves, mascots, 3D packaging boxes, and generic Linux clip art.
```

The `PKGBUILD` installs:

- `/usr/bin/marxist_quote`
- `/usr/bin/wikiquote-fetcher`
- `/usr/bin/desktop-quote`
- `/usr/bin/position-containers`
- `/usr/share/applications/marxist_quote.desktop`
- `/usr/lib/systemd/user/desktop-quote.service`
- `/usr/lib/systemd/user/marxist-quote-fetch.service`
- `/usr/lib/systemd/user/marxist-quote-fetch.timer`
- `/usr/share/pixmaps/engyls.ico`

## Services

After installation with `makepkg -si` or an AUR helper, reload the user systemd manager and enable the display service plus fetch timer:

```sh
systemctl --user daemon-reload
systemctl --user enable --now desktop-quote.service marxist-quote-fetch.timer
```

You can start the position containers tool manually with:

```sh
position-containers
```

## Repository Layout

```text
.
|-- assets/              systemd user units
|-- engyls/              shared library code (used by other components)
|-- wikiquote-fetcher/
|   `-- src/             WikiQuote fetch/update binary and library
|-- position-containers/ position containers settings tool
|-- desktop-quote/       desktop quote display binary
|-- gui/                 GTK/libadwaita settings UI
|-- PKGBUILD             Arch package recipe
`-- icon.ico             package icon placeholder
```
