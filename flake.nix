{
  description = "Projeto final de Laboratório de Bases de Dados (SCC0541) - 2022/1";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    {
      overlays = rec {
        default = f: p: {
          projeto-labbd = f.callPackage ./. { };
        };
      };
    } //
    (utils.lib.eachDefaultSystem (system:
      let
        inherit (builtins) attrValues;
        pkgs = import nixpkgs { inherit system; overlays = attrValues self.overlays; };
      in
      rec {
        packages = rec {
          inherit (pkgs) projeto-labbd;
          default = projeto-labbd;
        };

        devShells = rec {
          projeto-labbd = pkgs.mkShell {
            inputsFrom = [ packages.projeto-labbd ];
            buildInputs = with pkgs; [ rustc rust-analyzer cargo rustfmt clippy ];
          };
          default = projeto-labbd;
        };
      }));
}

