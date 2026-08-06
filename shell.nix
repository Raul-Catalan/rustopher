{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  # Build-time dependencies (e.g., system tools, compilers)
  nativeBuildInputs = with pkgs; [
    # Core Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer

    pkg-config
  ];

  # Runtime dependencies and libraries needed by crates (e.g., openssl, zlib)
  buildInputs = with pkgs; [
    # Common libraries used by many Rust projects
    openssl
  ];

  # Environment variables
  RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc; # Required for rust-analyzer LSP to find standard library source

  shellHook = ''
    echo "Rust version: $(rustc --version)"
  '';
}
