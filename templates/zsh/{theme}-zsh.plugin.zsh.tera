# Akari zsh-syntax-highlighting theme - zsh plugin entry point
# Loads the appropriate theme based on AKARI_VARIANT environment variable.

: ${AKARI_VARIANT:=night}

if [[ "$AKARI_VARIANT" == "dawn" ]]; then
  source "${0:A:h}/akari-dawn.zsh"
else
  source "${0:A:h}/akari-night.zsh"
fi
