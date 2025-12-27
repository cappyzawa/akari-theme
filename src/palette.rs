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
            base: resolve_expr(resolver, &self.base)?,
            surface: resolve_expr(resolver, &self.surface)?,
            sunken: resolve_expr(resolver, &self.sunken)?,
            raised: resolve_expr(resolver, &self.raised)?,
            border: resolve_expr(resolver, &self.border)?,
            inset: resolve_expr(resolver, &self.inset)?,
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
            selection_bg: resolve_expr(resolver, &self.selection_bg)?,
            selection_fg: resolve_expr(resolver, &self.selection_fg)?,
            match_bg: resolve_expr(resolver, &self.match_bg)?,
            cursor: resolve_expr(resolver, &self.cursor)?,
            cursor_text: resolve_expr(resolver, &self.cursor_text)?,
            info: resolve_expr(resolver, &self.info)?,
            hint: resolve_expr(resolver, &self.hint)?,
            warning: resolve_expr(resolver, &self.warning)?,
            error: resolve_expr(resolver, &self.error)?,
            active_bg: resolve_expr(resolver, &self.active_bg)?,
            diff_added: resolve_expr(resolver, &self.diff_added)?,
            diff_removed: resolve_expr(resolver, &self.diff_removed)?,
            diff_changed: resolve_expr(resolver, &self.diff_changed)?,
        })
    }
}

/// Common structure for ANSI color definitions (used by both ansi and ansi.bright)
#[derive(Debug, Deserialize)]
struct RawAnsiColors {
    black: ColorExpr,
    red: ColorExpr,
    green: ColorExpr,
    yellow: ColorExpr,
    blue: ColorExpr,
    magenta: ColorExpr,
    cyan: ColorExpr,
    white: ColorExpr,
}

