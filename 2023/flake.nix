{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
  };

  outputs = { self, nixpkgs }:
    let
      forEachSystem = function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
        ] (system: function (import nixpkgs { inherit system; }) system);

    in {
      devShells = forEachSystem (pkgs: system: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            clang-tools
          ];
        };
      });
    };
}
