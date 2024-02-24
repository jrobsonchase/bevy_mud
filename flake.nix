{
  description = "A Rust project using naersk";

  nixConfig = {
    allow-import-from-derivation = true;
    extra-substituters = [
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, naersk, flake-utils, fenix, ... }@inputs: flake-utils.lib.eachDefaultSystem (system:
    let
      # If you have a workspace and your binary isn't at the root of the
      # repository, you may need to modify this path.
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      name = cargoToml.package.name;
      lib = nixpkgs.lib;
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          fenix.overlays.default
          (import ./nix/overlay.nix)
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
        buildInputs = with pkgs; [
          cargo-binutils
        ];
        nativeBuidInputs = with pkgs; [
        ];
        singleStep = true;
        DATABASE_URL = "sqlite://base-db.sqlite";
      };
    in
    rec {
      packages.default = defaultPackage;
      packages.${name} = defaultPackage;

      # Update the `program` to match your binary's name.
      apps.default = {
        type = "app";
        program = "${defaultPackage}/bin/canton";
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = [
          defaultPackage
        ];
        buildInputs = with pkgs; [
          dev-toolchain
          rust-analyzer-nightly
          fix-n-fmt
          tracy
          tintin
          gnumake
          tealr_doc_gen
          luajitPackages.tl
          luajitPackages.lua
          jaeger
          lz4
          lldb
          sqlite-wrapped
          cargo-edit
          sqlx-cli
        ];
        RUST_SRC_PATH = "${dev-toolchain}/lib/rustlib/src/rust/library";
        DATABASE_URL = "sqlite://db.sqlite";
        NIX_LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
          stdenv.cc.cc
          zlib
        ];
      };
      hydraJobs = packages;
    }
  );
}
