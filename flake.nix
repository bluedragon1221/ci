{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = inputs: let
    system = "x86_64-linux";
    pkgs = inputs.nixpkgs.legacyPackages.${system};

    commonArgs = {
      RUST_LOG = "debug";
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
        libGL
        libxkbcommon
        wayland
      ]);
    };
  in {
    devShells.${system}.default = pkgs.mkShell (
      commonArgs
      // {
        packages = with pkgs; [
          cargo
          rustc
          rust-analyzer
        ];
      }
    );

    packages.${system} = {
      ci-term = pkgs.rustPlatform.buildRustPackage (
        commonArgs
        // {
          pname = "ci-term";
          version = "0.1.0";
          src = ./.;
          cargoBuildFlags = "--bin ci-term";
          cargoLock.lockFile = ./Cargo.lock;
        }
      );
      ci-gui = pkgs.rustPlatform.buildRustPackage (
        commonArgs
        // {
          pname = "ci-gui";
          versions = "0.1.0";
          src = ./.;
          cargoBuildFlags = "--bin ci-gui";
          cargoLock.lockFile = ./Cargo.lock;
        }
      );
    };
  };
}
