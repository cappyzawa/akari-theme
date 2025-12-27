use crate::{Error, Palette, Rgb};
use plist::Value;
use std::collections::BTreeMap;
use std::io::Cursor;

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map(|first| first.to_ascii_uppercase().to_string() + c.as_str())
        .unwrap_or_default()
}

fn encode_nscolor(hex: &str) -> Result<Vec<u8>, Error> {
    let rgb = Rgb::parse(hex)?;
    let (r, g, b) = rgb.as_floats();
    let nsrgb = format!("{r} {g} {b}");

    let color_obj = BTreeMap::from([
        ("$class".to_string(), Value::Uid(plist::Uid::new(2))),
        ("NSColorSpace".to_string(), Value::Integer(1.into())),
        ("NSRGB".to_string(), Value::Data(nsrgb.into_bytes())),
    ]);

    let class_def = BTreeMap::from([
        (
            "$classes".to_string(),
            Value::Array(vec![
                Value::String("NSColor".to_string()),
                Value::String("NSObject".to_string()),
            ]),
        ),
        (
            "$classname".to_string(),
            Value::String("NSColor".to_string()),
        ),
    ]);

    let objects = vec![
        Value::String("$null".to_string()),
        Value::Dictionary(color_obj.into_iter().collect()),
        Value::Dictionary(class_def.into_iter().collect()),
    ];

    let top = BTreeMap::from([("root".to_string(), Value::Uid(plist::Uid::new(1)))]);

    let root = BTreeMap::from([
        (
            "$archiver".to_string(),
            Value::String("NSKeyedArchiver".to_string()),
        ),
        ("$objects".to_string(), Value::Array(objects)),
        (
            "$top".to_string(),
            Value::Dictionary(top.into_iter().collect()),
        ),
        ("$version".to_string(), Value::Integer(100000.into())),
    ]);

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
