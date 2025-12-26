use crate::{Artifact, Error, Palette, Rgb, VARIANTS};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tera::{Context, Tera, Value};
use walkdir::WalkDir;

fn hex_to_rgb_filter(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let hex = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("hex_to_rgb requires a string"))?;
    let rgb = Rgb::parse(hex).map_err(|e| tera::Error::msg(e.to_string()))?;
    Ok(Value::String(rgb.to_array_string()))
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
        let mut tera = Tera::new(pattern_str).map_err(Error::TemplateInit)?;
        tera.register_filter("hex_to_rgb", hex_to_rgb_filter);
        Ok(Self {
            tera,
            templates_dir,
        })
    }

    /// Generate artifacts for a specific tool
    pub fn generate_tool(
        &self,
        tool: &str,
        night: &Palette,
        dawn: &Palette,
    ) -> Result<Vec<Artifact>, Error> {
        let mut artifacts = Vec::new();

        // Terminal requires special handling (plist with NSColor binary encoding)
        if tool == "terminal" {
            self.generate_terminal(&mut artifacts, night, dawn)?;
        }

        self.process_tool_directory(tool, &mut artifacts, night, dawn)?;

        Ok(artifacts)
    }

    /// Generate Terminal.app theme files
    fn generate_terminal(
        &self,
        artifacts: &mut Vec<Artifact>,
        night: &Palette,
        dawn: &Palette,
    ) -> Result<(), Error> {
        for palette in [night, dawn] {
            let content = crate::terminal::generate(palette)?;
            let filename = format!("Akari-{}.terminal", palette.variant.title());
            artifacts.push(Artifact::text(
                PathBuf::from("terminal").join(filename),
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

    /// List available tools (directories in templates/)
    pub fn available_tools(&self) -> std::io::Result<Vec<String>> {
        Ok(std::fs::read_dir(&self.templates_dir)?
            .filter_map(|e| {
                let e = e.ok()?;
                e.file_type()
                    .ok()?
                    .is_dir()
                    .then(|| e.file_name().into_string().ok())?
            })
            .collect())
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
            .map_err(Error::TemplateRender)
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
            .map_err(Error::TemplateRender)
    }
}

fn strip_tera_extension(path: &Path) -> PathBuf {
    path.to_string_lossy()
        .strip_suffix(".tera")
        .map(PathBuf::from)
        .unwrap_or_else(|| path.to_path_buf())
}
