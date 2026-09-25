---
globs:
  - "src/theme.rs"
  - "src/expr.rs"
  - "themes/*/*.toml"
---

# Palette Evaluation Pipeline

`themes/<id>/theme.toml` + `<variant>.toml` → raw serde structs (`ThemeFile`, `RawVariant`, `RawRoles`, ...) → `ColorExpr` parsing → `Resolver` staged resolution → `Theme` (`ResolvedVariant` per variant) → Tera templates → `dist/`

## ColorExpr Syntax

- Literal hex: `"#E26A3B"`
- Reference: `"colors.lantern.mid"`, `"base.background"`, `"ansi.bright.red"`
- Functions (nestable):
  - `lighten(expr, factor)` — increase lightness proportionally
  - `darken(expr, factor)` — decrease lightness proportionally
  - `brighten(expr, amount)` — adjust lightness by absolute amount
  - `mix(expr1, expr2, factor)` — blend two colors

## Referenceable Sections

Only `colors`, `base`, `ansi` (`Section::ALLOWED`). `roles` is a consumer — it cannot be referenced, and roles cannot reference other roles.

## Staged Resolution Order

`colors` and `base` are plain hex strings (not `ColorExpr`), always available as reference targets.

1. `ansi` (the base 8 slots) — can reference `colors` and `base`
2. `ansi.bright` — can also reference `ansi` (but not vice versa)
3. `roles.*` (`ui`, `diagnostic`, `diff`, `syntax`, `markup`, `series`) — can reference `colors`, `base` and all of `ansi`

## resolve_fields! Macro

Resolves all `ColorExpr` fields of a Raw struct into resolved hex strings in one call. Each `roles.*` table has a `Raw*` struct (`RawUi`, `RawDiagnostic`, `RawDiff`, `RawSyntax`, `RawMarkup`) in `src/theme.rs`, each with its own `resolve_fields!` invocation; `series` resolves by mapping `resolve_expr` over its `Vec<ColorExpr>` instead, since it is a fixed-size array rather than a named-field struct. When adding a new field to a section, add it to both the `Raw*` struct and the corresponding `resolve_fields!` invocation.

