# Akari theme library for Nix
{ lib, config }:

{
  types.variant = lib.types.enum [
    "night"
    "dawn"
  ];

  mkAkariOption =
    { name }:
    {
      enable = lib.mkEnableOption "Akari theme for ${name}" // {
        default = config.akari.enable;
        defaultText = lib.literalExpression "config.akari.enable";
      };
      variant = lib.mkOption {
        type = lib.types.enum [
          "night"
          "dawn"
        ];
        default = config.akari.variant;
        defaultText = lib.literalExpression "config.akari.variant";
        description = "Akari variant for ${name}";
      };
    };
}
