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
        ];
      };
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          taplo
          rust-analyzer-nightly
          fenix.complete.cargo
          fenix.complete.rustc
          fenix.complete.clippy
          fenix.complete.rustfmt
        ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        RUST_BACKTRACE = "true";
      };
    });
}
