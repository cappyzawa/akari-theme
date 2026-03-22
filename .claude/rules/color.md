---
globs:
  - "src/color.rs"
---

# Rgb Color Manipulation

`lighten`/`darken`/`brighten` operate via HSL conversion. `mix` interpolates directly in RGB space.

## Test Patterns

Include boundary values (0.0, 1.0, midpoint) for each method. Test round-trip stability for edge cases (pure black, pure white, saturated colors).
