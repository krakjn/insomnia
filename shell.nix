with import <nixpkgs> {};
mkShell {
  buildInputs = [
    dbus
    pkg-config
    rustup
    #musl
archlinux-keyring
autoconf
automake
binutils
bison
debugedit
fakeroot
file
findutils
flex
gawk
gcc
gettext
git
gnumake
grep
groff
gzip
libtool
m4
pacman
patch
perl
pkg-config
pkgconf
python3
sed
sudo
texinfo
which
  ];
  PKG_CONFIG_PATH = "${dbus}/lib/pkgconfig";

  shellHook = ''
	rustup default stable
	#rustup target add x86_64-unknown-linux-musl
	#rustup default stable-x86_64-unknown-linux-musl
  '';
}

