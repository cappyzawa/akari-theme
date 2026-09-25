use crate::theme::{ResolvedVariant, Theme, ThemeMetadata, VariantMetadata};
use crate::{Artifact, Error, Palette, Rgb, VARIANTS};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tera::{Context, Tera, Value};
use walkdir::WalkDir;

/// Tools generated from a `Theme` directory instead of the legacy palette pair.
/// One tool per line: migrations of separate tools add entries in parallel.
#[rustfmt::skip]
const THEME_TOOLS: &[&str] = &[
    "alacritty",
    "bat",
    "chrome",
    "codex",
    "delta",
    "gh-dash",
    "ghostty",
    "helix",
    "lazygit",
    "nix",
    "slack",
    "starship",
    "terminal",
    "zed",
    "zellij",
];

/// Adapter keys a `THEME_TOOLS` entry requires in `theme.adapters.<tool>`.
/// One tool per line. Tools left out need nothing.
#[rustfmt::skip]
const ADAPTER_KEYS: &[(&str, &[&str])] = &[
    ("chrome", &["version"]),
];

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

    /// Generate artifacts for a specific tool on the legacy (Palette) route.
    pub fn generate_tool(
        &self,
        tool: &str,
        night: &Palette,
        dawn: &Palette,
    ) -> Result<Vec<Artifact>, Error> {
        if THEME_TOOLS.contains(&tool) {
            return Err(Error::ToolMigrated(tool.to_string()));
        }

        let mut artifacts = Vec::new();
        self.process_tool_directory(tool, &mut artifacts, night, dawn)?;
        Ok(artifacts)
    }

    /// Generate artifacts for one of `THEME_TOOLS` from a resolved `Theme`.
    pub fn generate_theme_tool(&self, tool: &str, theme: &Theme) -> Result<Vec<Artifact>, Error> {
        if !THEME_TOOLS.contains(&tool) {
            return Err(Error::ToolNotThemed(tool.to_string()));
        }

        if let Some((_, required)) = ADAPTER_KEYS.iter().find(|(t, _)| *t == tool) {
            check_adapter_keys(tool, required, theme.adapters.get(tool))?;
        }

        let mut artifacts = Vec::new();
        let tool_dir = self.templates_dir.join(tool);
        let empty_adapter = toml::Table::new();
        let adapter = theme.adapters.get(tool).unwrap_or(&empty_adapter);

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
                self.process_theme_template(tool, path, rel_path, &mut artifacts, theme, adapter)?;
            } else {
                self.process_static(tool, path, rel_path, &mut artifacts);
            }
        }

        if tool == "terminal" {
            self.generate_terminal_from_theme(&mut artifacts, theme)?;
        }

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
        adapter: &toml::Table,
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
                artifacts.push(Artifact::text(
                    PathBuf::from(tool).join(final_path),
                    content,
                ));
            }
        } else {
            let content = render(combined_context(theme, adapter))?;
            let final_path = theme_output_name(&out_str, &theme.metadata, None);
            artifacts.push(Artifact::text(
                PathBuf::from(tool).join(final_path),
                content,
            ));
        }

        Ok(())
    }

    /// Walk tool directory and process files
    fn process_tool_directory(
        &self,
        tool: &str,
        artifacts: &mut Vec<Artifact>,
        night: &Palette,
        dawn: &Palette,
    ) -> Result<(), Error> {
        let tool_dir = self.templates_dir.join(tool);

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
                self.process_template(tool, path, rel_path, artifacts, night, dawn)?;
            } else {
                self.process_static(tool, path, rel_path, artifacts);
            }
        }

        Ok(())
    }

    /// Process a .tera template file
    fn process_template(
        &self,
        tool: &str,
        path: &Path,
        rel_path: &Path,
        artifacts: &mut Vec<Artifact>,
        night: &Palette,
        dawn: &Palette,
    ) -> Result<(), Error> {
        let out_path = strip_tera_extension(rel_path);
        let out_str = out_path.to_string_lossy();

        let template_name = path
            .strip_prefix(&self.templates_dir)
            .map_err(|_| Error::InvalidPath(path.to_path_buf()))?
            .to_string_lossy()
            .replace('\\', "/"); // Windows compatibility

        if out_str.contains("{name}") || out_str.contains("{Name}") {
            self.render_per_variant(tool, &template_name, &out_str, artifacts, night, dawn)?;
        } else {
            let content = self.render_combined(&template_name, night, dawn)?;
            artifacts.push(Artifact::text(PathBuf::from(tool).join(out_path), content));
        }

        Ok(())
    }

    /// Render template for each variant (night/dawn)
    fn render_per_variant(
        &self,
        tool: &str,
        template_name: &str,
        out_pattern: &str,
        artifacts: &mut Vec<Artifact>,
        night: &Palette,
        dawn: &Palette,
    ) -> Result<(), Error> {
        for variant in VARIANTS {
            let palette = match variant {
                crate::Variant::Night => night,
                crate::Variant::Dawn => dawn,
            };
            let content = self.render(template_name, palette)?;
            let final_path = out_pattern
                .replace("{name}", variant.name())
                .replace("{Name}", variant.title());
            artifacts.push(Artifact::text(
                PathBuf::from(tool).join(&*final_path),
                content,
            ));
        }
        Ok(())
    }

    /// Process a static (non-template) file
    fn process_static(
        &self,
        tool: &str,
        path: &Path,
        rel_path: &Path,
        artifacts: &mut Vec<Artifact>,
    ) {
        artifacts.push(Artifact::copy(
            PathBuf::from(tool).join(rel_path),
            path.to_path_buf(),
        ));
    }

    /// Lists the tools the legacy route still owns (excludes `THEME_TOOLS`).
    pub fn available_tools(&self) -> std::io::Result<Vec<String>> {
        let mut tools = Vec::new();
        for entry in std::fs::read_dir(&self.templates_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir()
                && let Ok(name) = entry.file_name().into_string()
                && !THEME_TOOLS.contains(&name.as_str())
            {
                tools.push(name);
            }
        }
        Ok(tools)
    }

    /// Lists `THEME_TOOLS`.
    #[must_use]
    pub fn available_theme_tools(&self) -> Vec<String> {
        THEME_TOOLS.iter().map(|s| s.to_string()).collect()
    }

    fn render(&self, template: &str, palette: &Palette) -> Result<String, Error> {
        let mut context = Context::new();

        context.insert("name", &palette.name);
        context.insert("description", &palette.description);
        context.insert("variant", palette.variant.name());

        context.insert("colors", &palette.colors);
        context.insert("base", &palette.base);
        context.insert("layers", &palette.layers);
        context.insert("state", &palette.state);
        context.insert("semantic", &palette.semantic);
        context.insert("ansi", &palette.ansi);
        context.insert("ansi_bright", &palette.ansi_bright);

        self.tera
            .render(template, &context)
            .map_err(|e| Error::Template {
                context: "render failed",
                source: e,
            })
    }

    fn render_combined(
        &self,
        template: &str,
        night: &Palette,
        dawn: &Palette,
    ) -> Result<String, Error> {
        let mut context = Context::new();

        context.insert("night_colors", &night.colors);
        context.insert("night_base", &night.base);
        context.insert("night_layers", &night.layers);
        context.insert("night_state", &night.state);
        context.insert("night_semantic", &night.semantic);
        context.insert("night_ansi", &night.ansi);
        context.insert("night_ansi_bright", &night.ansi_bright);

        context.insert("dawn_colors", &dawn.colors);
        context.insert("dawn_base", &dawn.base);
        context.insert("dawn_layers", &dawn.layers);
        context.insert("dawn_state", &dawn.state);
        context.insert("dawn_semantic", &dawn.semantic);
        context.insert("dawn_ansi", &dawn.ansi);
        context.insert("dawn_ansi_bright", &dawn.ansi_bright);

        self.tera
            .render(template, &context)
            .map_err(|e| Error::Template {
                context: "render failed",
                source: e,
            })
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

/// Per-variant theme-route context. Built from `ResolvedVariant` so adapters
/// cannot reach `colors`.
fn theme_context(
    theme: &ThemeMetadata,
    variant: &ResolvedVariant,
    adapter: &toml::Table,
) -> Context {
    let mut context = Context::new();
    context.insert("theme", theme);
    context.insert("variant", &variant.variant);
    context.insert("base", &variant.base);
    context.insert("ansi", &variant.ansi);
    context.insert("roles", &variant.roles);
    context.insert("adapter", adapter);
    context
}

/// Context for a template rendered once per theme. Each `variants` element
/// has the same `variant`, `base`, `ansi`, `roles` as the per-variant context.
fn combined_context(theme: &Theme, adapter: &toml::Table) -> Context {
    let mut context = Context::new();
    context.insert("theme", &theme.metadata);
    context.insert("variants", &theme.variants);
    context.insert("adapter", adapter);
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
