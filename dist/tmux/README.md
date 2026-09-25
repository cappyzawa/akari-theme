# Akari tmux Theme

tmux themes for Akari. A terminal color palette inspired by Japanese alleys lit by round lanterns.

## Installation

### Plugin entry (recommended)

Set the variant and run the plugin entry from your `.tmux.conf`:

```tmux
set -g @akari_variant 'night'  # or 'dawn'
run-shell /path/to/akari.tmux
```

### Manual

Source a variant's theme file in your `.tmux.conf` instead; the options below then do not apply:

```tmux
source-file /path/to/akari-night.conf
```

## Variants

- **akari-night** - A dark palette inspired by Japanese alleys lit by round lanterns.
- **akari-dawn** - A light palette inspired by Japanese alleys lit by round lanterns.

## Features

### Prefix Indicator

The status bar icon changes when you press the prefix key:

- 󰛩 - Normal mode (outline)
- 󰛨 - Prefix mode (filled)

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
