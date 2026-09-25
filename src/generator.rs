use crate::theme::{ResolvedVariant, Theme, ThemeMetadata, VariantMetadata};
use crate::{Artifact, Error, Rgb};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera, Value};
use walkdir::WalkDir;

/// Tools generated from a `Theme` directory.
/// One tool per line: migrations of separate tools add entries in parallel.
#[rustfmt::skip]
const THEME_TOOLS: &[&str] = &[
    "alacritty",
    "bat",
    "chrome",
    "codex",
    "delta",
    "fzf",
    "gh-dash",
    "ghostty",
    "helix",
    "lazygit",
    "nix",
    "nvim",
    "slack",
    "starship",
    "terminal",
    "tmux",
    "vscode",
    "zed",
    "zellij",
    "zsh",
];

/// Adapter keys a `THEME_TOOLS` entry requires in `theme.adapters.<tool>`.
/// One tool per line. Tools left out need nothing.
#[rustfmt::skip]
const ADAPTER_KEYS: &[(&str, &[&str])] = &[
    ("chrome", &["version"]),
    ("vscode", &["publisher", "version"]),
];

/// A file a `THEME_TOOLS` entry ships from the theme directory itself,
/// rather than from `templates/<tool>/`.
enum ThemeAsset {
    /// `adapters.<tool>.<key>` names a file relative to the theme directory; optional key.
    AdapterPath(&'static str),
    /// A file at the theme directory root, shipped only when present.
    IfPresent(&'static str),
}

/// `THEME_ASSETS` entries per tool. One tool per line.
#[rustfmt::skip]
const THEME_ASSETS: &[(&str, &[ThemeAsset])] = &[
    ("vscode", &[ThemeAsset::AdapterPath("icon"), ThemeAsset::IfPresent("LICENSE")]),
];

/// Adapter keys whose value names a text file in the theme directory that is
/// read into the template context (`adapter_text.<key>`, "" when the key is
/// absent) instead of being shipped as an artifact. One tool per line.
#[rustfmt::skip]
const ADAPTER_TEXTS: &[(&str, &[&str])] = &[
    ("vscode", &["readme"]),
];

/// Checks that `value` is a TOML string naming a relative path with no
/// empty, `.` or `..` components, returning it as a `Path` when it is.
fn relative_path_in(value: &toml::Value) -> Option<&Path> {
    let s = value.as_str()?;
    let path = Path::new(s);
    let is_relative_inside = !s.is_empty()
        && path.is_relative()
        && path
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)));
    is_relative_inside.then_some(path)
}

/// Resolves `adapters.<tool>.<key> = value` to an existing file inside
/// `theme_dir`, returning the path relative to it and the joined path.
fn adapter_file<'v>(
    tool: &str,
    key: &str,
    value: &'v toml::Value,
    theme_dir: &Path,
) -> Result<(&'v Path, PathBuf), Error> {
    let rel = relative_path_in(value).ok_or_else(|| Error::AdapterAssetPath {
        tool: tool.to_string(),
        key: key.to_string(),
        value: value
            .as_str()
            .map(str::to_string)
            .unwrap_or_else(|| value.to_string()),
    })?;
    let path = theme_dir.join(rel);
    if !path.is_file() {
        return Err(Error::AdapterAssetMissing {
            tool: tool.to_string(),
            key: key.to_string(),
            path,
        });
    }
    Ok((rel, path))
}

/// Builds the `adapter_text` context map for `tool`: one entry per
/// `ADAPTER_TEXTS` key declared for it, "" when the theme does not set that
/// key. Empty when `tool` has no `ADAPTER_TEXTS` entry.
fn adapter_text_map(
    tool: &str,
    theme: &Theme,
    theme_dir: &Path,
) -> Result<HashMap<String, String>, Error> {
    let Some((_, keys)) = ADAPTER_TEXTS.iter().find(|(t, _)| *t == tool) else {
        return Ok(HashMap::new());
    };

    let adapter = theme.adapters.get(tool);
    let mut map = HashMap::new();
    for key in *keys {
        let value = adapter.and_then(|a| a.get(*key));
        let text = match value {
            Some(value) => {
                let (_, path) = adapter_file(tool, key, value, theme_dir)?;
                fs::read_to_string(&path).map_err(|source| Error::Read { path, source })?
            }
            None => String::new(),
        };
        map.insert((*key).to_string(), text);
    }
    Ok(map)
}

fn check_adapter_keys(
    tool: &str,
    required: &[&str],
    table: Option<&toml::Table>,
) -> Result<(), Error> {
    for key in required {
        let present = table.is_some_and(|t| t.contains_key(*key));
        if !present {
            return Err(Error::AdapterKeyMissing {
                tool: tool.to_string(),
                key: key.to_string(),
            });
        }
    }
    Ok(())
}

