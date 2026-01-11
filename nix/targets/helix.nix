# Akari theme for helix
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.helix;
  themeName = "akari-${cfg.variant}";
in
{
  options.akari.helix = akariLib.mkAkariOption { name = "helix"; };

  config = lib.mkIf cfg.enable {
    xdg.configFile."helix/themes/${themeName}.toml".source = "${sources}/helix/${themeName}.toml";
    programs.helix.settings.theme = themeName;
  };
}
