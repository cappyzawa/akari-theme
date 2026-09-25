//! Generates color theme files for terminals, editors and other tools from a theme directory.
//!
//! There are two supported uses: the `katazome generate --theme-dir <dir>
//! --tool <tool|all> --out-dir <dir>` CLI, and loading a theme with
//! [`theme::Theme::load`] and reading its resolved roles to write your own
//! adapter. [`Generator`] renders the built-in tools for the first use.
//!
//! # Public API
//!
//! - [`theme::Theme`], [`theme::Theme::load`]
//! - [`theme::ThemeMetadata`], [`theme::VariantMetadata`], [`theme::Id`], [`theme::Appearance`]
//! - [`theme::ResolvedVariant`], [`theme::Base`], [`theme::Ansi`], [`theme::AnsiColors`]
//! - [`theme::Roles`], [`theme::Ui`], [`theme::Diagnostic`], [`theme::Diff`], [`theme::Syntax`], [`theme::Markup`]
//! - [`Rgb`]
//! - [`Generator`], [`Generator::embedded`], [`Generator::from_dir`],
//!   [`Generator::available_tools`], [`Generator::default_tools`], [`Generator::generate`]
//!   (require the `generator` feature, default)
//! - [`Artifact`], [`ArtifactContent`] (require the `generator` feature, default)
//! - [`Error`]

mod ansi;
mod color;
mod expr;
#[cfg(feature = "generator")]
mod generator;
#[cfg(feature = "generator")]
mod terminal;
pub mod theme;

pub use color::Rgb;
#[cfg(feature = "generator")]
pub use generator::Generator;

use std::path::PathBuf;
use theme::Id;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
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
    #[error("unknown tool {0}")]
    UnknownTool(String),
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
    #[cfg(feature = "generator")]
    #[error("template {0} is not valid UTF-8")]
    TemplateNotUtf8(String),
}

#[cfg(feature = "generator")]
/// Content of an artifact
#[derive(Debug, Clone)]
pub enum ArtifactContent {
    /// Text content to be written
    Text(String),
    /// Raw bytes to be written
    Bytes(Vec<u8>),
}

#[cfg(feature = "generator")]
/// A generated file
#[derive(Debug, Clone)]
pub struct Artifact {
    /// Relative path from output root (e.g., "helix/akari-night.toml")
    pub rel_path: PathBuf,
    /// Content to be written
    pub content: ArtifactContent,
    /// Whether the artifact is written with the executable bit.
    pub executable: bool,
}

#[cfg(feature = "generator")]
impl Artifact {
    #[must_use]
    pub(crate) fn text(rel_path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        Self {
            rel_path: rel_path.into(),
            content: ArtifactContent::Text(content.into()),
            executable: false,
        }
    }

    #[must_use]
    pub(crate) fn rendered(
        rel_path: impl Into<PathBuf>,
        content: impl Into<String>,
        executable: bool,
    ) -> Self {
        Self {
            executable,
            ..Self::text(rel_path, content)
        }
    }

    #[must_use]
    pub(crate) fn bytes(
        rel_path: impl Into<PathBuf>,
        contents: impl Into<Vec<u8>>,
        executable: bool,
    ) -> Self {
        Self {
            rel_path: rel_path.into(),
            content: ArtifactContent::Bytes(contents.into()),
            executable,
        }
    }
}
