{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  outputs = inputs: let
    system = "x86_64-linux";
    pkgs = inputs.nixpkgs.legacyPackages.${system};

    libPath = pkgs.lib.makeLibraryPath (with pkgs; [
      libGL
      libxkbcommon
      wayland
    ]);
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        cargo
        rustc
        rust-analyzer
      ];

      RUST_LOG = "debug";
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      LD_LIBRARY_PATH = libPath;
    };
  };
}
