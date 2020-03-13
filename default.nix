{ pkgs ? import ./nix }:
let
  generated = pkgs.callPackage ./Cargo.nix {
    inherit pkgs;
    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      chromaprint_sys = attrs: {
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [
          clang
          chromaprint
        ];
        LIBCLANG_PATH = "${pkgs.clang.cc.lib}/lib";
      };
    };
  };
  tested = generated.rootCrate.build.override {
    runTests = true;
    # testInputs = with pkgs; [ hello ];
  };
in
{
  inherit pkgs;
  chromaprint_sys = tested;
  shellBuildInputs = with pkgs; [
    cachix
    cargo-edit
    crate2nix
    niv
    nixpkgs-fmt

    pkg-config
    rustFull
  ];
}
