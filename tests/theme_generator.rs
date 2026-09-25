//! Black-box tests for the theme-based generation route
//! (`Generator::generate_theme_tool`), which consumes `Theme` instead of
//! the legacy `Palette` pair. Covers the migrated tools and the
//! legacy/theme route boundary.

use akari_theme::theme::Theme;
use akari_theme::{Artifact, ArtifactContent, Error, Generator};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn templates_dir() -> PathBuf {
    root_dir().join("templates")
}

fn generator() -> Generator {
    Generator::new(templates_dir()).unwrap()
}

fn artifact_text<'a>(artifacts: &'a [Artifact], rel: &str) -> &'a str {
    let artifact = artifacts
        .iter()
        .find(|a| a.rel_path == Path::new(rel))
        .unwrap_or_else(|| panic!("missing artifact {rel}"));
    match &artifact.content {
        ArtifactContent::Text(text) => text,
        ArtifactContent::Copy(_) => panic!("{rel} is a Copy artifact, expected Text"),
    }
}

const MIGRATED_TOOLS: [&str; 6] = ["helix", "terminal", "delta", "lazygit", "gh-dash", "nix"];

/// Tools whose every Akari artifact must equal the committed `dist/` file.
const DIST_EXACT_TOOLS: [&str; 4] = ["delta", "lazygit", "gh-dash", "nix"];

#[test]
fn legacy_available_tools_excludes_theme_tools() {
    let generator = generator();
    let tools = generator.available_tools().unwrap();

    assert!(!tools.is_empty());
    for tool in MIGRATED_TOOLS {
        assert!(
            !tools.iter().any(|t| t == tool),
            "{tool} still on legacy route"
        );
    }
}

#[test]
fn legacy_generate_tool_rejects_theme_tools() {
    let generator = generator();
    let night = akari_theme::Palette::night();
    let dawn = akari_theme::Palette::dawn();

    for tool in MIGRATED_TOOLS {
        let result = generator.generate_tool(tool, &night, &dawn);
        match result {
            Err(Error::ToolMigrated(t)) => assert_eq!(t, tool),
            other => panic!("expected ToolMigrated for {tool}, got {other:?}"),
        }
    }
}

#[test]
fn theme_context_rejects_unknown_tool() {
    let theme = Theme::load(root_dir().join("themes/akari")).unwrap();
    let generator = generator();

    let result = generator.generate_theme_tool("codex", &theme);
    match result {
        Err(Error::ToolNotThemed(t)) => assert_eq!(t, "codex"),
        other => panic!("expected ToolNotThemed, got {other:?}"),
    }
}

#[test]
fn theme_helix_copies_static_readme() {
    let theme = Theme::load(root_dir().join("themes/akari")).unwrap();
    let generator = generator();
    let artifacts = generator.generate_theme_tool("helix", &theme).unwrap();

    let artifact = artifacts
        .iter()
        .find(|a| a.rel_path == Path::new("helix/README.md"))
        .expect("README.md artifact missing");
    assert!(matches!(artifact.content, ArtifactContent::Copy(_)));
}

/// The style value's resolved (fg, bg, underline color, underline style,
/// modifiers) tuple, with color names looked up against `[palette]`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct StyleResolution {
    fg: Option<String>,
    bg: Option<String>,
    underline_color: Option<String>,
    underline_style: Option<String>,
    modifiers: Vec<String>,
}

fn resolve_color(name: &str, palette: &toml::Table) -> String {
    palette
        .get(name)
        .and_then(toml::Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| name.to_string())
}

