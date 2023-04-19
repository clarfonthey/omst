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
sha256sums=('06521b747b1062471f7760b930a684c7a8aaaf830c16e888f1d108b1b8e380ee')

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
