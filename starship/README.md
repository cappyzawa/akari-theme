# Akari Starship Themes

Starship prompt themes inspired by Japanese alleys lit by round lanterns.

## Installation

Add the contents of the theme file to your Starship config:

```bash
cat starship/akari-night.toml >> ~/.config/starship.toml
```

Then set the palette in your config:

```toml
palette = 'akari-night'
```

Or for dawn:

```bash
cat starship/akari-dawn.toml >> ~/.config/starship.toml
```

```toml
palette = 'akari-dawn'
```

## How It Works

This overrides Starship's standard named colors, so your existing module configurations will automatically use Akari colors without any changes.

## Variants

- **akari-night.toml** - Dark theme with lantern-lit atmosphere
- **akari-dawn.toml** - Light theme with morning warmth
