mod generator;
mod palette;
pub mod terminal;
pub mod tools;

pub use generator::Generator;
pub use palette::Palette;

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read palette file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse palette: {0}")]
    ParsePalette(#[from] toml::de::Error),
    #[error("unresolved reference: {0}")]
    UnresolvedRef(String),
    #[error("failed to initialize template engine: {0}")]
    TemplateInit(tera::Error),
    #[error("failed to render template: {0}")]
    TemplateRender(tera::Error),
    #[error("invalid hex color: {0}")]
    InvalidHex(String),
    #[error("failed to write plist: {0}")]
    Plist(#[from] plist::Error),
    #[error("could not find project root (directory containing palette/ and Cargo.toml)")]
    ProjectRootNotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Night,
    Dawn,
}

impl Variant {
    pub fn name(self) -> &'static str {
        match self {
            Self::Night => "night",
            Self::Dawn => "dawn",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Night => "Night",
            Self::Dawn => "Dawn",
        }
    }

    pub fn palette_filename(self) -> &'static str {
        match self {
            Self::Night => "akari-night.toml",
            Self::Dawn => "akari-dawn.toml",
        }
    }
}

pub const VARIANTS: [Variant; 2] = [Variant::Night, Variant::Dawn];

/// A generated theme file
#[derive(Debug, Clone)]
pub struct Artifact {
    /// Relative path from output root (e.g., "helix/akari-night.toml")
    pub rel_path: PathBuf,
    /// Generated content
    pub content: String,
}

impl Artifact {
    pub fn new(rel_path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        Self {
            rel_path: rel_path.into(),
            content: content.into(),
        }
    }
}

pub fn find_project_root() -> Result<PathBuf, Error> {
    let mut current = std::env::current_dir()?;

    loop {
        if current.join("palette").is_dir() && current.join("Cargo.toml").is_file() {
            return Ok(current);
        }

        if !current.pop() {
            return Err(Error::ProjectRootNotFound);
        }
    }
}
