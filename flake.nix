{
  description = "competitive programming contest workspace for Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      ojApiClient = pkgs.python3Packages.online-judge-api-client.overrideAttrs (oldAttrs: {
        patches = (oldAttrs.patches or [ ]) ++ [
          ./patches/atcoder-memory-units.patch
        ];
      });
      ojTools = pkgs.python3Packages.online-judge-tools.override {
        online-judge-api-client = ojApiClient;
      };

      oj = pkgs.python3Packages.toPythonApplication ojTools;
      ojApi = pkgs.python3Packages.toPythonApplication ojApiClient;

      newCommand = pkgs.writeShellApplication {
        name = "new";
        runtimeInputs = [
          pkgs.coreutils
          pkgs.gnused
        ];
        text = ''
          exec ${pkgs.bash}/bin/bash ${./tools/new}/main.sh "$@"
        '';
      };
      fetchCommand = pkgs.writeShellApplication {
        name = "fetch";
        runtimeInputs = [
          oj
          ojApi
          pkgs.jq
        ];
        text = ''
          exec ${pkgs.bash}/bin/bash ${./tools/fetch}/main.sh "$@"
        '';
      };
      ckCommand = pkgs.writeShellApplication {
        name = "ck";
        runtimeInputs = [
          oj
          pkgs.jq
          pkgs.time
        ];
        text = ''
          exec ${pkgs.bash}/bin/bash ${./tools/ck}/main.sh "$@"
        '';
      };
      bundleRs = pkgs.rustPlatform.buildRustPackage {
        pname = "bundle-rs";
        version = "0.1.0";
        src = ./tools/bd;
        cargoLock.lockFile = ./tools/bd/Cargo.lock;
      };
      bdCommand = pkgs.writeShellApplication {
        name = "bd";
        runtimeInputs = [
          bundleRs
          pkgs.jq
          pkgs.coreutils
          pkgs.wl-clipboard
        ];
        text = ''
          exec ${pkgs.bash}/bin/bash ${./tools/bd}/main.sh "$@"
        '';
      };

      commonPackages = [
        oj
        ojApi

        pkgs.jq
      ];
      mkSiteShell =
        sitePackages:
        pkgs.mkShell {
          packages = commonPackages ++ sitePackages;
        };
    in
    {
      formatter.${system} = pkgs.nixfmt-tree.override {
        settings.tree-root-file = "flake.nix";
      };

      devShells.${system} = {
        atcoder = mkSiteShell [
          pkgs.rust-bin.stable."1.89.0".default
          newCommand
          fetchCommand
          ckCommand
          bdCommand
        ];
      };
    };
}
