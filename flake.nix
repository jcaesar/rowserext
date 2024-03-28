{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in rec {
        devShells.default = pkgs.mkShell {
          inputsFrom = [packages.default];
          buildInputs = with pkgs; [
            rustfmt
            cargo-watch
            rust-analyzer
          ];
          CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
        };
        packages.default = pkgs.callPackage ./. {};
      }
    );
}
