//! Black-box tests for the theme-based generation route
//! (`Generator::generate_theme_tool`), which consumes `Theme` instead of
//! the legacy `Palette` pair. Covers the two migrated tools (helix,
//! terminal) and the legacy/theme route boundary.

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

#[test]
fn legacy_available_tools_excludes_theme_tools() {
    let generator = generator();
    let tools = generator.available_tools().unwrap();

    assert!(!tools.is_empty());
    for tool in generator.available_theme_tools() {
        assert!(!tools.contains(&tool), "{tool} still legacy");
    }
}

#[test]
fn legacy_generate_tool_rejects_theme_tools() {
    let generator = generator();
    let night = akari_theme::Palette::night();
    let dawn = akari_theme::Palette::dawn();

    for tool in generator.available_theme_tools() {
        let result = generator.generate_tool(&tool, &night, &dawn);
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

    let result = generator.generate_theme_tool("lazygit", &theme);
    match result {
        Err(Error::ToolNotThemed(t)) => assert_eq!(t, "lazygit"),
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

/// Theme-route tools checked below, with the extension of their output files.
const MIGRATED_TOOLS: &[(&str, &str)] = &[
    ("alacritty", "toml"),
    ("bat", "tmTheme"),
    ("codex", "txt"),
    ("ghostty", ""),
    ("slack", "txt"),
    ("starship", "toml"),
    ("zellij", "kdl"),
];

fn variant_artifact_path(tool: &str, ext: &str, variant_id: &str) -> String {
    if ext.is_empty() {
        format!("{tool}/akari-{variant_id}")
    } else {
        format!("{tool}/akari-{variant_id}.{ext}")
    }
}

#[test]
fn theme_route_generates_night_and_dawn_outputs_for_each_migrated_tool() {
    let theme = Theme::load(root_dir().join("themes/akari")).unwrap();
    let generator = generator();

    for (tool, ext) in MIGRATED_TOOLS {
        let artifacts = generator.generate_theme_tool(tool, &theme).unwrap();
        for variant_id in ["night", "dawn"] {
            let rel = variant_artifact_path(tool, ext, variant_id);
            artifact_text(&artifacts, &rel);
        }
    }
}

fn ninja_theme() -> Theme {
    Theme::load(root_dir().join("themes/ninja")).unwrap()
}

fn is_hex_color(s: &str) -> bool {
    match s.strip_prefix('#') {
        Some(hex) => hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()),
        None => false,
    }
}

#[test]
fn ninja_shadow_alacritty_and_starship_outputs_parse_as_toml() {
    let generator = generator();
    let theme = ninja_theme();

    for tool in ["alacritty", "starship"] {
        let artifacts = generator.generate_theme_tool(tool, &theme).unwrap();
        let text = artifact_text(&artifacts, &format!("{tool}/ninja-shadow.toml"));
        text.parse::<toml::Table>()
            .unwrap_or_else(|e| panic!("{tool} output is not valid TOML: {e}"));
    }
}

#[test]
fn ninja_shadow_starship_palette_table_values_are_hex_colors() {
    let generator = generator();
    let artifacts = generator
        .generate_theme_tool("starship", &ninja_theme())
        .unwrap();
    let text = artifact_text(&artifacts, "starship/ninja-shadow.toml");
    let doc: toml::Table = text.parse().unwrap();

    let palette = doc["palettes"]["ninja-shadow"]
        .as_table()
        .expect("palettes.ninja-shadow table");
    assert!(!palette.is_empty());
    for (key, value) in palette {
        let s = value
            .as_str()
            .unwrap_or_else(|| panic!("palettes.ninja-shadow.{key} is not a string"));
        assert!(is_hex_color(s), "palettes.ninja-shadow.{key} = {s:?}");
    }
}

#[test]
fn ninja_shadow_zellij_output_has_balanced_braces_and_the_theme_block() {
    let generator = generator();
    let artifacts = generator
        .generate_theme_tool("zellij", &ninja_theme())
        .unwrap();
    let text = artifact_text(&artifacts, "zellij/ninja-shadow.kdl");

    let mut depth: i32 = 0;
    for ch in text.chars() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                assert!(depth >= 0, "closing brace with no matching opener");
            }
            _ => {}
        }
    }
    assert_eq!(depth, 0, "braces are not balanced");
    assert!(text.contains("ninja-shadow {"));
}

