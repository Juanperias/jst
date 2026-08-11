{
  description = "Toggle flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        toolchain = with fenix.packages.${system};
          combine [
            stable.toolchain
            stable.rust-src
            stable.rustfmt
          ];

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
      in
      {
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            toolchain
            just
            nasm
          ];
        };
      });
}
