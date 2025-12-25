# Akari zsh-syntax-highlighting Theme

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

zsh-syntax-highlighting themes inspired by Japanese alleys lit by round lanterns.

## Requirements

- [zsh-syntax-highlighting](https://github.com/zsh-users/zsh-syntax-highlighting)

## Installation

### Manual

Clone the repository and source the theme file in your `.zshrc`:

```bash
git clone https://github.com/cappyzawa/akari-zsh.git

# Night (default)
source /path/to/akari-zsh/akari.zsh

# Or for dawn:
AKARI_VARIANT=dawn source /path/to/akari-zsh/akari.zsh
```

### zinit

```zsh
zinit light cappyzawa/akari-zsh
```

For dawn variant:

```zsh
export AKARI_VARIANT=dawn
zinit light cappyzawa/akari-zsh
```

### sheldon

Add to your `~/.config/sheldon/plugins.toml`:

```toml
[plugins.akari-zsh]
github = "cappyzawa/akari-zsh"
```

### oh-my-zsh

Clone as a custom plugin:

```bash
git clone https://github.com/cappyzawa/akari-zsh ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/akari-zsh
```

Then add to your `.zshrc`:

```zsh
plugins=(... akari-zsh)
```

### antigen

```zsh
antigen bundle cappyzawa/akari-zsh
```

## Variants

- **night** (default) - Dark theme with lantern-lit atmosphere
- **dawn** - Light theme with morning warmth

## Notes

The theme automatically detects the variant based on the `AKARI_VARIANT` environment variable. If not set, it defaults to night.
