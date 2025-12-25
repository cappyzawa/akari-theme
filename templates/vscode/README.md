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

### Akari Night (Dark)

| Role       | Hex       |
|------------|-----------|
| Background | `#171B22` |
| Foreground | `#E6DED3` |
| Lantern    | `#E26A3B` |
| Ember      | `#D65A3A` |
| Amber      | `#D4A05A` |
| Life       | `#7FAF6A` |
| Night      | `#5A6F82` |
| Muted      | `#7C6A8A` |

### Akari Dawn (Light)

| Role       | Hex       |
|------------|-----------|
| Background | `#E4DED6` |
| Foreground | `#1A1816` |
| Lantern    | `#8A4530` |
| Ember      | `#7A3828` |
| Amber      | `#B07840` |
| Life       | `#3A5830` |
| Night      | `#304050` |
| Muted      | `#806080` |

## License

MIT
