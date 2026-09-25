//! The theme model contract: `docs/theme-model.md` defines the role
//! vocabulary once, and every variant file under `themes/` must assign
//! exactly that vocabulary.

use akari_theme::Rgb;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_toml(path: &Path) -> Value {
    let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    toml::from_str::<Value>(&content).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

struct Theme {
    dir: PathBuf,
    manifest: Value,
}

impl Theme {
    fn all() -> Vec<Theme> {
        let mut themes: Vec<Theme> = fs::read_dir(repo_root().join("themes"))
            .expect("themes/ directory")
            .map(|e| e.expect("dir entry").path())
            .filter(|p| p.is_dir())
            .map(|dir| Theme {
                manifest: read_toml(&dir.join("theme.toml")),
                dir,
            })
            .collect();
        themes.sort_by(|a, b| a.dir.cmp(&b.dir));
        assert!(!themes.is_empty(), "no themes found");
        themes
    }

    fn id(&self) -> &str {
        self.manifest["theme"]["id"].as_str().expect("theme.id")
    }

    fn variant_ids(&self) -> Vec<&str> {
        self.manifest["theme"]["variants"]
            .as_array()
            .expect("theme.variants")
            .iter()
            .map(|v| v.as_str().expect("variant id"))
            .collect()
    }

    fn variants(&self) -> Vec<(PathBuf, Value)> {
        self.variant_ids()
            .into_iter()
            .map(|id| {
                let path = self.dir.join(format!("{id}.toml"));
                (path.clone(), read_toml(&path))
            })
            .collect()
    }
}

fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}

/// Role keys listed in the tables between "## Role vocabulary" and the
/// next top-level section of the document.
fn documented_roles() -> BTreeSet<String> {
    let doc = fs::read_to_string(repo_root().join("docs/theme-model.md")).expect("doc");
    let section = doc
        .split("\n## ")
        .find(|s| s.starts_with("Role vocabulary"))
        .expect("Role vocabulary section");
    section
        .lines()
        .filter_map(|line| line.strip_prefix("| `"))
        .map(|rest| {
            rest.split('`')
                .next()
                .expect("closing backtick")
                .to_string()
        })
        .collect()
}

fn flatten(prefix: &str, value: &Value, out: &mut BTreeSet<String>) {
    match value {
        Value::Table(table) => {
            for (k, v) in table {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                flatten(&key, v, out);
            }
        }
        _ => {
            out.insert(prefix.to_string());
        }
    }
}

/// Keys a variant file assigns, in the spelling the document uses:
/// `base.*` and `ansi.*` keep their prefix, `roles.*` drops it.
fn assigned_roles(variant: &Value) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    flatten("base", &variant["base"], &mut keys);
    flatten("ansi", &variant["ansi"], &mut keys);
    flatten("", &variant["roles"], &mut keys);
    keys
}

fn leaf_strings(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Table(table) => table.values().for_each(|v| leaf_strings(v, out)),
        Value::String(s) => out.push(s.clone()),
        other => panic!("expected string or table, got {other:?}"),
    }
}

#[test]
fn every_theme_lists_existing_variant_files() {
    for theme in Theme::all() {
        for (path, variant) in theme.variants() {
            assert!(path.is_file(), "{} listed but missing", path.display());
            let id = variant["variant"]["id"].as_str().expect("variant.id");
            assert_eq!(
                path.file_stem().and_then(|s| s.to_str()),
                Some(id),
                "variant.id must match the file name"
            );
        }
    }
}

#[test]
fn theme_and_variant_ids_are_lowercase_alphanumeric() {
    for theme in Theme::all() {
        assert!(is_identifier(theme.id()), "theme id {:?}", theme.id());
        for id in theme.variant_ids() {
            assert!(is_identifier(id), "variant id {id:?} in {}", theme.id());
        }
    }
}

#[test]
fn variant_appearance_is_dark_or_light() {
    for theme in Theme::all() {
        for (path, variant) in theme.variants() {
            let appearance = variant["variant"]["appearance"].as_str();
            assert!(
                matches!(appearance, Some("dark" | "light")),
                "{}: appearance {appearance:?}",
                path.display()
            );
        }
    }
}

#[test]
fn base_and_colors_are_hex_literals() {
    for theme in Theme::all() {
        for (path, variant) in theme.variants() {
            let mut literals = Vec::new();
            leaf_strings(&variant["base"], &mut literals);
            leaf_strings(&variant["colors"], &mut literals);
            for literal in literals {
                literal
                    .parse::<Rgb>()
                    .unwrap_or_else(|e| panic!("{}: {literal:?}: {e}", path.display()));
            }
        }
    }
}

#[test]
fn each_variant_assigns_exactly_the_documented_roles() {
    let documented = documented_roles();
    assert!(
        documented.contains("ui.accent"),
        "doc parsing found no roles"
    );
    for theme in Theme::all() {
        for (path, variant) in theme.variants() {
            let assigned = assigned_roles(&variant);
            let missing: Vec<_> = documented.difference(&assigned).collect();
            let extra: Vec<_> = assigned.difference(&documented).collect();
            assert!(
                missing.is_empty() && extra.is_empty(),
                "{}: missing {missing:?}, undocumented {extra:?}",
                path.display()
            );
        }
    }
}

#[test]
fn series_has_eight_entries() {
    for theme in Theme::all() {
        for (path, variant) in theme.variants() {
            let series = variant["roles"]["series"].as_array().expect("roles.series");
            assert_eq!(series.len(), 8, "{}", path.display());
        }
    }
}

/// The local `Theme` above is the test-file's own manifest reader; the
/// loader under test is `akari_theme::theme::Theme`, referenced fully
/// qualified to avoid colliding with it.
#[test]
fn resolved_variants_serialize_exactly_the_documented_roles() {
    let documented = documented_roles();
    for theme in Theme::all() {
        let loaded = akari_theme::theme::Theme::load(&theme.dir)
            .unwrap_or_else(|e| panic!("{}: {e}", theme.dir.display()));
        for variant in &loaded.variants {
            let value = Value::try_from(variant).unwrap();
            let assigned = assigned_roles(&value);
            let missing: Vec<_> = documented.difference(&assigned).collect();
            let extra: Vec<_> = assigned.difference(&documented).collect();
            assert!(
                missing.is_empty() && extra.is_empty(),
                "{}: missing {missing:?}, undocumented {extra:?}",
                theme.dir.display()
            );
            assert!(
                value.get("colors").is_none(),
                "{}: serialized variant still has a colors key",
                theme.dir.display()
            );
        }
    }
}
