with (import ./. {});
pkgs.mkShell {
  name = "chromaprint_sys";
  buildInputs = shellBuildInputs;
}
