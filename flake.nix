{
  description = "Projeto final de Laboratório de Bases de Dados (SCC0541) - 2022/1";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    {
      # Overlay para adicionar o projeto ao conjunto de pacotes
      overlays = rec {
        default = f: p: {
          projeto-labbd = f.callPackage ./nix { };
        };
      };
      # Módulo para ser super simples criar o serviço no NixOS
      nixosModules = rec {
        default = projeto-labbd;
        projeto-labbd = import ./nix/module.nix;
      };
    } //
    (utils.lib.eachDefaultSystem (system:
      let
        inherit (builtins) attrValues;
        pkgs = import nixpkgs { inherit system; overlays = attrValues self.overlays; };
      in
      rec {
        # Exportar o pacote (permite usar nix build e nix run)
        packages = rec {
          inherit (pkgs) projeto-labbd;
          default = projeto-labbd;
        };

        # Shell de desenvolvimento
        devShells = rec {
          projeto-labbd = pkgs.mkShell {
            inputsFrom = [ packages.projeto-labbd ];
            # Algumas ferramentas úteis
            buildInputs = with pkgs; [ rustc rust-analyzer cargo rustfmt clippy ];
          };
          default = projeto-labbd;
        };
      }));
}

