# Akari theme for bat
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.bat;
  themeName = "Akari ${if cfg.variant == "night" then "Night" else "Dawn"}";
in
{
  options.akari.bat = akariLib.mkAkariOption { name = "bat"; };

  config = lib.mkIf cfg.enable {
    programs.bat = {
      config.theme = themeName;
      themes.${themeName} = {
        src = sources + "/bat";
        file = "akari-${cfg.variant}.tmTheme";
      };
    };
  };
}
