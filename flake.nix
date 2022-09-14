{
  description = "Fission CLI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-22.05";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
      rec {
        # `nix build`
        packages.fission = naersk-lib.buildPackage {
          pname = "fission";
          root = ./.;
        };
        packages.default = packages.fission;

        # `nix run`
        apps.fission = utils.lib.mkApp {
          drv = packages.fission;
        };
        apps.default = apps.fission;

        # `nix develop`
        devShells.default = pkgs.mkShell {
          name = "fission";
          nativeBuildInputs = with pkgs; [ libiconv rustup ];
        };
      });
}
