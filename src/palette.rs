use crate::ansi::{AnsiColors, RawAnsi};
use crate::expr::{ColorExpr, ResolveRef, Resolver, resolve_fields};
use crate::theme::Base;
use crate::{Error, Rgb, Variant};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

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
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<Layers, Error> {
        resolve_fields!(resolver, self => Layers { base, surface, sunken, raised, border, inset })
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
    diff_added_bg: ColorExpr,
    diff_removed: ColorExpr,
    diff_removed_bg: ColorExpr,
    diff_changed: ColorExpr,
    diff_moved: ColorExpr,
    conflict: ColorExpr,
}

impl RawState {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<State, Error> {
        resolve_fields!(resolver, self => State {
            selection_bg, selection_fg, match_bg, cursor, cursor_text,
            info, hint, warning, error, active_bg,
            diff_added, diff_added_bg, diff_removed, diff_removed_bg,
            diff_changed, diff_moved, conflict,
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
    member: ColorExpr,
    success: ColorExpr,
    path: ColorExpr,
    r#macro: ColorExpr,
    escape: ColorExpr,
    regexp: ColorExpr,
    link: ColorExpr,
    directory: ColorExpr,
}

impl RawSemantic {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<Semantic, Error> {
        resolve_fields!(resolver, self => Semantic {
            text, comment, string, keyword, number, constant,
            r#type, function, variable, member, success, path,
            r#macro, escape, regexp, link, directory,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lantern {
    pub ember: Rgb, // inner heat — flame, fuel, origin of light
    pub near: Rgb,  // hibukuro — paper seen up close
    pub mid: Rgb,   // glow — lantern as perceived light
    pub far: Rgb,   // warm blur — light at a distance
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Colors {
    pub lantern: Lantern,
    pub life: Rgb,
    pub night: Rgb,
    pub rain: Rgb,
    pub muted: Rgb,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Layers {
    pub base: Rgb,
    pub surface: Rgb,
    pub sunken: Rgb,
    pub raised: Rgb,
    pub border: Rgb,
    pub inset: Rgb,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub selection_bg: Rgb,
    pub selection_fg: Rgb,
    pub match_bg: Rgb,
    pub cursor: Rgb,
    pub cursor_text: Rgb,
    pub info: Rgb,
    pub hint: Rgb,
    pub warning: Rgb,
    pub error: Rgb,
    pub active_bg: Rgb,
    pub diff_added: Rgb,
    pub diff_added_bg: Rgb,
    pub diff_removed: Rgb,
    pub diff_removed_bg: Rgb,
    pub diff_changed: Rgb,
    pub diff_moved: Rgb,
    pub conflict: Rgb,
}

#[derive(Debug, Clone, Serialize)]
pub struct Semantic {
    pub text: Rgb,
    pub comment: Rgb,
    pub string: Rgb,
    pub keyword: Rgb,
    pub number: Rgb,
    pub constant: Rgb,
    pub r#type: Rgb,
    pub function: Rgb,
    pub variable: Rgb,
    pub member: Rgb,
    pub success: Rgb,
    pub path: Rgb,
    pub r#macro: Rgb,
    pub escape: Rgb,
    pub regexp: Rgb,
    pub link: Rgb,
    pub directory: Rgb,
}

impl RawPalette {
    fn resolve(&self, variant: Variant) -> Result<Palette, Error> {
        let colors: BTreeMap<String, Rgb> = [
            ("lantern.ember".to_string(), self.colors.lantern.ember),
            ("lantern.near".to_string(), self.colors.lantern.near),
            ("lantern.mid".to_string(), self.colors.lantern.mid),
            ("lantern.far".to_string(), self.colors.lantern.far),
            ("life".to_string(), self.colors.life),
            ("night".to_string(), self.colors.night),
            ("rain".to_string(), self.colors.rain),
            ("muted".to_string(), self.colors.muted),
        ]
        .into_iter()
        .collect();
        let base: BTreeMap<String, Rgb> = [
            ("background".to_string(), self.base.background),
            ("foreground".to_string(), self.base.foreground),
        ]
        .into_iter()
        .collect();

        // Stages 1 and 2: resolve ansi and ansi.bright.
        let (ansi, ansi_map) = self.ansi.resolve(&colors, &base)?;

        // Stage 3: resolve remaining sections (depends on all ansi).
        let resolver = Resolver {
            colors: &colors,
            base: &base,
            ansi: Some(&ansi_map),
        };

        Ok(Palette {
            variant,
            name: self.name.clone(),
            description: self.description.clone(),
            colors: self.colors.clone(),
            base: self.base.clone(),
            layers: self.layers.resolve(&resolver)?,
            state: self.state.resolve(&resolver)?,
            semantic: self.semantic.resolve(&resolver)?,
            ansi: ansi.normal,
            ansi_bright: ansi.bright,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Palette {
    pub variant: Variant,
    pub name: String,
    pub description: String,
    pub colors: Colors,
    pub base: Base,
    pub layers: Layers,
    pub state: State,
    pub semantic: Semantic,
    pub ansi: AnsiColors,
    pub ansi_bright: AnsiColors,
}

impl Palette {
    /// Embedded Night palette TOML content.
    const NIGHT_TOML: &'static str = include_str!("../palette/akari-night.toml");

    /// Embedded Dawn palette TOML content.
    const DAWN_TOML: &'static str = include_str!("../palette/akari-dawn.toml");

    /// Returns the embedded Night palette.
    ///
    /// # Panics
    ///
    /// Panics if the embedded palette is invalid (should never happen in normal use).
    #[must_use]
    pub fn night() -> Self {
        Self::from_str(Self::NIGHT_TOML, Variant::Night)
            .expect("embedded Night palette should be valid")
    }

    /// Returns the embedded Dawn palette.
    ///
    /// # Panics
    ///
    /// Panics if the embedded palette is invalid (should never happen in normal use).
    #[must_use]
    pub fn dawn() -> Self {
        Self::from_str(Self::DAWN_TOML, Variant::Dawn)
            .expect("embedded Dawn palette should be valid")
    }

    /// Load palette from a file path.
    pub fn from_path(path: impl AsRef<Path>, variant: Variant) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        Self::from_str(&content, variant)
    }

    /// Parse palette from TOML string content.
    pub fn from_str(content: &str, variant: Variant) -> Result<Self, Error> {
        let raw: RawPalette = toml::from_str(content)?;
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

    fn hex(s: &str) -> Rgb {
        s.parse().unwrap()
    }

    #[test]
    fn colors_are_loaded() {
        let palette = Palette::from_path(palette_path(), Variant::Night).unwrap();
        assert_eq!(palette.colors.lantern.mid, hex("#E26A3B"));
        assert_eq!(palette.colors.lantern.ember, hex("#D65A3A"));
        assert_eq!(palette.colors.lantern.near, hex("#D25046"));
        assert_eq!(palette.colors.lantern.far, hex("#D4A05A"));
    }

    #[test]
    fn base_colors_are_loaded() {
        let palette = Palette::from_path(palette_path(), Variant::Night).unwrap();
        assert_eq!(palette.base.background, hex("#25231F"));
        assert_eq!(palette.base.foreground, hex("#E6DED3"));
    }

    #[test]
    fn semantic_references_resolved() {
        let palette = Palette::from_path(palette_path(), Variant::Night).unwrap();
        // semantic.keyword = "colors.lantern.mid" -> "#E26A3B"
        assert_eq!(palette.semantic.keyword, hex("#E26A3B"));
        // semantic.string = "colors.life" -> "#7FAF6A"
        assert_eq!(palette.semantic.string, hex("#7FAF6A"));
        // semantic.member = "colors.night" -> "#7A8FA2"
        assert_eq!(palette.semantic.member, hex("#7A8FA2"));
    }

    #[test]
    fn ansi_references_resolved() {
        let palette = Palette::from_path(palette_path(), Variant::Night).unwrap();
        // ansi.green = "colors.life" -> "#7FAF6A"
        assert_eq!(palette.ansi.green, hex("#7FAF6A"));
        // ansi.white = "base.foreground" -> "#E6DED3"
        assert_eq!(palette.ansi.white, hex("#E6DED3"));
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
diff_added_bg = "#2A3A2A"
diff_removed = "#D65A3A"
diff_removed_bg = "#3A2A2A"
diff_changed = "#D4A05A"
diff_moved = "#5A6F82"
conflict = "#D65A3A"

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
member = "colors.night"
success = "colors.life"
path = "ansi.green"
macro = "ansi.bright.magenta"
escape = "ansi.bright.magenta"
regexp = "ansi.bright.green"
link = "ansi.bright.blue"
directory = "ansi.cyan"

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
}
