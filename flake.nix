{
  description = "Build environment for pico-pet";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }@inputs:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ rust-overlay.overlays.default ];
    };

    rust = pkgs.rust-bin.nightly."2025-12-01".default.override {
      extensions = [
        "rust-src"
        "rust-std"
      ];

      targets = [
        "thumbv6m-none-eabi"
      ];
    };
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        vscodium
        rust
        rust-analyzer
        alsa-lib
        pkg-config
        udev
        freecad
        kicad
        elf2uf2-rs
        libresprite
        openscad
      ];

      shellHook = ''
        export PATH=${rust}/bin:$PATH
        export RUST_SRC_PATH=${rust}/lib/rustlib/src/rust/library
      '';
    };
  };
}
