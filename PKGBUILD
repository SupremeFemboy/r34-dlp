# Maintainer: RustBlyat
pkgname=r34-dlp
pkgver=0.1.0
pkgrel=1
pkgdesc="Rule34 image and video downloader with resume capability"
arch=('x86_64' 'aarch64')
url="https://github.com/RustBlyat/r34-dlp"
license=('MIT')
depends=('gcc-libs' 'openssl')
makedepends=('cargo')
source=()
sha256sums=()

build() {
    cd "${startdir:-.}"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --release --locked
}

check() {
    cd "${startdir:-.}"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --release --locked
}

package() {
    cd "${startdir:-.}"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
