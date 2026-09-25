//! Theme directories as defined by `docs/theme-model.md`: `theme.toml` plus one
//! self-contained file per variant, resolved into role colors for adapters.

pub use crate::ansi::{Ansi, AnsiColors};
use crate::{Error, Rgb};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

/// A theme or variant id: `[a-z][a-z0-9]*`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct Id(String);

impl Id {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for Id {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut chars = s.chars();
        let valid = chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
        if valid {
            Ok(Self(s))
        } else {
            Err(Error::InvalidId(s))
        }
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Appearance {
    Dark,
    Light,
}

/// `[theme]` in `theme.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub id: Id,
    pub name: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    pub variants: Vec<Id>,
}

/// `[variant]` in a variant file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantMetadata {
    pub id: Id,
    pub name: String,
    pub appearance: Appearance,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base {
    pub background: Rgb,
    pub foreground: Rgb,
}

#[derive(Debug, Clone, Serialize)]
pub struct Ui {
    pub surface: Rgb,
    pub sunken: Rgb,
    pub raised: Rgb,
    pub inset: Rgb,
    pub border: Rgb,
    pub muted: Rgb,
    pub accent: Rgb,
    pub accent_secondary: Rgb,
    pub on_accent: Rgb,
    pub selection_bg: Rgb,
    pub selection_fg: Rgb,
    pub match_bg: Rgb,
    pub cursor: Rgb,
    pub cursor_text: Rgb,
    pub active_bg: Rgb,
    pub link: Rgb,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub error: Rgb,
    pub warning: Rgb,
    pub info: Rgb,
    pub hint: Rgb,
    pub success: Rgb,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diff {
    pub added: Rgb,
    pub added_bg: Rgb,
    pub removed: Rgb,
    pub removed_bg: Rgb,
    pub changed: Rgb,
    pub moved: Rgb,
    pub conflict: Rgb,
    pub ours: Rgb,
    pub theirs: Rgb,
}

#[derive(Debug, Clone, Serialize)]
pub struct Syntax {
    pub text: Rgb,
    pub comment: Rgb,
    pub keyword: Rgb,
    pub string: Rgb,
    pub character: Rgb,
    pub number: Rgb,
    pub constant: Rgb,
    pub r#type: Rgb,
    pub function: Rgb,
    pub variable: Rgb,
    pub member: Rgb,
    pub builtin: Rgb,
    pub namespace: Rgb,
    pub attribute: Rgb,
    pub label: Rgb,
    pub tag: Rgb,
    pub punctuation_special: Rgb,
    pub decorator: Rgb,
    pub r#macro: Rgb,
    pub escape: Rgb,
    pub regexp: Rgb,
    pub path: Rgb,
    pub directory: Rgb,
}

#[derive(Debug, Clone, Serialize)]
pub struct Markup {
    pub heading_1: Rgb,
    pub heading_2: Rgb,
    pub heading_3: Rgb,
    pub heading_4: Rgb,
    pub list: Rgb,
    pub raw: Rgb,
}

/// `[roles.*]`, resolved. Consumers only: nothing references a role.
#[derive(Debug, Clone, Serialize)]
pub struct Roles {
    pub ui: Ui,
    pub diagnostic: Diagnostic,
    pub diff: Diff,
    pub syntax: Syntax,
    pub markup: Markup,
    pub series: [Rgb; 8],
}

/// One variant with every expression resolved. Carries no `colors`: adapters never read pigments.
#[derive(Debug, Clone, Serialize)]
pub struct ResolvedVariant {
    pub variant: VariantMetadata,
    pub base: Base,
    pub ansi: Ansi,
    pub roles: Roles,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub metadata: ThemeMetadata,
    /// `[adapters.<tool>]` tables, kept as written; each adapter validates its own keys.
    pub adapters: BTreeMap<String, toml::Table>,
    /// In `theme.variants` order.
    pub variants: Vec<ResolvedVariant>,
}

impl Theme {
    /// Loads `<dir>/theme.toml` and every variant file it lists.
    pub fn load(dir: impl AsRef<Path>) -> Result<Self, Error> {
        let _ = dir;
        todo!()
    }
}
