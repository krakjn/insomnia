with import <nixpkgs> {};
mkShell {
  buildInputs = [
    dbus
    pkg-config
  ];
  PKG_CONFIG_PATH = "${dbus}/lib/pkgconfig";
}

