{
  # inspired by: https://serokell.io/blog/practical-nix-flakes#packaging-existing-applications
  description = "A Hello World in Haskell with a dependency and a devShell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = nixpkgs.lib;
        haskellPackages = pkgs.haskellPackages;
      in {
        packages = {
          poker-assistant =
            haskellPackages.callCabal2nix "poker-assistant" ./. { };
          default = self.packages.${system}.poker-assistant;
        };

        checks = self.packages.${system};

        devShells.default = haskellPackages.shellFor {
          packages = p: [ self.packages.${system}.poker-assistant ];
          withHoogle = true;
          buildInputs = with haskellPackages; [
            cabal-install
            ghcid
            haskell-language-server
            hpack
            nixfmt
            hindent
          ];
        };
      });
}
