{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.vim
    pkgs.git
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.nodejs_22
    pkgs.wasm-pack
    pkgs.llvmPackages.bintools
    pkgs.llvmPackages.clang  # Required for compiling C code to WASM (secp256k1-sys)
    pkgs.lld                  # LLVM linker for WASM
  ];

  # Set environment variables for cross-compiling C to WebAssembly
  # The cc-rs crate needs these to use Clang instead of GCC for wasm32 targets
  shellHook = ''
    export CC_wasm32_unknown_unknown="${pkgs.llvmPackages.clang-unwrapped}/bin/clang"
    export AR_wasm32_unknown_unknown="${pkgs.llvmPackages.bintools-unwrapped}/bin/llvm-ar"
    export CFLAGS_wasm32_unknown_unknown="--target=wasm32-unknown-unknown -I${pkgs.llvmPackages.clang-unwrapped.lib}/lib/clang/19/include"
  '';
}
