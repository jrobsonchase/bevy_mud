{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, utils, nixpkgs, ... }@inputs: utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          inputs.fenix.overlays.default
          (import ./nix/overlay.nix)
        ];
      };
      cargoWorkspace = pkgs.callPackage ./Cargo.nix {
        buildRustCrateForPkgs = pkgs: with pkgs; buildRustCrate.override {
          rustc = fenix.complete.rustc;
          cargo = fenix.complete.cargo;
        };
      };
    in
    {
      inherit cargoWorkspace;
      packages.default = cargoWorkspace.rootCrate.build;
      devShells.default = pkgs.mkShell {
        inputsFrom = [
          cargoWorkspace.rootCrate.build
        ];
        buildInputs = with pkgs; [
          crate2nix
          lua-language-server
          sqlite-wrapped
          tintin
          rust-analyzer-nightly
          fenix.complete.clippy
          fenix.complete.rustfmt
        ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        RUST_BACKTRACE = "true";
      };
    });
}
