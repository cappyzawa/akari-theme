use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read palette file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse palette: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("unresolved reference: {0}")]
    UnresolvedRef(String),
}

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

        // Resolve semantic references
        for (key, value) in &raw.semantic {
            let resolved = palette.resolve_ref(value)?;
            palette.semantic.insert(key.clone(), resolved);
        }

        // Build ansi map with resolved references
        palette.ansi.insert("black".into(), raw.ansi.black);
        palette.ansi.insert("red".into(), raw.ansi.red);
        palette
            .ansi
            .insert("green".into(), palette.resolve_ref(&raw.ansi.green)?);
        palette
            .ansi
            .insert("yellow".into(), palette.resolve_ref(&raw.ansi.yellow)?);
        palette
            .ansi
            .insert("blue".into(), palette.resolve_ref(&raw.ansi.blue)?);
        palette
            .ansi
            .insert("magenta".into(), palette.resolve_ref(&raw.ansi.magenta)?);
        palette.ansi.insert("cyan".into(), raw.ansi.cyan);
        palette
            .ansi
            .insert("white".into(), palette.resolve_ref(&raw.ansi.white)?);

        // Build ansi_bright map with resolved references
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

        let parts: Vec<&str> = value.split('.').collect();
        if parts.len() != 2 {
            return Err(Error::UnresolvedRef(value.to_string()));
        }

        let (section, key) = (parts[0], parts[1]);
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
