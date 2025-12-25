# Akari zsh-syntax-highlighting Theme

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

zsh-syntax-highlighting themes inspired by Japanese alleys lit by round lanterns.

## Requirements

- [zsh-syntax-highlighting](https://github.com/zsh-users/zsh-syntax-highlighting)

## Installation

Clone the repository and source the theme file in your `.zshrc`:

```bash
git clone https://github.com/cappyzawa/akari-zsh.git

# Night (default)
source /path/to/akari-zsh/akari.zsh

# Or for dawn:
AKARI_VARIANT=dawn source /path/to/akari-zsh/akari.zsh
```

## Variants

- **night** (default) - Dark theme with lantern-lit atmosphere
- **dawn** - Light theme with morning warmth

## Notes

The theme automatically detects the variant based on the `AKARI_VARIANT` environment variable. If not set, it defaults to night.
