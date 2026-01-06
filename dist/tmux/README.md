# Akari tmux Theme

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

tmux themes inspired by Japanese alleys lit by round lanterns.

## Installation

### Using TPM (recommended)

Add to your `.tmux.conf`:

```tmux
set -g @plugin 'cappyzawa/akari-tmux'
set -g @akari_variant 'night'  # or 'dawn'
```

Then press `prefix + I` to install.

To pin a specific version:

```tmux
set -g @plugin 'cappyzawa/akari-tmux#v0.9.0'
```

### Manual

Download the theme file and source it in your `.tmux.conf`:

```tmux
source-file /path/to/akari-night.conf
```

## Variants

- **akari-night** - Dark theme with lantern-lit atmosphere
- **akari-dawn** - Light theme with morning warmth

## Features

### Prefix Indicator

The status bar icon changes when you press the prefix key:

- 󰛩 - Normal mode (outline)
- 󰛨 - Prefix mode (filled, lantern lit!)

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `@akari_variant` | `night` | Theme variant (`night` or `dawn`) |
| `@akari_icon_normal` | `󰛩` | Icon shown in normal mode |
| `@akari_icon_prefix` | `󰛨` | Icon shown when prefix is pressed |

### Example: Custom Icons

```tmux
set -g @akari_icon_normal ''
set -g @akari_icon_prefix ''
```
