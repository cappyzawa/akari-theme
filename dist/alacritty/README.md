# Akari Alacritty Theme

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

Alacritty terminal emulator themes inspired by Japanese alleys lit by round lanterns.

## Installation

Clone the repository and copy the theme files:

```bash
git clone https://github.com/cappyzawa/akari-alacritty.git
cp akari-alacritty/*.toml ~/.config/alacritty/themes/
```

Then import the theme in your `alacritty.toml`:

```toml
[general]
import = ["~/.config/alacritty/themes/akari-night.toml"]
```

Or for dawn:

```toml
[general]
import = ["~/.config/alacritty/themes/akari-dawn.toml"]
```

## Variants

- **akari-night** - Dark theme with lantern-lit atmosphere
- **akari-dawn** - Light theme with morning warmth