fn resolve_style(value: &toml::Value, palette: &toml::Table) -> StyleResolution {
    match value {
        toml::Value::String(name) => StyleResolution {
            fg: Some(resolve_color(name, palette)),
            bg: None,
            underline_color: None,
            underline_style: None,
            modifiers: Vec::new(),
        },
        toml::Value::Table(table) => {
            let fg = table
                .get("fg")
                .and_then(toml::Value::as_str)
                .map(|n| resolve_color(n, palette));
            let bg = table
                .get("bg")
                .and_then(toml::Value::as_str)
                .map(|n| resolve_color(n, palette));
            let (underline_color, underline_style) = match table.get("underline") {
                Some(toml::Value::Table(u)) => (
                    u.get("color")
                        .and_then(toml::Value::as_str)
                        .map(|n| resolve_color(n, palette)),
                    u.get("style")
                        .and_then(toml::Value::as_str)
                        .map(str::to_string),
                ),
                _ => (None, None),
            };
            let mut modifiers: Vec<String> = table
                .get("modifiers")
                .and_then(toml::Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(toml::Value::as_str)
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default();
            modifiers.sort();
            StyleResolution {
                fg,
                bg,
                underline_color,
                underline_style,
                modifiers,
            }
        }
        other => panic!("unexpected style value: {other:?}"),
    }
}

fn style_keys(doc: &toml::Table) -> HashSet<String> {
    doc.iter()
        .filter(|(k, _)| k.as_str() != "palette")
        .map(|(k, _)| k.clone())
        .collect()
}

#[test]
fn theme_helix_akari_matches_dist_after_palette_resolution() {
    let theme = Theme::load(root_dir().join("themes/akari")).unwrap();
    let generator = generator();
    let artifacts = generator.generate_theme_tool("helix", &theme).unwrap();

    for variant in &theme.variants {
        let variant_id = variant.variant.id.as_str();
        let generated_path = format!("helix/akari-{variant_id}.toml");
        let generated_text = artifact_text(&artifacts, &generated_path);

        let dist_path = root_dir().join(format!("dist/helix/akari-{variant_id}.toml"));
        let dist_text = fs::read_to_string(&dist_path).unwrap();

        let dist_header: Vec<&str> = dist_text.lines().take(3).collect();
        let generated_header: Vec<&str> = generated_text.lines().take(3).collect();
        assert_eq!(
            generated_header, dist_header,
            "header mismatch for variant {variant_id}"
        );

        let dist_doc: toml::Table = dist_text.parse().unwrap();
        let generated_doc: toml::Table = generated_text.parse().unwrap();

        let dist_palette = dist_doc["palette"].as_table().unwrap();
        let generated_palette = generated_doc["palette"].as_table().unwrap();

        let dist_keys = style_keys(&dist_doc);
        let generated_keys = style_keys(&generated_doc);
        assert_eq!(
            generated_keys, dist_keys,
            "style key set mismatch for variant {variant_id}"
        );

        for key in &dist_keys {
            // These six keys intentionally diverge for the dawn variant only.
            // The old template read them from a raw pigment/proxy color that
            // happened to equal the dedicated semantic value for night but not
            // for dawn:
            //   - `hint`/`diagnostic.hint` read `comment`; `roles.diagnostic.hint`
            //     is `darken(base.foreground, 0.40)`, not `comment`'s
            //     `darken(colors.night, 0.30)`.
            //   - `diff.plus`/`diff.plus.gutter` read raw `green` (`colors.life`);
            //     `roles.diff.added` is `darken(colors.life, 0.15)`.
            //   - `diff.delta`/`diff.delta.gutter` read raw `amber`
            //     (`colors.lantern.far`); `roles.diff.changed` is
            //     `darken(colors.lantern.far, 0.10)`.
            // The new palette keys (`hint`, `diff-added`, `diff-changed`) are
            // documented to replace those raw references (see the design table
            // and docs/theme-model.md, "Akari settings to settle during
            // migration"), so these hex values are expected to change on dawn.
            // Night is unaffected because its roles happen to equal the old raw
            // pigment. Every other style key must still match byte-for-byte.
            if variant_id == "dawn"
                && matches!(
                    key.as_str(),
                    "hint"
                        | "diagnostic.hint"
                        | "diff.plus"
                        | "diff.plus.gutter"
                        | "diff.delta"
                        | "diff.delta.gutter"
                )
            {
                continue;
            }

            let dist_resolved = resolve_style(&dist_doc[key], dist_palette);
            let generated_resolved = resolve_style(&generated_doc[key], generated_palette);
            assert_eq!(
                generated_resolved, dist_resolved,
                "style mismatch for variant {variant_id}, key {key}"
            );
        }
    }
}

const HELIX_BUILTIN_COLORS: &[&str] = &[
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "gray",
    "light-red",
    "light-green",
    "light-yellow",
    "light-blue",
    "light-magenta",
    "light-cyan",
    "light-gray",
    "white",
    "default",
];

fn collect_color_names(value: &toml::Value, out: &mut HashSet<String>) {
    match value {
        toml::Value::String(s) => {
            out.insert(s.clone());
        }
        toml::Value::Table(t) => {
            if let Some(toml::Value::String(fg)) = t.get("fg") {
                out.insert(fg.clone());
            }
            if let Some(toml::Value::String(bg)) = t.get("bg") {
                out.insert(bg.clone());
            }
            if let Some(toml::Value::Table(u)) = t.get("underline")
                && let Some(toml::Value::String(c)) = u.get("color")
            {
                out.insert(c.clone());
            }
        }
        _ => {}
    }
}

#[test]
fn theme_helix_ninja_parses_and_resolves_all_color_names() {
    let theme = Theme::load(root_dir().join("themes/ninja")).unwrap();
    let generator = generator();
    let artifacts = generator.generate_theme_tool("helix", &theme).unwrap();

    let text = artifact_text(&artifacts, "helix/ninja-shadow.toml");
    let doc: toml::Table = text.parse().unwrap();
    let palette = doc["palette"].as_table().unwrap();

    let mut referenced = HashSet::new();
    for (key, value) in &doc {
        if key == "palette" {
            continue;
        }
        collect_color_names(value, &mut referenced);
    }
    assert!(!referenced.is_empty());

    for name in referenced {
        let known = palette.contains_key(&name) || HELIX_BUILTIN_COLORS.contains(&name.as_str());
        assert!(known, "unknown color name referenced: {name}");
    }
}

#[test]
fn theme_terminal_akari_matches_dist_exactly() {
    let theme = Theme::load(root_dir().join("themes/akari")).unwrap();
    let generator = generator();
    let artifacts = generator.generate_theme_tool("terminal", &theme).unwrap();

    for name in ["Night", "Dawn"] {
        let generated = artifact_text(&artifacts, &format!("terminal/Akari-{name}.terminal"));
        let dist =
            fs::read_to_string(root_dir().join(format!("dist/terminal/Akari-{name}.terminal")))
                .unwrap();
        assert_eq!(generated, dist, "terminal artifact mismatch for {name}");
    }
}

#[test]
fn theme_terminal_ninja_profile_name_and_file_name() {
    let theme = Theme::load(root_dir().join("themes/ninja")).unwrap();
    let generator = generator();
    let artifacts = generator.generate_theme_tool("terminal", &theme).unwrap();

    let text = artifact_text(&artifacts, "terminal/Ninja-Shadow.terminal");
    assert!(text.contains("<string>Ninja-Shadow</string>"));
}

fn dist_files(tool: &str) -> HashSet<PathBuf> {
    walkdir_files(&root_dir().join("dist").join(tool))
        .into_iter()
        .map(|p| {
            PathBuf::from(tool).join(p.strip_prefix(root_dir().join("dist").join(tool)).unwrap())
        })
        .collect()
}

fn walkdir_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            out.extend(walkdir_files(&path));
        } else {
            out.push(path);
        }
    }
    out
}

