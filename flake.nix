{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = inputs: let
    forAllSystems = function:
      inputs.nixpkgs.lib.genAttrs [
        "x86_64-linux"
        # Don't have access to any non-linux systems to test on right now
      ] (system: function inputs.nixpkgs.legacyPackages.${system});

    commonArgs = pkgs: {
    };
  in {
    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          rust-analyzer
        ];

        RUST_LOG = "debug";
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
          libGL
          libxkbcommon
          wayland
        ]);
      };
    });

    packages = forAllSystems (pkgs: {
      ci-term = pkgs.rustPlatform.buildRustPackage (
        (commonArgs pkgs)
        // {
          pname = "ci-term";
          version = "0.1.0";
          src = ./.;
          cargoBuildFlags = "--bin ci-term";
          cargoLock.lockFile = ./Cargo.lock;
        }
      );
      ci-gui = pkgs.rustPlatform.buildRustPackage (
        (commonArgs pkgs)
        // {
          pname = "ci-gui";
          version = "0.1.0";
          src = ./.;
          cargoBuildFlags = "--bin ci-gui";
          cargoLock.lockFile = ./Cargo.lock;
        }
      );
    });
  };
}
