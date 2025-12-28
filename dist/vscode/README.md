# Akari Theme for Visual Studio Code

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

A color theme inspired by Japanese alleys lit by round lanterns.

Akari (灯) means *light* in Japanese.
This theme is not about darkness, rain, or neon —
it is about warm light, quiet streets, and the presence of life.

## Variants

- **Akari Night** — Dark theme where lanterns are the primary light source
- **Akari Dawn** — Light theme, the same alley as the night fades into morning

## Color Philosophy

- **Light is singular** — Only one warm color (lantern orange) serves as the primary accent
- **Blue is air, not light** — Blue represents the night sky, not a light source
- **Purple stays quiet** — Muted purple for distance, never neon
- **Green is life** — Represents plants and human presence
- **Black is gray** — True black doesn't exist in a lit alley; use warm grays instead

## Installation

### From VS Code Marketplace (recommended)

1. Open **Extensions** sidebar in VS Code (`Cmd+Shift+X` / `Ctrl+Shift+X`)
2. Search for `Akari`
3. Click **Install**
4. Open Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`)
5. Select **Preferences: Color Theme**
6. Choose **Akari Night** or **Akari Dawn**

### From Open VSX (for Cursor, VSCodium, etc.)

1. Open **Extensions** sidebar
2. Search for `Akari`
3. Click **Install**
4. Select **Preferences: Color Theme**
5. Choose **Akari Night** or **Akari Dawn**

Or install directly from [Open VSX](https://open-vsx.org/extension/cappyzawa/akari-theme).

### From this repository

Clone and symlink to your VS Code extensions directory:

```bash
git clone https://github.com/cappyzawa/akari-vscode.git
ln -s $(pwd)/akari-vscode ~/.vscode/extensions/akari-theme
```

## Palette

Color definitions are the single source of truth in TOML format:

- [Akari Night (Dark)](https://github.com/cappyzawa/akari-theme/blob/main/palette/akari-night.toml)
- [Akari Dawn (Light)](https://github.com/cappyzawa/akari-theme/blob/main/palette/akari-dawn.toml)

## License

MIT
