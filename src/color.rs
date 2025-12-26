use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn parse(hex: &str) -> Result<Self, Error> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        let invalid = || Error::InvalidHex(hex.to_string());

        if hex.len() != 6 {
            return Err(invalid());
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| invalid())?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| invalid())?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| invalid())?;

        Ok(Self { r, g, b })
    }

    pub const fn as_floats(self) -> (f64, f64, f64) {
        (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
        )
    }

    pub fn to_array_string(self) -> String {
        format!("[{}, {}, {}]", self.r, self.g, self.b)
    }

    /// Convert to hex string with leading `#`.
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Lighten the color by blending toward white.
    ///
    /// `factor` of 0.0 returns the original color, 1.0 returns white.
    pub fn lighten(self, factor: f64) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            r: self.blend_channel(self.r, 255, factor),
            g: self.blend_channel(self.g, 255, factor),
            b: self.blend_channel(self.b, 255, factor),
        }
    }

    /// Darken the color by blending toward black.
    ///
    /// `factor` of 0.0 returns the original color, 1.0 returns black.
    pub fn darken(self, factor: f64) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            r: self.blend_channel(self.r, 0, factor),
            g: self.blend_channel(self.g, 0, factor),
            b: self.blend_channel(self.b, 0, factor),
        }
    }

    /// Mix two colors together.
    ///
    /// `factor` of 0.0 returns self, 1.0 returns other.
    pub fn mix(self, other: Self, factor: f64) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            r: self.blend_channel(self.r, other.r, factor),
            g: self.blend_channel(self.g, other.g, factor),
            b: self.blend_channel(self.b, other.b, factor),
        }
    }

    fn blend_channel(self, from: u8, to: u8, factor: f64) -> u8 {
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
        let rgb = Rgb::parse("#E26A3B").unwrap();
        assert_eq!(rgb.r, 226);
        assert_eq!(rgb.g, 106);
        assert_eq!(rgb.b, 59);
    }

    #[test]
    fn parse_without_hash() {
        let rgb = Rgb::parse("E26A3B").unwrap();
        assert_eq!(rgb.r, 226);
        assert_eq!(rgb.g, 106);
        assert_eq!(rgb.b, 59);
    }

    #[test]
    fn parse_black() {
        let rgb = Rgb::parse("#000000").unwrap();
        assert_eq!(rgb, Rgb { r: 0, g: 0, b: 0 });
    }

    #[test]
    fn parse_white() {
        let rgb = Rgb::parse("#FFFFFF").unwrap();
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
        let rgb = Rgb::parse("#aabbcc").unwrap();
        assert_eq!(rgb.r, 170);
        assert_eq!(rgb.g, 187);
        assert_eq!(rgb.b, 204);
    }

    #[test]
    fn parse_invalid_length_short() {
        assert!(Rgb::parse("#FFF").is_err());
    }

    #[test]
    fn parse_invalid_length_long() {
        assert!(Rgb::parse("#FFFFFFFF").is_err());
    }

    #[test]
    fn parse_invalid_chars() {
        assert!(Rgb::parse("#GGGGGG").is_err());
    }

    #[test]
    fn parse_empty() {
        assert!(Rgb::parse("").is_err());
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
    fn to_hex_uppercase() {
        let rgb = Rgb {
            r: 226,
            g: 106,
            b: 59,
        };
        assert_eq!(rgb.to_hex(), "#E26A3B");
    }

    #[test]
    fn to_hex_with_leading_zeros() {
        let rgb = Rgb { r: 1, g: 2, b: 3 };
        assert_eq!(rgb.to_hex(), "#010203");
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
