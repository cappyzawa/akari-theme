#!/usr/bin/env bash
# Akari tmux theme - TPM plugin entry point

CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

get_tmux_option() {
  local option=$1
  local default_value=$2
  local option_value
  option_value=$(tmux show-option -gqv "$option")
  echo "${option_value:-$default_value}"
}

# Color palette per variant (bash 3.2 compatible, no associative arrays)
# Base
NIGHT_BG="#25231F"
NIGHT_FG="#E6DED3"
# Layers
NIGHT_SURFACE="#323333"
# Semantic
NIGHT_KEYWORD="#E26A3B"

# Base
DAWN_BG="#E4DED6"
DAWN_FG="#1A1816"
# Layers
DAWN_SURFACE="#CFC4B6"
# Semantic
DAWN_KEYWORD="#8A4530"

# Default icons (nerdfont)
DEFAULT_ICON_NORMAL="󰛩"
DEFAULT_ICON_PREFIX="󰛨"

setup_status_left() {
  local variant=$1
  local icon_normal icon_prefix bg fg surface keyword

  icon_normal=$(get_tmux_option "@akari_icon_normal" "$DEFAULT_ICON_NORMAL")
  icon_prefix=$(get_tmux_option "@akari_icon_prefix" "$DEFAULT_ICON_PREFIX")

  if [[ "$variant" == "dawn" ]]; then
    bg="$DAWN_BG"
    fg="$DAWN_FG"
    surface="$DAWN_SURFACE"
    keyword="$DAWN_KEYWORD"
  else
    bg="$NIGHT_BG"
    fg="$NIGHT_FG"
    surface="$NIGHT_SURFACE"
    keyword="$NIGHT_KEYWORD"
  fi

  # Icon changes on prefix, icon and session name have surface background
  tmux set-option -g status-left "#[bg=${surface}#,fg=${keyword}] #{?client_prefix,${icon_prefix},${icon_normal}} #[fg=${fg}#,bold]#S #[bg=${bg}#,nobold] "
}

main() {
  local variant
  variant=$(get_tmux_option "@akari_variant" "night")

  tmux source-file "$CURRENT_DIR/akari-${variant}.conf"

  setup_status_left "$variant"
}

main
