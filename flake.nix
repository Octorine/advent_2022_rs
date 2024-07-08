{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
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
      packages.${system}.default =
        pkgs.stdenv.mkDerivation
          {
            src = ./.;
            name = "advent_2022_rs";
            inherit system;
            nativeBuildInputs = [ pkgs.aoc-cli pkgs.cargo pkgs.rustc pkgs.rust-analyzer pkgs.lldb ];
            buildPhase = ''
              cargo build --release --examples
            '';
            installPhase = ''
              mkdir -p $out/bin
              cp -R target/release/examples/d?? $out/bin
              chmod +x $out
            '';
          };
    };
}




