use super::{Palettes, ThemeGenerator};
use crate::{Artifact, Error, Generator};

pub struct Zsh;

impl ThemeGenerator for Zsh {
    fn name(&self) -> &'static str {
        "zsh"
    }

    fn artifacts(
        &self,
        palettes: &Palettes,
        generator: &Generator,
    ) -> Result<Vec<Artifact>, Error> {
        let content = generator.render_combined("zsh.tera", &palettes.night, &palettes.dawn)?;
        Ok(vec![Artifact::new("zsh/akari.zsh", content)])
    }
}
