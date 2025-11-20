{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          system = system;
        };

        # Rust toolchain for building the app
        rustToolchain =
          with fenix.packages.${system};
          combine [
            latest.rustc
            latest.cargo
            latest.rust-src
            latest.rust-analyzer
            latest.clippy
          ];

        # Crane library for building Rust packages
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Source filtering
        src = pkgs.lib.cleanSourceWith {
          src = ./.;

          filter = path: type: (craneLib.filterCargoSources path type);

        };

        # Get crate name and version from app's Cargo.toml
        crateInfo = craneLib.crateNameFromCargoToml { cargoToml = ./app/Cargo.toml; };

        commonArgs = {
          inherit src;
          inherit (crateInfo) pname version;
          strictDeps = true;
          buildInputs = [ ];
          nativeBuildInputs = [ ];
        };

        # Build dependencies only (for caching)
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual application
        komake = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;

            cargoExtraArgs = "-p app";

            CARGO_PROFILE = "release";
          }
        );

        # Package containing templates and assets with compiled CSS
        appData =
          pkgs.runCommand "app-data"
            {
              nativeBuildInputs = [ pkgs.tailwindcss_4 ];
            }
            ''
              mkdir -p $out/templates $out/assets

              # Set up directory structure for tailwind to scan
              # styles/input.css references ../templates/**/*.html
              mkdir -p styles templates
              cp ${./styles/input.css} styles/input.css
              cp -r ${./templates}/* templates/
              cp -r ${./assets}/* $out/assets/

              # Compile Tailwind CSS - the @source directive will find ../templates
              cd styles
              tailwindcss -i input.css -o ../output.css --minify

              # Copy compiled CSS to assets and templates to output
              cd ..
              cp output.css $out/assets/output.css
              cp -r templates/* $out/templates/
            '';

        # Wrapper script to run app with correct working directory
        runApp = pkgs.writeShellScript "run-app" ''
          cd ${appData}
          exec ${komake}/bin/app
        '';

        # Container image for Fly.io deployment
        komakeImg = pkgs.dockerTools.streamLayeredImage {
          name = "komake";
          tag = "latest";
          contents = [
            komake
            appData
            pkgs.bashInteractive
          ];
          config = {
            Cmd = [ "${runApp}" ];
          };
        };
      in
      {
        packages = {
          inherit komake komakeImg;
          default = komake;
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            flyctl
            just
            cargo-info
            cargo-udeps
            cargo-deny
            pkg-config
            just
            just-lsp
            tailwindcss_4
            nodePackages.prettier
            watchman
            taplo
            sqlx-cli
          ];
        };
      }
    );
}
