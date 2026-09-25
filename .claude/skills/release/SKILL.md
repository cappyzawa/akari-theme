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
2. **Validate** — read the current version with `cat VERSION`, then ensure `$ARGUMENTS` is provided and is a valid semver greater than both the latest tag and `VERSION`
3. **Version bump** — overwrite `VERSION` with the target version (one line, no `v` prefix)
4. **Regenerate dist** — run `cargo run -- generate --theme-dir themes/akari --tool all --out-dir dist`
5. **Check** — run the full verification suite:
   ```
   cargo clippy --all-targets -- -D warnings
   cargo fmt -- --check
   cargo test
   cargo test --no-default-features
   ```
6. **Diff guard** — run `git diff --stat` and confirm only `VERSION` changed
7. **Commit** — `git add VERSION && git commit -s -m "Prepare for v<version> release"`
8. **Tag** — `git tag v<version>`
9. **Push** — ask the user for confirmation, then `git push origin main && git push origin v<version>`
