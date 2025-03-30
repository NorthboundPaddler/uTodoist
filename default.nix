{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "uTodoist";
  version = "0.1.2";

  src = pkgs.fetchFromGitHub {
    owner = "NorthboundPaddler";
    repo = "uTodoist";
    rev = "v0.1.2-alpha";
    sha256 = "sha256-4AZU94WAHihJkXBDUKWf+29NWz5K+O1v4tI8fl0MYcU=";
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
}

