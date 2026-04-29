pkgname=engyls
pkgver=0.1.0
pkgrel=1
pkgdesc="Desktop quote overlay and settings UI"
arch=('x86_64')
license=('unknown')
depends=('gtk3' 'gtk4' 'libadwaita' 'pango' 'cairo' 'glib2')
makedepends=('cargo')
install="$pkgname.install"
source=()
sha256sums=()

build() {
  cd "$startdir"
  cargo build --release --locked
}

package() {
  cd "$startdir"

  install -Dm755 target/release/gui "$pkgdir/usr/bin/marxist_quote"
  install -Dm755 target/release/engyls-quote "$pkgdir/usr/bin/engyls-quote"
  install -Dm755 target/release/engyls-place "$pkgdir/usr/bin/engyls-place"

  install -Dm644 assets/engyls-quote.service \
    "$pkgdir/usr/lib/systemd/user/engyls-quote.service"
  install -Dm644 assets/marxist-quote-fetch.service \
    "$pkgdir/usr/lib/systemd/user/marxist-quote-fetch.service"
  install -Dm644 assets/marxist-quote-fetch.timer \
    "$pkgdir/usr/lib/systemd/user/marxist-quote-fetch.timer"
}
