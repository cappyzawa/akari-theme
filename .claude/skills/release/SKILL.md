---
name: release
description: Create a new release by bumping version, regenerating dist, running checks, and tagging
argument-hint: <version>
disable-model-invocation: true
---

# Release

Create a new release for akari-theme.

## Arguments

`$ARGUMENTS` — target version (e.g. `1.18.1`). The `v` prefix is optional and will be normalized.

## Current state

!`git tag --sort=-v:refname | head -3`

!`git tag --sort=-v:refname | head -1 | xargs -I{} git log --oneline {}..HEAD`

## Steps

1. **Validate** — ensure `$ARGUMENTS` is provided and is a valid semver greater than the latest tag
2. **Version bump** — update `version` in `Cargo.toml` to the target version
3. **Regenerate dist** — run `cargo run --features generator -- generate --tool all`
4. **Check** — run the full verification suite:
   ```
   cargo clippy --features generator -- -D warnings
   cargo fmt -- --check
   cargo test --features generator
   ```
5. **Diff guard** — run `git diff --stat` and confirm only `Cargo.toml` and `Cargo.lock` changed
6. **Commit** — `git add Cargo.toml Cargo.lock && git commit -s -m "Prepare for v<version> release"`
7. **Tag** — `git tag v<version>`
8. **Push** — ask the user for confirmation, then `git push origin main && git push origin v<version>`
