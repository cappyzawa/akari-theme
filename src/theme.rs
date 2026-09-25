//! Theme directories as defined by `docs/theme-model.md`: `theme.toml` plus one
//! self-contained file per variant, resolved into role colors for adapters.

use crate::ansi::RawAnsi;
pub use crate::ansi::{Ansi, AnsiColors};
use crate::expr::{ColorExpr, ResolveRef, Resolver, resolve_expr, resolve_fields};
use crate::{Error, Rgb};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
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
    /// Tool names to generate by default, in the order given. `None` means
    /// every tool the generator knows; names are not validated here since
    /// the tool set belongs to the generator, not the theme model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
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

/// `theme.toml`.
#[derive(Debug, Deserialize)]
struct ThemeFile {
    theme: ThemeMetadata,
    #[serde(default)]
    adapters: BTreeMap<String, toml::Table>,
}

/// A variant file, before color expressions are resolved.
#[derive(Debug, Deserialize)]
struct RawVariant {
    variant: VariantMetadata,
    base: Base,
    colors: toml::Table,
    ansi: RawAnsi,
    roles: RawRoles,
}

#[derive(Debug, Deserialize)]
struct RawUi {
    surface: ColorExpr,
    sunken: ColorExpr,
    raised: ColorExpr,
    inset: ColorExpr,
    border: ColorExpr,
    muted: ColorExpr,
    accent: ColorExpr,
    accent_secondary: ColorExpr,
    on_accent: ColorExpr,
    selection_bg: ColorExpr,
    selection_fg: ColorExpr,
    match_bg: ColorExpr,
    cursor: ColorExpr,
    cursor_text: ColorExpr,
    active_bg: ColorExpr,
    link: ColorExpr,
}

