{ pkgs ? import ./nix }:
let
  generated = pkgs.callPackage ./Cargo.nix {
    inherit pkgs;
    release = false;
    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      chromaprint_sys = attrs: {
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [
          chromaprint
          clang
          # vendoring
          cmake
          ffmpeg_4
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
    cargo-edit
    crate2nix
    niv
    nixpkgs-fmt
    rnix-lsp

    gdb
    cgdb
    rr

    rustFull
  ];
}
