---
globs:
  - "templates/**"
---

# Tera Template Conventions

## Context Variables

Templates receive a flattened `Palette` struct. Access fields as: `{{ base.background }}`, `{{ ansi.red }}`, `{{ ansi_bright.cyan }}`, `{{ semantic.keyword }}`, `{{ colors.lantern.mid }}`, etc.

## Per-Variant Output

Directory/file names use `{name}` (lowercase: night/dawn) and `{Name}` (capitalized: Night/Dawn) placeholders for per-variant generation.

## Custom Filters

- `hex_to_rgb` — `"#E26A3B" | hex_to_rgb` → `"226, 106, 59"`
- `hex_to_rgb_space` — `"#E26A3B" | hex_to_rgb_space` → `"226 106 59"`

## Static Files

Non-`.tera` files in template directories are copied as-is to `dist/`.
