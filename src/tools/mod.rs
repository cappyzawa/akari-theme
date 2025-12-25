mod chrome;
mod fzf;
mod ghostty;
mod helix;
mod nvim;
mod starship;
mod terminal;
mod tmux;
mod vscode;
mod zsh;

use crate::{Artifact, Error, Generator, Palette, Variant, VARIANTS};
use std::path::PathBuf;

/// A tool that can generate theme files
pub trait ThemeGenerator {
    fn name(&self) -> &'static str;

    /// Generate artifacts (content + relative paths) for this tool
    fn artifacts(
        &self,
        palettes: &Palettes,
        generator: &Generator,
    ) -> Result<Vec<Artifact>, Error>;
}

/// Palettes for both variants
pub struct Palettes {
    pub night: Palette,
    pub dawn: Palette,
}

impl Palettes {
    pub fn load(palette_dir: &std::path::Path) -> Result<Self, Error> {
        Ok(Self {
            night: Palette::from_path(palette_dir.join("akari-night.toml"))?,
            dawn: Palette::from_path(palette_dir.join("akari-dawn.toml"))?,
        })
    }

    pub fn get(&self, variant: Variant) -> &Palette {
        match variant {
            Variant::Night => &self.night,
            Variant::Dawn => &self.dawn,
        }
    }
}

/// Tools that generate one file per variant using templates
pub trait PerVariant {
    fn name(&self) -> &'static str;
    fn template(&self) -> &'static str;
    fn rel_path(&self, variant: Variant) -> PathBuf;
}

impl<T: PerVariant> ThemeGenerator for T {
    fn name(&self) -> &'static str {
        PerVariant::name(self)
    }

    fn artifacts(
        &self,
        palettes: &Palettes,
        generator: &Generator,
    ) -> Result<Vec<Artifact>, Error> {
        let mut artifacts = Vec::new();
        for variant in VARIANTS {
            let palette = palettes.get(variant);
            let content = generator.render(self.template(), palette)?;
            artifacts.push(Artifact::new(self.rel_path(variant), content));
        }
        Ok(artifacts)
    }
}

/// Define a template-based tool that generates one file per variant.
///
/// # Example
/// ```ignore
/// impl_template_tool!(Helix, "helix", "helix.tera", "helix/akari-{name}.toml");
/// ```
///
/// `{name}` is replaced with the variant name (e.g., "night", "dawn").
#[macro_export]
macro_rules! impl_template_tool {
    ($struct:ident, $name:literal, $template:literal, $pattern:literal) => {
        pub struct $struct;

        impl $crate::tools::PerVariant for $struct {
            fn name(&self) -> &'static str {
                $name
            }

            fn template(&self) -> &'static str {
                $template
            }

            fn rel_path(&self, variant: $crate::Variant) -> std::path::PathBuf {
                std::path::PathBuf::from($pattern.replace("{name}", variant.name()))
            }
        }
    };
}

pub use chrome::Chrome;
pub use fzf::Fzf;
pub use ghostty::Ghostty;
pub use helix::Helix;
pub use nvim::Nvim;
pub use starship::Starship;
pub use terminal::Terminal;
pub use tmux::Tmux;
pub use vscode::Vscode;
pub use zsh::Zsh;

/// All available theme generators
pub fn all() -> Vec<Box<dyn ThemeGenerator>> {
    vec![
        Box::new(Helix),
        Box::new(Ghostty),
        Box::new(Fzf),
        Box::new(Starship),
        Box::new(Tmux),
        Box::new(Vscode),
        Box::new(Chrome),
        Box::new(Terminal),
        Box::new(Zsh),
        Box::new(Nvim),
    ]
}

/// Get a specific tool by name
pub fn by_name(name: &str) -> Option<Box<dyn ThemeGenerator>> {
    match name {
        "helix" => Some(Box::new(Helix)),
        "ghostty" => Some(Box::new(Ghostty)),
        "fzf" => Some(Box::new(Fzf)),
        "starship" => Some(Box::new(Starship)),
        "tmux" => Some(Box::new(Tmux)),
        "vscode" => Some(Box::new(Vscode)),
        "chrome" => Some(Box::new(Chrome)),
        "terminal" => Some(Box::new(Terminal)),
        "zsh" => Some(Box::new(Zsh)),
        "nvim" => Some(Box::new(Nvim)),
        _ => None,
    }
}
