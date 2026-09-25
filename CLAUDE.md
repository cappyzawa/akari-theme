# CLAUDE.md

## Build & Verify

```sh
cargo clippy --features generator --all-targets -- -D warnings
cargo fmt -- --check
cargo test --features generator
cargo test
```

## Verification (after theme, color or template changes)

```sh
cargo run --features generator -- generate --theme-dir themes/akari --tool all --out-dir dist
git diff --exit-code
```

## Feature Flags

`generator` — template rendering, CLI, integration tests

## Rules Index

- `.claude/rules/palette.md` — palette evaluation pipeline, ColorExpr, staged resolution
- `.claude/rules/color.md` — Rgb color manipulation
- `.claude/rules/templates.md` — Tera template conventions
