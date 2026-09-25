# Tera Template Conventions

## Context Variables

### Theme route (`generate-theme`, tools in `THEME_TOOLS`)

A template whose output name contains `{variant}` renders once per variant with:

- `theme` — `theme.toml` `[theme]`: `theme.id`, `theme.name`, `theme.description`, `theme.repository`, `theme.license`, `theme.variants`
- `variant` — `variant.id`, `variant.name`, `variant.appearance` (`dark`/`light`), `variant.description`
- `base`, `ansi` (`ansi.red`, `ansi.bright.red`), `roles` (`roles.ui.accent`, `roles.series[0]`, ...)
- `adapter` — `theme.toml` `[adapters.<tool>]` as written (empty table if absent)

A template whose output name has no `{variant}` renders once per theme with:

- `theme`, `adapter` — as above
- `variants` — array in `theme.variants` order; each element has `variant`, `base`, `ansi`, `roles` shaped as above

Keys an adapter requires in `[adapters.<tool>]` are listed in `ADAPTER_KEYS` (`src/generator.rs`); a missing one fails generation for that tool only.
Never read `colors`; the context does not carry it.

### Legacy route only (`generate`)

Templates receive a flattened `Palette` struct: `{{ base.background }}`, `{{ ansi.red }}`, `{{ ansi_bright.cyan }}`, `{{ semantic.keyword }}`, `{{ colors.lantern.mid }}`, `layers`, `state`, `name`, `description`, `variant`. Combined templates get the same sections prefixed `night_` and `dawn_`.

## Per-Variant Output

- Theme route: `{theme}` (`theme.id`) and `{variant}` (`variant.id`).
- Legacy route only: `{name}` (night/dawn) and `{Name}` (Night/Dawn).

## Custom Filters

- `hex_to_rgb` — `"#E26A3B" | hex_to_rgb` → `"[226, 106, 59]"`
- `hex_to_rgb_space` — `"#E26A3B" | hex_to_rgb_space` → `"226 106 59"`

## Static Files

Non-`.tera` files in template directories are copied as-is to `dist/`.
