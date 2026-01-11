# Akari theme for ghostty
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.ghostty;
  themeName = "akari-${cfg.variant}";
in
{
  options.akari.ghostty = akariLib.mkAkariOption { name = "ghostty"; };

  config = lib.mkIf cfg.enable {
    xdg.configFile."ghostty/themes/${themeName}".source = "${sources}/ghostty/${themeName}";
    programs.ghostty.settings.theme = themeName;
  };
}
