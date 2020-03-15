{ pkgs ? import ./nix }:
let
  generated = pkgs.callPackage ./Cargo.nix {
    inherit pkgs;
    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      chromaprint_sys = attrs: {
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [
          clang
          (pkgs.enableDebugging chromaprint)
        ];
        src = pkgs.lib.sourceFilesBySuffices ./. [ ".rs" ".opus" ".lock" ];
        LIBCLANG_PATH = "${pkgs.clang.cc.lib}/lib";
      };
    };
  };
  tested = generated.rootCrate.build.override {
    runTests = false;
    # testInputs = with pkgs; [ chromaprint ];
  };
in
{
  inherit pkgs;
  chromaprint_sys = tested;
  shellBuildInputs = with pkgs; [
    cachix
    cargo-edit
    crate2nix
    gdb
    cgdb
    niv
    nixpkgs-fmt
    rnix-lsp
    rr

    pkg-config
    rustFull
  ];
}
