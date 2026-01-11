# Akari theme for zellij
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.zellij;
  themeName = "akari-${cfg.variant}";
in
{
  options.akari.zellij = akariLib.mkAkariOption { name = "zellij"; };

  config = lib.mkIf cfg.enable {
    xdg.configFile."zellij/themes/${themeName}.kdl".source = "${sources}/zellij/${themeName}.kdl";
    programs.zellij.settings.theme = themeName;
  };
}
