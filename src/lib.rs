mod ansi;
mod color;
mod expr;
#[cfg(feature = "generator")]
mod generator;
#[cfg(feature = "generator")]
pub mod terminal;
pub mod theme;

pub use color::Rgb;
#[cfg(feature = "generator")]
pub use generator::Generator;

use std::path::PathBuf;
use theme::Id;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
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
    #[error("tool {0} has no theme-based generator")]
    ToolNotThemed(String),
    #[cfg(feature = "generator")]
    #[error("adapters.{tool}.{key} is required to generate {tool}")]
    AdapterKeyMissing { tool: String, key: String },
    #[cfg(feature = "generator")]
    #[error("adapters.{tool}.{key} = {value:?} must be a relative path inside the theme directory")]
    AdapterAssetPath {
        tool: String,
        key: String,
        value: String,
    },
    #[cfg(feature = "generator")]
    #[error("adapters.{tool}.{key}: {path} not found")]
    AdapterAssetMissing {
        tool: String,
        key: String,
        path: PathBuf,
    },
}

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
    /// Whether a `Text` artifact is written with the executable bit; a `Copy`
    /// artifact keeps its source's mode instead.
    pub executable: bool,
}

#[cfg(all(feature = "generator", unix))]
fn is_executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path).is_ok_and(|m| m.permissions().mode() & 0o111 != 0)
}

#[cfg(all(feature = "generator", not(unix)))]
fn is_executable(_path: &std::path::Path) -> bool {
    false
}

#[cfg(feature = "generator")]
impl Artifact {
    #[must_use]
    pub fn text(rel_path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        Self {
            rel_path: rel_path.into(),
            content: ArtifactContent::Text(content.into()),
            executable: false,
        }
    }

    /// A `Text` artifact rendered from `template`, executable when it is.
    #[must_use]
    pub fn rendered(
        rel_path: impl Into<PathBuf>,
        content: impl Into<String>,
        template: &std::path::Path,
    ) -> Self {
        Self {
            executable: is_executable(template),
            ..Self::text(rel_path, content)
        }
    }

    #[must_use]
    pub fn copy(rel_path: impl Into<PathBuf>, src: impl Into<PathBuf>) -> Self {
        Self {
            rel_path: rel_path.into(),
            content: ArtifactContent::Copy(src.into()),
            executable: false,
        }
    }
}
