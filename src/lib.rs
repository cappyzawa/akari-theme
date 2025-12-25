mod color;
mod generator;
mod palette;
pub mod terminal;

pub use color::Rgb;
pub use generator::Generator;
pub use palette::Palette;

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid palette: {0}")]
    ParsePalette(#[from] toml::de::Error),
    #[error("unresolved reference: {0}")]
    UnresolvedRef(String),
    #[error("template init failed: {0}")]
    TemplateInit(tera::Error),
    #[error("template render failed: {0}")]
    TemplateRender(tera::Error),
    #[error("invalid hex color: {0}")]
    InvalidHex(String),
    #[error("non-UTF-8 path: {0}")]
    InvalidPath(PathBuf),
    #[error("project root not found (expected palette/ and Cargo.toml)")]
    ProjectRootNotFound,
    #[error("plist error: {0}")]
    Plist(#[from] plist::Error),
    #[error("plist output was not valid UTF-8")]
    PlistUtf8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Night,
    Dawn,
}

impl Variant {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Night => "night",
            Self::Dawn => "dawn",
        }
    }

    pub const fn title(self) -> &'static str {
        match self {
            Self::Night => "Night",
            Self::Dawn => "Dawn",
        }
    }

    pub const fn palette_filename(self) -> &'static str {
        match self {
            Self::Night => "akari-night.toml",
            Self::Dawn => "akari-dawn.toml",
        }
    }
}

pub const VARIANTS: [Variant; 2] = [Variant::Night, Variant::Dawn];

/// Content of an artifact
#[derive(Debug, Clone)]
pub enum ArtifactContent {
    /// Text content to be written
    Text(String),
    /// Source path to be copied
    Copy(PathBuf),
}

/// A generated or copied file
#[derive(Debug, Clone)]
pub struct Artifact {
    /// Relative path from output root (e.g., "helix/akari-night.toml")
    pub rel_path: PathBuf,
    /// Content or source path
    pub content: ArtifactContent,
}

impl Artifact {
    pub fn text(rel_path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        Self {
            rel_path: rel_path.into(),
            content: ArtifactContent::Text(content.into()),
        }
    }

    pub fn copy(rel_path: impl Into<PathBuf>, src: impl Into<PathBuf>) -> Self {
        Self {
            rel_path: rel_path.into(),
            content: ArtifactContent::Copy(src.into()),
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
