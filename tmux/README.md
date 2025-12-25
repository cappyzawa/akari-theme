# Akari tmux Themes

tmux themes inspired by Japanese alleys lit by round lanterns.

## Installation

### Using TPM (recommended)

Add to your `.tmux.conf`:

```tmux
set -g @plugin 'cappyzawa/akari-tmux'
set -g @akari_variant 'night'  # or 'dawn'
```

Then press `prefix + I` to install.

### Manual

Copy the theme file to your tmux config directory:

```bash
cp tmux/akari-night.conf ~/.config/tmux/
cp tmux/akari-dawn.conf ~/.config/tmux/
```

Then add to your `.tmux.conf`:

```tmux
source-file ~/.config/tmux/akari-night.conf
```

Or for dawn:

```tmux
source-file ~/.config/tmux/akari-dawn.conf
```

## Variants

- **akari-night** - Dark theme with lantern-lit atmosphere
- **akari-dawn** - Light theme with morning warmth

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `@akari_variant` | `night` | Theme variant (`night` or `dawn`) |
