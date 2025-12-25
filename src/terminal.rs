use crate::{Error, Palette};
use plist::Value;
use std::collections::BTreeMap;
use std::io::Cursor;

fn hex_to_rgb(hex: &str) -> Result<(f64, f64, f64), Error> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(Error::InvalidHex(hex.to_string()));
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| Error::InvalidHex(hex.to_string()))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| Error::InvalidHex(hex.to_string()))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| Error::InvalidHex(hex.to_string()))?;

    Ok((
        f64::from(r) / 255.0,
        f64::from(g) / 255.0,
        f64::from(b) / 255.0,
    ))
}

fn encode_nscolor(hex: &str) -> Result<Vec<u8>, Error> {
    let (r, g, b) = hex_to_rgb(hex)?;
    let nsrgb = format!("{r} {g} {b}");

    let mut objects: Vec<Value> = Vec::new();

    objects.push(Value::String("$null".to_string()));

    let mut color_obj: BTreeMap<String, Value> = BTreeMap::new();
    color_obj.insert("$class".to_string(), Value::Uid(plist::Uid::new(2)));
    color_obj.insert("NSColorSpace".to_string(), Value::Integer(1.into()));
    color_obj.insert("NSRGB".to_string(), Value::Data(nsrgb.into_bytes()));
    objects.push(Value::Dictionary(color_obj.into_iter().collect()));

    let mut class_def: BTreeMap<String, Value> = BTreeMap::new();
    class_def.insert(
        "$classes".to_string(),
        Value::Array(vec![
            Value::String("NSColor".to_string()),
            Value::String("NSObject".to_string()),
        ]),
    );
    class_def.insert(
        "$classname".to_string(),
        Value::String("NSColor".to_string()),
    );
    objects.push(Value::Dictionary(class_def.into_iter().collect()));

    let mut root: BTreeMap<String, Value> = BTreeMap::new();
    root.insert(
        "$archiver".to_string(),
        Value::String("NSKeyedArchiver".to_string()),
    );
    root.insert("$objects".to_string(), Value::Array(objects));

    let mut top: BTreeMap<String, Value> = BTreeMap::new();
    top.insert("root".to_string(), Value::Uid(plist::Uid::new(1)));
    root.insert(
        "$top".to_string(),
        Value::Dictionary(top.into_iter().collect()),
    );
    root.insert("$version".to_string(), Value::Integer(100000.into()));

    let plist_value = Value::Dictionary(root.into_iter().collect());

    let mut buf = Cursor::new(Vec::new());
    plist_value.to_writer_binary(&mut buf)?;

    Ok(buf.into_inner())
}

fn color_data(hex: &str) -> Result<Value, Error> {
    encode_nscolor(hex).map(Value::Data)
}

pub fn generate(palette: &Palette) -> Result<String, Error> {
    let variant = palette.variant();
    let name = format!(
        "Akari-{}",
        if variant == "night" { "Night" } else { "Dawn" }
    );

    let mut dict: BTreeMap<String, Value> = BTreeMap::new();

    let ansi_colors = [
        ("ANSIBlackColor", "black"),
        ("ANSIRedColor", "red"),
        ("ANSIGreenColor", "green"),
        ("ANSIYellowColor", "yellow"),
        ("ANSIBlueColor", "blue"),
        ("ANSIMagentaColor", "magenta"),
        ("ANSICyanColor", "cyan"),
        ("ANSIWhiteColor", "white"),
    ];

    for (key, color) in ansi_colors {
        dict.insert(key.to_string(), color_data(&palette.ansi[color])?);
    }

    let bright_colors = [
        ("ANSIBrightBlackColor", "black"),
        ("ANSIBrightRedColor", "red"),
        ("ANSIBrightGreenColor", "green"),
        ("ANSIBrightYellowColor", "yellow"),
        ("ANSIBrightBlueColor", "blue"),
        ("ANSIBrightMagentaColor", "magenta"),
        ("ANSIBrightCyanColor", "cyan"),
        ("ANSIBrightWhiteColor", "white"),
    ];

    for (key, color) in bright_colors {
        dict.insert(key.to_string(), color_data(&palette.ansi_bright[color])?);
    }

    dict.insert(
        "BackgroundColor".to_string(),
        color_data(&palette.base["background"])?,
    );
    dict.insert(
        "TextColor".to_string(),
        color_data(&palette.base["foreground"])?,
    );
    dict.insert(
        "TextBoldColor".to_string(),
        color_data(&palette.base["foreground"])?,
    );

    dict.insert(
        "CursorColor".to_string(),
        color_data(&palette.state["cursor"])?,
    );
    dict.insert(
        "CursorTextColor".to_string(),
        color_data(&palette.state["cursor_text"])?,
    );

    dict.insert(
        "SelectionColor".to_string(),
        color_data(&palette.state["selection_bg"])?,
    );
    dict.insert(
        "SelectedTextColor".to_string(),
        color_data(&palette.state["selection_fg"])?,
    );

    dict.insert("name".to_string(), Value::String(name));
    dict.insert("ProfileCurrentVersion".to_string(), Value::Real(2.08));
    dict.insert(
        "type".to_string(),
        Value::String("Window Settings".to_string()),
    );

    let plist_value = Value::Dictionary(dict.into_iter().collect());
    let mut buf = Cursor::new(Vec::new());
    plist_value.to_writer_xml(&mut buf)?;

    String::from_utf8(buf.into_inner()).map_err(|e| Error::InvalidHex(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn hex_to_rgb_with_hash() {
        let (r, g, b) = hex_to_rgb("#E26A3B").unwrap();
        assert!(approx_eq(r, 226.0 / 255.0));
        assert!(approx_eq(g, 106.0 / 255.0));
        assert!(approx_eq(b, 59.0 / 255.0));
    }

    #[test]
    fn hex_to_rgb_without_hash() {
        let (r, g, b) = hex_to_rgb("E26A3B").unwrap();
        assert!(approx_eq(r, 226.0 / 255.0));
        assert!(approx_eq(g, 106.0 / 255.0));
        assert!(approx_eq(b, 59.0 / 255.0));
    }

    #[test]
    fn hex_to_rgb_black() {
        let (r, g, b) = hex_to_rgb("#000000").unwrap();
        assert!(approx_eq(r, 0.0));
        assert!(approx_eq(g, 0.0));
        assert!(approx_eq(b, 0.0));
    }

    #[test]
    fn hex_to_rgb_white() {
        let (r, g, b) = hex_to_rgb("#FFFFFF").unwrap();
        assert!(approx_eq(r, 1.0));
        assert!(approx_eq(g, 1.0));
        assert!(approx_eq(b, 1.0));
    }

    #[test]
    fn hex_to_rgb_lowercase() {
        let (r, g, b) = hex_to_rgb("#aabbcc").unwrap();
        assert!(approx_eq(r, 170.0 / 255.0));
        assert!(approx_eq(g, 187.0 / 255.0));
        assert!(approx_eq(b, 204.0 / 255.0));
    }

    #[test]
    fn hex_to_rgb_invalid_length_short() {
        assert!(hex_to_rgb("#FFF").is_err());
    }

    #[test]
    fn hex_to_rgb_invalid_length_long() {
        assert!(hex_to_rgb("#FFFFFFFF").is_err());
    }

    #[test]
    fn hex_to_rgb_invalid_chars() {
        assert!(hex_to_rgb("#GGGGGG").is_err());
    }

    #[test]
    fn hex_to_rgb_empty() {
        assert!(hex_to_rgb("").is_err());
    }
}
