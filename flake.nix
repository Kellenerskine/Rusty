# flake.nix
{
  description = "Rust dev environment with direnv support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/25.05";
  };

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux"; # Change if needed: "aarch64-linux", etc.
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          pkgs.cargo
          pkgs.rustc
          pkgs.rust-analyzer
          pkgs.rustfmt
          pkgs.pkg-config
          pkgs.openssl
        ];

        shellHook = ''
          export RUST_BACKTRACE=full
        '';
      };
    };
}

