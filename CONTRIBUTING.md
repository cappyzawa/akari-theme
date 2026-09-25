# Contributing to akari-theme

Thank you for your interest in contributing to akari-theme!

## Requirements

- Rust 1.85+ (edition 2024)

## Development Setup

```bash
# Clone
git clone https://github.com/cappyzawa/akari-theme.git
cd akari-theme

# Build
cargo build

# Run tests
cargo test
cargo test --no-default-features

# Generate the Akari theme
cargo run -- generate --theme-dir themes/akari --tool all --out-dir dist

# Check for differences
git diff dist/
```

An installed `katazome` bundles its own templates; pass `--templates-dir templates` to try changes from a checkout instead.

## Project Structure

```
akari-theme/
├── themes/            # Source of Truth (theme directories)
│   └── akari/
│       ├── theme.toml     # identity, variant order, adapter metadata
│       ├── night.toml     # one self-contained file per variant
│       └── dawn.toml
├── templates/         # Templates and static files
│   └── {tool}/
│       ├── *.tera     # Tera templates
│       └── *          # Static files (copied as-is)
├── dist/              # Generated output (committed)
├── src/               # katazome, the theme engine and CLI
└── tests/             # Integration tests
```

## Adding a New Tool

1. Create `templates/{tool}/` directory with `.tera` templates. Output names use
   `{theme}` and `{variant}` (e.g. `{theme}-{variant}.toml`); a template without
   `{variant}` renders once per theme with the `variants` array instead.
2. Add the tool to `THEME_TOOLS` in `src/generator.rs`.
3. If the tool needs metadata from `theme.toml`, add the required keys to
   `ADAPTER_KEYS` in `src/generator.rs`, any theme-directory files it ships
   (an icon, a license) to `THEME_ASSETS`, and any adapter-named text files it
   reads into the template context to `ADAPTER_TEXTS`.
4. Add `[adapters.{tool}]` to `themes/akari/theme.toml` when the tool requires it.
5. Add `README.md.tera` with installation instructions.
6. Verify generation:
   ```bash
   cargo run -- generate --theme-dir themes/akari --tool {tool} --out-dir dist
   git diff dist/{tool}/
   ```

## Template Variables

See `.claude/rules/templates.md` for the full context each template receives
(`theme`, `variant`, `base`, `ansi`, `roles`, `adapter`, `adapter_text`, `variants`).

## Color Philosophy

When creating new themes, follow these principles:

- **Light is singular** — Use warm orange (lantern) as the primary accent
- **Blue is air, not light** — Blue represents the night sky
- **Green is life** — Green represents plants and vitality
- **Black is gray** — Use warm grays, no pure black

## Pull Request Guidelines

CI automatically runs:
- `cargo clippy --all-targets -- -D warnings` / `cargo fmt --check` / `cargo test` / `cargo test --no-default-features`
- `cargo run -- generate --theme-dir themes/akari --tool all --out-dir dist` with diff check

Just ensure your commit message is in English with a title under 50 characters.

## Modifying Colors

To modify colors, edit `themes/akari/night.toml` or `themes/akari/dawn.toml` and
regenerate all tools:

```bash
cargo run -- generate --theme-dir themes/akari --tool all --out-dir dist
cargo test
cargo test --no-default-features
git diff dist/
```

## Questions?

Please open an issue.
