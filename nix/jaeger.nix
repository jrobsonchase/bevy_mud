{ stdenv, fetchzip }:
stdenv.mkDerivation rec {
  pname = "jaeger";
  version = "1.50.0";
  src = fetchzip {
    url = "https://github.com/jaegertracing/jaeger/releases/download/v${version}/${pname}-${version}-linux-amd64.tar.gz";
    sha256 = "sha256-CEByUORavOmDDX5hBNc2x5cFV1VIh0HqhYE56fuQVVk=";
  };

  buildPhase = ''
    mkdir -p $out/bin
    cp jaeger-* $out/bin/
  '';
}
