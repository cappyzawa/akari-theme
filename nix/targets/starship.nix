# Akari theme for starship
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.starship;
  paletteName = "akari-${cfg.variant}";
in
{
  options.akari.starship = akariLib.mkAkariOption { name = "starship"; };

  config = lib.mkIf cfg.enable {
    programs.starship.settings =
      {
        palette = lib.mkDefault paletteName;
      }
      // lib.importTOML "${sources}/starship/akari-${cfg.variant}.toml";
  };
}
