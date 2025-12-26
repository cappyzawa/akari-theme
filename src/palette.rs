use crate::{Error, Rgb, Variant};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

// Raw types for TOML deserialization (before reference resolution)
#[derive(Debug, Deserialize)]
struct RawPalette {
    name: String,
    description: String,
    colors: Colors,
    base: Base,
    layers: RawLayers,
    state: RawState,
    semantic: RawSemantic,
    ansi: RawAnsi,
}

#[derive(Debug, Deserialize)]
struct RawLayers {
    base: ColorExpr,
    surface: ColorExpr,
    sunken: ColorExpr,
    raised: ColorExpr,
    border: ColorExpr,
    inset: ColorExpr,
}

impl RawLayers {
    fn resolve(&self, resolver: &Resolver) -> Result<Layers, Error> {
        Ok(Layers {
            base: resolver.resolve_expr(&self.base)?,
            surface: resolver.resolve_expr(&self.surface)?,
            sunken: resolver.resolve_expr(&self.sunken)?,
            raised: resolver.resolve_expr(&self.raised)?,
            border: resolver.resolve_expr(&self.border)?,
            inset: resolver.resolve_expr(&self.inset)?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawState {
    selection_bg: ColorExpr,
    selection_fg: ColorExpr,
    match_bg: ColorExpr,
    cursor: ColorExpr,
    cursor_text: ColorExpr,
    info: ColorExpr,
    hint: ColorExpr,
    warning: ColorExpr,
    error: ColorExpr,
    active_bg: ColorExpr,
    diff_added: ColorExpr,
    diff_removed: ColorExpr,
    diff_changed: ColorExpr,
}

impl RawState {
    fn resolve(&self, resolver: &Resolver) -> Result<State, Error> {
        Ok(State {
            selection_bg: resolver.resolve_expr(&self.selection_bg)?,
            selection_fg: resolver.resolve_expr(&self.selection_fg)?,
            match_bg: resolver.resolve_expr(&self.match_bg)?,
            cursor: resolver.resolve_expr(&self.cursor)?,
            cursor_text: resolver.resolve_expr(&self.cursor_text)?,
            info: resolver.resolve_expr(&self.info)?,
            hint: resolver.resolve_expr(&self.hint)?,
            warning: resolver.resolve_expr(&self.warning)?,
            error: resolver.resolve_expr(&self.error)?,
            active_bg: resolver.resolve_expr(&self.active_bg)?,
            diff_added: resolver.resolve_expr(&self.diff_added)?,
            diff_removed: resolver.resolve_expr(&self.diff_removed)?,
            diff_changed: resolver.resolve_expr(&self.diff_changed)?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawAnsi {
    black: ColorExpr,
    red: ColorExpr,
    green: ColorExpr,
    yellow: ColorExpr,
    blue: ColorExpr,
    magenta: ColorExpr,
    cyan: ColorExpr,
    white: ColorExpr,
    bright: RawAnsiBright,
}

#[derive(Debug, Deserialize)]
struct RawAnsiBright {
    black: ColorExpr,
    red: ColorExpr,
    green: ColorExpr,
    yellow: ColorExpr,
    blue: ColorExpr,
    magenta: ColorExpr,
    cyan: ColorExpr,
    white: ColorExpr,
}

impl RawAnsiBright {
    fn resolve(&self, resolver: &Resolver) -> Result<Ansi, Error> {
        Ok(Ansi {
            black: resolver.resolve_expr(&self.black)?,
            red: resolver.resolve_expr(&self.red)?,
            green: resolver.resolve_expr(&self.green)?,
            yellow: resolver.resolve_expr(&self.yellow)?,
            blue: resolver.resolve_expr(&self.blue)?,
            magenta: resolver.resolve_expr(&self.magenta)?,
            cyan: resolver.resolve_expr(&self.cyan)?,
            white: resolver.resolve_expr(&self.white)?,
        })
    }
}

impl RawAnsi {
    fn resolve(&self, resolver: &Resolver) -> Result<Ansi, Error> {
        Ok(Ansi {
            black: resolver.resolve_expr(&self.black)?,
            red: resolver.resolve_expr(&self.red)?,
            green: resolver.resolve_expr(&self.green)?,
            yellow: resolver.resolve_expr(&self.yellow)?,
            blue: resolver.resolve_expr(&self.blue)?,
            magenta: resolver.resolve_expr(&self.magenta)?,
            cyan: resolver.resolve_expr(&self.cyan)?,
            white: resolver.resolve_expr(&self.white)?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawSemantic {
    text: ColorExpr,
    comment: ColorExpr,
    string: ColorExpr,
    keyword: ColorExpr,
    number: ColorExpr,
    constant: ColorExpr,
    r#type: ColorExpr,
    function: ColorExpr,
    variable: ColorExpr,
    success: ColorExpr,
    path: ColorExpr,
}

impl RawSemantic {
    fn resolve(&self, resolver: &Resolver) -> Result<Semantic, Error> {
        Ok(Semantic {
            text: resolver.resolve_expr(&self.text)?,
            comment: resolver.resolve_expr(&self.comment)?,
            string: resolver.resolve_expr(&self.string)?,
            keyword: resolver.resolve_expr(&self.keyword)?,
            number: resolver.resolve_expr(&self.number)?,
            constant: resolver.resolve_expr(&self.constant)?,
            r#type: resolver.resolve_expr(&self.r#type)?,
            function: resolver.resolve_expr(&self.function)?,
            variable: resolver.resolve_expr(&self.variable)?,
            success: resolver.resolve_expr(&self.success)?,
            path: resolver.resolve_expr(&self.path)?,
        })
    }
}

// Resolved types (used for both deserialization and template rendering)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Colors {
    pub lantern: String,
    pub ember: String,
    pub amber: String,
    pub life: String,
    pub night: String,
    pub muted: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Base {
    pub background: String,
    pub foreground: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Layers {
    pub base: String,
    pub surface: String,
    pub sunken: String,
    pub raised: String,
    pub border: String,
    pub inset: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub selection_bg: String,
    pub selection_fg: String,
    pub match_bg: String,
    pub cursor: String,
    pub cursor_text: String,
    pub info: String,
    pub hint: String,
    pub warning: String,
    pub error: String,
    pub active_bg: String,
    pub diff_added: String,
    pub diff_removed: String,
    pub diff_changed: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Semantic {
    pub text: String,
    pub comment: String,
    pub string: String,
    pub keyword: String,
    pub number: String,
    pub constant: String,
    pub r#type: String,
    pub function: String,
    pub variable: String,
    pub success: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Ansi {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

impl<'a> IntoIterator for &'a Ansi {
    type Item = (&'static str, &'a str);
    type IntoIter = std::array::IntoIter<Self::Item, 8>;

    fn into_iter(self) -> Self::IntoIter {
        [
            ("black", self.black.as_str()),
            ("red", self.red.as_str()),
            ("green", self.green.as_str()),
            ("yellow", self.yellow.as_str()),
            ("blue", self.blue.as_str()),
            ("magenta", self.magenta.as_str()),
            ("cyan", self.cyan.as_str()),
            ("white", self.white.as_str()),
        ]
        .into_iter()
    }
}

macro_rules! make_map {
    ($obj:expr, $($field:ident),+ $(,)?) => {
        [$(
            (stringify!($field), $obj.$field.as_str()),
        )+].into_iter().collect()
    };
}

/// A color expression that can be deserialized from TOML.
///
/// Supports:
/// - Literal hex colors: `"#E26A3B"`
/// - References: `"colors.lantern"`
/// - Functions:
///   - `"lighten(colors.lantern, 0.1)"` — blend toward white
///   - `"darken(base.background, 0.2)"` — blend toward black
///   - `"mix(base.background, colors.night, 0.15)"` — blend two colors
#[derive(Debug, Clone)]
enum ColorExpr {
    /// A literal hex color (e.g., "#E26A3B")
    Literal(String),
    /// A reference to another field (e.g., "colors.lantern")
    Ref { section: String, key: String },
    /// Lighten a color by a factor (0.0 = unchanged, 1.0 = white)
    Lighten(Box<ColorExpr>, f64),
    /// Darken a color by a factor (0.0 = unchanged, 1.0 = black)
    Darken(Box<ColorExpr>, f64),
    /// Mix two colors (0.0 = first color, 1.0 = second color)
    Mix(Box<ColorExpr>, Box<ColorExpr>, f64),
}

impl<'de> Deserialize<'de> for ColorExpr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_color_expr(&s).map_err(serde::de::Error::custom)
    }
}

/// Parse a color expression string into a ColorExpr.
fn parse_color_expr(s: &str) -> Result<ColorExpr, String> {
    let s = s.trim();

    // Literal hex color
    if s.starts_with('#') {
        return Ok(ColorExpr::Literal(s.to_string()));
    }

    // Function call: lighten(...), darken(...), mix(...)
    if let Some(rest) = s.strip_prefix("lighten(").and_then(|r| r.strip_suffix(')')) {
        let (inner, factor) = parse_unary_fn_args(rest)?;
        return Ok(ColorExpr::Lighten(Box::new(inner), factor));
    }
    if let Some(rest) = s.strip_prefix("darken(").and_then(|r| r.strip_suffix(')')) {
        let (inner, factor) = parse_unary_fn_args(rest)?;
        return Ok(ColorExpr::Darken(Box::new(inner), factor));
    }
    if let Some(rest) = s.strip_prefix("mix(").and_then(|r| r.strip_suffix(')')) {
        let (color1, color2, factor) = parse_mix_args(rest)?;
        return Ok(ColorExpr::Mix(Box::new(color1), Box::new(color2), factor));
    }

    // Reference: section.key
    let (section, key) = s
        .split_once('.')
        .ok_or_else(|| format!("invalid color expression: {s}"))?;
    Ok(ColorExpr::Ref {
        section: section.to_string(),
        key: key.to_string(),
    })
}

/// Parse unary function arguments: "colors.lantern, 0.1" -> (ColorExpr, f64)
fn parse_unary_fn_args(args: &str) -> Result<(ColorExpr, f64), String> {
    let (color_str, factor_str) = args
        .rsplit_once(',')
        .ok_or_else(|| format!("expected 'color, factor': {args}"))?;
    let inner = parse_color_expr(color_str.trim())?;
    let factor = factor_str
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("invalid factor: {}", factor_str.trim()))?;
    Ok((inner, factor))
}

/// Parse mix function arguments: "color1, color2, 0.15" -> (ColorExpr, ColorExpr, f64)
fn parse_mix_args(args: &str) -> Result<(ColorExpr, ColorExpr, f64), String> {
    // Split from right to get factor first
    let (rest, factor_str) = args
        .rsplit_once(',')
        .ok_or_else(|| format!("expected 'color1, color2, factor': {args}"))?;
    let factor = factor_str
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("invalid factor: {}", factor_str.trim()))?;

    // Split remaining to get two colors
    let (color1_str, color2_str) = rest
        .rsplit_once(',')
        .ok_or_else(|| format!("expected 'color1, color2, factor': {args}"))?;
    let color1 = parse_color_expr(color1_str.trim())?;
    let color2 = parse_color_expr(color2_str.trim())?;

    Ok((color1, color2, factor))
}

struct Resolver<'a> {
    colors: BTreeMap<&'a str, &'a str>,
    base: BTreeMap<&'a str, &'a str>,
    /// Resolved hex values for ansi_bright (needed for semantic.path references)
    ansi_bright: BTreeMap<&'static str, String>,
}

impl<'a> Resolver<'a> {
    fn new(raw: &'a RawPalette) -> Result<Self, Error> {
        let colors: BTreeMap<&str, &str> =
            make_map!(raw.colors, lantern, ember, amber, life, night, muted);
        let base: BTreeMap<&str, &str> = make_map!(raw.base, background, foreground);

        // Build a temporary resolver without ansi_bright to resolve ansi_bright references
        let temp = TempResolver {
            colors: &colors,
            base: &base,
        };

        // Resolve ansi_bright first (it only depends on colors/base)
        let ansi_bright = [
            ("black", temp.resolve_expr(&raw.ansi.bright.black)?),
            ("red", temp.resolve_expr(&raw.ansi.bright.red)?),
            ("green", temp.resolve_expr(&raw.ansi.bright.green)?),
            ("yellow", temp.resolve_expr(&raw.ansi.bright.yellow)?),
            ("blue", temp.resolve_expr(&raw.ansi.bright.blue)?),
            ("magenta", temp.resolve_expr(&raw.ansi.bright.magenta)?),
            ("cyan", temp.resolve_expr(&raw.ansi.bright.cyan)?),
            ("white", temp.resolve_expr(&raw.ansi.bright.white)?),
        ]
        .into_iter()
        .collect();

        Ok(Self {
            colors,
            base,
            ansi_bright,
        })
    }

    fn resolve_ref(&self, section: &str, key: &str) -> Result<String, Error> {
        match section {
            "colors" => self
                .colors
                .get(key)
                .map(|s| (*s).to_string())
                .ok_or_else(|| Error::UnresolvedRef(format!("{section}.{key}"))),
            "base" => self
                .base
                .get(key)
                .map(|s| (*s).to_string())
                .ok_or_else(|| Error::UnresolvedRef(format!("{section}.{key}"))),
            "ansi_bright" => self
                .ansi_bright
                .get(key)
                .cloned()
                .ok_or_else(|| Error::UnresolvedRef(format!("{section}.{key}"))),
            _ => Err(Error::UnresolvedRef(format!("{section}.{key}"))),
        }
    }

    fn resolve_expr(&self, expr: &ColorExpr) -> Result<String, Error> {
        match expr {
            ColorExpr::Literal(hex) => Ok(hex.clone()),
            ColorExpr::Ref { section, key } => self.resolve_ref(section, key),
            ColorExpr::Lighten(inner, factor) => {
                let hex = self.resolve_expr(inner)?;
                Ok(Rgb::parse(&hex)?.lighten(*factor).to_hex())
            }
            ColorExpr::Darken(inner, factor) => {
                let hex = self.resolve_expr(inner)?;
                Ok(Rgb::parse(&hex)?.darken(*factor).to_hex())
            }
            ColorExpr::Mix(color1, color2, factor) => {
                let rgb1 = Rgb::parse(&self.resolve_expr(color1)?)?;
                let rgb2 = Rgb::parse(&self.resolve_expr(color2)?)?;
                Ok(rgb1.mix(rgb2, *factor).to_hex())
            }
        }
    }
}

/// Temporary resolver for bootstrapping (only colors and base)
struct TempResolver<'a> {
    colors: &'a BTreeMap<&'a str, &'a str>,
    base: &'a BTreeMap<&'a str, &'a str>,
}

impl TempResolver<'_> {
    fn resolve_expr(&self, expr: &ColorExpr) -> Result<String, Error> {
        match expr {
            ColorExpr::Literal(hex) => Ok(hex.clone()),
            ColorExpr::Ref { section, key } => {
                let map = match section.as_str() {
                    "colors" => self.colors,
                    "base" => self.base,
                    _ => return Err(Error::UnresolvedRef(format!("{section}.{key}"))),
                };
                map.get(key.as_str())
                    .map(|s| (*s).to_string())
                    .ok_or_else(|| Error::UnresolvedRef(format!("{section}.{key}")))
            }
            ColorExpr::Lighten(inner, factor) => {
                let hex = self.resolve_expr(inner)?;
                Ok(Rgb::parse(&hex)?.lighten(*factor).to_hex())
            }
            ColorExpr::Darken(inner, factor) => {
                let hex = self.resolve_expr(inner)?;
                Ok(Rgb::parse(&hex)?.darken(*factor).to_hex())
            }
            ColorExpr::Mix(color1, color2, factor) => {
                let rgb1 = Rgb::parse(&self.resolve_expr(color1)?)?;
                let rgb2 = Rgb::parse(&self.resolve_expr(color2)?)?;
                Ok(rgb1.mix(rgb2, *factor).to_hex())
            }
        }
    }
}

impl RawPalette {
    fn resolve(&self, variant: Variant) -> Result<Palette, Error> {
        let resolver = Resolver::new(self)?;
        Ok(Palette {
            variant,
            name: self.name.clone(),
            description: self.description.clone(),
            colors: self.colors.clone(),
            base: self.base.clone(),
            layers: self.layers.resolve(&resolver)?,
            state: self.state.resolve(&resolver)?,
            semantic: self.semantic.resolve(&resolver)?,
            ansi: self.ansi.resolve(&resolver)?,
            ansi_bright: self.ansi.bright.resolve(&resolver)?,
        })
    }
}

#[derive(Debug)]
pub struct Palette {
    pub variant: Variant,
    pub name: String,
    pub description: String,
    pub colors: Colors,
    pub base: Base,
    pub layers: Layers,
    pub state: State,
    pub semantic: Semantic,
    pub ansi: Ansi,
    pub ansi_bright: Ansi,
}

impl Palette {
    pub fn from_path(path: impl AsRef<Path>, variant: Variant) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        let raw: RawPalette = toml::from_str(&content)?;
        raw.resolve(variant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn palette_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("palette/akari-night.toml")
    }

    #[test]
    fn load_night_palette() {
        let palette = Palette::from_path(palette_path(), Variant::Night).unwrap();
        assert_eq!(palette.name, "akari-night");
        assert_eq!(palette.variant, Variant::Night);
    }

    #[test]
    fn colors_are_loaded() {
        let palette = Palette::from_path(palette_path(), Variant::Night).unwrap();
        assert_eq!(palette.colors.lantern, "#E26A3B");
        assert_eq!(palette.colors.ember, "#D65A3A");
    }

    #[test]
    fn base_colors_are_loaded() {
        let palette = Palette::from_path(palette_path(), Variant::Night).unwrap();
        assert_eq!(palette.base.background, "#10141C");
        assert_eq!(palette.base.foreground, "#E6DED3");
    }

    #[test]
    fn semantic_references_resolved() {
        let palette = Palette::from_path(palette_path(), Variant::Night).unwrap();
        // semantic.keyword = "colors.lantern" -> "#E26A3B"
        assert_eq!(palette.semantic.keyword, "#E26A3B");
        // semantic.string = "colors.life" -> "#7FAF6A"
        assert_eq!(palette.semantic.string, "#7FAF6A");
    }

    #[test]
    fn ansi_references_resolved() {
        let palette = Palette::from_path(palette_path(), Variant::Night).unwrap();
        // ansi.green = "colors.life" -> "#7FAF6A"
        assert_eq!(palette.ansi.green, "#7FAF6A");
        // ansi.white = "base.foreground" -> "#E6DED3"
        assert_eq!(palette.ansi.white, "#E6DED3");
    }

    #[test]
    fn missing_semantic_field_fails() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let toml = r##"
name = "test"
description = "test"

[colors]
lantern = "#E26A3B"
ember = "#D65A3A"
amber = "#D4A05A"
life = "#7FAF6A"
night = "#5A6F82"
muted = "#7C6A8A"

[base]
background = "#171B22"
foreground = "#E6DED3"

[layers]
base = "#171B22"
surface = "#1E2329"
sunken = "#13171D"
raised = "#252B33"
border = "#2E353E"
inset = "#3A424D"

[state]
selection_bg = "#3A424D"
selection_fg = "#E6DED3"
match_bg = "#4A3A2A"
cursor = "#E26A3B"
cursor_text = "#171B22"
info = "#5A6F82"
hint = "#7C6A8A"
warning = "#D4A05A"
error = "#D65A3A"
active_bg = "#2A3540"
diff_added = "#7FAF6A"
diff_removed = "#D65A3A"
diff_changed = "#D4A05A"

[semantic]
comment = "#7D8797"
string = "colors.life"
keyword = "colors.lantern"
number = "colors.amber"
constant = "colors.amber"
type = "colors.amber"
function = "colors.lantern"
variable = "base.foreground"
success = "colors.life"

[ansi]
black = "#171B22"
red = "colors.ember"
green = "colors.life"
yellow = "colors.amber"
blue = "colors.night"
magenta = "colors.muted"
cyan = "colors.night"
white = "base.foreground"

[ansi.bright]
black = "#3A424D"
red = "colors.lantern"
green = "colors.life"
yellow = "colors.amber"
blue = "colors.night"
magenta = "colors.muted"
cyan = "colors.night"
white = "base.foreground"
"##;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(toml.as_bytes()).unwrap();

        let result = Palette::from_path(file.path(), Variant::Night);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::ParsePalette(_)));
    }

    #[test]
    fn invalid_reference_fails() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let toml = r##"
name = "test"
description = "test"

[colors]
lantern = "#E26A3B"
ember = "#D65A3A"
amber = "#D4A05A"
life = "#7FAF6A"
night = "#5A6F82"
muted = "#7C6A8A"

[base]
background = "#171B22"
foreground = "#E6DED3"

[layers]
base = "#171B22"
surface = "#1E2329"
sunken = "#13171D"
raised = "#252B33"
border = "#2E353E"
inset = "#3A424D"

[state]
selection_bg = "#3A424D"
selection_fg = "#E6DED3"
match_bg = "#4A3A2A"
cursor = "#E26A3B"
cursor_text = "#171B22"
info = "#5A6F82"
hint = "#7C6A8A"
warning = "#D4A05A"
error = "#D65A3A"
active_bg = "#2A3540"
diff_added = "#7FAF6A"
diff_removed = "#D65A3A"
diff_changed = "#D4A05A"

[semantic]
text = "base.foreground"
comment = "#7D8797"
string = "colors.nonexistent"
keyword = "colors.lantern"
number = "colors.amber"
constant = "colors.amber"
type = "colors.amber"
function = "colors.lantern"
variable = "base.foreground"
success = "colors.life"
path = "ansi_bright.green"

[ansi]
black = "#171B22"
red = "colors.ember"
green = "colors.life"
yellow = "colors.amber"
blue = "colors.night"
magenta = "colors.muted"
cyan = "colors.night"
white = "base.foreground"

[ansi.bright]
black = "#3A424D"
red = "colors.lantern"
green = "colors.life"
yellow = "colors.amber"
blue = "colors.night"
magenta = "colors.muted"
cyan = "colors.night"
white = "base.foreground"
"##;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(toml.as_bytes()).unwrap();

        let result = Palette::from_path(file.path(), Variant::Night);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::UnresolvedRef(_)));
    }

    #[test]
    fn parse_color_expr_literal() {
        let expr = parse_color_expr("#E26A3B").unwrap();
        assert!(matches!(expr, ColorExpr::Literal(s) if s == "#E26A3B"));
    }

    #[test]
    fn parse_color_expr_reference() {
        let expr = parse_color_expr("colors.lantern").unwrap();
        assert!(
            matches!(expr, ColorExpr::Ref { section, key } if section == "colors" && key == "lantern")
        );
    }

    #[test]
    fn parse_color_expr_lighten() {
        let expr = parse_color_expr("lighten(colors.lantern, 0.1)").unwrap();
        match expr {
            ColorExpr::Lighten(inner, factor) => {
                assert!(
                    matches!(*inner, ColorExpr::Ref { section, key } if section == "colors" && key == "lantern")
                );
                assert!((factor - 0.1).abs() < 0.001);
            }
            _ => panic!("expected Lighten"),
        }
    }

    #[test]
    fn parse_color_expr_darken() {
        let expr = parse_color_expr("darken(base.background, 0.2)").unwrap();
        match expr {
            ColorExpr::Darken(inner, factor) => {
                assert!(
                    matches!(*inner, ColorExpr::Ref { section, key } if section == "base" && key == "background")
                );
                assert!((factor - 0.2).abs() < 0.001);
            }
            _ => panic!("expected Darken"),
        }
    }

    #[test]
    fn parse_color_expr_nested() {
        let expr = parse_color_expr("lighten(darken(colors.lantern, 0.1), 0.2)").unwrap();
        match expr {
            ColorExpr::Lighten(inner, outer_factor) => {
                assert!((outer_factor - 0.2).abs() < 0.001);
                match *inner {
                    ColorExpr::Darken(innermost, inner_factor) => {
                        assert!(
                            matches!(*innermost, ColorExpr::Ref { section, key } if section == "colors" && key == "lantern")
                        );
                        assert!((inner_factor - 0.1).abs() < 0.001);
                    }
                    _ => panic!("expected Darken"),
                }
            }
            _ => panic!("expected Lighten"),
        }
    }

    #[test]
    fn parse_color_expr_mix() {
        let expr = parse_color_expr("mix(base.background, colors.night, 0.15)").unwrap();
        match expr {
            ColorExpr::Mix(color1, color2, factor) => {
                assert!(
                    matches!(*color1, ColorExpr::Ref { section, key } if section == "base" && key == "background")
                );
                assert!(
                    matches!(*color2, ColorExpr::Ref { section, key } if section == "colors" && key == "night")
                );
                assert!((factor - 0.15).abs() < 0.001);
            }
            _ => panic!("expected Mix"),
        }
    }
}
