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
  ];
}
