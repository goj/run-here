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
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        featuresFlag = features: "--features ${nixpkgs.lib.concatStringsSep "," features}";
        buildDeps = { features }: craneLib.buildDepsOnly {
          inherit src;
          cargoExtraArgs = featuresFlag features;
        };
        buildProgram = { pname, features }: craneLib.buildPackage {
          inherit pname;
          inherit version;
          inherit features;
          inherit src;
          cargoExtraArgs = featuresFlag features;
          cargoArtifacts = buildDeps { inherit features; };
          nativeBuildInputs = [ pkgs.makeWrapper ];
          buildInputs = [ pkgs.direnv ];
          postInstall = ''
            wrapProgram $out/bin/$pname \
              --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.direnv ]}
          '';
        };
        swayRunHere = buildProgram {
          pname = "sway-run-here";
          features = [ "default" "direnv" "sway" ];
        };
        hyprRunHere = buildProgram {
          pname = "hypr-run-here";
          features = [ "default" "direnv" "hyprland" ];
        };
      in
      rec {
        checks.sway-run-here-builds = swayRunHere;
        checks.hypr-run-here-builds = hyprRunHere;
        packages.sway-run-here = swayRunHere;
        packages.hypr-run-here = hyprRunHere;
        packages.default = packages.hypr-run-here;
        apps.sway-run-here = flake-utils.lib.mkApp {
          drv = swayRunHere;
        };
        apps.hypr-run-here = flake-utils.lib.mkApp {
          drv = hyprRunHere;
        };
        apps.default = apps.hypr-run-here;
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
