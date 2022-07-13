{
  description = "Projeto final de Laboratório de Bases de Dados (SCC0541) - 2022/1";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, naersk }:
    let
      inherit (nixpkgs.lib) genAttrs systems;
      forAllSystems = genAttrs systems.flakeExposed;
      pkgsFor = forAllSystems (system: import nixpkgs {
        inherit system; overlays = [ self.overlays.default ];
      });
    in
    {
      # Overlay para adicionar o projeto ao conjunto de pacotes
      overlays = rec {
        default = final: prev: {
          projeto-labbd = prev.callPackage ./nix { inherit naersk; };
        };
      };
      # Módulo para ser super simples criar o serviço no NixOS
      nixosModules = rec {
        default = projeto-labbd;
        projeto-labbd = import ./nix/module.nix;
      };

      packages = forAllSystems (s:
        let pkgs = pkgsFor.${s}; in
        rec {
          inherit (pkgs) projeto-labbd;
          default = projeto-labbd;
        });

      devShells = forAllSystems (s:
        let pkgs = pkgsFor.${s}; in
        rec {
          projeto-labbd = pkgs.mkShell {
            inputsFrom = [ pkgs.projeto-labbd ];
            # Algumas ferramentas úteis
            buildInputs = with pkgs; [ rustc rust-analyzer cargo rustfmt clippy ];
          };
          default = projeto-labbd;
        });
    };
}

