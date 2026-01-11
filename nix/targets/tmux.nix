# Akari theme for tmux
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.tmux;
in
{
  options.akari.tmux = akariLib.mkAkariOption { name = "tmux"; } // {
    iconNormal = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "Icon for normal state (default: 󰛩)";
    };
    iconPrefix = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "Icon for prefix state (default: 󰛨)";
    };
  };

  config = lib.mkIf cfg.enable {
    xdg.configFile = {
      "tmux/akari.tmux".source = "${sources}/tmux/akari.tmux";
      "tmux/akari-night.conf".source = "${sources}/tmux/akari-night.conf";
      "tmux/akari-dawn.conf".source = "${sources}/tmux/akari-dawn.conf";
    };
    programs.tmux.extraConfig =
      ''
        set -g @akari_variant '${cfg.variant}'
      ''
      + lib.optionalString (cfg.iconNormal != null) ''
        set -g @akari_icon_normal '${cfg.iconNormal}'
      ''
      + lib.optionalString (cfg.iconPrefix != null) ''
        set -g @akari_icon_prefix '${cfg.iconPrefix}'
      ''
      + ''
        run-shell "$XDG_CONFIG_HOME/tmux/akari.tmux"
      '';
  };
}
