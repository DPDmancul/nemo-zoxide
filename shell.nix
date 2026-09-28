{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    glib
    gtk3
    nemo
    zoxide
  ];
}


