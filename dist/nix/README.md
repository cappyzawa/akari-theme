# Akari Nix Templates

These templates generate Nix attrsets for Home Manager module integration.

## Generated Files

- `akari-night-fzf.nix` / `akari-dawn-fzf.nix` - fzf colors for `programs.fzf.colors`
- `akari-night-gh-dash.nix` / `akari-dawn-gh-dash.nix` - gh-dash theme for `programs.gh-dash.settings`

## Usage

These files are imported by the Home Manager module in `nix/targets/`.
See the main README for Home Manager integration instructions.