impl RawAnsiColors {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<Ansi, Error> {
        Ok(Ansi {
            black: resolve_expr(resolver, &self.black)?,
            red: resolve_expr(resolver, &self.red)?,
            green: resolve_expr(resolver, &self.green)?,
            yellow: resolve_expr(resolver, &self.yellow)?,
            blue: resolve_expr(resolver, &self.blue)?,
            magenta: resolve_expr(resolver, &self.magenta)?,
            cyan: resolve_expr(resolver, &self.cyan)?,
            white: resolve_expr(resolver, &self.white)?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawAnsi {
    #[serde(flatten)]
    base: RawAnsiColors,
    bright: RawAnsiColors,
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
    r#macro: ColorExpr,
    escape: ColorExpr,
    regexp: ColorExpr,
    link: ColorExpr,
}

impl RawSemantic {
    fn resolve(&self, resolver: &Resolver) -> Result<Semantic, Error> {
        Ok(Semantic {
            text: resolve_expr(resolver, &self.text)?,
            comment: resolve_expr(resolver, &self.comment)?,
            string: resolve_expr(resolver, &self.string)?,
            keyword: resolve_expr(resolver, &self.keyword)?,
            number: resolve_expr(resolver, &self.number)?,
            constant: resolve_expr(resolver, &self.constant)?,
            r#type: resolve_expr(resolver, &self.r#type)?,
            function: resolve_expr(resolver, &self.function)?,
            variable: resolve_expr(resolver, &self.variable)?,
            success: resolve_expr(resolver, &self.success)?,
            path: resolve_expr(resolver, &self.path)?,
            r#macro: resolve_expr(resolver, &self.r#macro)?,
            escape: resolve_expr(resolver, &self.escape)?,
            regexp: resolve_expr(resolver, &self.regexp)?,
            link: resolve_expr(resolver, &self.link)?,
        })
    }
}

// Resolved types (used for both deserialization and template rendering)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lantern {
    pub ember: String, // inner heat — flame, fuel, origin of light
    pub near: String,  // hibukuro — paper seen up close
    pub mid: String,   // glow — lantern as perceived light
    pub far: String,   // warm blur — light at a distance
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Colors {
    pub lantern: Lantern,
    pub life: String,
    pub night: String,
    pub rain: String,
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
    pub r#macro: String,
    pub escape: String,
    pub regexp: String,
    pub link: String,
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

impl Ansi {
    /// Convert to a map for use in Resolver
    fn to_map(&self) -> BTreeMap<&'static str, String> {
        self.into_iter().map(|(k, v)| (k, v.to_string())).collect()
    }
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

/// Reference section for color expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Colors,
    Base,
    Ansi,
    AnsiBright,
}

impl Section {
    fn parse(s: &str) -> Result<Self, Error> {
        match s {
            "colors" => Ok(Self::Colors),
            "base" => Ok(Self::Base),
            "ansi" => Ok(Self::Ansi),
            _ => Err(Error::InvalidColorExpr(format!("unknown section: {s}"))),
        }
    }

    const fn as_str(&self) -> &'static str {
        match self {
            Self::Colors => "colors",
            Self::Base => "base",
            Self::Ansi => "ansi",
            Self::AnsiBright => "ansi.bright",
        }
    }
}

/// A color expression that can be deserialized from TOML.
///
/// Supports:
/// - Literal hex colors: `"#E26A3B"`
/// - References: `"colors.lantern"`
/// - Functions:
///   - `"lighten(colors.lantern, 0.1)"` — increase lightness proportionally
///   - `"darken(base.background, 0.2)"` — decrease lightness proportionally
///   - `"brighten(ansi.red, 0.1)"` — adjust lightness by absolute amount
///   - `"mix(base.background, colors.night, 0.15)"` — blend two colors
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "String")]
enum ColorExpr {
    /// A literal hex color (e.g., "#E26A3B")
    Literal(String),
    /// A reference to another field (e.g., "colors.lantern")
    Ref { section: Section, key: String },
    /// Lighten a color by a factor (0.0 = unchanged, 1.0 = white)
    Lighten(Box<ColorExpr>, f64),
    /// Darken a color by a factor (0.0 = unchanged, 1.0 = black)
    Darken(Box<ColorExpr>, f64),
    /// Brighten a color by absolute amount (positive = brighter, negative = dimmer)
    Brighten(Box<ColorExpr>, f64),
    /// Mix two colors (0.0 = first color, 1.0 = second color)
    Mix(Box<ColorExpr>, Box<ColorExpr>, f64),
}

impl TryFrom<String> for ColorExpr {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        parse_color_expr(&s)
    }
}

/// Strip function call syntax: "fn_name(args)" -> Some("args")
fn strip_fn_call<'a>(s: &'a str, name: &str) -> Option<&'a str> {
    s.strip_prefix(name)
        .and_then(|r| r.strip_prefix('('))
        .and_then(|r| r.strip_suffix(')'))
}

/// Parse a color expression string into a ColorExpr.
fn parse_color_expr(s: &str) -> Result<ColorExpr, Error> {
    let s = s.trim();

    // Literal hex color
    if s.starts_with('#') {
        return Ok(ColorExpr::Literal(s.to_string()));
    }

    // Function call: lighten(...), darken(...), brighten(...), mix(...)
    if let Some(args) = strip_fn_call(s, "lighten") {
        let (inner, factor) = parse_unary_fn_args(args)?;
        return Ok(ColorExpr::Lighten(Box::new(inner), factor));
    }
    if let Some(args) = strip_fn_call(s, "darken") {
        let (inner, factor) = parse_unary_fn_args(args)?;
        return Ok(ColorExpr::Darken(Box::new(inner), factor));
    }
    if let Some(args) = strip_fn_call(s, "brighten") {
        let (inner, amount) = parse_unary_fn_args(args)?;
        return Ok(ColorExpr::Brighten(Box::new(inner), amount));
    }
    if let Some(args) = strip_fn_call(s, "mix") {
        let (color1, color2, factor) = parse_mix_args(args)?;
        return Ok(ColorExpr::Mix(Box::new(color1), Box::new(color2), factor));
    }

    // Reference: section.key or section.subsection.key (for ansi.bright.*)
    // Handle ansi.bright.* specially
    if let Some(rest) = s.strip_prefix("ansi.bright.") {
        return Ok(ColorExpr::Ref {
            section: Section::AnsiBright,
            key: rest.to_string(),
        });
    }

    let (section_str, key) = s
        .split_once('.')
        .ok_or_else(|| Error::InvalidColorExpr(s.to_string()))?;
    let section = Section::parse(section_str)?;
    Ok(ColorExpr::Ref {
        section,
        key: key.to_string(),
    })
}