#[test]
fn theme_akari_artifacts_match_dist_exactly() {
    let theme = Theme::load(root_dir().join("themes/akari")).unwrap();
    let generator = generator();

    for tool in DIST_EXACT_TOOLS {
        let artifacts = generator.generate_theme_tool(tool, &theme).unwrap();

        let generated: HashSet<PathBuf> = artifacts.iter().map(|a| a.rel_path.clone()).collect();
        assert_eq!(generated, dist_files(tool), "file set mismatch for {tool}");

        for artifact in &artifacts {
            let dist = fs::read(root_dir().join("dist").join(&artifact.rel_path)).unwrap();
            let generated = match &artifact.content {
                ArtifactContent::Text(text) => text.as_bytes().to_vec(),
                ArtifactContent::Copy(src) => fs::read(src).unwrap(),
            };
            assert!(
                generated == dist,
                "{} differs from dist",
                artifact.rel_path.display()
            );
        }
    }
}

#[test]
fn theme_ninja_artifacts_are_named_by_theme_and_variant_ids() {
    let theme = Theme::load(root_dir().join("themes/ninja")).unwrap();
    let generator = generator();

    let expected = [
        ("delta", "delta/ninja-shadow.gitconfig"),
        ("lazygit", "lazygit/ninja-shadow.yml"),
        ("gh-dash", "gh-dash/ninja-shadow.yml"),
        ("nix", "nix/ninja-shadow-delta.nix"),
        ("nix", "nix/ninja-shadow-fzf.nix"),
        ("nix", "nix/ninja-shadow-gh-dash.nix"),
    ];
    for (tool, rel) in expected {
        let artifacts = generator.generate_theme_tool(tool, &theme).unwrap();
        artifact_text(&artifacts, rel);
    }
}

#[test]
fn theme_delta_names_section_and_bat_theme_from_metadata() {
    let theme = Theme::load(root_dir().join("themes/ninja")).unwrap();
    let generator = generator();
    let artifacts = generator.generate_theme_tool("delta", &theme).unwrap();

    let text = artifact_text(&artifacts, "delta/ninja-shadow.gitconfig");
    assert!(text.contains(r#"[delta "ninja-shadow"]"#));
    assert!(text.contains(r#"syntax-theme = "Ninja Shadow""#));
}

/// Ninja's only variant is dark but not named "night", so a switch that
/// tests the variant id instead of its appearance emits `light = true`.
#[test]
fn theme_delta_switches_dark_mode_on_appearance_not_variant_id() {
    let theme = Theme::load(root_dir().join("themes/ninja")).unwrap();
    let generator = generator();

    let gitconfig = generator.generate_theme_tool("delta", &theme).unwrap();
    let nix = generator.generate_theme_tool("nix", &theme).unwrap();
    for text in [
        artifact_text(&gitconfig, "delta/ninja-shadow.gitconfig"),
        artifact_text(&nix, "nix/ninja-shadow-delta.nix"),
    ] {
        let settings: Vec<&str> = text.lines().map(str::trim).collect();
        assert!(settings.iter().any(|l| l.starts_with("dark = true")));
        assert!(!settings.iter().any(|l| l.starts_with("light = true")));
    }
}
