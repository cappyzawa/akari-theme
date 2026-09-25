use crate::expr::{ColorExpr, ResolveRef, Resolver, resolve_fields};
use crate::{Error, Rgb};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The eight slots of one intensity.
#[derive(Debug, Clone, Serialize)]
pub struct AnsiColors {
    pub black: Rgb,
    pub red: Rgb,
    pub green: Rgb,
    pub yellow: Rgb,
    pub blue: Rgb,
    pub magenta: Rgb,
    pub cyan: Rgb,
    pub white: Rgb,
}

impl IntoIterator for &AnsiColors {
    type Item = (&'static str, Rgb);
    type IntoIter = std::array::IntoIter<Self::Item, 8>;

    fn into_iter(self) -> Self::IntoIter {
        [
            ("black", self.black),
            ("red", self.red),
            ("green", self.green),
            ("yellow", self.yellow),
            ("blue", self.blue),
            ("magenta", self.magenta),
            ("cyan", self.cyan),
            ("white", self.white),
        ]
        .into_iter()
    }
}

/// The 16 terminal slots: `ansi.black` … `ansi.bright.white`.
#[derive(Debug, Clone, Serialize)]
pub struct Ansi {
    #[serde(flatten)]
    pub normal: AnsiColors,
    pub bright: AnsiColors,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawAnsiColors {
    black: ColorExpr,
    red: ColorExpr,
    green: ColorExpr,
    yellow: ColorExpr,
    blue: ColorExpr,
    magenta: ColorExpr,
    cyan: ColorExpr,
    white: ColorExpr,
}

impl RawAnsiColors {
    fn resolve(&self, resolver: &impl ResolveRef) -> Result<AnsiColors, Error> {
        resolve_fields!(resolver, self => AnsiColors {
            black, red, green, yellow, blue, magenta, cyan, white,
        })
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawAnsi {
    #[serde(flatten)]
    normal: RawAnsiColors,
    bright: RawAnsiColors,
}

impl RawAnsi {
    /// Stages 1 and 2 of the resolution order: `normal` sees `colors` and `base`,
    /// `bright` additionally sees `normal`. Returns the slots and the flat lookup map
    /// (`black`, …, `bright.black`, …) that stage 3 resolvers take as `Resolver.ansi`.
    pub(crate) fn resolve(
        &self,
        colors: &BTreeMap<String, Rgb>,
        base: &BTreeMap<String, Rgb>,
    ) -> Result<(Ansi, BTreeMap<String, Rgb>), Error> {
        let resolver = Resolver {
            colors,
            base,
            ansi: None,
        };
        let normal = self.normal.resolve(&resolver)?;

        let mut ansi_map: BTreeMap<String, Rgb> = (&normal)
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let resolver = Resolver {
            colors,
            base,
            ansi: Some(&ansi_map),
        };
        let bright = self.bright.resolve(&resolver)?;

        for (k, v) in &bright {
            ansi_map.insert(format!("bright.{k}"), v);
        }

        Ok((Ansi { normal, bright }, ansi_map))
    }
}
