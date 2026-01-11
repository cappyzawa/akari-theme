# Akari theme for zsh-syntax-highlighting
{ akariLib, sources }:
{
  config,
  lib,
  ...
}:

let
  cfg = config.akari.zsh;
in
{
  options.akari.zsh = akariLib.mkAkariOption { name = "zsh-syntax-highlighting"; };

  config = lib.mkIf cfg.enable {
    programs.zsh.initExtra = lib.mkOrder 950 ''
      source '${sources}/zsh/akari-${cfg.variant}.zsh'
    '';
  };
}
