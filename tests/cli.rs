//! Black-box tests for the `katazome` binary's CLI surface.

use katazome::Generator;
use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Every file under `dir`, as a set of paths relative to it.
fn collect_relative_files(dir: &Path) -> HashSet<PathBuf> {
    let mut out = HashSet::new();
    collect_relative_files_into(dir, dir, &mut out);
    out
}

fn collect_relative_files_into(dir: &Path, base: &Path, out: &mut HashSet<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_relative_files_into(&path, base, out);
        } else {
            out.insert(path.strip_prefix(base).unwrap().to_path_buf());
        }
    }
}

/// The top-level entry names directly under `dir` (e.g. one per tool
/// `generate` wrote to), as a set.
fn top_level_entries(dir: &Path) -> HashSet<String> {
    fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect()
}

fn to_owned_set(names: &[&str]) -> HashSet<String> {
    names.iter().map(|s| s.to_string()).collect()
}

/// Runs `generate` with exactly `tool_args` (e.g. `["--tool", "helix"]`,
/// possibly repeated, or `&[]` to omit `--tool` entirely) and returns the
/// out-dir's top-level entries.
fn generate_with_tools(theme_dir: &Path, tool_args: &[&str]) -> HashSet<String> {
    let out = tempfile::tempdir().unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_katazome"))
        .current_dir(root_dir())
        .args(["generate", "--theme-dir"])
        .arg(theme_dir)
        .args(tool_args)
        .args(["--out-dir"])
        .arg(out.path())
        .status()
        .unwrap();
    assert!(
        status.success(),
        "katazome generate failed for {}",
        theme_dir.display()
    );
    top_level_entries(out.path())
}

fn generate(theme_dir: &Path, extra_args: &[&str], out_dir: &Path) {
    let status = Command::new(env!("CARGO_BIN_EXE_katazome"))
        .current_dir(root_dir())
        .args(["generate", "--theme-dir"])
        .arg(theme_dir)
        .args(["--tool", "all", "--out-dir"])
        .arg(out_dir)
        .args(extra_args)
        .status()
        .unwrap();
    assert!(
        status.success(),
        "katazome generate failed for {}",
        theme_dir.display()
    );
}

#[test]
fn templates_dir_override_generates_the_same_files_as_the_built_in_templates() {
    for theme_dir in [
        root_dir().join("themes/ninja"),
        root_dir().join("tests/fixtures/duo"),
    ] {
        let embedded_out = tempfile::tempdir().unwrap();
        let overridden_out = tempfile::tempdir().unwrap();

        generate(&theme_dir, &[], embedded_out.path());
        generate(
            &theme_dir,
            &[
                "--templates-dir",
                root_dir().join("templates").to_str().unwrap(),
            ],
            overridden_out.path(),
        );

        let embedded_files = collect_relative_files(embedded_out.path());
        let overridden_files = collect_relative_files(overridden_out.path());
        assert!(
            !embedded_files.is_empty(),
            "no files generated for {}",
            theme_dir.display()
        );
        assert_eq!(
            embedded_files,
            overridden_files,
            "file sets differ for {}",
            theme_dir.display()
        );

        for rel in &embedded_files {
            let embedded_path = embedded_out.path().join(rel);
            let overridden_path = overridden_out.path().join(rel);

            let embedded_bytes = fs::read(&embedded_path).unwrap();
            let overridden_bytes = fs::read(&overridden_path).unwrap();
            assert_eq!(
                embedded_bytes,
                overridden_bytes,
                "{} differs between embedded and --templates-dir for {}",
                rel.display(),
                theme_dir.display()
            );

            let embedded_mode = fs::metadata(&embedded_path).unwrap().permissions().mode() & 0o111;
            let overridden_mode =
                fs::metadata(&overridden_path).unwrap().permissions().mode() & 0o111;
            assert_eq!(
                embedded_mode,
                overridden_mode,
                "{} has a different executable bit between embedded and --templates-dir for {}",
                rel.display(),
                theme_dir.display()
            );
        }
    }
}

// -- `--tool`: optional, repeatable, and theme.toml's default tools --------

/// `themes/ninja/theme.toml` declares `tools = ["helix", "nvim", "terminal"]`.
const NINJA_DEFAULT_TOOLS: [&str; 3] = ["helix", "nvim", "terminal"];

#[test]
fn ninja_without_a_tool_flag_generates_exactly_its_default_tools() {
    let generated = generate_with_tools(&root_dir().join("themes/ninja"), &[]);
    assert_eq!(generated, to_owned_set(&NINJA_DEFAULT_TOOLS));
}

#[test]
fn repeated_tool_flags_on_duo_override_its_absent_default_tools() {
    let generated = generate_with_tools(
        &root_dir().join("tests/fixtures/duo"),
        &["--tool", "helix", "--tool", "nvim"],
    );
    assert_eq!(generated, to_owned_set(&["helix", "nvim"]));
}

#[test]
fn a_tool_flag_on_ninja_overrides_its_default_tools() {
    let generated = generate_with_tools(&root_dir().join("themes/ninja"), &["--tool", "zed"]);
    assert_eq!(generated, to_owned_set(&["zed"]));
}

#[test]
fn tool_all_on_ninja_generates_every_tool_despite_its_default_tools() {
    let generated = generate_with_tools(&root_dir().join("themes/ninja"), &["--tool", "all"]);
    assert_eq!(generated, to_owned_set(Generator::available_tools()));
}
