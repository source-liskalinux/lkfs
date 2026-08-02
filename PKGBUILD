# PKGBUILD For lkfs

# Contributor: Janorovic Volkov <janorovicvolkov@gmail.com>
# Maintainer: Janorovic Volkov <janorovicvolkov@gmail.com>

pkgname=lkfs
pkgver=1.0.0
pkgrel=1
pkgdesc="Liska Linux Centralized Filesystem Formatter and Swap Utility"
arch=('x86_64')
license=('GPL-3.0-or-later')
depends=('e2fsprogs' 'dosfstools' 'util-linux' 'btrfs-progs' 'f2fs-tools' 'xfsprogs' 'ntfs-3g')
makedepends=('rust')

build() {
    echo "--> [BUILD] Compiling...."
    cargo build --release
}

package() {
    echo "--> [INSTALL] Installing lkfs...."
    install -d "${pkgdir}/usr/bin"
    install -Dm755 "${srcdir}/../target/release/lkfs" "${pkgdir}/usr/bin/lkfs"
}