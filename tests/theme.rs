//! Black-box tests for `katazome::theme::Theme::load` against the sample
//! themes under `themes/`.

use katazome::Error;
use katazome::theme::*;
use std::fs;
use std::path::PathBuf;

fn themes_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("themes")
}

#[test]
fn akari_loads_variants_in_theme_toml_order() {
    let theme = Theme::load(themes_dir().join("akari")).unwrap();

    assert_eq!(theme.metadata.id.as_str(), "akari");

    let variant_ids: Vec<&str> = theme
        .variants
        .iter()
        .map(|v| v.variant.id.as_str())
        .collect();
    assert_eq!(variant_ids, vec!["night", "dawn"]);

    let metadata_ids: Vec<&str> = theme.metadata.variants.iter().map(Id::as_str).collect();
    assert_eq!(metadata_ids, variant_ids);
}

#[test]
fn ninja_loads_single_dark_variant() {
    let theme = Theme::load(themes_dir().join("ninja")).unwrap();

    assert_eq!(theme.variants.len(), 1);
    assert_eq!(theme.variants[0].variant.id.as_str(), "shadow");
    assert_eq!(theme.variants[0].variant.appearance, Appearance::Dark);
}

#[test]
fn akari_keeps_adapter_tables_as_written() {
    let theme = Theme::load(themes_dir().join("akari")).unwrap();

    let vscode = &theme.adapters["vscode"];
    assert_eq!(
        vscode.get("publisher").and_then(toml::Value::as_str),
        Some("cappyzawa")
    );
    assert!(theme.adapters.contains_key("chrome"));
}

#[test]
fn id_accepts_lowercase_alphanumeric_starting_with_a_letter() {
    assert!("akari".parse::<Id>().is_ok());
    assert!("a1".parse::<Id>().is_ok());
}

#[test]
fn id_rejects_uppercase_leading_digit_hyphen_and_empty() {
    for invalid in ["Akari", "1a", "a-b", ""] {
        let err = invalid.parse::<Id>().unwrap_err();
        assert!(matches!(err, Error::InvalidId(_)), "{invalid:?}: {err}");
    }
}

/// Copies `themes/ninja` into a fresh temp dir, lets `mutate` edit
/// `theme.toml` and `shadow.toml` in place, writes them back, and returns
/// the error `Theme::load` produces for the mutated directory.
fn ninja_load_err(mutate: impl FnOnce(&mut toml::Table, &mut toml::Table)) -> Error {
    let dir = tempfile::tempdir().unwrap();
    for entry in fs::read_dir(themes_dir().join("ninja")).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), dir.path().join(entry.file_name())).unwrap();
    }

    let theme_path = dir.path().join("theme.toml");
    let shadow_path = dir.path().join("shadow.toml");

    let mut theme_toml: toml::Table = fs::read_to_string(&theme_path).unwrap().parse().unwrap();
    let mut shadow_toml: toml::Table = fs::read_to_string(&shadow_path).unwrap().parse().unwrap();

    mutate(&mut theme_toml, &mut shadow_toml);

    fs::write(&theme_path, toml::to_string(&theme_toml).unwrap()).unwrap();
    fs::write(&shadow_path, toml::to_string(&shadow_toml).unwrap()).unwrap();

    Theme::load(dir.path()).unwrap_err()
}

#[test]
fn missing_role_reports_the_role_key() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["roles"]["ui"]
            .as_table_mut()
            .unwrap()
            .remove("accent");
    });
    assert!(err.to_string().contains("accent"), "{err}");
}

#[test]
fn non_hex_colors_entry_reports_the_key() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["colors"]["vortex"] = toml::Value::String("colors.sakura".to_string());
    });
    assert!(err.to_string().contains("colors.vortex"), "{err}");
}

#[test]
fn non_hex_base_entry_reports_the_key() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["base"]["background"] = toml::Value::String("darken(#000000, 0.1)".to_string());
    });
    assert!(err.to_string().contains("background"), "{err}");
}

#[test]
fn undefined_reference_reports_the_reference() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["roles"]["ui"]["accent"] = toml::Value::String("colors.nothing".to_string());
    });
    assert!(err.to_string().contains("colors.nothing"), "{err}");
}

#[test]
fn role_referencing_another_role_is_forbidden() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["roles"]["ui"]["accent"] = toml::Value::String("roles.ui.cursor".to_string());
    });
    assert!(err.to_string().contains("roles"), "{err}");
}

#[test]
fn ansi_referencing_ansi_bright_is_forbidden() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["ansi"]["black"] = toml::Value::String("ansi.bright.red".to_string());
    });
    assert!(err.to_string().contains("ansi.bright.red"), "{err}");
}

#[test]
fn invalid_theme_id_format_reports_the_value() {
    let err = ninja_load_err(|theme, _shadow| {
        theme["theme"]["id"] = toml::Value::String("Ninja".to_string());
    });
    assert!(err.to_string().contains("Ninja"), "{err}");
}

#[test]
fn invalid_variant_id_format_reports_the_value() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["variant"]["id"] = toml::Value::String("Shadow".to_string());
    });
    assert!(err.to_string().contains("Shadow"), "{err}");
}

#[test]
fn invalid_appearance_reports_the_value() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["variant"]["appearance"] = toml::Value::String("dusk".to_string());
    });
    assert!(err.to_string().contains("dusk"), "{err}");
}

#[test]
fn listed_variant_file_missing_reports_the_filename() {
    let err = ninja_load_err(|theme, _shadow| {
        theme["theme"]["variants"] = toml::Value::Array(vec![
            toml::Value::String("shadow".to_string()),
            toml::Value::String("dusk".to_string()),
        ]);
    });
    assert!(err.to_string().contains("dusk.toml"), "{err}");
}

#[test]
fn series_length_mismatch_reports_series() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["roles"]["series"].as_array_mut().unwrap().pop();
    });
    assert!(err.to_string().contains("series"), "{err}");
}

#[test]
fn variant_id_mismatch_reports_both_ids() {
    let err = ninja_load_err(|_theme, shadow| {
        shadow["variant"]["id"] = toml::Value::String("dusk".to_string());
    });
    let message = err.to_string();
    assert!(message.contains("dusk"), "{message}");
    assert!(message.contains("shadow"), "{message}");
}

#[test]
fn missing_theme_toml_reports_the_filename() {
    let dir = tempfile::tempdir().unwrap();
    let err = Theme::load(dir.path()).unwrap_err();
    assert!(err.to_string().contains("theme.toml"), "{err}");
}
