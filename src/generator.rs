use crate::{Error, Palette};
use std::collections::HashMap;
use std::path::Path;
use tera::{Context, Tera, Value};

fn hex_to_rgb(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let hex = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("hex_to_rgb requires a string"))?;
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(tera::Error::msg("invalid hex color"));
    }
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| tera::Error::msg("invalid hex"))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| tera::Error::msg("invalid hex"))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| tera::Error::msg("invalid hex"))?;
    Ok(Value::String(format!("[{r}, {g}, {b}]")))
}

pub struct Generator {
    tera: Tera,
}

impl Generator {
    pub fn new(templates_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let pattern = templates_dir.as_ref().join("**/*.tera");
        let mut tera =
            Tera::new(pattern.to_str().unwrap()).map_err(Error::TemplateInit)?;
        tera.register_filter("hex_to_rgb", hex_to_rgb);
        Ok(Self { tera })
    }

    pub fn render(&self, template: &str, palette: &Palette) -> Result<String, Error> {
        let mut context = Context::new();

        context.insert("name", &palette.name);
        context.insert("description", &palette.description);
        context.insert("variant", palette.variant());

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

    pub fn render_combined(
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
