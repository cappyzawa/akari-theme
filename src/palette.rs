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
