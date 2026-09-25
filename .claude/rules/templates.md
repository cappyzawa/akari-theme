# Tera Template Conventions

## Context Variables

### `generate` subcommand (tools in `THEME_TOOLS`)

A template whose output name contains `{variant}` renders once per variant with:

- `theme` — `theme.toml` `[theme]`: `theme.id`, `theme.name`, `theme.description`, `theme.repository`, `theme.license`, `theme.variants`
- `variant` — `variant.id`, `variant.name`, `variant.appearance` (`dark`/`light`), `variant.description`
- `base`, `ansi` (`ansi.red`, `ansi.bright.red`), `roles` (`roles.ui.accent`, `roles.series[0]`, ...)
- `adapter` — `theme.toml` `[adapters.<tool>]` as written (empty table if absent)
- `adapter_text` — one entry per key declared for the tool in `ADAPTER_TEXTS` (`src/generator.rs`); the value is that adapter file's contents, or `""` when the theme does not set the key. Same path rules and errors (`Error::AdapterAssetPath`, `Error::AdapterAssetMissing`) as `ThemeAsset::AdapterPath`; the file is read into the context instead of shipped as an artifact.

A template whose output name has no `{variant}` renders once per theme with:

- `theme`, `adapter` — as above
- `variants` — array in `theme.variants` order; each element has `variant`, `base`, `ansi`, `roles` shaped as above

Keys an adapter requires in `[adapters.<tool>]` are listed in `ADAPTER_KEYS` (`src/generator.rs`); a missing one fails generation for that tool only.
Never read `colors`; the context does not carry it.

`Generator::generate_theme_tool` takes the theme directory (the directory `Theme::load` read) alongside the tool and the `Theme`, so it can resolve `THEME_ASSETS` entries below.

## Per-Variant Output

`{theme}` (`theme.id`) and `{variant}` (`variant.id`). `{theme}` is also substituted in a static (non-`.tera`) file's path, e.g. `templates/nvim/lua/{theme}/highlights/editor.lua` → `nvim/lua/akari/highlights/editor.lua`.

## Theme-Directory Assets (`THEME_ASSETS`)

Some `THEME_TOOLS` entries ship a file that lives in the theme directory itself rather than under `templates/<tool>/`. These are declared in `THEME_ASSETS` (`src/generator.rs`), one tool per line, as either:

- `AdapterPath(key)` — `adapters.<tool>.<key>` names the file, relative to the theme directory. The key is optional: absent, nothing is emitted. Present, the value must be a relative path with no `..`/`.` components (`Error::AdapterAssetPath` otherwise) and the file must exist (`Error::AdapterAssetMissing` otherwise).
- `IfPresent(name)` — a fixed file at the theme directory root (e.g. `LICENSE`), shipped only when it exists; missing is not an error.

## Custom Filters

- `hex_to_rgb` — `"#E26A3B" | hex_to_rgb` → `"[226, 106, 59]"`
- `hex_to_rgb_space` — `"#E26A3B" | hex_to_rgb_space` → `"226 106 59"`

## Static Files

Non-`.tera` files in template directories are copied as-is to `dist/`.
On the theme route a rendered file is executable exactly when its `.tera` template is (e.g. a TPM entry), so set the bit on the template, then `touch` it: Cargo reruns `build.rs` on a changed mtime, not a changed mode, so a bare `chmod` leaves the embedded bit stale.

## Embedded Templates

`build.rs` embeds every file under `templates/` (path, contents, executable bit) into the crate, so a new template or static file needs no registration. `katazome generate` uses the embedded copy by default; `--templates-dir <dir>` reads a directory instead.
