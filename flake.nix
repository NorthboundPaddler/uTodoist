{
  description = "Rust environment to the uTodoist CLI tool";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
    in
    {
      devShells.${system}.default =
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        pkgs.mkShell {
          packages = with pkgs;  [
            cargo
            rustc
            pkg-config
            openssl
          ];
          shellHook = ''
            echo "Developing with Rust!"
          '';
        };
    };
}
