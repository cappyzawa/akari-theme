use crate::{Error, Variant};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

// Raw types for TOML deserialization (before reference resolution)
// Note: Colors, Base, Layers, State are defined later and used directly here
// since they don't need reference resolution.
#[derive(Debug, Deserialize)]
struct RawPalette {
    name: String,
    description: String,
    colors: Colors,
    base: Base,
    layers: Layers,
    state: State,
    semantic: RawSemantic,
    ansi: RawAnsi,
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
/// Represents either a literal hex color or a reference to another palette field.
/// Future variants (e.g., `Lighten`, `Darken`) can be added here.
#[derive(Debug, Clone)]
enum ColorExpr {
    /// A literal hex color (e.g., "#E26A3B")
    Literal(String),
    /// A reference to another field (e.g., "colors.lantern")
    Ref { section: String, key: String },
}

impl<'de> Deserialize<'de> for ColorExpr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with('#') {
            Ok(ColorExpr::Literal(s))
        } else {
            let (section, key) = s
                .split_once('.')
                .ok_or_else(|| serde::de::Error::custom(format!("invalid color reference: {s}")))?;
            Ok(ColorExpr::Ref {
                section: section.to_string(),
                key: key.to_string(),
            })
        }
    }
}

struct Resolver<'a> {
    colors: BTreeMap<&'a str, &'a str>,
    base: BTreeMap<&'a str, &'a str>,
    layers: BTreeMap<&'a str, &'a str>,
    state: BTreeMap<&'a str, &'a str>,
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
            layers: make_map!(raw.layers, base, surface, sunken, raised, border, inset),
            state: make_map!(
                raw.state,
                selection_bg,
                selection_fg,
                match_bg,
                cursor,
                cursor_text,
                info,
                hint,
                warning,
                error,
                active_bg,
                diff_added,
                diff_removed,
                diff_changed,
            ),
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
            "layers" => self
                .layers
                .get(key)
                .map(|s| (*s).to_string())
                .ok_or_else(|| Error::UnresolvedRef(format!("{section}.{key}"))),
            "state" => self
                .state
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
            layers: self.layers.clone(),
            state: self.state.clone(),
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
}
