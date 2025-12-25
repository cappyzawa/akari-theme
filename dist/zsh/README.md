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
source /path/to/akari-zsh/akari-night.zsh
```

Or for dawn:

```bash
source /path/to/akari-zsh/akari-dawn.zsh
```

### zinit

```zsh
zinit snippet https://github.com/cappyzawa/akari-zsh/raw/main/akari-night.zsh
```

Or for dawn:

```zsh
zinit snippet https://github.com/cappyzawa/akari-zsh/raw/main/akari-dawn.zsh
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

And source the theme file after oh-my-zsh is loaded:

```zsh
source ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/akari-zsh/akari-night.zsh
```

### antigen

```zsh
antigen bundle cappyzawa/akari-zsh --loc=akari-night.zsh
```

Or for dawn:

```zsh
antigen bundle cappyzawa/akari-zsh --loc=akari-dawn.zsh
```

## Variants

- **akari-night.zsh** - Dark theme with lantern-lit atmosphere
- **akari-dawn.zsh** - Light theme with morning warmth
