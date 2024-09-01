with import <nixpkgs> {};
mkShell {
  buildInputs = [
    dbus
    pkg-config
    rustup
    #musl
  ];
  PKG_CONFIG_PATH = "${dbus}/lib/pkgconfig";

  shellHook = ''
	rustup default stable
	#rustup target add x86_64-unknown-linux-musl
	#rustup default stable-x86_64-unknown-linux-musl
  '';
}

