# Akari Chrome Theme Icons

このディレクトリには以下のアイコンが必要です:

- `icon-16.png` - 16x16px
- `icon-48.png` - 48x48px
- `icon-128.png` - 128x128px

## デザインガイドライン

### コンセプト

提灯（ランタン）をモチーフにしたシンプルなアイコン

### 配色

- 背景: 透明 or #1C1A17 (background)
- 提灯の光: #E26A3B (lantern)
- 縁取り/暗部: #3A3530 or #7A7A75

### デザイン案

```
┌──────────┐
│          │
│    ◯     │  ← 丸い提灯を上から見た図
│   ╱ ╲    │     中心に光源
│  ╱   ╲   │     
│ │  ●  │  │  ● = #E26A3B (光)
│  ╲   ╱   │  外側 = #3A3530 (影)
│   ╲ ╱    │
│    ◯     │
│          │
└──────────┘
```

または単純な円形でグラデーション:
- 中心: #F08A5D (bright_yellow)
- 外側: #E26A3B (lantern)
- 最外: #3A3530 (selection_bg)

## 生成方法

ImageMagick、GIMP、Figma等で生成してください。

### ImageMagickの例

```bash
# 128x128の円形グラデーション
convert -size 128x128 xc:none \\
  -fill "radial-gradient:circle at center, #F08A5D 0%, #E26A3B 70%, #3A3530 100%)" \\
  -draw "circle 64,64 64,10" \\
  icon-128.png

# 48x48と16x16にリサイズ
convert icon-128.png -resize 48x48 icon-48.png
convert icon-128.png -resize 16x16 icon-16.png
```

