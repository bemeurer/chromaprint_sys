{ pkgs ? import ./nix }:
let
  generated = pkgs.callPackage ./Cargo.nix {
    inherit pkgs;
    release = true;
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
        LIBCLANG_PATH = "${pkgs.clang.cc.lib}/lib";
      };
    };
  };
  chromaprint_sys = generated.rootCrate.build.override {
    runTests = false;
    # features = [ "vendor" ];
  };
in
{
  inherit chromaprint_sys pkgs;
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
