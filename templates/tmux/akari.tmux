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

main() {
  local variant
  variant=$(get_tmux_option "@akari_variant" "night")

  tmux source-file "$CURRENT_DIR/akari-${variant}.conf"
}

main
