# akari-theme

A terminal color theme inspired by Japanese alleys lit by round lanterns.

Akari (灯) means *light* in Japanese.
This theme is not about darkness, rain, or neon —
it is about warm light, quiet streets, and the presence of life.
 Akari provides two palettes:

- **Akari Night** — lanterns are the primary light source
- **Akari Dawn** — the same alley, as the night fades into morning

![Akari Night vs. Akari Dawn](assets/akari-concept.png)

## Concept

Akari is inspired by a familiar Japanese scene:

- Narrow residential alleys
- Soft, **round lanterns** glowing above
- Wood, stone, plants, and warm shadows
- A night that feels alive, not silent

The goal is to translate this atmosphere into a terminal experience that feels calm,
warm, and readable for long sessions.

## Color Philosophy

- **Light is singular** — Only one warm color (lantern orange) serves as the primary accent
- **Blue is air, not light** — Blue represents the night sky, not a light source
- **Purple stays quiet** — Muted purple for distance, never neon
- **Green is life** — Represents plants and human presence
- **Black is gray** — True black doesn't exist in a lit alley; use warm grays instead

## Supported Tools

| Tool | Category | Installation |
|------|----------|--------------|
| [Ghostty](dist/ghostty/README.md) | Terminal Emulator | Copy theme to `~/.config/ghostty/themes/` |
| [Alacritty](dist/alacritty/README.md) | Terminal Emulator | Import theme in `alacritty.toml` |
| [Helix](dist/helix/README.md) | Editor | Copy theme to `~/.config/helix/themes/` |
| [Neovim](dist/nvim/README.md) | Editor | Install via plugin manager |
| [Visual Studio Code](dist/vscode/README.md) | Editor | Install from [Marketplace](https://marketplace.visualstudio.com/items?itemName=cappyzawa.akari-theme) or [Open VSX](https://open-vsx.org/extension/cappyzawa/akari-theme) |
| [Starship](dist/starship/README.md) | Prompt | Add palette to `~/.config/starship.toml` |
| [tmux](dist/tmux/README.md) | Terminal Multiplexer | Source config in `.tmux.conf` |
| [Zellij](dist/zellij/README.md) | Terminal Multiplexer | Copy theme to `~/.config/zellij/themes/` |
| [macOS Terminal](dist/terminal/README.md) | Terminal Emulator | Double-click to import profile |
| [zsh-syntax-highlighting](dist/zsh/README.md) | Shell | Source in `.zshrc` |
| [fzf](dist/fzf/README.md) | CLI | Source in `.bashrc` or `.zshrc` |
| [bat](dist/bat/README.md) | CLI | Copy theme to `$(bat --config-dir)/themes/` |
| [delta](dist/delta/README.md) | CLI | Include gitconfig in `~/.gitconfig` |
| [Lazygit](dist/lazygit/README.md) | CLI | Copy theme to `~/.config/lazygit/themes/` |
| [gh-dash](dist/gh-dash/README.md) | CLI | Copy theme to `~/.config/gh-dash/config.yml` |
| [Chrome](dist/chrome/README.md) | Browser | Load unpacked extension |
| [Slack](dist/slack/README.md) | App | Import theme string in Preferences |

## Nix (Home Manager)

Akari theme is available as a Home Manager module via Nix flakes.

### Installation

Add akari-theme as a flake input and import the Home Manager module:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    akari-theme.url = "github:cappyzawa/akari-theme";
  };

  outputs = { nixpkgs, home-manager, akari-theme, ... }: {
    homeConfigurations."your-username" = home-manager.lib.homeManagerConfiguration {
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      modules = [
        akari-theme.homeModules.akari
        # your other modules...
      ];
    };
  };
}
```

### Configuration

```nix
{
  # Global settings (applies to all supported tools)
  akari = {
    enable = true;     # default: true
    variant = "night"; # "night" or "dawn", default: "night"
  };

  # Per-tool overrides (optional)
  akari.ghostty.variant = "dawn";  # Use dawn for Ghostty only
  akari.helix.enable = false;      # Disable Akari for Helix
}
```

### Supported Tools

The Home Manager module supports:
alacritty, bat, delta, fzf, gh-dash, ghostty, helix, lazygit, starship, tmux, zellij, zsh

Each tool inherits the global `akari.enable` and `akari.variant` settings by default, but can be individually overridden.

## Palette

Color definitions are the single source of truth in TOML format:

- [Akari Night (Dark)](palette/akari-night.toml)
- [Akari Dawn (Light)](palette/akari-dawn.toml)

## Crate Usage

Use akari-theme as a library to access palette colors in your Rust projects:

```toml
# Palette only (minimal dependencies: serde, toml, thiserror)
akari-theme = "1.9"

# With generator functionality
akari-theme = { version = "1.9", features = ["generator"] }
```

```rust
use akari_theme::{Palette, Rgb};

let night = Palette::night();
let bg: Rgb = night.base.background.parse().unwrap();
let color = bg.to_array();  // [f32; 3] for wgpu
```
