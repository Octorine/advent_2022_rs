{
    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    outputs =
    { self
        , nixpkgs
            ,
    }:
    let
        system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    in
    {
        formatter.${system} = nixpkgs.legacyPackages.${system}.nixpkgs-fmt;
        devShells.${system}.default =
            pkgs.mkShell {
                name = "advent-2022-rs shell";
                packages = [
                    pkgs.aoc-cli
                        pkgs.cargo
                        pkgs.rustc
                        pkgs.rust-analyzer
                        pkgs.lldb
                        pkgs.nodejs
                        pkgs.rustfmt		
                ];
            };
    };
}




