use base64::Engine;
use plist::Value;
use std::collections::BTreeMap;
use std::io::Cursor;

use crate::palette::Palette;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid hex color: {0}")]
    InvalidHex(String),
    #[error("failed to write plist: {0}")]
    Plist(#[from] plist::Error),
}

/// Convert hex color (#RRGGBB) to RGB floats (0.0-1.0)
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

/// Create NSColor as NSKeyedArchiver base64-encoded data
fn encode_nscolor(hex: &str) -> Result<String, Error> {
    let (r, g, b) = hex_to_rgb(hex)?;
    let nsrgb = format!("{r} {g} {b}");

    // Build NSKeyedArchiver structure
    let mut objects: Vec<Value> = Vec::new();

    // $null
    objects.push(Value::String("$null".to_string()));

    // NSColor object (index 1)
    let mut color_obj: BTreeMap<String, Value> = BTreeMap::new();
    color_obj.insert(
        "$class".to_string(),
        Value::Uid(plist::Uid::new(2)), // Reference to class definition
    );
    color_obj.insert("NSColorSpace".to_string(), Value::Integer(1.into()));
    color_obj.insert("NSRGB".to_string(), Value::Data(nsrgb.into_bytes()));
    objects.push(Value::Dictionary(color_obj.into_iter().collect()));

    // Class definition (index 2)
    let mut class_def: BTreeMap<String, Value> = BTreeMap::new();
    class_def.insert(
        "$classes".to_string(),
        Value::Array(vec![
            Value::String("NSColor".to_string()),
            Value::String("NSObject".to_string()),
        ]),
    );
    class_def.insert("$classname".to_string(), Value::String("NSColor".to_string()));
    objects.push(Value::Dictionary(class_def.into_iter().collect()));

    // Build root dictionary
    let mut root: BTreeMap<String, Value> = BTreeMap::new();
    root.insert(
        "$archiver".to_string(),
        Value::String("NSKeyedArchiver".to_string()),
    );
    root.insert("$objects".to_string(), Value::Array(objects));

    let mut top: BTreeMap<String, Value> = BTreeMap::new();
    top.insert("root".to_string(), Value::Uid(plist::Uid::new(1)));
    root.insert("$top".to_string(), Value::Dictionary(top.into_iter().collect()));
    root.insert("$version".to_string(), Value::Integer(100000.into()));

    let plist_value = Value::Dictionary(root.into_iter().collect());

    // Serialize to binary plist
    let mut buf = Cursor::new(Vec::new());
    plist_value.to_writer_binary(&mut buf)?;

    // Base64 encode
    let encoded = base64::engine::general_purpose::STANDARD.encode(buf.into_inner());
    Ok(encoded)
}

/// Generate .terminal file content
pub fn generate(palette: &Palette) -> Result<String, Error> {
    let variant = palette.variant();
    let name = format!("Akari-{}", if variant == "night" { "Night" } else { "Dawn" });

    let mut dict: BTreeMap<String, Value> = BTreeMap::new();

    // ANSI colors
    dict.insert(
        "ANSIBlackColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi["black"])?).unwrap()),
    );
    dict.insert(
        "ANSIRedColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi["red"])?).unwrap()),
    );
    dict.insert(
        "ANSIGreenColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi["green"])?).unwrap()),
    );
    dict.insert(
        "ANSIYellowColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi["yellow"])?).unwrap()),
    );
    dict.insert(
        "ANSIBlueColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi["blue"])?).unwrap()),
    );
    dict.insert(
        "ANSIMagentaColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi["magenta"])?).unwrap()),
    );
    dict.insert(
        "ANSICyanColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi["cyan"])?).unwrap()),
    );
    dict.insert(
        "ANSIWhiteColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi["white"])?).unwrap()),
    );

    // Bright ANSI colors
    dict.insert(
        "ANSIBrightBlackColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi_bright["black"])?).unwrap()),
    );
    dict.insert(
        "ANSIBrightRedColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi_bright["red"])?).unwrap()),
    );
    dict.insert(
        "ANSIBrightGreenColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi_bright["green"])?).unwrap()),
    );
    dict.insert(
        "ANSIBrightYellowColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi_bright["yellow"])?).unwrap()),
    );
    dict.insert(
        "ANSIBrightBlueColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi_bright["blue"])?).unwrap()),
    );
    dict.insert(
        "ANSIBrightMagentaColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi_bright["magenta"])?).unwrap()),
    );
    dict.insert(
        "ANSIBrightCyanColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi_bright["cyan"])?).unwrap()),
    );
    dict.insert(
        "ANSIBrightWhiteColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.ansi_bright["white"])?).unwrap()),
    );

    // Background and foreground
    dict.insert(
        "BackgroundColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.base["background"])?).unwrap()),
    );
    dict.insert(
        "TextColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.base["foreground"])?).unwrap()),
    );
    dict.insert(
        "TextBoldColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.base["foreground"])?).unwrap()),
    );

    // Cursor
    dict.insert(
        "CursorColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.state["cursor"])?).unwrap()),
    );
    dict.insert(
        "CursorTextColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.state["cursor_text"])?).unwrap()),
    );

    // Selection
    dict.insert(
        "SelectionColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.state["selection_bg"])?).unwrap()),
    );
    dict.insert(
        "SelectedTextColor".to_string(),
        Value::Data(base64::engine::general_purpose::STANDARD.decode(encode_nscolor(&palette.state["selection_fg"])?).unwrap()),
    );

    // Profile name
    dict.insert("name".to_string(), Value::String(name.clone()));
    dict.insert("ProfileCurrentVersion".to_string(), Value::Real(2.08));
    dict.insert("type".to_string(), Value::String("Window Settings".to_string()));

    // Serialize to XML plist
    let plist_value = Value::Dictionary(dict.into_iter().collect());
    let mut buf = Cursor::new(Vec::new());
    plist_value.to_writer_xml(&mut buf)?;

    Ok(String::from_utf8(buf.into_inner()).unwrap())
}
