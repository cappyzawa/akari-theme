# Akari Starship Theme

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

Starship prompt themes for Akari. A terminal color palette inspired by Japanese alleys lit by round lanterns.

## Installation

Clone the repository and add the palette to your Starship config:

```bash
git clone https://github.com/cappyzawa/akari-starship.git
cat akari-starship/akari-night.toml >> ~/.config/starship.toml
```

Then set the palette in your config:

```toml
palette = 'akari-night'
```

Or for dawn:

```bash
cat akari-starship/akari-dawn.toml >> ~/.config/starship.toml
```

```toml
palette = 'akari-dawn'
```

## How It Works

This overrides Starship's standard named colors, so your existing module configurations will automatically use Akari colors without any changes.

## Variants

- **akari-night.toml** - A dark palette inspired by Japanese alleys lit by round lanterns.
- **akari-dawn.toml** - A light palette inspired by Japanese alleys lit by round lanterns.
