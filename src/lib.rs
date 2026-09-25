mod ansi;
mod color;
mod expr;
#[cfg(feature = "generator")]
mod generator;
mod palette;
#[cfg(feature = "generator")]
pub mod terminal;
pub mod theme;

pub use color::Rgb;
#[cfg(feature = "generator")]
pub use generator::Generator;
pub use palette::Palette;

use std::path::PathBuf;
use theme::Id;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid palette: {0}")]
    ParsePalette(#[from] toml::de::Error),
    #[error("unresolved reference: {0}")]
    UnresolvedRef(String),
    #[cfg(feature = "generator")]
    #[error("template {context}: {source}")]
    Template {
        context: &'static str,
        #[source]
        source: tera::Error,
    },
    #[error("invalid hex color: {0}")]
    InvalidHex(String),
    #[cfg(feature = "generator")]
    #[error("non-UTF-8 path: {0}")]
    InvalidPath(PathBuf),
    #[cfg(feature = "generator")]
    #[error("project root not found (expected palette/ and Cargo.toml)")]
    ProjectRootNotFound,
    #[cfg(feature = "generator")]
    #[error("plist error: {0}")]
    Plist(#[from] plist::Error),
    #[cfg(feature = "generator")]
    #[error("plist output was not valid UTF-8")]
    PlistUtf8,
    #[error("invalid color expression: {0}")]
    InvalidColorExpr(String),
    #[error("{path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("{path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("{path}: {source}")]
    Resolve {
        path: PathBuf,
        #[source]
        source: Box<Error>,
    },
    #[error("invalid id {0:?}, expected [a-z][a-z0-9]*")]
    InvalidId(String),
    #[error("{path}: variant.id is {found}, theme.variants lists {expected}")]
    VariantIdMismatch {
        path: PathBuf,
        expected: Id,
        found: Id,
    },
    #[error("{key}: expected a hex literal, found {value}")]
    ExpectedHexLiteral { key: String, value: toml::Value },
    #[error("roles.series must have exactly 8 entries, found {0}")]
    SeriesLength(usize),
    #[cfg(feature = "generator")]
    #[error("tool {0} is generated from a theme directory; use `generate-theme`")]
    ToolMigrated(String),
    #[cfg(feature = "generator")]
    #[error("tool {0} has no theme-based generator")]
    ToolNotThemed(String),
    #[cfg(feature = "generator")]
    #[error("adapters.{tool}.{key} is required to generate {tool}")]
    AdapterKeyMissing { tool: String, key: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Variant {
    Night,
    Dawn,
}

impl Variant {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Night => "night",
            Self::Dawn => "dawn",
        }
    }

    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::Night => "Night",
            Self::Dawn => "Dawn",
        }
    }

    #[must_use]
    pub const fn palette_filename(self) -> &'static str {
        match self {
            Self::Night => "akari-night.toml",
            Self::Dawn => "akari-dawn.toml",
        }
    }
}

pub const VARIANTS: [Variant; 2] = [Variant::Night, Variant::Dawn];

#[cfg(feature = "generator")]
/// Content of an artifact
#[derive(Debug, Clone)]
pub enum ArtifactContent {
    /// Text content to be written
    Text(String),
    /// Source path to be copied
    Copy(PathBuf),
}

#[cfg(feature = "generator")]
/// A generated or copied file
#[derive(Debug, Clone)]
pub struct Artifact {
    /// Relative path from output root (e.g., "helix/akari-night.toml")
    pub rel_path: PathBuf,
    /// Content or source path
    pub content: ArtifactContent,
}

#[cfg(feature = "generator")]
impl Artifact {
    #[must_use]
    pub fn text(rel_path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        Self {
            rel_path: rel_path.into(),
            content: ArtifactContent::Text(content.into()),
        }
    }

    #[must_use]
    pub fn copy(rel_path: impl Into<PathBuf>, src: impl Into<PathBuf>) -> Self {
        Self {
            rel_path: rel_path.into(),
            content: ArtifactContent::Copy(src.into()),
        }
    }
}

#[cfg(feature = "generator")]
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
