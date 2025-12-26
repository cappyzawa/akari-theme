# Akari zsh-syntax-highlighting Theme

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

zsh-syntax-highlighting themes inspired by Japanese alleys lit by round lanterns.

## Requirements

- [zsh-syntax-highlighting](https://github.com/zsh-users/zsh-syntax-highlighting)

> [!WARNING]
> This theme must be loaded **after** zsh-syntax-highlighting.
> The theme works by overriding `ZSH_HIGHLIGHT_STYLES` variables set by zsh-syntax-highlighting.

## Installation

### Manual

Clone the repository and source the theme file in your `.zshrc`:

```bash
git clone https://github.com/cappyzawa/akari-zsh.git
source /path/to/akari-zsh/akari-night.zsh
```

Or for dawn:

```bash
source /path/to/akari-zsh/akari-dawn.zsh
```

### zinit

```zsh
zinit light-mode for \
  pick"akari-night.zsh" \
  cappyzawa/akari-zsh
```

Or for dawn:

```zsh
zinit light-mode for \
  pick"akari-dawn.zsh" \
  cappyzawa/akari-zsh
```

### sheldon

Add to your `~/.config/sheldon/plugins.toml`:

```toml
[plugins.akari-zsh]
github = "cappyzawa/akari-zsh"
use = ["akari-night.zsh"]
```

Or for dawn:

```toml
[plugins.akari-zsh]
github = "cappyzawa/akari-zsh"
use = ["akari-dawn.zsh"]
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

To use dawn variant, set before loading oh-my-zsh:

```zsh
export AKARI_VARIANT=dawn
```

### antigen

```zsh
antigen bundle cappyzawa/akari-zsh
```

To use dawn variant, set before loading:

```zsh
export AKARI_VARIANT=dawn
```

## Variants

- **akari-night.zsh** - Dark theme with lantern-lit atmosphere
- **akari-dawn.zsh** - Light theme with morning warmth
