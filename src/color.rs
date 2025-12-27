use crate::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Rgb {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').unwrap_or(s);

        if !hex.is_ascii() || hex.len() != 6 {
            return Err(Error::InvalidHex(hex.to_string()));
        }

        let parse = |range: std::ops::Range<usize>| u8::from_str_radix(&hex[range], 16);

        match (parse(0..2), parse(2..4), parse(4..6)) {
            (Ok(r), Ok(g), Ok(b)) => Ok(Self { r, g, b }),
            _ => Err(Error::InvalidHex(hex.to_string())),
        }
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl Rgb {
    #[must_use]
    pub const fn as_floats(self) -> (f64, f64, f64) {
        (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
        )
    }

    #[must_use]
    pub fn to_array_string(self) -> String {
        format!("[{}, {}, {}]", self.r, self.g, self.b)
    }

    /// Lighten the color by increasing lightness in HSL space.
    ///
    /// `factor` of 0.0 returns the original color, 1.0 returns white.
    /// The lightness is increased proportionally to the remaining headroom.
    #[must_use]
    pub fn lighten(self, factor: f64) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        let (h, s, l) = self.to_hsl();
        let new_l = l + (1.0 - l) * factor;
        Self::from_hsl(h, s, new_l)
    }

    /// Darken the color by decreasing lightness in HSL space.
    ///
    /// `factor` of 0.0 returns the original color, 1.0 returns black.
    /// The lightness is decreased proportionally to the current lightness.
    #[must_use]
    pub fn darken(self, factor: f64) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        let (h, s, l) = self.to_hsl();
        let new_l = l * (1.0 - factor);
        Self::from_hsl(h, s, new_l)
    }

    /// Adjust lightness by an absolute amount in HSL space.
    ///
    /// Positive values brighten, negative values dim.
    /// The amount is added directly to lightness (0.0 to 1.0 scale).
    #[must_use]
    pub fn brighten(self, amount: f64) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_l = (l + amount).clamp(0.0, 1.0);
        Self::from_hsl(h, s, new_l)
    }

    /// Mix two colors together.
    ///
    /// `factor` of 0.0 returns self, 1.0 returns other.
    #[must_use]
    pub fn mix(self, other: Self, factor: f64) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            r: Self::blend_channel(self.r, other.r, factor),
            g: Self::blend_channel(self.g, other.g, factor),
            b: Self::blend_channel(self.b, other.b, factor),
        }
    }

    /// Convert RGB to HSL.
    ///
    /// Returns (hue, saturation, lightness) where:
    /// - hue: 0.0 to 360.0
    /// - saturation: 0.0 to 1.0
    /// - lightness: 0.0 to 1.0
    fn to_hsl(self) -> (f64, f64, f64) {
        let (r, g, b) = self.as_floats();

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if (max - min).abs() < f64::EPSILON {
            return (0.0, 0.0, l);
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        let h = if (max - r).abs() < f64::EPSILON {
            let mut h = (g - b) / d;
            if g < b {
                h += 6.0;
            }
            h
        } else if (max - g).abs() < f64::EPSILON {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };

        (h * 60.0, s, l)
    }

    /// Convert HSL to RGB.
    fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        if s.abs() < f64::EPSILON {
            let v = (l * 255.0).round() as u8;
            return Self { r: v, g: v, b: v };
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        let h = h / 360.0;

        let r = Self::hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = Self::hue_to_rgb(p, q, h);
        let b = Self::hue_to_rgb(p, q, h - 1.0 / 3.0);

        Self {
            r: (r * 255.0).round() as u8,
            g: (g * 255.0).round() as u8,
            b: (b * 255.0).round() as u8,
        }
    }

    fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
        let t = t.rem_euclid(1.0);

        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 1.0 / 2.0 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    }

    fn blend_channel(from: u8, to: u8, factor: f64) -> u8 {
        let from = from as f64;
        let to = to as f64;
        (from + (to - from) * factor).round() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn parse_with_hash() {
        let rgb: Rgb = "#E26A3B".parse().unwrap();
        assert_eq!(rgb.r, 226);
        assert_eq!(rgb.g, 106);
        assert_eq!(rgb.b, 59);
    }

    #[test]
    fn parse_without_hash() {
        let rgb: Rgb = "E26A3B".parse().unwrap();
        assert_eq!(rgb.r, 226);
        assert_eq!(rgb.g, 106);
        assert_eq!(rgb.b, 59);
    }

    #[test]
    fn parse_black() {
        let rgb: Rgb = "#000000".parse().unwrap();
        assert_eq!(rgb, Rgb { r: 0, g: 0, b: 0 });
    }

    #[test]
    fn parse_white() {
        let rgb: Rgb = "#FFFFFF".parse().unwrap();
        assert_eq!(
            rgb,
            Rgb {
                r: 255,
                g: 255,
                b: 255
            }
        );
    }

    #[test]
    fn parse_lowercase() {
        let rgb: Rgb = "#aabbcc".parse().unwrap();
        assert_eq!(rgb.r, 170);
        assert_eq!(rgb.g, 187);
        assert_eq!(rgb.b, 204);
    }

    #[test]
    fn parse_invalid_length_short() {
        assert!("#FFF".parse::<Rgb>().is_err());
    }

    #[test]
    fn parse_invalid_length_long() {
        assert!("#FFFFFFFF".parse::<Rgb>().is_err());
    }

    #[test]
    fn parse_invalid_chars() {
        assert!("#GGGGGG".parse::<Rgb>().is_err());
    }

    #[test]
    fn parse_empty() {
        assert!("".parse::<Rgb>().is_err());
    }

    #[test]
    fn parse_non_ascii() {
        assert!("#ＡＢＣＤＥＦ".parse::<Rgb>().is_err());
    }

    #[test]
    fn as_floats_black() {
        let rgb = Rgb { r: 0, g: 0, b: 0 };
        let (r, g, b) = rgb.as_floats();
        assert!(approx_eq(r, 0.0));
        assert!(approx_eq(g, 0.0));
        assert!(approx_eq(b, 0.0));
    }

    #[test]
    fn as_floats_white() {
        let rgb = Rgb {
            r: 255,
            g: 255,
            b: 255,
        };
        let (r, g, b) = rgb.as_floats();
        assert!(approx_eq(r, 1.0));
        assert!(approx_eq(g, 1.0));
        assert!(approx_eq(b, 1.0));
    }

    #[test]
    fn to_array_string_format() {
        let rgb = Rgb {
            r: 226,
            g: 106,
            b: 59,
        };
        assert_eq!(rgb.to_array_string(), "[226, 106, 59]");
    }

    #[test]
    fn display_uppercase() {
        let rgb = Rgb {
            r: 226,
            g: 106,
            b: 59,
        };
        assert_eq!(rgb.to_string(), "#E26A3B");
    }

    #[test]
    fn display_with_leading_zeros() {
        let rgb = Rgb { r: 1, g: 2, b: 3 };
        assert_eq!(rgb.to_string(), "#010203");
    }

    #[test]
    fn lighten_zero_unchanged() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        assert_eq!(rgb.lighten(0.0), rgb);
    }

    #[test]
    fn lighten_full_becomes_white() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        assert_eq!(
            rgb.lighten(1.0),
            Rgb {
                r: 255,
                g: 255,
                b: 255
            }
        );
    }

    #[test]
    fn lighten_half() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        // 100 + (255 - 100) * 0.5 = 100 + 77.5 = 177.5 -> 178
        assert_eq!(
            rgb.lighten(0.5),
            Rgb {
                r: 178,
                g: 178,
                b: 178
            }
        );
    }

    #[test]
    fn darken_zero_unchanged() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        assert_eq!(rgb.darken(0.0), rgb);
    }

    #[test]
    fn darken_full_becomes_black() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        assert_eq!(rgb.darken(1.0), Rgb { r: 0, g: 0, b: 0 });
    }

    #[test]
    fn darken_half() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        // 100 + (0 - 100) * 0.5 = 100 - 50 = 50
        assert_eq!(
            rgb.darken(0.5),
            Rgb {
                r: 50,
                g: 50,
                b: 50
            }
        );
    }

    #[test]
    fn lighten_clamps_factor() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        // Factor > 1.0 should be clamped to 1.0
        assert_eq!(
            rgb.lighten(2.0),
            Rgb {
                r: 255,
                g: 255,
                b: 255
            }
        );
    }

    #[test]
    fn darken_clamps_negative_factor() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        // Factor < 0.0 should be clamped to 0.0
        assert_eq!(rgb.darken(-0.5), rgb);
    }

    #[test]
    fn brighten_zero_unchanged() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        assert_eq!(rgb.brighten(0.0), rgb);
    }

    #[test]
    fn brighten_positive_increases_lightness() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        let brightened = rgb.brighten(0.2);
        // Lightness increases, so RGB values should increase
        assert!(brightened.r > rgb.r);
        assert!(brightened.g > rgb.g);
        assert!(brightened.b > rgb.b);
    }

    #[test]
    fn brighten_negative_decreases_lightness() {
        let rgb = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        let dimmed = rgb.brighten(-0.2);
        // Lightness decreases, so RGB values should decrease
        assert!(dimmed.r < rgb.r);
        assert!(dimmed.g < rgb.g);
        assert!(dimmed.b < rgb.b);
    }

    #[test]
    fn brighten_clamps_to_white() {
        let rgb = Rgb {
            r: 200,
            g: 200,
            b: 200,
        };
        let result = rgb.brighten(1.0);
        assert_eq!(
            result,
            Rgb {
                r: 255,
                g: 255,
                b: 255
            }
        );
    }

    #[test]
    fn brighten_clamps_to_black() {
        let rgb = Rgb {
            r: 50,
            g: 50,
            b: 50,
        };
        let result = rgb.brighten(-1.0);
        assert_eq!(result, Rgb { r: 0, g: 0, b: 0 });
    }

    #[test]
    fn mix_zero_returns_self() {
        let a = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        let b = Rgb {
            r: 200,
            g: 200,
            b: 200,
        };
        assert_eq!(a.mix(b, 0.0), a);
    }

    #[test]
    fn mix_one_returns_other() {
        let a = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        let b = Rgb {
            r: 200,
            g: 200,
            b: 200,
        };
        assert_eq!(a.mix(b, 1.0), b);
    }

    #[test]
    fn mix_half() {
        let a = Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        let b = Rgb {
            r: 200,
            g: 200,
            b: 200,
        };
        // 100 + (200 - 100) * 0.5 = 150
        assert_eq!(
            a.mix(b, 0.5),
            Rgb {
                r: 150,
                g: 150,
                b: 150
            }
        );
    }
}