#[test]
fn ninja_shadow_codex_output_is_json_after_the_codex_theme_prefix() {
    let generator = generator();
    let artifacts = generator
        .generate_theme_tool("codex", &ninja_theme())
        .unwrap();
    let text = artifact_text(&artifacts, "codex/ninja-shadow.txt");
    let json = text
        .trim()
        .strip_prefix("codex-theme-v1:")
        .expect("codex-theme-v1: prefix");
    let _: serde_json::Value = serde_json::from_str(json).unwrap();
}

#[test]
fn ninja_shadow_bat_output_parses_as_xml_plist() {
    let generator = generator();
    let artifacts = generator
        .generate_theme_tool("bat", &ninja_theme())
        .unwrap();
    let text = artifact_text(&artifacts, "bat/ninja-shadow.tmTheme");
    plist::Value::from_reader_xml(text.as_bytes())
        .unwrap_or_else(|e| panic!("bat output is not valid plist XML: {e}"));
}

#[test]
fn ninja_shadow_slack_output_is_four_hex_colors() {
    let generator = generator();
    let artifacts = generator
        .generate_theme_tool("slack", &ninja_theme())
        .unwrap();
    let text = artifact_text(&artifacts, "slack/ninja-shadow.txt");
    let values: Vec<&str> = text.trim().split(',').collect();
    assert_eq!(
        values.len(),
        4,
        "expected 4 comma-separated values: {values:?}"
    );
    for value in values {
        assert!(is_hex_color(value), "{value:?} is not #RRGGBB");
    }
}

#[test]
fn ninja_shadow_ghostty_lines_are_key_value_pairs() {
    let generator = generator();
    let artifacts = generator
        .generate_theme_tool("ghostty", &ninja_theme())
        .unwrap();
    let text = artifact_text(&artifacts, "ghostty/ninja-shadow");

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        assert!(line.contains(" = "), "line is not `key = value`: {line:?}");
    }
}

#[test]
fn ninja_shadow_text_artifacts_never_mention_akari() {
    let generator = generator();
    let theme = ninja_theme();

    for (tool, _) in MIGRATED_TOOLS {
        let artifacts = generator.generate_theme_tool(tool, &theme).unwrap();
        for artifact in &artifacts {
            let ArtifactContent::Text(text) = &artifact.content else {
                continue;
            };
            assert!(
                !text.to_lowercase().contains("akari"),
                "{} ({tool}) mentions akari",
                artifact.rel_path.display()
            );
        }
    }
}

#[test]
fn codex_variant_and_contrast_and_role_colors_match_the_loaded_theme() {
    let generator = generator();
    let akari = Theme::load(root_dir().join("themes/akari")).unwrap();
    let ninja = ninja_theme();

    // (theme, variant id, expected `variant`, expected `contrast`)
    let cases = [
        (&akari, "night", "dark", 60),
        (&akari, "dawn", "light", 45),
        (&ninja, "shadow", "dark", 60),
    ];

    for (theme, variant_id, expected_appearance, expected_contrast) in cases {
        let artifacts = generator.generate_theme_tool("codex", theme).unwrap();
        let rel = format!("codex/{}-{variant_id}.txt", theme.metadata.id);
        let text = artifact_text(&artifacts, &rel);
        let json = text.trim().strip_prefix("codex-theme-v1:").unwrap();
        let value: serde_json::Value = serde_json::from_str(json).unwrap();

        let resolved = theme
            .variants
            .iter()
            .find(|v| v.variant.id.as_str() == variant_id)
            .unwrap();

        assert_eq!(value["variant"], expected_appearance);
        assert_eq!(value["theme"]["contrast"], expected_contrast);
        assert_eq!(
            value["theme"]["accent"].as_str().unwrap(),
            resolved.roles.ui.accent.to_string()
        );
        assert_eq!(
            value["theme"]["semanticColors"]["skill"].as_str().unwrap(),
            resolved.roles.syntax.function.to_string()
        );
        assert_eq!(
            value["theme"]["semanticColors"]["diffAdded"]
                .as_str()
                .unwrap(),
            resolved.roles.diff.added.to_string()
        );
        assert_eq!(
            value["theme"]["semanticColors"]["diffRemoved"]
                .as_str()
                .unwrap(),
            resolved.roles.diff.removed.to_string()
        );
        assert_eq!(
            value["theme"]["ink"].as_str().unwrap(),
            resolved.base.foreground.to_string()
        );
        assert_eq!(
            value["theme"]["surface"].as_str().unwrap(),
            resolved.base.background.to_string()
        );
    }
}
