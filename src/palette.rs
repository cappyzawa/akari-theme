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
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
    bright: RawAnsiBright,
}

#[derive(Debug, Deserialize)]
struct RawAnsiBright {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
}

impl RawAnsiBright {
    fn resolve(&self, resolver: &Resolver) -> Result<Ansi, Error> {
        Ok(Ansi {
            black: resolver.resolve(&self.black)?,
            red: resolver.resolve(&self.red)?,
            green: resolver.resolve(&self.green)?,
            yellow: resolver.resolve(&self.yellow)?,
            blue: resolver.resolve(&self.blue)?,
            magenta: resolver.resolve(&self.magenta)?,
            cyan: resolver.resolve(&self.cyan)?,
            white: resolver.resolve(&self.white)?,
        })
    }
}

impl RawAnsi {
    fn resolve(&self, resolver: &Resolver) -> Result<Ansi, Error> {
        Ok(Ansi {
            black: resolver.resolve(&self.black)?,
            red: resolver.resolve(&self.red)?,
            green: resolver.resolve(&self.green)?,
            yellow: resolver.resolve(&self.yellow)?,
            blue: resolver.resolve(&self.blue)?,
            magenta: resolver.resolve(&self.magenta)?,
            cyan: resolver.resolve(&self.cyan)?,
            white: resolver.resolve(&self.white)?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawSemantic {
    text: String,
    comment: String,
    string: String,
    keyword: String,
    number: String,
    constant: String,
    r#type: String,
    function: String,
    variable: String,
    success: String,
    path: String,
}

impl RawSemantic {
    fn resolve(&self, resolver: &Resolver) -> Result<Semantic, Error> {
        Ok(Semantic {
            text: resolver.resolve(&self.text)?,
            comment: resolver.resolve(&self.comment)?,
            string: resolver.resolve(&self.string)?,
            keyword: resolver.resolve(&self.keyword)?,
            number: resolver.resolve(&self.number)?,
            constant: resolver.resolve(&self.constant)?,
            r#type: resolver.resolve(&self.r#type)?,
            function: resolver.resolve(&self.function)?,
            variable: resolver.resolve(&self.variable)?,
            success: resolver.resolve(&self.success)?,
            path: resolver.resolve(&self.path)?,
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

/// Parsed color value: either a literal hex color or a reference to another field.
enum ParsedColor<'a> {
    /// A literal hex color (e.g., "#E26A3B")
    Literal(&'a str),
    /// A reference to another field (e.g., "colors.lantern")
    Ref { section: &'a str, key: &'a str },
}

/// Parse a color string into a ParsedColor.
fn parse_color(value: &str) -> Result<ParsedColor<'_>, Error> {
    if value.starts_with('#') {
        Ok(ParsedColor::Literal(value))
    } else {
        let (section, key) = value
            .split_once('.')
            .ok_or_else(|| Error::UnresolvedRef(value.to_string()))?;
        Ok(ParsedColor::Ref { section, key })
    }
}

struct Resolver<'a> {
    colors: BTreeMap<&'a str, &'a str>,
    base: BTreeMap<&'a str, &'a str>,
    layers: BTreeMap<&'a str, &'a str>,
    state: BTreeMap<&'a str, &'a str>,
    ansi_bright: BTreeMap<&'a str, &'a str>,
}

impl<'a> Resolver<'a> {
    fn new(raw: &'a RawPalette) -> Self {
        Self {
            colors: make_map!(raw.colors, lantern, ember, amber, life, night, muted),
            base: make_map!(raw.base, background, foreground),
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
            ansi_bright: make_map!(
                raw.ansi.bright,
                black,
                red,
                green,
                yellow,
                blue,
                magenta,
                cyan,
                white,
            ),
        }
    }

    fn resolve_ref(&self, section: &str, key: &str) -> Result<String, Error> {
        let map = match section {
            "colors" => &self.colors,
            "base" => &self.base,
            "layers" => &self.layers,
            "state" => &self.state,
            "ansi_bright" => &self.ansi_bright,
            _ => {
                return Err(Error::UnresolvedRef(format!("{}.{}", section, key)));
            }
        };
        map.get(key)
            .map(|s| (*s).to_string())
            .ok_or_else(|| Error::UnresolvedRef(format!("{}.{}", section, key)))
    }

    fn resolve(&self, value: &str) -> Result<String, Error> {
        match parse_color(value)? {
            ParsedColor::Literal(v) => Ok(v.to_string()),
            ParsedColor::Ref { section, key } => self.resolve_ref(section, key),
        }
    }
}

impl RawPalette {
    fn resolve(&self, variant: Variant) -> Result<Palette, Error> {
        let resolver = Resolver::new(self);
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
