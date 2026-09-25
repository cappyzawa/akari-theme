# CLAUDE.md

## Verification (after theme or color changes)

```sh
scripts/install-katazome
.katazome/bin/katazome generate --theme-dir themes/akari --tool all --out-dir dist
git diff --exit-code
```

Engine, template and tool changes belong in [katazome](https://github.com/cappyzawa/katazome), not here.
