use crate::{Error, Palette, Rgb};
use plist::Value;
use std::collections::BTreeMap;
use std::io::Cursor;

/// Capitalize the first ASCII character of a string.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
    }
}

fn encode_nscolor(hex: &str) -> Result<Vec<u8>, Error> {
    let rgb = Rgb::parse(hex)?;
    let (r, g, b) = rgb.as_floats();
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
    let name = format!("Akari-{}", palette.variant.title());

    let mut dict: BTreeMap<String, Value> = BTreeMap::new();

    // ANSI colors
    for (name, hex) in &palette.ansi {
        let key = format!("ANSI{}Color", capitalize(name));
        dict.insert(key, color_data(hex)?);
    }

    // ANSI bright colors
    for (name, hex) in &palette.ansi_bright {
        let key = format!("ANSIBright{}Color", capitalize(name));
        dict.insert(key, color_data(hex)?);
    }

    // Base colors
    dict.insert(
        "BackgroundColor".to_string(),
        color_data(&palette.base.background)?,
    );
    dict.insert(
        "TextColor".to_string(),
        color_data(&palette.base.foreground)?,
    );
    dict.insert(
        "TextBoldColor".to_string(),
        color_data(&palette.base.foreground)?,
    );

    // Cursor
    dict.insert(
        "CursorColor".to_string(),
        color_data(&palette.state.cursor)?,
    );
    dict.insert(
        "CursorTextColor".to_string(),
        color_data(&palette.state.cursor_text)?,
    );

    // Selection
    dict.insert(
        "SelectionColor".to_string(),
        color_data(&palette.state.selection_bg)?,
    );
    dict.insert(
        "SelectedTextColor".to_string(),
        color_data(&palette.state.selection_fg)?,
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

    String::from_utf8(buf.into_inner()).map_err(|_| Error::PlistUtf8)
}
