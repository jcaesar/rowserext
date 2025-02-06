{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs = {
    self,
    nixpkgs,
  }: let
    eachSys = f: nixpkgs.lib.mapAttrs (_: pkgs: f pkgs) nixpkgs.legacyPackages;
  in {
    devShells = eachSys (pkgs: {
      default = pkgs.mkShell {
        inputsFrom = [self.packages.${pkgs.system}.default];
        buildInputs = with pkgs; [
          rustfmt
          cargo-watch
          rust-analyzer
        ];
        CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
      };
    });
    packages = eachSys (pkgs: {
      default = pkgs.callPackage ./. {};
    });
  };
}