/// Parse unary function arguments: "colors.lantern, 0.1" -> (ColorExpr, f64)
fn parse_unary_fn_args(args: &str) -> Result<(ColorExpr, f64), Error> {
    let (color_str, factor_str) = args
        .rsplit_once(',')
        .ok_or_else(|| Error::InvalidColorExpr(format!("expected 'color, factor': {args}")))?;
    let inner = parse_color_expr(color_str.trim())?;
    let factor = factor_str
        .trim()
        .parse::<f64>()
        .map_err(|_| Error::InvalidColorExpr(format!("invalid factor: {}", factor_str.trim())))?;
    Ok((inner, factor))
}

/// Parse mix function arguments: "color1, color2, 0.15" -> (ColorExpr, ColorExpr, f64)
fn parse_mix_args(args: &str) -> Result<(ColorExpr, ColorExpr, f64), Error> {
    // Split from right to get factor first
    let (rest, factor_str) = args.rsplit_once(',').ok_or_else(|| {
        Error::InvalidColorExpr(format!("expected 'color1, color2, factor': {args}"))
    })?;
    let factor = factor_str
        .trim()
        .parse::<f64>()
        .map_err(|_| Error::InvalidColorExpr(format!("invalid factor: {}", factor_str.trim())))?;

    // Split remaining to get two colors
    let (color1_str, color2_str) = rest.rsplit_once(',').ok_or_else(|| {
        Error::InvalidColorExpr(format!("expected 'color1, color2, factor': {args}"))
    })?;
    let color1 = parse_color_expr(color1_str.trim())?;
    let color2 = parse_color_expr(color2_str.trim())?;

    Ok((color1, color2, factor))
}

/// Trait for resolving color references.
trait ResolveRef {
    fn resolve_ref(&self, section: Section, key: &str) -> Result<String, Error>;
}

/// Resolve a color expression using a resolver.
fn resolve_expr(resolver: &impl ResolveRef, expr: &ColorExpr) -> Result<String, Error> {
    match expr {
        ColorExpr::Literal(hex) => Ok(hex.clone()),
        ColorExpr::Ref { section, key } => resolver.resolve_ref(*section, key),
        ColorExpr::Lighten(inner, factor) => {
            let hex = resolve_expr(resolver, inner)?;
            Ok(hex.parse::<Rgb>()?.lighten(*factor).to_string())
        }
        ColorExpr::Darken(inner, factor) => {
            let hex = resolve_expr(resolver, inner)?;
            Ok(hex.parse::<Rgb>()?.darken(*factor).to_string())
        }
        ColorExpr::Brighten(inner, amount) => {
            let hex = resolve_expr(resolver, inner)?;
            Ok(hex.parse::<Rgb>()?.brighten(*amount).to_string())
        }
        ColorExpr::Mix(color1, color2, factor) => {
            let rgb1: Rgb = resolve_expr(resolver, color1)?.parse()?;
            let rgb2: Rgb = resolve_expr(resolver, color2)?.parse()?;
            Ok(rgb1.mix(rgb2, *factor).to_string())
        }
    }
}

