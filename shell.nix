with import ./. {};
chromaprint_sys.overrideAttrs (
  drv: {
    buildInputs = drv.buildInputs ++ shellBuildInputs;
  }
)
