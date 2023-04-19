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
sha256sums=('663da4f032f2f0388189d165b552d38ee09c0501325f37e4d63e34f8ab44d304')

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
