{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  outputs = inputs: let
    system = "x86_64-linux";
    pkgs = inputs.nixpkgs.legacyPackages.${system};
  in {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [
        rustc
        cargo
        rust-analyzer
      ];
    };
  };
}
