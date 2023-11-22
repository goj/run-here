{
  description = "A small utility to a given command in focused window's CWD.";

  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        drv = crane.lib.${system}.buildPackage { src = ./.; };
      in
      {
        checks.app-builds = drv;
        packages.default = drv;
        apps.default = flake-utils.lib.mkApp { inherit drv; };
        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;
          nativeBuildInputs = with pkgs; [ cargo cargo-watch cargo-edit clippy rust-analyzer rustc rustfmt ];
        };
      });
}
