use crate::Error;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct RawPalette {
    name: String,
    description: String,
    colors: BTreeMap<String, String>,
    base: BTreeMap<String, String>,
    layers: BTreeMap<String, String>,
    state: BTreeMap<String, String>,
    semantic: BTreeMap<String, String>,
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
    bright: BTreeMap<String, String>,
}

#[derive(Debug)]
pub struct Palette {
    pub name: String,
    pub description: String,
    pub colors: BTreeMap<String, String>,
    pub base: BTreeMap<String, String>,
    pub layers: BTreeMap<String, String>,
    pub state: BTreeMap<String, String>,
    pub semantic: BTreeMap<String, String>,
    pub ansi: BTreeMap<String, String>,
    pub ansi_bright: BTreeMap<String, String>,
}

impl Palette {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        let raw: RawPalette = toml::from_str(&content)?;

        let mut palette = Self {
            name: raw.name,
            description: raw.description,
            colors: raw.colors,
            base: raw.base,
            layers: raw.layers,
            state: raw.state,
            semantic: BTreeMap::new(),
            ansi: BTreeMap::new(),
            ansi_bright: BTreeMap::new(),
        };

        for (key, value) in &raw.semantic {
            let resolved = palette.resolve_ref(value)?;
            palette.semantic.insert(key.clone(), resolved);
        }

        let ansi_fields = [
            ("black", &raw.ansi.black),
            ("red", &raw.ansi.red),
            ("green", &raw.ansi.green),
            ("yellow", &raw.ansi.yellow),
            ("blue", &raw.ansi.blue),
            ("magenta", &raw.ansi.magenta),
            ("cyan", &raw.ansi.cyan),
            ("white", &raw.ansi.white),
        ];

        for (name, value) in ansi_fields {
            let resolved = palette.resolve_ref(value)?;
            palette.ansi.insert(name.into(), resolved);
        }

        for (key, value) in &raw.ansi.bright {
            let resolved = palette.resolve_ref(value)?;
            palette.ansi_bright.insert(key.clone(), resolved);
        }

        Ok(palette)
    }

    fn resolve_ref(&self, value: &str) -> Result<String, Error> {
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
            _ => return Err(Error::UnresolvedRef(value.to_string())),
        };

        map.get(key)
            .cloned()
            .ok_or_else(|| Error::UnresolvedRef(value.to_string()))
    }

    pub fn variant(&self) -> &str {
        if self.name.contains("night") {
            "night"
        } else if self.name.contains("dawn") {
            "dawn"
        } else {
            "unknown"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_palette() -> Palette {
        let mut colors = BTreeMap::new();
        colors.insert("lantern".into(), "#E26A3B".into());
        colors.insert("ember".into(), "#D65A3A".into());

        let mut base = BTreeMap::new();
        base.insert("background".into(), "#171B22".into());
        base.insert("foreground".into(), "#E6DED3".into());

        let mut layers = BTreeMap::new();
        layers.insert("surface".into(), "#1E2329".into());

        let mut state = BTreeMap::new();
        state.insert("cursor".into(), "#E26A3B".into());

        Palette {
            name: "akari-night".into(),
            description: "test".into(),
            colors,
            base,
            layers,
            state,
            semantic: BTreeMap::new(),
            ansi: BTreeMap::new(),
            ansi_bright: BTreeMap::new(),
        }
    }

    #[test]
    fn resolve_ref_direct_hex() {
        let palette = test_palette();
        assert_eq!(palette.resolve_ref("#FFFFFF").unwrap(), "#FFFFFF");
    }

    #[test]
    fn resolve_ref_colors_section() {
        let palette = test_palette();
        assert_eq!(palette.resolve_ref("colors.lantern").unwrap(), "#E26A3B");
    }

    #[test]
    fn resolve_ref_base_section() {
        let palette = test_palette();
        assert_eq!(
            palette.resolve_ref("base.background").unwrap(),
            "#171B22"
        );
    }

    #[test]
    fn resolve_ref_layers_section() {
        let palette = test_palette();
        assert_eq!(palette.resolve_ref("layers.surface").unwrap(), "#1E2329");
    }

    #[test]
    fn resolve_ref_state_section() {
        let palette = test_palette();
        assert_eq!(palette.resolve_ref("state.cursor").unwrap(), "#E26A3B");
    }

    #[test]
    fn resolve_ref_unknown_section() {
        let palette = test_palette();
        assert!(palette.resolve_ref("unknown.key").is_err());
    }

    #[test]
    fn resolve_ref_unknown_key() {
        let palette = test_palette();
        assert!(palette.resolve_ref("colors.nonexistent").is_err());
    }

    #[test]
    fn resolve_ref_no_dot() {
        let palette = test_palette();
        assert!(palette.resolve_ref("colors").is_err());
    }

    #[test]
    fn resolve_ref_empty() {
        let palette = test_palette();
        assert!(palette.resolve_ref("").is_err());
    }

    #[test]
    fn variant_night() {
        let palette = test_palette();
        assert_eq!(palette.variant(), "night");
    }

    #[test]
    fn variant_dawn() {
        let mut palette = test_palette();
        palette.name = "akari-dawn".into();
        assert_eq!(palette.variant(), "dawn");
    }

    #[test]
    fn variant_unknown() {
        let mut palette = test_palette();
        palette.name = "other".into();
        assert_eq!(palette.variant(), "unknown");
    }
}
