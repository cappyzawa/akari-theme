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
  options.akari.tmux = akariLib.mkAkariOption { name = "tmux"; };

  config = lib.mkIf cfg.enable {
    xdg.configFile = {
      "tmux/akari.tmux".source = "${sources}/tmux/akari.tmux";
      "tmux/akari-night.conf".source = "${sources}/tmux/akari-night.conf";
      "tmux/akari-dawn.conf".source = "${sources}/tmux/akari-dawn.conf";
    };
    programs.tmux.extraConfig = ''
      set -g @akari_variant '${cfg.variant}'
      run-shell "$XDG_CONFIG_HOME/tmux/akari.tmux"
    '';
  };
}
