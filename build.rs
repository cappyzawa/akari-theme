use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_files(dir: &Path, base: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()));
    for entry in entries {
        let path = entry
            .unwrap_or_else(|e| panic!("read_dir entry in {}: {e}", dir.display()))
            .path();
        if path.is_dir() {
            collect_files(&path, base, out);
        } else {
            out.push(
                path.strip_prefix(base)
                    .unwrap_or_else(|_| {
                        panic!("{} is not under {}", path.display(), base.display())
                    })
                    .to_path_buf(),
            );
        }
    }
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(_path: &Path) -> bool {
    false
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let templates_dir = Path::new(&manifest_dir).join("templates");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    let dest = Path::new(&out_dir).join("embedded_templates.rs");

    let mut rel_paths = Vec::new();
    collect_files(&templates_dir, &templates_dir, &mut rel_paths);
    rel_paths.sort();

    let mut code = String::from("&[\n");
    for rel in &rel_paths {
        let rel_str = rel
            .components()
            .map(|c| {
                c.as_os_str()
                    .to_str()
                    .unwrap_or_else(|| panic!("non-UTF-8 path component in {}", rel.display()))
            })
            .collect::<Vec<_>>()
            .join("/");
        let executable = is_executable(&templates_dir.join(rel));
        let include_literal = format!("/templates/{rel_str}");
        code.push_str(&format!(
            "    TemplateFile {{ path: ::std::borrow::Cow::Borrowed({rel_str:?}), contents: ::std::borrow::Cow::Borrowed(include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), {include_literal:?}))), executable: {executable} }},\n",
        ));
    }
    code.push_str("]\n");

    fs::write(&dest, code).unwrap_or_else(|e| panic!("write {}: {e}", dest.display()));

    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=build.rs");
}
