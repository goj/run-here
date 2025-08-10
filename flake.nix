{
  description = "A small utility to a given command in focused window's CWD.";

  inputs = {
    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        version = cargoToml.package.version;
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
        drv = craneLib.buildPackage {
          pname = "run-here";
          inherit version;
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          nativeBuildInputs = [ pkgs.makeWrapper ];
          buildInputs = [ pkgs.direnv ];
          postInstall = ''
            wrapProgram $out/bin/sway-run-here \
              --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.direnv ]}
            wrapProgram $out/bin/hypr-run-here \
              --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.direnv ]}
          '';
        };
      in
      {
        checks.app-builds = drv;
        packages.default = drv;
        apps.default = flake-utils.lib.mkApp { inherit drv; };
        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;
          nativeBuildInputs = with pkgs; [
            cargo
            cargo-watch
            cargo-edit
            clippy
            rust-analyzer
            rustc
            rustfmt
            direnv
          ];
        };
      });
}
