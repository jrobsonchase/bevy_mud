final: prev: {
  tealr_doc_gen = final.callPackage ./tealr_doc_gen.nix { };
  jaeger = final.callPackage ./jaeger.nix { };
  tracy = prev.tracy.overrideAttrs (attrs: rec {
    version = "0.10";
    src = final.fetchFromGitHub {
      owner = "wolfpld";
      repo = "tracy";
      rev = "v${version}";
      sha256 = "sha256-DN1ExvQ5wcIUyhMAfiakFbZkDsx+5l8VMtYGvSdboPA=";
    };
  });

  # Wrap sqlite3 in a shell script that enables foreign keys by default.
  sqlite-wrapped = final.symlinkJoin {
    name = "sqlite";
    paths = [
      (final.writeShellScriptBin "sqlite3" ''
        exec ${final.sqlite-interactive}/bin/sqlite3 -cmd "PRAGMA foreign_keys = on" -column -header "$@"
      '')
      final.sqlite-interactive
    ];
  };
}
