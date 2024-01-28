{ rustPlatform, fetchFromGitHub }:
rustPlatform.buildRustPackage
rec {
  pname = "tealr_doc_gen";
  version = "697293c83bea080da5b0967eb15e34688ac69c92";

  src = fetchFromGitHub {
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
}
