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
| [Codex](dist/codex/README.md) | App | Import theme string in Settings → Appearance |

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

Color definitions are the single source of truth, as a theme directory:

- [Akari](themes/akari) — `theme.toml`, [`night.toml`](themes/akari/night.toml), [`dawn.toml`](themes/akari/dawn.toml)

All tools in [Supported Tools](#supported-tools) are generated from this directory with:

```sh
cargo run -- generate --theme-dir themes/akari --tool all --out-dir dist
```

The generator is katazome, described in
[Theme engine: katazome](#theme-engine-katazome). It lives in this repository
until it moves to its own repository.

## Theme engine: katazome

katazome generates color theme files for terminals, editors and other tools from
a theme directory. A theme assigns its colors to a shared set of semantic roles
once, and every tool katazome supports is rendered from those roles.

### Install

```sh
cargo install katazome --locked
```

### Theme directory

```text
my-theme/
├── theme.toml        # identity, variant order, adapter metadata
└── <variant>.toml    # one self-contained file per variant
```

[docs/theme-model.md](docs/theme-model.md) defines the keys, the required roles
and the color expression syntax.

### Generate

```sh
katazome generate --theme-dir my-theme --tool all --out-dir dist
```

`--tool` names a tool to generate and can be repeated; `all` generates every
tool. Without `--tool`, katazome generates the tools listed in `theme.tools`, or
every tool when the theme does not list any. The templates are built into the
binary; `--templates-dir <dir>` renders from a templates directory instead.

### Tools

alacritty, bat, chrome, codex, delta, fzf, gh-dash, ghostty, helix, lazygit, nix,
nvim, slack, starship, terminal (macOS Terminal), tmux, vscode, zed, zellij, zsh
(zsh-syntax-highlighting).

Some tools need metadata from `[adapters.<tool>]` in `theme.toml`:

| Tool | Required keys | Optional keys |
|---|---|---|
| chrome | `version` | |
| vscode | `publisher`, `version` | `icon`, `readme`, `keywords` |

A missing required key fails generation for that tool only.

### Library

`Theme::load` resolves a theme directory into role colors, so an adapter for a
tool katazome does not ship can be written in Rust:

```rust
use katazome::theme::Theme;

fn main() -> Result<(), katazome::Error> {
    let theme = Theme::load("my-theme")?;
    for variant in &theme.variants {
        println!(
            "{} {}: accent {}",
            theme.metadata.name, variant.variant.name, variant.roles.ui.accent
        );
    }
    Ok(())
}
```

The default `generator` feature adds `Generator` and the CLI. Depend on it with
`default-features = false` to get only the theme model. The crate documentation
lists the whole public API.
