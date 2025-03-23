{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "uTodoist";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "NorthboundPaddler";
    repo = "uTodoist";
    rev = "v0.1.0-alpha";
    sha256 = "sha256-YH5qD5m0CBDCq3dp32E69unO3MA8TrdNjTZu8o9tYRE=";
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

