# Akari bat Theme

[bat](https://github.com/sharkdp/bat) themes inspired by Japanese alleys lit by round lanterns.

## Installation

1. Create the themes directory:

   ```bash
   mkdir -p "$(bat --config-dir)/themes"
   ```

2. Copy the theme file(s) to the themes directory:

   ```bash
   cp akari-night.tmTheme "$(bat --config-dir)/themes/"
   cp akari-dawn.tmTheme "$(bat --config-dir)/themes/"
   ```

3. Rebuild the cache:

   ```bash
   bat cache --build
   ```

4. Verify the theme is available:

   ```bash
   bat --list-themes | grep -i akari
   ```

## Usage

```bash
# Use once
bat --theme="akari-night" <file>

# Set as default (add to shell config)
export BAT_THEME="akari-night"
```

You can also use `--theme-dark` and `--theme-light` for automatic switching:

```bash
export BAT_THEME_DARK="akari-night"
export BAT_THEME_LIGHT="akari-dawn"
```

## Variants

- **akari-night** - Dark theme with lantern-lit atmosphere
- **akari-dawn** - Light theme with morning warmth
