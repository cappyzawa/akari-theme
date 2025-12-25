# Akari fzf Theme

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

fzf themes inspired by Japanese alleys lit by round lanterns.

## Installation

### Manual

Clone the repository and source the theme file in your `.bashrc` or `.zshrc`:

```bash
git clone https://github.com/cappyzawa/akari-fzf.git
source /path/to/akari-fzf/akari-night.sh
```

Or for dawn:

```bash
source /path/to/akari-fzf/akari-dawn.sh
```

### zinit

```zsh
zinit light-mode for \
  pick"akari-night.sh" \
  cappyzawa/akari-fzf
```

Or for dawn:

```zsh
zinit light-mode for \
  pick"akari-dawn.sh" \
  cappyzawa/akari-fzf
```

### sheldon

Add to your `~/.config/sheldon/plugins.toml`:

```toml
[plugins.akari-fzf]
github = "cappyzawa/akari-fzf"
```

### oh-my-zsh

Clone as a custom plugin:

```bash
git clone https://github.com/cappyzawa/akari-fzf ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/akari-fzf
```

Then add to your `.zshrc`:

```zsh
plugins=(... akari-fzf)
```

### antigen

```zsh
antigen bundle cappyzawa/akari-fzf
```

## Variants

- **akari-night.sh** - Dark theme with lantern-lit atmosphere
- **akari-dawn.sh** - Light theme with morning warmth