struct Resolver<'a> {
    colors: BTreeMap<&'a str, &'a str>,
    base: BTreeMap<&'a str, &'a str>,
    /// Resolved hex values for ansi (for reference lookup)
    ansi_map: BTreeMap<&'static str, String>,
    /// Resolved hex values for ansi.bright (for reference lookup)
    ansi_bright_map: BTreeMap<&'static str, String>,
    /// Resolved ansi colors (to avoid re-resolving)
    resolved_ansi: Ansi,
    /// Resolved ansi.bright colors (to avoid re-resolving)
    resolved_ansi_bright: Ansi,
}

impl<'a> Resolver<'a> {
    fn new(raw: &'a RawPalette) -> Result<Self, Error> {
        // Flatten nested lantern structure into colors map
        let colors: BTreeMap<&str, &str> = [
            ("lantern.ember", raw.colors.lantern.ember.as_str()),
            ("lantern.near", raw.colors.lantern.near.as_str()),
            ("lantern.mid", raw.colors.lantern.mid.as_str()),
            ("lantern.far", raw.colors.lantern.far.as_str()),
            ("life", raw.colors.life.as_str()),
            ("night", raw.colors.night.as_str()),
            ("rain", raw.colors.rain.as_str()),
            ("muted", raw.colors.muted.as_str()),
        ]
        .into_iter()
        .collect();
        let base: BTreeMap<&str, &str> = [
            ("background", raw.base.background.as_str()),
            ("foreground", raw.base.foreground.as_str()),
        ]
        .into_iter()
        .collect();

        // Resolve ansi first (it only depends on colors/base)
        let partial = PartialResolver {
            colors: &colors,
            base: &base,
            ansi: None,
        };
        let resolved_ansi = raw.ansi.base.resolve(&partial)?;
        let ansi_map = resolved_ansi.to_map();

        // Resolve ansi.bright (depends on ansi)
        let partial_with_ansi = PartialResolver {
            colors: &colors,
            base: &base,
            ansi: Some(&ansi_map),
        };
        let resolved_ansi_bright = raw.ansi.bright.resolve(&partial_with_ansi)?;
        let ansi_bright_map = resolved_ansi_bright.to_map();

        Ok(Self {
            colors,
            base,
            ansi_map,
            ansi_bright_map,
            resolved_ansi,
            resolved_ansi_bright,
        })
    }
}

impl ResolveRef for Resolver<'_> {
    fn resolve_ref(&self, section: Section, key: &str) -> Result<String, Error> {
        let ref_str = || format!("{}.{key}", section.as_str());
        match section {
            Section::Colors => self
                .colors
                .get(key)
                .copied()
                .map(str::to_string)
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
            Section::Base => self
                .base
                .get(key)
                .copied()
                .map(str::to_string)
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
            Section::Ansi => self
                .ansi_map
                .get(key)
                .cloned()
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
            Section::AnsiBright => self
                .ansi_bright_map
                .get(key)
                .cloned()
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
        }
    }
}

/// Resolver for bootstrapping ansi/ansi.bright resolution.
/// Only colors, base, and optionally ansi are available.
struct PartialResolver<'a> {
    colors: &'a BTreeMap<&'a str, &'a str>,
    base: &'a BTreeMap<&'a str, &'a str>,
    ansi: Option<&'a BTreeMap<&'static str, String>>,
}

