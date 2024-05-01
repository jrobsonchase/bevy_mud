{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    crate2nix.url = "github:nix-community/crate2nix";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, utils, nixpkgs, crate2nix, ... }@inputs: utils.lib.eachDefaultSystem (system:
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
        };
      };

    in
    {
      # inherit cargoWorkspace;
      # packages.default = cargoWorkspace.rootCrate.build;
      devShells.default = pkgs.mkShell {
        # inputsFrom = [
        #   cargoWorkspace.rootCrate.build
        # ];
        buildInputs = with pkgs; [
          lua-language-server
          sqlite-wrapped
          tintin
          rust-analyzer-nightly
          fenix.complete.clippy
          fenix.complete.rustfmt
          fenix.complete.rustc
          fenix.complete.cargo
        ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        RUST_BACKTRACE = "true";
        DATABASE_URL = "sqlite://base-db.sqlite";
        RUSTC_WRAPPR = "${pkgs.sccache}/bin/sccache";
      };
    });
}
