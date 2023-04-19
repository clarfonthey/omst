# Maintainer: ltdk <usr@ltdk.xyz>
pkgname=omst
pkgver=3.0.0
pkgrel=1
pkgdesc='Reveals whomst thou art with a single character.'
arch=(aarch64 x86_64)
url="https://vc.ltdk.xyz/cli/omst"
license=(ACSL)
makedepends=(rustup)
source=("$pkgname-v$pkgver.tar.xz")
sha256sums=('8482604e146f97b109feb2ef50acbce2826bd0ebae9e2e041e729b6f73fc7eb1')

prepare() {
    rustup install nightly
}

build() {
    cd "$srcdir"
    cargo +nightly build --release
    mkdir -p usr/bin
    cp target/release/omst usr/bin
    cp target/release/omst-be usr/bin
}

package() {
  cp -R "$srcdir/usr" -T "$pkgdir/usr"
  install -D "$srcdir/LICENSE.md" -T "$pkgdir/usr/share/licenses/omst/LICENSE.md"
}
