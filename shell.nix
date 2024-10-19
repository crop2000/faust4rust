{ sources ? import ./npins }:
let
  system = builtins.currentSystem;
  pkgs = import sources.nixpkgs { config = { allowUnfree = true; }; overlays = [ ]; };
  # extensions = (import sources.nix-vscode-extensions).extensions.${system};
  rustcodium =
    let
      inherit (pkgs) vscode-with-extensions vscodium;
      rustExtensions = builtins.attrValues {
        inherit (pkgs.vscode-extensions.jnoortheen) nix-ide;
        inherit (pkgs.vscode-extensions.rust-lang) rust-analyzer;
        inherit (pkgs.vscode-extensions.vadimcn) vscode-lldb;
	inherit (pkgs.vscode-extensions.ms-vscode) cpptools-extension-pack;
	inherit (pkgs.vscode-extensions.ms-vscode) cpptools;
	inherit (pkgs.vscode-extensions.davidlday) languagetool-linter;
      };
    in
    (vscode-with-extensions.override {
      vscode = vscodium;
      vscodeExtensions = rustExtensions;
    });
in
pkgs.mkShell rec {
  packages = with pkgs; [
    rustcodium
  ];
  nativeBuildInputs = with pkgs; [
    cmake
    cargo
    rustc
    rustfmt
    rustPackages.clippy
    pkgs.rustPlatform.bindgenHook
  ];
  buildInputs = with pkgs; [
    pkg-config
    alsa-lib
    jack2
    fontconfig
    wayland
    libxkbcommon
    libGL
    faust
    llvm
  ];
  #LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
}
