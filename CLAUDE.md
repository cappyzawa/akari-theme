# CLAUDE.md

## Build & Verify

```sh
cargo clippy --all-targets -- -D warnings
cargo fmt -- --check
cargo test
cargo test --no-default-features
```

## Verification (after theme, color or template changes)

```sh
cargo run -- generate --theme-dir themes/akari --tool all --out-dir dist
git diff --exit-code
```

## Feature Flags

`generator` (default) — template rendering, CLI, integration tests. `--no-default-features` builds only the theme model

## Rules Index

- `.claude/rules/palette.md` — palette evaluation pipeline, ColorExpr, staged resolution
- `.claude/rules/color.md` — Rgb color manipulation
- `.claude/rules/templates.md` — Tera template conventions