impl ResolveRef for PartialResolver<'_> {
    fn resolve_ref(&self, section: Section, key: &str) -> Result<String, Error> {
        let ref_str = || format!("{}.{key}", section.as_str());
        match section {
            Section::Colors => self
                .colors
                .get(key)
                .copied()
                .map(str::to_string)
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
            Section::Base => self
                .base
                .get(key)
                .copied()
                .map(str::to_string)
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
            Section::Ansi => self
                .ansi
                .and_then(|m| m.get(key).cloned())
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
            Section::AnsiBright => Err(Error::UnresolvedRef(ref_str())),
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
            ansi: resolver.resolved_ansi,
            ansi_bright: resolver.resolved_ansi_bright,
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
        assert_eq!(palette.colors.lantern.mid, "#E26A3B");
        assert_eq!(palette.colors.lantern.ember, "#D65A3A");
        assert_eq!(palette.colors.lantern.near, "#D25046");
        assert_eq!(palette.colors.lantern.far, "#D4A05A");
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
        // semantic.keyword = "colors.lantern.mid" -> "#E26A3B"
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

[colors.lantern]
ember = "#D65A3A"
near = "#D25046"
mid = "#E26A3B"
far = "#D4A05A"

[colors]
life = "#7FAF6A"
night = "#5A6F82"
rain = "#6F8F8A"
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
keyword = "colors.lantern.mid"
number = "colors.lantern.far"
constant = "colors.lantern.far"
type = "colors.lantern.far"
function = "colors.lantern.mid"
variable = "base.foreground"
success = "colors.life"

[ansi]
black = "#171B22"
red = "colors.lantern.near"
green = "colors.life"
yellow = "colors.lantern.far"
blue = "colors.night"
magenta = "colors.muted"
cyan = "colors.rain"
white = "base.foreground"

[ansi.bright]
black = "#3A424D"
red = "colors.lantern.mid"
green = "colors.life"
yellow = "colors.lantern.far"
blue = "colors.night"
magenta = "colors.muted"
cyan = "colors.rain"
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

[colors.lantern]
ember = "#D65A3A"
near = "#D25046"
mid = "#E26A3B"
far = "#D4A05A"

[colors]
life = "#7FAF6A"
night = "#5A6F82"
rain = "#6F8F8A"
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
keyword = "colors.lantern.mid"
number = "colors.lantern.far"
constant = "colors.lantern.far"
type = "colors.lantern.far"
function = "colors.lantern.mid"
variable = "base.foreground"
success = "colors.life"
path = "ansi.green"
macro = "ansi.bright.magenta"
escape = "ansi.bright.magenta"
regexp = "ansi.bright.green"
link = "ansi.bright.blue"

[ansi]
black = "#171B22"
red = "colors.lantern.near"
green = "colors.life"
yellow = "colors.lantern.far"
blue = "colors.night"
magenta = "colors.muted"
cyan = "colors.rain"
white = "base.foreground"

[ansi.bright]
black = "#3A424D"
red = "colors.lantern.mid"
green = "colors.life"
yellow = "colors.lantern.far"
blue = "colors.night"
magenta = "colors.muted"
cyan = "colors.rain"
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
        let expr = parse_color_expr("colors.lantern.mid").unwrap();
        assert!(
            matches!(expr, ColorExpr::Ref { section, key } if section == Section::Colors && key == "lantern.mid")
        );
    }

    #[test]
    fn parse_color_expr_lighten() {
        let expr = parse_color_expr("lighten(colors.lantern.mid, 0.1)").unwrap();
        match expr {
            ColorExpr::Lighten(inner, factor) => {
                assert!(
                    matches!(*inner, ColorExpr::Ref { section, key } if section == Section::Colors && key == "lantern.mid")
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
                    matches!(*inner, ColorExpr::Ref { section, key } if section == Section::Base && key == "background")
                );
                assert!((factor - 0.2).abs() < 0.001);
            }
            _ => panic!("expected Darken"),
        }
    }

    #[test]
    fn parse_color_expr_nested() {
        let expr = parse_color_expr("lighten(darken(colors.lantern.mid, 0.1), 0.2)").unwrap();
        match expr {
            ColorExpr::Lighten(inner, outer_factor) => {
                assert!((outer_factor - 0.2).abs() < 0.001);
                match *inner {
                    ColorExpr::Darken(innermost, inner_factor) => {
                        assert!(
                            matches!(*innermost, ColorExpr::Ref { section, key } if section == Section::Colors && key == "lantern.mid")
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
                    matches!(*color1, ColorExpr::Ref { section, key } if section == Section::Base && key == "background")
                );
                assert!(
                    matches!(*color2, ColorExpr::Ref { section, key } if section == Section::Colors && key == "night")
                );
                assert!((factor - 0.15).abs() < 0.001);
            }
            _ => panic!("expected Mix"),
        }
    }
}
