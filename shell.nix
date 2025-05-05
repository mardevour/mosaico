{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # Paquetes que quieres incluir en el entorno
  buildInputs = [
    pkgs.bacon
  ];
}