use crate::{Error, Rgb};
use std::collections::BTreeMap;

/// Resolve all fields of a raw struct by calling `resolve_expr` on each `ColorExpr` field.
macro_rules! resolve_fields {
    ($resolver:expr, $raw:expr => $Target:ident { $($field:ident),+ $(,)? }) => {
        Ok($Target {
            $( $field: $crate::expr::resolve_expr($resolver, &$raw.$field)? ),+
        })
    };
}
pub(crate) use resolve_fields;

/// Sections that can be referenced in color expressions.
///
/// Only `colors`, `base`, and `ansi` are valid reference targets.
/// `ansi.bright.*` is accessed via `Section::Ansi` with key `"bright.*"`.
/// Other sections like `layers`, `state`, and `semantic` (or `roles`) are
/// consumers of colors, not sources, and cannot be referenced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Section {
    Colors,
    Base,
    Ansi,
}

impl Section {
    /// Referenceable sections in color expressions.
    pub(crate) const ALLOWED: &[&str] = &["colors", "base", "ansi"];

    pub(crate) fn parse(s: &str) -> Result<Self, Error> {
        match s {
            "colors" => Ok(Self::Colors),
            "base" => Ok(Self::Base),
            "ansi" => Ok(Self::Ansi),
            _ => Err(Error::InvalidColorExpr(format!(
                "'{s}' cannot be referenced (allowed: {})",
                Self::ALLOWED.join(", ")
            ))),
        }
    }

    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::Colors => "colors",
            Self::Base => "base",
            Self::Ansi => "ansi",
        }
    }
}

/// A color expression that can be deserialized from TOML.
///
/// Supports:
/// - Literal hex colors: `"#E26A3B"`
/// - References: `"colors.lantern"`
/// - Functions:
///   - `"lighten(colors.lantern, 0.1)"` — increase lightness proportionally
///   - `"darken(base.background, 0.2)"` — decrease lightness proportionally
///   - `"brighten(ansi.red, 0.1)"` — adjust lightness by absolute amount
///   - `"mix(base.background, colors.night, 0.15)"` — blend two colors
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(try_from = "String")]
pub(crate) enum ColorExpr {
    /// A literal hex color (e.g., "#E26A3B")
    Literal(Rgb),
    /// A reference to another field (e.g., "colors.lantern")
    Ref { section: Section, key: String },
    /// Lighten a color by a factor (0.0 = unchanged, 1.0 = white)
    Lighten(Box<ColorExpr>, f64),
    /// Darken a color by a factor (0.0 = unchanged, 1.0 = black)
    Darken(Box<ColorExpr>, f64),
    /// Brighten a color by absolute amount (positive = brighter, negative = dimmer)
    Brighten(Box<ColorExpr>, f64),
    /// Mix two colors (0.0 = first color, 1.0 = second color)
    Mix(Box<ColorExpr>, Box<ColorExpr>, f64),
}

impl TryFrom<String> for ColorExpr {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        parse_color_expr(&s)
    }
}

/// Strip function call syntax: "fn_name(args)" -> Some("args")
fn strip_fn_call<'a>(s: &'a str, name: &str) -> Option<&'a str> {
    s.strip_prefix(name)
        .and_then(|r| r.strip_prefix('('))
        .and_then(|r| r.strip_suffix(')'))
}

/// Parse a color expression string into a ColorExpr.
pub(crate) fn parse_color_expr(s: &str) -> Result<ColorExpr, Error> {
    let s = s.trim();

    // Literal hex color
    if s.starts_with('#') {
        let rgb: Rgb = s.parse().map_err(|_| Error::InvalidHex(s.to_string()))?;
        return Ok(ColorExpr::Literal(rgb));
    }

    // Function call: lighten(...), darken(...), brighten(...), mix(...)
    if let Some(args) = strip_fn_call(s, "lighten") {
        let (inner, factor) = parse_unary_fn_args(args)?;
        return Ok(ColorExpr::Lighten(Box::new(inner), factor));
    }
    if let Some(args) = strip_fn_call(s, "darken") {
        let (inner, factor) = parse_unary_fn_args(args)?;
        return Ok(ColorExpr::Darken(Box::new(inner), factor));
    }
    if let Some(args) = strip_fn_call(s, "brighten") {
        let (inner, amount) = parse_unary_fn_args(args)?;
        return Ok(ColorExpr::Brighten(Box::new(inner), amount));
    }
    if let Some(args) = strip_fn_call(s, "mix") {
        let (color1, color2, factor) = parse_mix_args(args)?;
        return Ok(ColorExpr::Mix(Box::new(color1), Box::new(color2), factor));
    }

    // Reference: section.key (e.g., "colors.lantern.mid", "ansi.bright.red")
    let (section_str, key) = s
        .split_once('.')
        .ok_or_else(|| Error::InvalidColorExpr(s.to_string()))?;
    let section = Section::parse(section_str)?;
    Ok(ColorExpr::Ref {
        section,
        key: key.to_string(),
    })
}

