# Contributing to akari-theme

Thank you for your interest in contributing to akari-theme!

## Modifying Colors

To modify colors, edit `themes/akari/night.toml` or `themes/akari/dawn.toml`, then
regenerate all tools:

```sh
scripts/install-katazome
.katazome/bin/katazome generate --theme-dir themes/akari --tool all --out-dir dist
```

This needs a Rust toolchain, since `scripts/install-katazome` builds katazome with
`cargo install`. Review `git diff dist/` and commit the theme change together with
`dist/`. CI regenerates `dist/` with the same pinned katazome and fails if it
differs from what is committed.

## Engine and Templates

Changes to templates, supported tools, or the engine go to
[katazome](https://github.com/cappyzawa/katazome). akari-theme picks them up by
updating the pin in `scripts/install-katazome` and regenerating `dist/`.

## Mirror Repositories

Each `cappyzawa/akari-<tool>` repository is published from `dist/<tool>/` by
a `.github/workflows/publish-*.yml` workflow, but its README comes from
`mirrors/<tool>/README.md` instead of the katazome output. Edit that file to
change what a mirror shows; color or variant changes in `themes/akari/` do not
reach it. `scripts/check-mirrors` (run in CI) checks that every publish workflow
has a mirror README and that `scripts/assemble-mirror` places it.
