# PKGBUILD For lkfs

# Contributor: Janorovic Volkov <janorovicvolkov@gmail.com>
# Maintainer: Janorovic Volkov <janorovicvolkov@gmail.com>

pkgname=lkfs
pkgver=1.0.0
pkgrel=1
pkgdesc="Liska Linux Centralized Filesystem Formatter and Swap Utility"
arch=('x86_64')
url="https://github.com/source-liskalinux/lkfs"
license=('GPL-3.0-or-later')
depends=('e2fsprogs' 'dosfstools' 'util-linux' 'btrfs-progs' 'f2fs-tools' 'xfsprogs' 'ntfs-3g')
makedepends=('rustup')

prepare() {
    cargo check --release --all-targets
}

build() {
    cargo build --release
}

check() {
    cargo test --release
}

package() {
    install -d "${pkgdir}/usr/bin"
    install -Dm755 "./target/release/lkfs" "${pkgdir}/usr/bin/lkfs"
}
