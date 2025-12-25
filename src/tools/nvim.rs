use super::{Palettes, ThemeGenerator};
use crate::{Artifact, Error, Generator};

pub struct Nvim;

impl ThemeGenerator for Nvim {
    fn name(&self) -> &'static str {
        "nvim"
    }

    fn artifacts(
        &self,
        palettes: &Palettes,
        generator: &Generator,
    ) -> Result<Vec<Artifact>, Error> {
        let content =
            generator.render_combined("nvim-palette.tera", &palettes.night, &palettes.dawn)?;
        Ok(vec![Artifact::new("nvim/lua/akari/palette.lua", content)])
    }
}
