# Akari Theme for delta

Akari Theme for [delta](https://github.com/dandavison/delta), a syntax-highlighting pager for git.

## Prerequisites

This theme uses the Akari bat theme for syntax highlighting. Install the bat theme first:

```bash
# See ../bat/README.md for full instructions
cp ../bat/akari-night.tmTheme "$(bat --config-dir)/themes/"
bat cache --build
```

## Installation

1. Copy the theme file to your delta config directory:

   ```bash
   # Create directory if it doesn't exist
   mkdir -p ~/.config/delta

   # Copy the theme (choose night or dawn)
   cp akari-night.gitconfig ~/.config/delta/
   # or
   cp akari-dawn.gitconfig ~/.config/delta/
   ```

2. Include the theme in your `~/.gitconfig`:

   ```gitconfig
   [include]
       path = ~/.config/delta/akari-night.gitconfig

   [core]
       pager = delta

   [interactive]
       diffFilter = delta --color-only

   [delta]
       features = akari-night
   ```

   Or for dawn theme:

   ```gitconfig
   [include]
       path = ~/.config/delta/akari-dawn.gitconfig

   [delta]
       features = akari-dawn
   ```

## Alternative: Direct Configuration

You can also copy the settings directly into your `~/.gitconfig`:

```gitconfig
[delta]
    dark = true
    syntax-theme = "Akari Night"
    line-numbers = true
    # ... paste other settings from akari-night.gitconfig
```

## Customization

The theme enables `line-numbers` by default. You can add additional options:

```gitconfig
[delta]
    features = akari-night
    side-by-side = true          # Enable side-by-side view
    navigate = true              # Use n/N to navigate between files
    hyperlinks = true            # Enable clickable file paths
```

## Variants

- **akari-night** - Dark theme with lantern-lit atmosphere
- **akari-dawn** - Light theme with morning warmth
