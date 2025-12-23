# Akari zsh-syntax-highlighting Themes

zsh-syntax-highlighting themes inspired by Japanese alleys lit by round lanterns.

## Requirements

- [zsh-syntax-highlighting](https://github.com/zsh-users/zsh-syntax-highlighting)

## Installation

Source the theme file in your `.zshrc`:

```bash
# Night (default)
source /path/to/akari-theme/zsh/akari.zsh

# Or for dawn:
AKARI_VARIANT=dawn source /path/to/akari-theme/zsh/akari.zsh
```

## Variants

- **night** (default) - Dark theme with lantern-lit atmosphere
- **dawn** - Light theme with morning warmth

## Notes

The theme automatically detects the variant based on the `AKARI_VARIANT` environment variable. If not set, it defaults to night.
