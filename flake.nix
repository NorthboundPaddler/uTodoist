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


      packages.${system}.default =
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        pkgs.rustPlatform.buildRustPackage
          rec {
            pname = "uTodoist";
            version = "0.2.0";

            src = pkgs.fetchFromGitHub {
              owner = "NorthboundPaddler";
              repo = "uTodoist";
              rev = "v0.2.0-alpha";
              sha256 = "sha256-tBcmNI9Jb9H4OV5eLjBBe1E9Mo1aqRy/v1pWSG7BbIg=";
            };

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];

            meta = with pkgs.lib; {
              description = "Simple CLI tool to list, add, and complete Todoist jtasks. Written in Rust. ";
              homepage = "https://github.com/NorthboundPaddler/uTodoist";
              license = licenses.gpl3Only; # Replace with the actual license
              maintainers = [ maintainers.NorthboundPaddler ];
            };
          };
    };
}
