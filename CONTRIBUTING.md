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

# Generate all themes
cargo run -- generate --tool all

# Check for differences
git diff dist/
```

## Project Structure

```
akari-theme/
├── palette/           # Source of Truth (color definitions)
│   ├── akari-night.toml
│   └── akari-dawn.toml
├── templates/         # Templates and static files
│   └── {tool}/
│       ├── *.tera     # Tera templates
│       └── *          # Static files (copied as-is)
├── dist/              # Generated output (committed)
├── src/               # Rust CLI (akari-gen)
└── tests/             # Integration tests
```

## Adding a New Tool

1. Create `templates/{tool}/` directory
2. Add template files:
   - `akari-{name}.ext.tera` — expands to night/dawn
   - `{Name}` — expands to Night/Dawn
3. Add `README.md` with installation instructions
4. Verify generation:
   ```bash
   cargo run -- generate --tool {tool}
   git diff dist/{tool}/
   ```

## Template Variables

Available variables in templates:

| Variable | Description |
|----------|-------------|
| `{{ name }}` | Theme name (e.g., `akari-night`) |
| `{{ variant }}` | `night` or `dawn` |
| `{{ colors.lantern }}` | Core colors |
| `{{ base.background }}` | Background color |
| `{{ semantic.keyword }}` | Syntax colors |
| `{{ ansi.red }}` | ANSI colors |

See `palette/akari-night.toml` for the full structure.

## Color Philosophy

When creating new themes, follow these principles:

- **Light is singular** — Use warm orange (lantern) as the primary accent
- **Blue is air, not light** — Blue represents the night sky
- **Green is life** — Green represents plants and vitality
- **Black is gray** — Use warm grays, no pure black

## Pull Request Guidelines

CI automatically runs:
- `cargo clippy` / `cargo fmt --check` / `cargo test`
- `cargo run -- generate --tool all` with diff check

Just ensure your commit message is in English with a title under 50 characters.

## Modifying Colors

To modify colors, edit `palette/*.toml` and regenerate all tools:

```bash
# Edit palette/akari-night.toml or palette/akari-dawn.toml
cargo run -- generate --tool all
cargo test
git diff dist/
```

## Questions?

Please open an issue.
