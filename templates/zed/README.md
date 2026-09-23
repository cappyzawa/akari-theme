# Akari for Zed

A dark (Night) and light (Dawn) theme family for the [Zed](https://zed.dev)
editor, inspired by Japanese alleys lit by round lanterns.

Both variants ship in a single theme file (`akari.json`) exposing:

- **Akari Night** — warm dark
- **Akari Dawn** — warm light

## Install

### User theme (quickest, for local use)

Copy the generated theme into Zed's themes directory:

```sh
mkdir -p ~/.config/zed/themes
cp akari.json ~/.config/zed/themes/akari.json
```

Zed picks it up immediately. Open the theme selector with
`cmd-k cmd-t` (macOS) / `ctrl-k ctrl-t` (Linux), or set it in `settings.json`:

```json
{
  "theme": {
    "mode": "system",
    "light": "Akari Dawn",
    "dark": "Akari Night"
  }
}
```

### As an extension

Zed themes can also be distributed as an extension via the
[extensions repository](https://github.com/zed-industries/extensions). Place
`akari.json` under a `themes/` directory in an extension named `akari` with an
`extension.toml`, then publish or install as a dev extension
(`zed: install dev extension`).

## Regenerate

```sh
cargo run --features generator -- generate --tool zed
```
