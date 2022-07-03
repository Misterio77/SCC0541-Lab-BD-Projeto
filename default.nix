{ lib, rustPlatform }:

let manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  inherit (manifest) version;

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  postInstall = ''
    mkdir -p $out/etc
    cp -r templates db $out/etc
  '';
}
