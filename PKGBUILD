pkgname=muote
pkgver=0.3.0
pkgrel=3
pkgdesc="Graphical settings UI for Muote"
arch=('x86_64')
url="https://github.com/efebaykaraa/muote"
license=('GPL-3.0-or-later')
depends=(
  'gtk3'
  'gtk4'
  'libadwaita'
  'pango'
  'cairo'
  'glib2'
  'hicolor-icon-theme'
  'wikiquote-fetcher>=1.0.1-4'
)
makedepends=('cargo')
optdepends=('desktop-file-utils: update desktop entry cache during install hooks')
install=muote.install
options=('!lto')
source=()
sha256sums=()

build() {
  cd "$startdir"

  unset RUSTFLAGS
  export RUSTFLAGS=
  unset CARGO_ENCODED_RUSTFLAGS
  export CARGO_ENCODED_RUSTFLAGS=
  export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/gcc
  export CC=gcc

  cargo build --release --locked
}

package() {
  cd "$startdir"

  install -Dm755 target/release/muote-gui "$pkgdir/usr/bin/muote-gui"
  install -Dm755 target/release/muote-background "$pkgdir/usr/bin/muote-background"
  ln -s muote-gui "$pkgdir/usr/bin/muote"

  install -Dm644 assets/muote.desktop \
    "$pkgdir/usr/share/applications/muote.desktop"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"

  for size in 16 24 32 48 64 128 256; do
    for icon in \
      "assets/icons/hicolor/${size}x${size}/apps/muote.png" \
      "assets/icons/hicolor/${size}x${size}/apps/muote_${size}.png" \
      "assets/icons/hicolor/${size}x${size}/muote.png" \
      "assets/icons/hicolor/${size}x${size}/muote_${size}.png"; do
      if [ -f "$icon" ]; then
        install -Dm644 "$icon" \
          "$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps/muote.png"
        break
      fi
    done
  done

  for icon in \
    assets/icons/hicolor/scalable/apps/muote.svg \
    assets/icons/hicolor/scalable/apps/muote.svg \
    assets/icons/hicolor/scalable/muote.svg \
    assets/icons/hicolor/scalable/muote.svg; do
    if [ -f "$icon" ]; then
      install -Dm644 "$icon" \
        "$pkgdir/usr/share/icons/hicolor/scalable/apps/muote.svg"
      break
    fi
  done

  install -Dm644 assets/desktop-quote.service \
    "$pkgdir/usr/lib/systemd/user/desktop-quote.service"
  install -Dm644 assets/muote-fetch.service \
    "$pkgdir/usr/lib/systemd/user/muote-fetch.service"
  install -Dm644 assets/muote-fetch.timer \
    "$pkgdir/usr/lib/systemd/user/muote-fetch.timer"
}