fn parse_factor(s: &str) -> Result<f64, Error> {
    let s = s.trim();
    s.parse::<f64>()
        .map_err(|_| Error::InvalidColorExpr(format!("invalid factor: {s}")))
}

/// Split the last comma-separated token as an f64 factor, returning (rest, factor).
fn split_trailing_factor<'a>(args: &'a str, expected: &str) -> Result<(&'a str, f64), Error> {
    let (rest, factor_str) = args
        .rsplit_once(',')
        .ok_or_else(|| Error::InvalidColorExpr(format!("expected '{expected}': {args}")))?;
    Ok((rest, parse_factor(factor_str)?))
}

fn parse_unary_fn_args(args: &str) -> Result<(ColorExpr, f64), Error> {
    let (color_str, factor) = split_trailing_factor(args, "color, factor")?;
    let inner = parse_color_expr(color_str.trim())?;
    Ok((inner, factor))
}

fn parse_mix_args(args: &str) -> Result<(ColorExpr, ColorExpr, f64), Error> {
    let (rest, factor) = split_trailing_factor(args, "color1, color2, factor")?;
    let (color1_str, color2_str): (&str, &str) = rest.rsplit_once(',').ok_or_else(|| {
        Error::InvalidColorExpr(format!("expected 'color1, color2, factor': {args}"))
    })?;
    let color1 = parse_color_expr(color1_str.trim())?;
    let color2 = parse_color_expr(color2_str.trim())?;
    Ok((color1, color2, factor))
}

/// Trait for resolving color references.
pub(crate) trait ResolveRef {
    fn resolve_ref(&self, section: Section, key: &str) -> Result<Rgb, Error>;
}

/// Resolve a color expression using a resolver.
pub(crate) fn resolve_expr(resolver: &impl ResolveRef, expr: &ColorExpr) -> Result<Rgb, Error> {
    match expr {
        ColorExpr::Literal(rgb) => Ok(*rgb),
        ColorExpr::Ref { section, key } => resolver.resolve_ref(*section, key),
        ColorExpr::Lighten(inner, factor) => Ok(resolve_expr(resolver, inner)?.lighten(*factor)),
        ColorExpr::Darken(inner, factor) => Ok(resolve_expr(resolver, inner)?.darken(*factor)),
        ColorExpr::Brighten(inner, amount) => Ok(resolve_expr(resolver, inner)?.brighten(*amount)),
        ColorExpr::Mix(color1, color2, factor) => {
            let rgb1 = resolve_expr(resolver, color1)?;
            let rgb2 = resolve_expr(resolver, color2)?;
            Ok(rgb1.mix(rgb2, *factor))
        }
    }
}

/// Resolver for color references. Supports staged resolution:
/// ansi colors are optional during bootstrapping (ansi -> ansi.bright -> rest).
pub(crate) struct Resolver<'a> {
    pub(crate) colors: &'a BTreeMap<String, Rgb>,
    pub(crate) base: &'a BTreeMap<String, Rgb>,
    pub(crate) ansi: Option<&'a BTreeMap<String, Rgb>>,
}

