{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    crate2nix.url = "github:nix-community/crate2nix";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, utils, nixpkgs, crate2nix, ... }@inputs: utils.lib.eachSystem [ "x86_64-linux" ] (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          inputs.fenix.overlays.default
          (import ./nix/overlay.nix)
        ];
      };
      inherit (pkgs) lib;
      inherit (lib) cleanSource cleanSourceWith;
      cargoNix = crate2nix.tools.${system}.generatedCargoNix {
        name = "Cargo.nix";
        src = cleanSource (cleanSourceWith {
          src = ./.;
          # Skip the cargo config when generating
          filter = (name: type: !((baseNameOf (toString name)) == ".cargo"));
        });
      };

      cargoWorkspace = pkgs.callPackage cargoNix {
        buildRustCrateForPkgs = pkgs: with pkgs; buildRustCrate.override {
          rustc = fenix.complete.rustc;
          cargo = fenix.complete.cargo;
          defaultCrateOverrides = pkgs.defaultCrateOverrides // {
            bevy_mud = attrs: {
              SQLX_OFFLINE_DIR = ".sqlx";
            };
            bevy_sqlite = attrs: {
              SQLX_OFFLINE_DIR = ".sqlx";
            };
          };
        };
      };

    in
    {
      inherit cargoWorkspace;
      packages = {
        default = cargoWorkspace.workspaceMembers.bevy_mud.build;
        bevy_sqlite = cargoWorkspace.workspaceMembers.bevy_sqlite.build;
        bevy_async_util = cargoWorkspace.workspaceMembers.bevy_async_util.build;
        bevy_piccolo = cargoWorkspace.workspaceMembers.bevy_piccolo.build;
        piccolo = cargoWorkspace.workspaceMembers.piccolo.build;
      };
      devShells.default = pkgs.mkShell {
        inputsFrom = [
          cargoWorkspace.workspaceMembers.bevy_mud.build
        ];
        buildInputs = with pkgs; [
          lua-language-server
          sqlite-wrapped
          tintin
          sqlx-cli
          rust-analyzer-nightly
          fenix.complete.clippy
          fenix.complete.rustfmt
        ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        RUST_BACKTRACE = "true";
        DATABASE_URL = "sqlite://base-db.sqlite";
        RUSTC_WRAPPR = "${pkgs.sccache}/bin/sccache";
      };
    });
}
