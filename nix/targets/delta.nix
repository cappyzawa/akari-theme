# Akari theme for delta
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.delta;
in
{
  options.akari.delta = akariLib.mkAkariOption { name = "delta"; };

  config = lib.mkIf cfg.enable {
    programs.delta.options = import "${sources}/nix/akari-${cfg.variant}-delta.nix";
  };
}
