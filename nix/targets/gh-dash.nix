# Akari theme for gh-dash
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.gh-dash;
in
{
  options.akari.gh-dash = akariLib.mkAkariOption { name = "gh-dash"; };

  config = lib.mkIf cfg.enable {
    programs.gh-dash.settings = import "${sources}/nix/akari-${cfg.variant}-gh-dash.nix";
  };
}
