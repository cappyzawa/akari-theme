# Akari theme Home Manager module
{ self }:
{
  lib,
  config,
  ...
}:

let
  akariLib = import ./lib.nix { inherit lib config; };
  sources = self.outPath + "/dist";
in
{
  imports = map (path: (import path) { inherit akariLib sources; }) [
    ./targets/alacritty.nix
    ./targets/bat.nix
    ./targets/fzf.nix
    ./targets/gh-dash.nix
    ./targets/ghostty.nix
    ./targets/helix.nix
    ./targets/lazygit.nix
    ./targets/starship.nix
    ./targets/tmux.nix
    ./targets/zellij.nix
    ./targets/zsh.nix
  ];

  options.akari = {
    enable = lib.mkEnableOption "Akari theme globally" // {
      default = true;
    };
    variant = lib.mkOption {
      type = akariLib.types.variant;
      default = "night";
      description = "Global Akari variant (night or dawn)";
    };
  };
}
