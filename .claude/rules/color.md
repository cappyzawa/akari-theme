---
globs:
  - "src/color.rs"
---

# Rgb Color Manipulation

## Methods

- `lighten(factor)` — HSL-based, increases lightness proportionally toward 1.0
- `darken(factor)` — HSL-based, decreases lightness proportionally toward 0.0
- `brighten(amount)` — adds absolute amount to HSL lightness
- `mix(other, factor)` — linear interpolation in RGB space (0.0 = self, 1.0 = other)

## Constraints

- `factor` is clamped to 0.0–1.0 for lighten/darken/mix
- All conversions go through HSL (hue, saturation, lightness) and back to RGB

## Test Patterns

Include boundary values (0.0, 1.0, midpoint) for each method. Test round-trip stability for edge cases (pure black, pure white, saturated colors).
