---
name: release
description: Create a new release by bumping version, regenerating dist, running checks, and tagging
argument-hint: <version>
---

# Release

Create a new release for akari-theme.

## Arguments

`$ARGUMENTS` — target version (e.g. `1.18.1`). The `v` prefix is optional and will be normalized.

## Steps

1. **Current state** — run `git tag --sort=-v:refname | head -3` to show recent tags, then identify the latest tag and run `git log --oneline <latest_tag>..HEAD` to show commits since the last release
2. **Validate** — ensure `$ARGUMENTS` is provided and is a valid semver greater than the latest tag
3. **Version bump** — update `version` in `Cargo.toml` to the target version
4. **Regenerate dist** — run `cargo run --features generator -- generate --tool all`
5. **Check** — run the full verification suite:
   ```
   cargo clippy --features generator -- -D warnings
   cargo fmt -- --check
   cargo test --features generator
   ```
6. **Diff guard** — run `git diff --stat` and confirm only `Cargo.toml` and `Cargo.lock` changed
7. **Commit** — `git add Cargo.toml Cargo.lock && git commit -s -m "Prepare for v<version> release"`
8. **Tag** — `git tag v<version>`
9. **Push** — ask the user for confirmation, then `git push origin main && git push origin v<version>`