impl RawUi {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<Ui, Error> {
        resolve_fields!(resolver, self => Ui {
            surface, sunken, raised, inset, border, muted, accent, accent_secondary,
            on_accent, selection_bg, selection_fg, match_bg, cursor, cursor_text,
            active_bg, link,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawDiagnostic {
    error: ColorExpr,
    warning: ColorExpr,
    info: ColorExpr,
    hint: ColorExpr,
    success: ColorExpr,
}

impl RawDiagnostic {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<Diagnostic, Error> {
        resolve_fields!(resolver, self => Diagnostic { error, warning, info, hint, success })
    }
}

#[derive(Debug, Deserialize)]
struct RawDiff {
    added: ColorExpr,
    added_bg: ColorExpr,
    removed: ColorExpr,
    removed_bg: ColorExpr,
    changed: ColorExpr,
    moved: ColorExpr,
    conflict: ColorExpr,
    ours: ColorExpr,
    theirs: ColorExpr,
}

impl RawDiff {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<Diff, Error> {
        resolve_fields!(resolver, self => Diff {
            added, added_bg, removed, removed_bg, changed, moved, conflict, ours, theirs,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawSyntax {
    text: ColorExpr,
    comment: ColorExpr,
    keyword: ColorExpr,
    string: ColorExpr,
    character: ColorExpr,
    number: ColorExpr,
    constant: ColorExpr,
    r#type: ColorExpr,
    function: ColorExpr,
    variable: ColorExpr,
    member: ColorExpr,
    builtin: ColorExpr,
    namespace: ColorExpr,
    attribute: ColorExpr,
    label: ColorExpr,
    tag: ColorExpr,
    punctuation_special: ColorExpr,
    decorator: ColorExpr,
    r#macro: ColorExpr,
    escape: ColorExpr,
    regexp: ColorExpr,
    path: ColorExpr,
    directory: ColorExpr,
}

impl RawSyntax {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<Syntax, Error> {
        resolve_fields!(resolver, self => Syntax {
            text, comment, keyword, string, character, number, constant, r#type,
            function, variable, member, builtin, namespace, attribute, label, tag,
            punctuation_special, decorator, r#macro, escape, regexp, path, directory,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawMarkup {
    heading_1: ColorExpr,
    heading_2: ColorExpr,
    heading_3: ColorExpr,
    heading_4: ColorExpr,
    list: ColorExpr,
    raw: ColorExpr,
}

impl RawMarkup {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<Markup, Error> {
        resolve_fields!(resolver, self => Markup {
            heading_1, heading_2, heading_3, heading_4, list, raw,
        })
    }
}

/// `[roles.*]`, before color expressions are resolved.
#[derive(Debug, Deserialize)]
struct RawRoles {
    ui: RawUi,
    diagnostic: RawDiagnostic,
    diff: RawDiff,
    syntax: RawSyntax,
    markup: RawMarkup,
    series: Vec<ColorExpr>,
}

impl RawRoles {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<Roles, Error> {
        let series: Vec<Rgb> = self
            .series
            .iter()
            .map(|expr| resolve_expr(resolver, expr))
            .collect::<Result<_, _>>()?;
        let series: [Rgb; 8] = series
            .try_into()
            .map_err(|series: Vec<Rgb>| Error::SeriesLength(series.len()))?;

        Ok(Roles {
            ui: self.ui.resolve(resolver)?,
            diagnostic: self.diagnostic.resolve(resolver)?,
            diff: self.diff.resolve(resolver)?,
            syntax: self.syntax.resolve(resolver)?,
            markup: self.markup.resolve(resolver)?,
            series,
        })
    }
}

/// Walks `[colors]` into the dotted keys (`lantern.mid`) that `Resolver` looks
/// up. Walked by hand instead of deserialized so the error can name the key.
fn flatten_colors(
    prefix: &str,
    table: &toml::Table,
    out: &mut BTreeMap<String, Rgb>,
) -> Result<(), Error> {
    for (key, value) in table {
        let dotted = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };
        match value {
            toml::Value::Table(nested) => flatten_colors(&dotted, nested, out)?,
            toml::Value::String(hex) => {
                let rgb = hex.parse().map_err(|_| Error::ExpectedHexLiteral {
                    key: format!("colors.{dotted}"),
                    value: value.clone(),
                })?;
                out.insert(dotted, rgb);
            }
            other => {
                return Err(Error::ExpectedHexLiteral {
                    key: format!("colors.{dotted}"),
                    value: other.clone(),
                });
            }
        }
    }
    Ok(())
}

impl RawVariant {
    fn resolve(&self) -> Result<ResolvedVariant, Error> {
        let mut colors = BTreeMap::new();
        flatten_colors("", &self.colors, &mut colors)?;

        let base: BTreeMap<String, Rgb> = [
            ("background".to_string(), self.base.background),
            ("foreground".to_string(), self.base.foreground),
        ]
        .into_iter()
        .collect();

        // Stages 1 and 2: resolve ansi and ansi.bright.
        let (ansi, ansi_map) = self.ansi.resolve(&colors, &base)?;

        // Stage 3: roles may reference colors, base and all of ansi.
        let resolver = Resolver {
            colors: &colors,
            base: &base,
            ansi: Some(&ansi_map),
        };

        Ok(ResolvedVariant {
            variant: self.variant.clone(),
            base: self.base.clone(),
            ansi,
            roles: self.roles.resolve(&resolver)?,
        })
    }
}

fn read_toml<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
    let content = fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_path_buf(),
        source,
    })?;
    toml::from_str(&content).map_err(|source| Error::Parse {
        path: path.to_path_buf(),
        source,
    })
}

impl Theme {
    /// Loads `<dir>/theme.toml` and every variant file it lists.
    pub fn load(dir: impl AsRef<Path>) -> Result<Self, Error> {
        let dir = dir.as_ref();
        let theme_file: ThemeFile = read_toml(&dir.join("theme.toml"))?;

        let mut variants = Vec::with_capacity(theme_file.theme.variants.len());
        for id in &theme_file.theme.variants {
            let path = dir.join(format!("{id}.toml"));
            let raw: RawVariant = read_toml(&path)?;
            if raw.variant.id != *id {
                return Err(Error::VariantIdMismatch {
                    path,
                    expected: id.clone(),
                    found: raw.variant.id,
                });
            }
            let resolved = raw.resolve().map_err(|source| Error::Resolve {
                path,
                source: Box::new(source),
            })?;
            variants.push(resolved);
        }

        Ok(Self {
            metadata: theme_file.theme,
            adapters: theme_file.adapters,
            variants,
        })
    }
}
