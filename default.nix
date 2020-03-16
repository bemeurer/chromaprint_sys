{ pkgs ? import ./nix }:
let
  generated = pkgs.callPackage ./Cargo.nix {
    inherit pkgs;
    release = false;
    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      chromaprint_sys = attrs: {
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = [
          pkgs.clang
          (pkgs.enableDebugging pkgs.chromaprint)
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
    llvmPackages_latest.lldb

    pkg-config
    rustFull
  ];
}
