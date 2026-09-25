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

/// Tools whose every Akari artifact must equal the committed `dist/` file.
const DIST_EXACT_TOOLS: [&str; 7] = ["delta", "lazygit", "gh-dash", "nix", "fzf", "zsh", "tmux"];

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

    for tool in generator.available_tools().unwrap() {
        match generator.generate_theme_tool(&tool, &theme) {
            Err(Error::ToolNotThemed(t)) => assert_eq!(t, tool),
            other => panic!("expected ToolNotThemed for {tool}, got {other:?}"),
        }
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
        ("fzf", "fzf/ninja-shadow.sh"),
        ("fzf", "fzf/ninja-fzf.plugin.zsh"),
        ("zsh", "zsh/ninja-shadow.zsh"),
        ("zsh", "zsh/ninja-zsh.plugin.zsh"),
        ("tmux", "tmux/ninja-shadow.conf"),
        ("tmux", "tmux/ninja.tmux"),
    ];
    for (tool, rel) in expected {
        let artifacts = generator.generate_theme_tool(tool, &theme).unwrap();
        let text = artifact_text(&artifacts, rel);
        assert!(
            !text.to_lowercase().contains("akari"),
            "{rel} mentions akari"
        );
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

fn assert_no_akari_mentions(artifacts: &[Artifact], tool: &str) {
    for artifact in artifacts {
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

#[test]
fn ninja_shadow_text_artifacts_never_mention_akari() {
    let generator = generator();
    let theme = ninja_theme();

    for (tool, _) in MIGRATED_TOOLS {
        let artifacts = generator.generate_theme_tool(tool, &theme).unwrap();
        assert_no_akari_mentions(&artifacts, tool);
    }

    // zed and chrome don't fit `MIGRATED_TOOLS` (neither produces
    // `akari-{variant}.{ext}` paths), so they're checked here directly.
    for tool in ["zed", "chrome"] {
        let artifacts = generator.generate_theme_tool(tool, &theme).unwrap();
        assert_no_akari_mentions(&artifacts, tool);
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

// -- Combined (non-`{variant}`) templates and `adapter` context --------

fn write_template(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

/// `helix` stands in for a tool whose templates live in a temp dir, so these
/// tests drive `generate_theme_tool` without touching `THEME_TOOLS`.
fn theme_with_helix_adapter(name: &str, adapter: toml::Table) -> Theme {
    let mut theme = Theme::load(root_dir().join("themes").join(name)).unwrap();
    theme.adapters.insert("helix".to_string(), adapter);
    theme
}

#[test]
fn combined_template_without_variant_placeholder_renders_once_in_theme_toml_order() {
    let templates = tempfile::tempdir().unwrap();
    write_template(
        &templates.path().join("helix/{theme}.txt.tera"),
        "{% for v in variants %}{{ v.variant.id }}:{{ v.variant.appearance }}:\
         {{ v.base.background }}:{{ v.ansi.red }}:{{ v.roles.ui.accent }};{% endfor %}",
    );
    let generator = Generator::new(templates.path()).unwrap();

    let cases: [(&str, &[&str]); 2] = [("akari", &["night", "dawn"]), ("ninja", &["shadow"])];
    for (theme_name, expected_order) in cases {
        let theme = theme_with_helix_adapter(theme_name, toml::Table::new());
        let artifacts = generator.generate_theme_tool("helix", &theme).unwrap();

        assert_eq!(
            artifacts.len(),
            1,
            "expected exactly one artifact for {theme_name}"
        );
        let text = artifact_text(&artifacts, &format!("helix/{theme_name}.txt"));

        let entries: Vec<&str> = text.trim_end_matches(';').split(';').collect();
        assert_eq!(entries.len(), expected_order.len());
        for ((entry, expected_id), variant) in
            entries.iter().zip(expected_order).zip(&theme.variants)
        {
            let id = entry.split(':').next().unwrap();
            assert_eq!(id, *expected_id);
            assert!(entry.contains(&variant.base.background.to_string()));
            assert!(entry.contains(&variant.ansi.normal.red.to_string()));
            assert!(entry.contains(&variant.roles.ui.accent.to_string()));
        }
    }
}

#[test]
fn per_variant_and_combined_contexts_expose_adapter_contents() {
    let templates = tempfile::tempdir().unwrap();
    write_template(
        &templates.path().join("helix/{theme}-{variant}.txt.tera"),
        "{{ variant.id }}:{{ adapter.version }}",
    );
    write_template(
        &templates.path().join("helix/{theme}.txt.tera"),
        "{{ adapter.version }}",
    );
    let generator = Generator::new(templates.path()).unwrap();

    let mut adapter = toml::Table::new();
    adapter.insert(
        "version".to_string(),
        toml::Value::String("9.9.9".to_string()),
    );
    let theme = theme_with_helix_adapter("ninja", adapter);

    let artifacts = generator.generate_theme_tool("helix", &theme).unwrap();

    assert_eq!(
        artifact_text(&artifacts, "helix/ninja-shadow.txt"),
        "shadow:9.9.9"
    );
    assert_eq!(artifact_text(&artifacts, "helix/ninja.txt"), "9.9.9");
}

#[test]
fn adapter_context_defaults_to_empty_table_when_tool_has_no_adapters_entry() {
    let templates = tempfile::tempdir().unwrap();
    write_template(
        &templates.path().join("helix/{theme}-{variant}.txt.tera"),
        "{{ adapter.version | default(value=\"none\") }}",
    );
    let generator = Generator::new(templates.path()).unwrap();

    // Real `themes/ninja` has no `[adapters.helix]`.
    let theme = ninja_theme();
    let artifacts = generator.generate_theme_tool("helix", &theme).unwrap();

    assert_eq!(artifact_text(&artifacts, "helix/ninja-shadow.txt"), "none");
}

// -- Plugin entries that pick a variant ------------------------------------

/// Writes `tool`'s generated `entry` into a temp dir and replaces every other
/// generated file with a stub that prints its own file name, so running the
/// entry reports which variant file it loaded.
fn entry_with_stub_variant_files(tool: &str, theme: &Theme, entry: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for artifact in generator().generate_theme_tool(tool, theme).unwrap() {
        let ArtifactContent::Text(text) = &artifact.content else {
            continue;
        };
        let name = artifact.rel_path.file_name().unwrap().to_str().unwrap();
        let content = if name == entry {
            text.clone()
        } else {
            format!("echo {name}\n")
        };
        fs::write(dir.path().join(name), content).unwrap();
    }
    dir
}

fn variant_env_var(theme: &Theme) -> String {
    format!("{}_VARIANT", theme.metadata.id.as_str().to_uppercase())
}

/// Sources a zsh plugin entry under bash, which CI has and zsh is not
/// guaranteed to be; `${0:A:h}` is the one zsh-only expansion the entries
/// use, so it is bound to the entry's directory first.
fn zsh_entry_loads(tool: &str, theme: &Theme, entry: &str, variant: Option<&str>) -> String {
    let dir = entry_with_stub_variant_files(tool, theme, entry);
    let entry_path = dir.path().join(entry);
    let text = fs::read_to_string(&entry_path).unwrap();
    assert!(
        text.contains("${0:A:h}"),
        "{entry} no longer uses ${{0:A:h}}"
    );
    fs::write(
        &entry_path,
        text.replace("${0:A:h}", dir.path().to_str().unwrap()),
    )
    .unwrap();

    let mut command = std::process::Command::new("bash");
    command
        .arg("-c")
        .arg("source \"$1\"")
        .arg("bash")
        .arg(&entry_path);
    command.env_remove(variant_env_var(theme));
    if let Some(value) = variant {
        command.env(variant_env_var(theme), value);
    }
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{entry}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// Environment values to try against a theme's entry, each with the variant
/// it must load: unset and an unknown value fall back to the first variant.
fn variant_selections(theme: &Theme) -> Vec<(Option<String>, String)> {
    let ids: Vec<String> = theme
        .metadata
        .variants
        .iter()
        .map(|id| id.as_str().to_string())
        .collect();
    let default = ids[0].clone();
    let mut cases = vec![
        (None, default.clone()),
        (Some("unknown".to_string()), default),
    ];
    cases.extend(ids.into_iter().map(|id| (Some(id.clone()), id)));
    cases
}

const ZSH_PLUGIN_ENTRIES: [(&str, &str, &str); 2] = [
    ("fzf", "{theme}-fzf.plugin.zsh", "sh"),
    ("zsh", "{theme}-zsh.plugin.zsh", "zsh"),
];

#[test]
fn zsh_plugin_entries_load_the_selected_variant_and_default_to_the_first() {
    for theme in [
        Theme::load(root_dir().join("themes/akari")).unwrap(),
        ninja_theme(),
    ] {
        let id = theme.metadata.id.as_str();
        for (tool, entry, ext) in ZSH_PLUGIN_ENTRIES {
            let entry = entry.replace("{theme}", id);
            for (value, expected) in variant_selections(&theme) {
                let loaded = zsh_entry_loads(tool, &theme, &entry, value.as_deref());
                assert_eq!(
                    loaded,
                    format!("{id}-{expected}.{ext}"),
                    "{entry} with {}={value:?}",
                    variant_env_var(&theme)
                );
            }
        }
    }
}

/// Runs `{theme}.tmux` under bash with `tmux` replaced by a stub on `PATH`
/// that answers `show-option` for the variant option and logs every call.
fn tmux_entry_calls(theme: &Theme, variant: Option<&str>) -> String {
    let id = theme.metadata.id.as_str();
    let entry = format!("{id}.tmux");
    let dir = entry_with_stub_variant_files("tmux", theme, &entry);
    let bin = tempfile::tempdir().unwrap();
    let stub = bin.path().join("tmux");
    fs::write(
        &stub,
        format!(
            "#!/bin/sh\n\
             if [ \"$1\" = show-option ]; then\n\
             \x20 [ \"$3\" = @{id}_variant ] && printf '%s' \"$STUB_VARIANT\"\n\
             \x20 exit 0\n\
             fi\n\
             echo \"$@\"\n"
        ),
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&stub, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let path = format!(
        "{}:{}",
        bin.path().display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let output = std::process::Command::new("bash")
        .arg(dir.path().join(&entry))
        .env("PATH", path)
        .env("STUB_VARIANT", variant.unwrap_or(""))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{entry}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn tmux_entry_sources_and_colors_the_selected_variant_and_defaults_to_the_first() {
    for theme in [
        Theme::load(root_dir().join("themes/akari")).unwrap(),
        ninja_theme(),
    ] {
        let id = theme.metadata.id.as_str();
        for (value, expected) in variant_selections(&theme) {
            let calls = tmux_entry_calls(&theme, value.as_deref());
            let conf = format!("/{id}-{expected}.conf");
            assert!(
                calls
                    .lines()
                    .any(|l| l.starts_with("source-file ") && l.ends_with(&conf)),
                "{id} with {value:?} did not source {conf}: {calls}"
            );

            let resolved = theme
                .variants
                .iter()
                .find(|v| v.variant.id.as_str() == expected)
                .unwrap();
            let status_left = calls
                .lines()
                .find(|l| l.starts_with("set-option -g status-left "))
                .unwrap_or_else(|| panic!("{id} with {value:?} set no status-left: {calls}"));
            for color in [resolved.base.background, resolved.roles.ui.surface] {
                assert!(
                    status_left.contains(&color.to_string()),
                    "{id} with {value:?}: status-left lacks {color}: {status_left}"
                );
            }
        }
    }
}

// -- Phase 2: zed and chrome migrated to the theme route ---------------

#[test]
fn ninja_zed_theme_has_a_single_dark_shadow_entry() {
    let generator = generator();
    let artifacts = generator
        .generate_theme_tool("zed", &ninja_theme())
        .unwrap();
    let text = artifact_text(&artifacts, "zed/ninja.json");
    let doc: serde_json::Value = serde_json::from_str(text).unwrap();

    let themes = doc["themes"].as_array().unwrap();
    assert_eq!(themes.len(), 1);
    assert_eq!(themes[0]["appearance"], "dark");
    assert_eq!(themes[0]["name"], "Ninja Shadow");
}

#[test]
fn akari_zed_theme_has_night_then_dawn_entries_in_order() {
    let generator = generator();
    let theme = Theme::load(root_dir().join("themes/akari")).unwrap();
    let artifacts = generator.generate_theme_tool("zed", &theme).unwrap();
    let text = artifact_text(&artifacts, "zed/akari.json");
    let doc: serde_json::Value = serde_json::from_str(text).unwrap();

    let themes = doc["themes"].as_array().unwrap();
    assert_eq!(themes.len(), 2);
    assert_eq!(themes[0]["name"], "Akari Night");
    assert_eq!(themes[0]["appearance"], "dark");
    assert_eq!(themes[1]["name"], "Akari Dawn");
    assert_eq!(themes[1]["appearance"], "light");
}

#[test]
fn ninja_chrome_produces_exactly_one_output_dir_named_after_the_theme_and_variant() {
    let generator = generator();
    let theme = ninja_theme();
    let expected_version = theme.adapters["chrome"]["version"]
        .as_str()
        .unwrap()
        .to_string();

    let artifacts = generator.generate_theme_tool("chrome", &theme).unwrap();
    let manifest_dirs: HashSet<&Path> = artifacts
        .iter()
        .filter_map(|a| a.rel_path.parent())
        .filter(|p| *p != Path::new("chrome"))
        .collect();
    assert_eq!(
        manifest_dirs,
        HashSet::from([Path::new("chrome/ninja-shadow")])
    );

    let text = artifact_text(&artifacts, "chrome/ninja-shadow/manifest.json");
    let doc: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(doc["version"], expected_version);
}

#[test]
fn chrome_requires_adapters_chrome_version() {
    let generator = generator();
    let mut theme = ninja_theme();
    theme.adapters.remove("chrome");

    let err = generator.generate_theme_tool("chrome", &theme).unwrap_err();
    match &err {
        Error::AdapterKeyMissing { tool, key } => {
            assert_eq!(tool, "chrome");
            assert_eq!(key, "version");
        }
        other => panic!("expected AdapterKeyMissing, got {other:?}"),
    }
    let message = err.to_string();
    assert!(message.contains("chrome"), "{message}");
    assert!(message.contains("version"), "{message}");
}

#[test]
fn chrome_renders_with_only_the_declared_adapter_keys() {
    let generator = generator();
    let mut theme = ninja_theme();
    let mut adapter = toml::Table::new();
    adapter.insert(
        "version".to_string(),
        toml::Value::String("9.9.9".to_string()),
    );
    theme.adapters.insert("chrome".to_string(), adapter);

    let artifacts = generator.generate_theme_tool("chrome", &theme).unwrap();
    let text = artifact_text(&artifacts, "chrome/ninja-shadow/manifest.json");
    let doc: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(doc["version"], "9.9.9");
}
