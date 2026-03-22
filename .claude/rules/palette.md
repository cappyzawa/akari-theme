---
globs:
  - "src/palette.rs"
  - "palette/*.toml"
---

# Palette Evaluation Pipeline

`palette/*.toml` → `RawPalette` (serde) → `ColorExpr` parsing → `Resolver` staged resolution → `Palette` → Tera templates → `dist/`

## ColorExpr Syntax

- Literal hex: `"#E26A3B"`
- Reference: `"colors.lantern.mid"`, `"base.background"`, `"ansi.bright.red"`
- Functions (nestable):
  - `lighten(expr, factor)` — increase lightness proportionally
  - `darken(expr, factor)` — decrease lightness proportionally
  - `brighten(expr, amount)` — adjust lightness by absolute amount
  - `mix(expr1, expr2, factor)` — blend two colors

## Referenceable Sections

Only `colors`, `base`, `ansi` (`Section::ALLOWED`). `layers`, `state`, `semantic` are consumers — they cannot be referenced.

## Staged Resolution Order

`colors` and `base` are plain hex strings (not `ColorExpr`), always available as reference targets.

1. `ansi.base` — can reference `colors` and `base`
2. `ansi.bright` — can also reference `ansi.base` (but not vice versa)
3. `layers` / `state` / `semantic` — can reference all resolved sections

## resolve_fields! Macro

Resolves all `ColorExpr` fields of a Raw struct into resolved hex strings in one call. When adding a new field to a section, add it to both the `Raw*` struct and the corresponding `resolve_fields!` invocation.

