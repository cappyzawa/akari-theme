use crate::{Error, Variant};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

// Raw types for TOML deserialization (before reference resolution)
#[derive(Debug, Deserialize)]
struct RawPalette {
    name: String,
    description: String,
    colors: RawColors,
    base: RawBase,
    layers: RawLayers,
    state: RawState,
    semantic: RawSemantic,
    ansi: RawAnsi,
}

#[derive(Debug, Deserialize)]
struct RawColors {
    lantern: String,
    ember: String,
    amber: String,
    life: String,
    night: String,
    muted: String,
}

#[derive(Debug, Deserialize)]
struct RawBase {
    background: String,
    foreground: String,
}

#[derive(Debug, Deserialize)]
struct RawLayers {
    base: String,
    surface: String,
    sunken: String,
    raised: String,
    border: String,
    inset: String,
}

#[derive(Debug, Deserialize)]
struct RawState {
    selection_bg: String,
    selection_fg: String,
    match_bg: String,
    cursor: String,
    cursor_text: String,
    info: String,
    hint: String,
    warning: String,
    error: String,
    active_bg: String,
    diff_added: String,
    diff_removed: String,
    diff_changed: String,
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

// Resolved types (after reference resolution)
#[derive(Debug, Clone, Serialize)]
pub struct Colors {
    pub lantern: String,
    pub ember: String,
    pub amber: String,
    pub life: String,
    pub night: String,
    pub muted: String,
}

impl From<RawColors> for Colors {
    fn from(raw: RawColors) -> Self {
        Self {
            lantern: raw.lantern,
            ember: raw.ember,
            amber: raw.amber,
            life: raw.life,
            night: raw.night,
            muted: raw.muted,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Base {
    pub background: String,
    pub foreground: String,
}

impl From<RawBase> for Base {
    fn from(raw: RawBase) -> Self {
        Self {
            background: raw.background,
            foreground: raw.foreground,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Layers {
    pub base: String,
    pub surface: String,
    pub sunken: String,
    pub raised: String,
    pub border: String,
    pub inset: String,
}

impl From<RawLayers> for Layers {
    fn from(raw: RawLayers) -> Self {
        Self {
            base: raw.base,
            surface: raw.surface,
            sunken: raw.sunken,
            raised: raw.raised,
            border: raw.border,
            inset: raw.inset,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
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

impl From<RawState> for State {
    fn from(raw: RawState) -> Self {
        Self {
            selection_bg: raw.selection_bg,
            selection_fg: raw.selection_fg,
            match_bg: raw.match_bg,
            cursor: raw.cursor,
            cursor_text: raw.cursor_text,
            info: raw.info,
            hint: raw.hint,
            warning: raw.warning,
            error: raw.error,
            active_bg: raw.active_bg,
            diff_added: raw.diff_added,
            diff_removed: raw.diff_removed,
            diff_changed: raw.diff_changed,
        }
    }
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

impl Ansi {
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &str)> {
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
            colors: [
                ("lantern", raw.colors.lantern.as_str()),
                ("ember", raw.colors.ember.as_str()),
                ("amber", raw.colors.amber.as_str()),
                ("life", raw.colors.life.as_str()),
                ("night", raw.colors.night.as_str()),
                ("muted", raw.colors.muted.as_str()),
            ]
            .into_iter()
            .collect(),
            base: [
                ("background", raw.base.background.as_str()),
                ("foreground", raw.base.foreground.as_str()),
            ]
            .into_iter()
            .collect(),
            layers: [
                ("base", raw.layers.base.as_str()),
                ("surface", raw.layers.surface.as_str()),
                ("sunken", raw.layers.sunken.as_str()),
                ("raised", raw.layers.raised.as_str()),
                ("border", raw.layers.border.as_str()),
                ("inset", raw.layers.inset.as_str()),
            ]
            .into_iter()
            .collect(),
            state: [
                ("selection_bg", raw.state.selection_bg.as_str()),
                ("selection_fg", raw.state.selection_fg.as_str()),
                ("match_bg", raw.state.match_bg.as_str()),
                ("cursor", raw.state.cursor.as_str()),
                ("cursor_text", raw.state.cursor_text.as_str()),
                ("info", raw.state.info.as_str()),
                ("hint", raw.state.hint.as_str()),
                ("warning", raw.state.warning.as_str()),
                ("error", raw.state.error.as_str()),
                ("active_bg", raw.state.active_bg.as_str()),
                ("diff_added", raw.state.diff_added.as_str()),
                ("diff_removed", raw.state.diff_removed.as_str()),
                ("diff_changed", raw.state.diff_changed.as_str()),
            ]
            .into_iter()
            .collect(),
            ansi_bright: [
                ("black", raw.ansi.bright.black.as_str()),
                ("red", raw.ansi.bright.red.as_str()),
                ("green", raw.ansi.bright.green.as_str()),
                ("yellow", raw.ansi.bright.yellow.as_str()),
                ("blue", raw.ansi.bright.blue.as_str()),
                ("magenta", raw.ansi.bright.magenta.as_str()),
                ("cyan", raw.ansi.bright.cyan.as_str()),
                ("white", raw.ansi.bright.white.as_str()),
            ]
            .into_iter()
            .collect(),
        }
    }

    fn resolve(&self, value: &str) -> Result<String, Error> {
        if value.starts_with('#') {
            return Ok(value.to_string());
        }
        let (section, key) = value
            .split_once('.')
            .ok_or_else(|| Error::UnresolvedRef(value.to_string()))?;
        let map = match section {
            "colors" => &self.colors,
            "base" => &self.base,
            "layers" => &self.layers,
            "state" => &self.state,
            "ansi_bright" => &self.ansi_bright,
            _ => return Err(Error::UnresolvedRef(value.to_string())),
        };
        map.get(key)
            .map(|s| (*s).to_string())
            .ok_or_else(|| Error::UnresolvedRef(value.to_string()))
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
        let resolver = Resolver::new(&raw);

        let semantic = Semantic {
            text: resolver.resolve(&raw.semantic.text)?,
            comment: resolver.resolve(&raw.semantic.comment)?,
            string: resolver.resolve(&raw.semantic.string)?,
            keyword: resolver.resolve(&raw.semantic.keyword)?,
            number: resolver.resolve(&raw.semantic.number)?,
            constant: resolver.resolve(&raw.semantic.constant)?,
            r#type: resolver.resolve(&raw.semantic.r#type)?,
            function: resolver.resolve(&raw.semantic.function)?,
            variable: resolver.resolve(&raw.semantic.variable)?,
            success: resolver.resolve(&raw.semantic.success)?,
            path: resolver.resolve(&raw.semantic.path)?,
        };

        let ansi = Ansi {
            black: resolver.resolve(&raw.ansi.black)?,
            red: resolver.resolve(&raw.ansi.red)?,
            green: resolver.resolve(&raw.ansi.green)?,
            yellow: resolver.resolve(&raw.ansi.yellow)?,
            blue: resolver.resolve(&raw.ansi.blue)?,
            magenta: resolver.resolve(&raw.ansi.magenta)?,
            cyan: resolver.resolve(&raw.ansi.cyan)?,
            white: resolver.resolve(&raw.ansi.white)?,
        };

        let ansi_bright = Ansi {
            black: resolver.resolve(&raw.ansi.bright.black)?,
            red: resolver.resolve(&raw.ansi.bright.red)?,
            green: resolver.resolve(&raw.ansi.bright.green)?,
            yellow: resolver.resolve(&raw.ansi.bright.yellow)?,
            blue: resolver.resolve(&raw.ansi.bright.blue)?,
            magenta: resolver.resolve(&raw.ansi.bright.magenta)?,
            cyan: resolver.resolve(&raw.ansi.bright.cyan)?,
            white: resolver.resolve(&raw.ansi.bright.white)?,
        };

        Ok(Self {
            variant,
            name: raw.name,
            description: raw.description,
            colors: raw.colors.into(),
            base: raw.base.into(),
            layers: raw.layers.into(),
            state: raw.state.into(),
            semantic,
            ansi,
            ansi_bright,
        })
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
