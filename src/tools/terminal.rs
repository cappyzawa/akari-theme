use super::{Palettes, ThemeGenerator};
use crate::{Artifact, Error, Generator, VARIANTS, terminal as terminal_gen};

pub struct Terminal;

impl ThemeGenerator for Terminal {
    fn name(&self) -> &'static str {
        "terminal"
    }

    fn artifacts(
        &self,
        palettes: &Palettes,
        _generator: &Generator,
    ) -> Result<Vec<Artifact>, Error> {
        let mut artifacts = Vec::new();

        for variant in VARIANTS {
            let palette = palettes.get(variant);
            let content = terminal_gen::generate(palette)?;
            let rel_path = format!("terminal/Akari-{}.terminal", variant.title());
            artifacts.push(Artifact::new(rel_path, content));
        }

        Ok(artifacts)
    }
}
