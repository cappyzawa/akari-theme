# Akari theme for fzf
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.fzf;
  colors = import "${sources}/nix/akari-${cfg.variant}-fzf.nix";

  # Build FZF_DEFAULT_OPTS color string from attrset
  colorOpts = lib.concatStringsSep "," (
    lib.mapAttrsToList (name: value: "${name}:${value}") colors
  );
in
{
  options.akari.fzf = akariLib.mkAkariOption { name = "fzf"; };

  config = lib.mkIf cfg.enable {
    home.sessionVariables.FZF_DEFAULT_OPTS = "--color=${colorOpts}";
  };
}
