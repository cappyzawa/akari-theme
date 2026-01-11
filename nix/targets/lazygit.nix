# Akari theme for lazygit
{ akariLib, sources }:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.akari.lazygit;
  enableXdgConfig = !pkgs.stdenv.hostPlatform.isDarwin || config.xdg.enable;
  configDirectory =
    if enableXdgConfig then
      config.xdg.configHome
    else
      "${config.home.homeDirectory}/Library/Application Support";
  configFile = "${configDirectory}/lazygit/config.yml";
in
{
  options.akari.lazygit = akariLib.mkAkariOption { name = "lazygit"; };

  config = lib.mkIf cfg.enable {
    home.sessionVariables.LG_CONFIG_FILE = lib.concatStringsSep "," (
      [ "${sources}/lazygit/akari-${cfg.variant}.yml" ]
      ++ lib.optional (config.programs.lazygit.settings != { }) configFile
    );
  };
}
