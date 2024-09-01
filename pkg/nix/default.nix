{ stdenv, fetchurl }:

stdenv.mkDerivation {
  name = "insomnia-0.2.0";
  src = ../../target/release/insomnia;

  installPhase = ''
    mkdir -p $out/bin
    cp insomnia $out/bin/
  '';
}
