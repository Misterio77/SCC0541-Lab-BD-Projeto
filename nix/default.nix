{ lib, system, naersk }:

let manifest = (lib.importTOML ../Cargo.toml).package;
in
naersk.lib."${system}".buildPackage {
  pname = manifest.name;
  inherit (manifest) version;

  root = lib.cleanSource ../.;

  postInstall = ''
    mkdir -p $out/etc
    cp -r templates db $out/etc
  '';
}
