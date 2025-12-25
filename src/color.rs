use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn parse(hex: &str) -> Result<Self, Error> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(Error::InvalidHex(hex.to_string()));
        }

        let r =
            u8::from_str_radix(&hex[0..2], 16).map_err(|_| Error::InvalidHex(hex.to_string()))?;
        let g =
            u8::from_str_radix(&hex[2..4], 16).map_err(|_| Error::InvalidHex(hex.to_string()))?;
        let b =
            u8::from_str_radix(&hex[4..6], 16).map_err(|_| Error::InvalidHex(hex.to_string()))?;

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
}