/// An adapter's `[adapters.<tool>]` table and its `ADAPTER_TEXTS` contents.
struct AdapterContext<'a> {
    table: &'a toml::Table,
    text: &'a HashMap<String, String>,
}

fn hex_to_rgb_filter(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let hex = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("hex_to_rgb requires a string"))?;
    let rgb: Rgb = hex
        .parse()
        .map_err(|e: crate::Error| tera::Error::msg(e.to_string()))?;
    Ok(Value::String(rgb.to_array_string()))
}

fn hex_to_rgb_space_filter(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let hex = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("hex_to_rgb_space requires a string"))?;
    let rgb: Rgb = hex
        .parse()
        .map_err(|e: crate::Error| tera::Error::msg(e.to_string()))?;
    Ok(Value::String(rgb.to_space_separated()))
}

pub struct Generator {
    tera: Tera,
    templates_dir: PathBuf,
}

impl Generator {
    pub fn new(templates_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let templates_dir = templates_dir.as_ref().to_path_buf();
        let pattern = templates_dir.join("**/*.tera");
        let pattern_str = pattern
            .to_str()
            .ok_or_else(|| Error::InvalidPath(pattern.clone()))?;
        let mut tera = Tera::new(pattern_str).map_err(|e| Error::Template {
            context: "init failed",
            source: e,
        })?;
        tera.register_filter("hex_to_rgb", hex_to_rgb_filter);
        tera.register_filter("hex_to_rgb_space", hex_to_rgb_space_filter);
        Ok(Self {
            tera,
            templates_dir,
        })
    }

    /// Generate artifacts for one of `THEME_TOOLS` from a resolved `Theme`.
    /// `theme_dir` is the directory `theme` was loaded from, and is where
    /// `THEME_ASSETS` entries (e.g. an adapter-declared icon) are read from.
    pub fn generate_theme_tool(
        &self,
        tool: &str,
        theme: &Theme,
        theme_dir: &Path,
    ) -> Result<Vec<Artifact>, Error> {
        if !THEME_TOOLS.contains(&tool) {
            return Err(Error::ToolNotThemed(tool.to_string()));
        }

        if let Some((_, required)) = ADAPTER_KEYS.iter().find(|(t, _)| *t == tool) {
            check_adapter_keys(tool, required, theme.adapters.get(tool))?;
        }

        let mut artifacts = Vec::new();
        let tool_dir = self.templates_dir.join(tool);
        let empty_adapter = toml::Table::new();
        let adapter_text = adapter_text_map(tool, theme, theme_dir)?;
        let adapter = AdapterContext {
            table: theme.adapters.get(tool).unwrap_or(&empty_adapter),
            text: &adapter_text,
        };

        for entry in WalkDir::new(&tool_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let rel_path = path
                .strip_prefix(&tool_dir)
                .map_err(|_| Error::InvalidPath(path.to_path_buf()))?;

            if path.extension() == Some(OsStr::new("tera")) {
                self.process_theme_template(tool, path, rel_path, &mut artifacts, theme, &adapter)?;
            } else {
                self.process_theme_static(tool, path, rel_path, &mut artifacts, &theme.metadata);
            }
        }

        if tool == "terminal" {
            self.generate_terminal_from_theme(&mut artifacts, theme)?;
        }

        process_theme_assets(tool, theme, theme_dir, &mut artifacts)?;

        Ok(artifacts)
    }

    /// Generate Terminal.app theme files from a resolved `Theme`.
    fn generate_terminal_from_theme(
        &self,
        artifacts: &mut Vec<Artifact>,
        theme: &Theme,
    ) -> Result<(), Error> {
        for variant in &theme.variants {
            let content = crate::terminal::generate(&theme.metadata, variant)?;
            let filename = format!("{}-{}.terminal", theme.metadata.name, variant.variant.name);
            artifacts.push(Artifact::text(
                PathBuf::from("terminal").join(filename),
                content,
            ));
        }
        Ok(())
    }

    /// Renders one `.tera` template under `templates/<tool>/`: once per
    /// variant of `theme` when its output name has a `{variant}`
    /// placeholder, otherwise once for the whole theme.
    fn process_theme_template(
        &self,
        tool: &str,
        path: &Path,
        rel_path: &Path,
        artifacts: &mut Vec<Artifact>,
        theme: &Theme,
        adapter: &AdapterContext<'_>,
    ) -> Result<(), Error> {
        let out_path = strip_tera_extension(rel_path);
        let out_str = out_path.to_string_lossy();

        let template_name = path
            .strip_prefix(&self.templates_dir)
            .map_err(|_| Error::InvalidPath(path.to_path_buf()))?
            .to_string_lossy()
            .replace('\\', "/"); // Windows compatibility

        let render = |context: Context| {
            self.tera
                .render(&template_name, &context)
                .map_err(|e| Error::Template {
                    context: "render failed",
                    source: e,
                })
        };

        if out_str.contains("{variant}") {
            for variant in &theme.variants {
                let content = render(theme_context(&theme.metadata, variant, adapter))?;
                let final_path =
                    theme_output_name(&out_str, &theme.metadata, Some(&variant.variant));
                artifacts.push(Artifact::rendered(
                    PathBuf::from(tool).join(final_path),
                    content,
                    path,
                ));
            }
        } else {
            let content = render(combined_context(theme, adapter))?;
            let final_path = theme_output_name(&out_str, &theme.metadata, None);
            artifacts.push(Artifact::rendered(
                PathBuf::from(tool).join(final_path),
                content,
                path,
            ));
        }

        Ok(())
    }

