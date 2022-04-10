{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.rust-analyzer
    pkgs.cargo
    pkgs.cargo-edit
    # Override this in the future to include only
    # a single backend support
    pkgs.diesel-cli
  ];
}
