{
  description = "A Rust project using naersk";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    # Note: fenix packages are cached via cachix:
    #       cachix use nix-community
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, naersk, flake-utils, fenix }:
    let
      # If you have a workspace and your binary isn't at the root of the
      # repository, you may need to modify this path.
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      name = cargoToml.package.name;
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default
          ];
        };
        components = [
          "cargo"
          "rustc"
        ];
        dev-components = components ++ [
          "clippy"
          "rustfmt"
          "rust-src"
        ];
        toolchain = pkgs.fenix.complete.withComponents components;
        dev-toolchain = pkgs.fenix.complete.withComponents dev-components;
        naersk-lib = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };
        fix-n-fmt = pkgs.writeShellScriptBin "fix-n-fmt" ''
          set -euf -o pipefail
          ${dev-toolchain}/bin/cargo clippy --fix --allow-staged --allow-dirty --all-targets --all-features
          ${dev-toolchain}/bin/cargo fmt
        '';
        defaultPackage = naersk-lib.buildPackage {
          pname = name;
          root = ./.;
        };
      in
      rec {
        inherit defaultPackage;

        packages = builtins.listToAttrs [{ inherit name; value = defaultPackage; }];

        # Update the `program` to match your binary's name.
        defaultApp = {
          type = "app";
          program = "${defaultPackage}/bin/hello";
        };

        devShell = pkgs.mkShell {
          inputsFrom = [
            defaultPackage
          ];
          buildInputs = with pkgs; [
            dev-toolchain
            rust-analyzer
            clang
            lld
            fix-n-fmt
            nodejs
            # Wrap sqlite3 in a shell script that enables foreign keys by default.
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
          RUST_SRC_PATH = "${dev-toolchain}/lib/rustlib/src/rust/library";
          DATABASE_URL = "sqlite://db.sqlite";
        };
      }
    );
}