    /// Process a static (non-template) file on the theme route, substituting
    /// `{theme}` in its path the same way a template's output name is.
    fn process_theme_static(
        &self,
        tool: &str,
        path: &Path,
        rel_path: &Path,
        artifacts: &mut Vec<Artifact>,
        theme: &ThemeMetadata,
    ) {
        let rel_str = rel_path.to_string_lossy().replace('\\', "/"); // Windows compatibility
        let out_str = theme_output_name(&rel_str, theme, None);
        artifacts.push(Artifact::copy(
            PathBuf::from(tool).join(out_str),
            path.to_path_buf(),
        ));
    }

    /// Lists `THEME_TOOLS`.
    #[must_use]
    pub fn available_theme_tools(&self) -> Vec<String> {
        THEME_TOOLS.iter().map(|s| s.to_string()).collect()
    }
}

fn strip_tera_extension(path: &Path) -> PathBuf {
    path.to_string_lossy()
        .strip_suffix(".tera")
        .map(PathBuf::from)
        .unwrap_or_else(|| path.to_path_buf())
}

/// Substitutes `{theme}`, and `{variant}` for a per-variant template, in a
/// theme-route output name pattern.
fn theme_output_name(
    pattern: &str,
    theme: &ThemeMetadata,
    variant: Option<&VariantMetadata>,
) -> String {
    let pattern = pattern.replace("{theme}", theme.id.as_str());
    match variant {
        Some(variant) => pattern.replace("{variant}", variant.id.as_str()),
        None => pattern,
    }
}

/// Emits `THEME_ASSETS` entries for `tool`: files that live in `theme_dir`
/// rather than under `templates/`.
fn process_theme_assets(
    tool: &str,
    theme: &Theme,
    theme_dir: &Path,
    artifacts: &mut Vec<Artifact>,
) -> Result<(), Error> {
    let Some((_, assets)) = THEME_ASSETS.iter().find(|(t, _)| *t == tool) else {
        return Ok(());
    };

    for asset in *assets {
        match asset {
            ThemeAsset::AdapterPath(key) => {
                let Some(value) = theme.adapters.get(tool).and_then(|a| a.get(*key)) else {
                    continue;
                };
                let (rel, source) = adapter_file(tool, key, value, theme_dir)?;
                artifacts.push(Artifact::copy(PathBuf::from(tool).join(rel), source));
            }
            ThemeAsset::IfPresent(name) => {
                let source = theme_dir.join(name);
                if source.is_file() {
                    artifacts.push(Artifact::copy(PathBuf::from(tool).join(name), source));
                }
            }
        }
    }

    Ok(())
}

/// Per-variant theme-route context. Built from `ResolvedVariant` so adapters
/// cannot reach `colors`.
fn theme_context(
    theme: &ThemeMetadata,
    variant: &ResolvedVariant,
    adapter: &AdapterContext<'_>,
) -> Context {
    let mut context = Context::new();
    context.insert("theme", theme);
    context.insert("variant", &variant.variant);
    context.insert("base", &variant.base);
    context.insert("ansi", &variant.ansi);
    context.insert("roles", &variant.roles);
    context.insert("adapter", adapter.table);
    context.insert("adapter_text", adapter.text);
    context
}

/// Context for a template rendered once per theme. Each `variants` element
/// has the same `variant`, `base`, `ansi`, `roles` as the per-variant context.
fn combined_context(theme: &Theme, adapter: &AdapterContext<'_>) -> Context {
    let mut context = Context::new();
    context.insert("theme", &theme.metadata);
    context.insert("variants", &theme.variants);
    context.insert("adapter", adapter.table);
    context.insert("adapter_text", adapter.text);
    context
}

#[cfg(test)]
mod tests {
    use super::check_adapter_keys;

    #[test]
    fn check_adapter_keys_reports_missing_key_when_adapter_table_absent() {
        let err = check_adapter_keys("chrome", &["version"], None).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("chrome"), "{message}");
        assert!(message.contains("version"), "{message}");
    }

    #[test]
    fn check_adapter_keys_reports_missing_key_when_table_present_but_key_absent() {
        let table = toml::Table::new();
        let err = check_adapter_keys("chrome", &["version"], Some(&table)).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("chrome"), "{message}");
        assert!(message.contains("version"), "{message}");
    }
}
