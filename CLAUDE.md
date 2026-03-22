# CLAUDE.md

## Build & Verify

```sh
cargo clippy --features generator -- -D warnings
cargo fmt -- --check
cargo test --features generator
```

## Verification (after palette/color changes)

```sh
cargo run --features generator -- generate --tool all
git diff --exit-code
```

## Feature Flags

`generator` — template rendering, CLI, integration tests

## Rules Index

- `.claude/rules/palette.md` — palette evaluation pipeline, ColorExpr, staged resolution
- `.claude/rules/color.md` — Rgb color manipulation
- `.claude/rules/templates.md` — Tera template conventions