impl ResolveRef for Resolver<'_> {
    fn resolve_ref(&self, section: Section, key: &str) -> Result<Rgb, Error> {
        let ref_str = || format!("{}.{key}", section.as_str());
        match section {
            Section::Colors => self
                .colors
                .get(key)
                .copied()
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
            Section::Base => self
                .base
                .get(key)
                .copied()
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
            Section::Ansi => self
                .ansi
                .and_then(|m| m.get(key).copied())
                .ok_or_else(|| Error::UnresolvedRef(ref_str())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Rgb {
        s.parse().unwrap()
    }

    #[test]
    fn parse_color_expr_literal() {
        let expr = parse_color_expr("#E26A3B").unwrap();
        assert!(matches!(expr, ColorExpr::Literal(c) if c == hex("#E26A3B")));
    }

    #[test]
    fn parse_color_expr_reference() {
        let expr = parse_color_expr("colors.lantern.mid").unwrap();
        assert!(
            matches!(expr, ColorExpr::Ref { section, key } if section == Section::Colors && key == "lantern.mid")
        );
    }

    #[test]
    fn parse_color_expr_lighten() {
        let expr = parse_color_expr("lighten(colors.lantern.mid, 0.1)").unwrap();
        match expr {
            ColorExpr::Lighten(inner, factor) => {
                assert!(
                    matches!(*inner, ColorExpr::Ref { section, key } if section == Section::Colors && key == "lantern.mid")
                );
                assert!((factor - 0.1).abs() < 0.001);
            }
            _ => panic!("expected Lighten"),
        }
    }

    #[test]
    fn parse_color_expr_darken() {
        let expr = parse_color_expr("darken(base.background, 0.2)").unwrap();
        match expr {
            ColorExpr::Darken(inner, factor) => {
                assert!(
                    matches!(*inner, ColorExpr::Ref { section, key } if section == Section::Base && key == "background")
                );
                assert!((factor - 0.2).abs() < 0.001);
            }
            _ => panic!("expected Darken"),
        }
    }

    #[test]
    fn parse_color_expr_nested() {
        let expr = parse_color_expr("lighten(darken(colors.lantern.mid, 0.1), 0.2)").unwrap();
        match expr {
            ColorExpr::Lighten(inner, outer_factor) => {
                assert!((outer_factor - 0.2).abs() < 0.001);
                match *inner {
                    ColorExpr::Darken(innermost, inner_factor) => {
                        assert!(
                            matches!(*innermost, ColorExpr::Ref { section, key } if section == Section::Colors && key == "lantern.mid")
                        );
                        assert!((inner_factor - 0.1).abs() < 0.001);
                    }
                    _ => panic!("expected Darken"),
                }
            }
            _ => panic!("expected Lighten"),
        }
    }

    #[test]
    fn parse_color_expr_mix() {
        let expr = parse_color_expr("mix(base.background, colors.night, 0.15)").unwrap();
        match expr {
            ColorExpr::Mix(color1, color2, factor) => {
                assert!(
                    matches!(*color1, ColorExpr::Ref { section, key } if section == Section::Base && key == "background")
                );
                assert!(
                    matches!(*color2, ColorExpr::Ref { section, key } if section == Section::Colors && key == "night")
                );
                assert!((factor - 0.15).abs() < 0.001);
            }
            _ => panic!("expected Mix"),
        }
    }

    #[test]
    fn parse_color_expr_rejects_non_referenceable_sections() {
        // layers, state, semantic, roles exist in the theme but cannot be referenced
        let err = parse_color_expr("layers.base").unwrap_err();
        assert!(
            matches!(err, Error::InvalidColorExpr(msg) if msg.contains("cannot be referenced"))
        );

        let err = parse_color_expr("state.cursor").unwrap_err();
        assert!(
            matches!(err, Error::InvalidColorExpr(msg) if msg.contains("cannot be referenced"))
        );

        let err = parse_color_expr("semantic.keyword").unwrap_err();
        assert!(
            matches!(err, Error::InvalidColorExpr(msg) if msg.contains("cannot be referenced"))
        );
    }

    #[test]
    fn parse_color_expr_ansi_bright() {
        // ansi.bright.* is parsed as Section::Ansi with key "bright.*"
        let expr = parse_color_expr("ansi.bright.red").unwrap();
        assert!(
            matches!(expr, ColorExpr::Ref { section, key } if section == Section::Ansi && key == "bright.red")
        );
    }
}
