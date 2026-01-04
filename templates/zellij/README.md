# Akari Zellij Theme

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

Zellij themes inspired by Japanese alleys lit by round lanterns.

## Installation

1. Copy the theme file to your Zellij themes directory:
   ```bash
   # Find your config directory
   zellij setup --check

   # Copy the theme (example for Linux/macOS)
   cp akari-night.kdl ~/.config/zellij/themes/
   # or
   cp akari-dawn.kdl ~/.config/zellij/themes/
   ```

2. Add to your Zellij config (`~/.config/zellij/config.kdl`):
   ```kdl
   theme "akari-night"
   // or
   theme "akari-dawn"
   ```

## Variants

- **akari-night** - Dark theme with lantern-lit atmosphere
- **akari-dawn** - Light theme with morning warmth
