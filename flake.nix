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

  outputs = { self, nixpkgs, naersk, flake-utils, fenix, ... }@inputs:
    let
      # If you have a workspace and your binary isn't at the root of the
      # repository, you may need to modify this path.
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      name = cargoToml.package.name;
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        lib = nixpkgs.lib;
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default
          ];
        };
        jaeger = pkgs.stdenv.mkDerivation rec {
          pname = "jaeger";
          version = "1.50.0";
          src = pkgs.fetchzip {
            url = "https://github.com/jaegertracing/jaeger/releases/download/v${version}/${pname}-${version}-linux-amd64.tar.gz";
            sha256 = "sha256-CEByUORavOmDDX5hBNc2x5cFV1VIh0HqhYE56fuQVVk=";
          };

          buildPhase = ''
            mkdir -p $out/bin
            cp jaeger-* $out/bin/
          '';
        };
        tealr_doc_gen = pkgs.rustPlatform.buildRustPackage
          rec {
            pname = "tealr_doc_gen";
            version = "697293c83bea080da5b0967eb15e34688ac69c92";

            src = pkgs.fetchFromGitHub {
              owner = "lenscas";
              repo = pname;
              rev = version;
              hash = "sha256-n3+XCLiyWV27QFjehxNVu9q1UkUOyhsBpxvSfFCFwhg=";
            };

            cargoLock = {
              lockFile = ./Cargo.lock.tealr_doc_gen;
              outputHashes = {
                "tealr-0.9.0-alpha4" = "sha256-5FJvbNEiR8s7ENDbFhi8C9Nstio7C8qUDtmOMx+Ixcg=";
              };
            };
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
        inherit defaultPackage;

        packages = builtins.listToAttrs [{ inherit name; value = defaultPackage; }] // { inherit tealr_doc_gen jaeger; };

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
            rust-analyzer-nightly
            fix-n-fmt
            (tracy.overrideAttrs (attrs: rec {
              version = "0.10";
              src = fetchFromGitHub {
                owner = "wolfpld";
                repo = "tracy";
                rev = "v${version}";
                sha256 = "sha256-DN1ExvQ5wcIUyhMAfiakFbZkDsx+5l8VMtYGvSdboPA=";
              };
            }))
            tintin
            gnumake
            tealr_doc_gen
            luajitPackages.tl
            luajitPackages.lua
            jaeger
            lz4
            lldb
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
      ) // {
        hydraJobs = {
          inherit (self.packages) x86_64-linux;
        };
      };
}
