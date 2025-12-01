{
  inputs = { };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

    in {
      devShell.${system} = pkgs.mkShell {
        name = "aoc-2025";

        packages = with pkgs; [
          clang-tools
          cmake
          ninja
        ];
      };
    };
}
