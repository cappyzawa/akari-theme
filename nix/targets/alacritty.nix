# Akari theme for alacritty
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.alacritty;
in
{
  options.akari.alacritty = akariLib.mkAkariOption { name = "alacritty"; };

  config = lib.mkIf cfg.enable {
    programs.alacritty.settings.general.import =
      lib.mkBefore [ "${sources}/alacritty/akari-${cfg.variant}.toml" ];
  };
}
