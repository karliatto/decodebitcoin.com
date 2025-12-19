{
	description = "bitcoin development environment";

	inputs = {
		nixpkgs.url = "nixpkgs/nixos-25.05";
		nixpkgs-master.url = "github:NixOS/nixpkgs/master";
		flake-utils = {
			url = "github:numtide/flake-utils";
		};
		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, nixpkgs, nixpkgs-master, flake-utils, rust-overlay }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				bitcoinVersion = "29.0";
				rustVersion = "1.91.0";

				lib = nixpkgs.lib;
				isDarwin = pkgs.stdenv.hostPlatform.isDarwin;
				target = lib.strings.replaceStrings [ "-" ] [ "_" ] pkgs.stdenv.buildPlatform.config;
				overlays = [ rust-overlay.overlays.default ];
				pkgs = import nixpkgs {
					inherit system overlays;
				};

				masterPkgs = import nixpkgs-master {
					inherit system;
				};

				rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
					extensions = [ "rust-src" "rust-analyzer" ];
				};

			in
            with pkgs;
			{
				devShells.default = pkgs.mkShell {
					nativeBuildInput = [ ];
					buildInputs = [
					vim
					rustc
					cargo
					rustfmt
					nodejs_22
					wasm-pack
					llvmPackages.bintools
					llvmPackages.clang  # Required for compiling C code to WASM (secp256k1-sys)
					lld                  # LLVM linker for WASM
				];

				# Set environment variables for cross-compiling C to WebAssembly
				# The cc-rs crate needs these to use Clang instead of GCC for wasm32 targets
				shellHook = ''
					export CC_wasm32_unknown_unknown="${llvmPackages.clang-unwrapped}/bin/clang"
					export AR_wasm32_unknown_unknown="${llvmPackages.bintools-unwrapped}/bin/llvm-ar"
					export CFLAGS_wasm32_unknown_unknown="--target=wasm32-unknown-unknown -I${llvmPackages.clang-unwrapped.lib}/lib/clang/19/include"
				'';
			};
		}
	);
}
