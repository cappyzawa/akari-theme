# Akari theme for fzf
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.fzf;
in
{
  options.akari.fzf = akariLib.mkAkariOption { name = "fzf"; };

  config = lib.mkIf cfg.enable {
    programs.fzf.colors = import "${sources}/nix/akari-${cfg.variant}-fzf.nix";
  };
}
