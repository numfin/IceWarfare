{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };
  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # [PREPARE]
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # [RUST TOOLCHAIN]
        toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        });
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
      in
      {
        devShells.default = with pkgs; craneLib.devShell
          rec {
            packages = [
              cargo-make
              pkg-config
            ];
            buildInputs = [
              udev
              alsa-lib
              vulkan-loader
              libxkbcommon
              wayland # To use the wayland feature
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          } // lib.optionalAttrs stdenv.isLinux {
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.llvmPackages.clangUseLLVM}/bin/clang";
          CARGO_ENCODED_RUSTFLAGS = "-Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold";
        };
      }
    );
}
