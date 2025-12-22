# Akari Chrome Theme Icons

This directory requires the following icon files:

- `icon-16.png` - 16x16px
- `icon-48.png` - 48x48px
- `icon-128.png` - 128x128px

## Design Guidelines

### Concept

Simple icon inspired by a Japanese round lantern (chōchin)

### Color Scheme

- Background: Transparent or #1C1A17 (background)
- Lantern light: #E26A3B (lantern)
- Border/Shadow: #3A3530 or #7A7A75

### Design Ideas

```
┌──────────┐
│          │
│    ◯     │  ← Top view of round lantern
│   ╱ ╲    │     Light source at center
│  ╱   ╲   │
│ │  ●  │  │  ● = #E26A3B (light)
│  ╲   ╱   │  Outer = #3A3530 (shadow)
│   ╲ ╱    │
│    ◯     │
│          │
└──────────┘
```

Or a simple circular gradient:
- Center: #F08A5D (bright_yellow)
- Middle: #E26A3B (lantern)
- Outer: #3A3530 (selection_bg)

## Generation Methods

Generate with ImageMagick, GIMP, Figma, etc.

### ImageMagick Example

```bash
# 128x128 circular gradient
convert -size 128x128 xc:none \\
  -fill "radial-gradient:circle at center, #F08A5D 0%, #E26A3B 70%, #3A3530 100%)" \\
  -draw "circle 64,64 64,10" \\
  icon-128.png

# Resize to 48x48 and 16x16
convert icon-128.png -resize 48x48 icon-48.png
convert icon-128.png -resize 16x16 icon-16.png
```

