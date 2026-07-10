pkgname=marxist-quote
pkgver=0.1.2
pkgrel=1
pkgdesc="Desktop quote overlay and settings UI"
arch=('x86_64')
license=('unknown')
depends=('engyls' 'wikiquote-fetcher' 'gtk3' 'gtk4' 'libadwaita' 'pango' 'cairo' 'glib2' 'hicolor-icon-theme')
makedepends=('cargo')
optdepends=('desktop-file-utils: update desktop entry cache during install hooks')
install=marxist-quote.install
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

  cargo build --release --locked || true

  RING_OUT=""
  for d in target/release/build/ring-*/out; do
    if [ -d "$d" ]; then
      RING_OUT="$d"
      break
    fi
  done

  if [ -n "$RING_OUT" ] && [ -f "$RING_OUT/libring_core_0_17_14_.a" ]; then
    export RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,--whole-archive -C link-arg=$RING_OUT/libring_core_0_17_14_.a -C link-arg=-Wl,--no-whole-archive"

    libring_rlib=$(ls target/release/deps/libring-*.rlib 2>/dev/null | head -n1 || true)
    if [ -n "$libring_rlib" ]; then
      (cd "$RING_OUT" && ar x libring_core_0_17_14_.a)
      if ls "$RING_OUT"/*.o >/dev/null 2>&1; then
        ar r "$libring_rlib" "$RING_OUT"/*.o || true
        rm -f "$RING_OUT"/*.o
      fi
    fi
  fi

  cargo build --release --locked
}

package() {
  cd "$startdir"

  install -Dm755 target/release/gui "$pkgdir/usr/bin/marxist_quote"
  install -Dm755 target/release/desktop-quote "$pkgdir/usr/bin/desktop-quote"
  install -Dm755 target/release/position-containers "$pkgdir/usr/bin/position-containers"

  install -Dm644 assets/marxist_quote.desktop \
    "$pkgdir/usr/share/applications/marxist_quote.desktop"

  for size in 16 24 32 48 64 128 256; do
    for icon in \
      "assets/icons/hicolor/${size}x${size}/apps/marxist-quote.png" \
      "assets/icons/hicolor/${size}x${size}/apps/marxist_quote_${size}.png" \
      "assets/icons/hicolor/${size}x${size}/marxist-quote.png" \
      "assets/icons/hicolor/${size}x${size}/marxist_quote_${size}.png"; do
      if [ -f "$icon" ]; then
        install -Dm644 "$icon" \
          "$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps/marxist-quote.png"
        break
      fi
    done
  done

  for icon in \
    assets/icons/hicolor/scalable/apps/marxist-quote.svg \
    assets/icons/hicolor/scalable/apps/marxist_quote.svg \
    assets/icons/hicolor/scalable/marxist-quote.svg \
    assets/icons/hicolor/scalable/marxist_quote.svg; do
    if [ -f "$icon" ]; then
      install -Dm644 "$icon" \
        "$pkgdir/usr/share/icons/hicolor/scalable/apps/marxist-quote.svg"
      break
    fi
  done

  install -Dm644 assets/desktop-quote.service \
    "$pkgdir/usr/lib/systemd/user/desktop-quote.service"
  install -Dm644 assets/marxist-quote-fetch.service \
    "$pkgdir/usr/lib/systemd/user/marxist-quote-fetch.service"
  install -Dm644 assets/marxist-quote-fetch.timer \
    "$pkgdir/usr/lib/systemd/user/marxist-quote-fetch.timer"
}
