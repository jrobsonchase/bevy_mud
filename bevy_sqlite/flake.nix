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
        ];
      };
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-analyzer-nightly
          taplo
          fenix.complete.clippy
          fenix.complete.rustfmt
          fenix.complete.cargo
          fenix.complete.rustc
          sqlx-cli
          (symlinkJoin {
            name = "sqlite";
            paths = [
              (writeShellScriptBin "sqlite3" ''
                exec ${sqlite-interactive}/bin/sqlite3 -cmd "PRAGMA foreign_keys = on" -column -header "$@"
              '')
              sqlite-interactive
            ];
          })
        ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        RUSTC_WRAPPR = "${pkgs.sccache}/bin/sccache";
        RUST_BACKTRACE = "true";
        DATABASE_URL = "sqlite://base-db.sqlite";
      };
    });
}
