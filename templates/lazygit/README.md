# Akari Theme for Lazygit

Akari Theme for [Lazygit](https://github.com/jesseduffield/lazygit).

## Installation

1. Copy the theme file to your lazygit themes directory:

   ```bash
   # Create themes directory if it doesn't exist
   mkdir -p ~/.config/lazygit/themes

   # Copy the theme (choose night or dawn)
   cp akari-night.yml ~/.config/lazygit/themes/
   # or
   cp akari-dawn.yml ~/.config/lazygit/themes/
   ```

2. Set the `LG_CONFIG_FILE` environment variable to merge the theme with your config:

   ```bash
   # Add to your shell profile (.bashrc, .zshrc, etc.)
   export LG_CONFIG_FILE="$HOME/.config/lazygit/config.yml,$HOME/.config/lazygit/themes/akari-night.yml"
   ```

   Or for dawn theme:

   ```bash
   export LG_CONFIG_FILE="$HOME/.config/lazygit/config.yml,$HOME/.config/lazygit/themes/akari-dawn.yml"
   ```

## Alternative: Direct Configuration

You can also copy the theme settings directly into your `config.yml`:

```yaml
gui:
  theme:
    # ... paste the theme settings from akari-night.yml or akari-dawn.yml
```
