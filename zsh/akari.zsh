# Akari v0.1.0 — zsh-syntax-highlighting configuration
# A color palette inspired by Japanese alleys lit by round lanterns.
#
# Usage:
#   AKARI_VARIANT=night source /path/to/akari.zsh  # default
#   AKARI_VARIANT=dawn source /path/to/akari.zsh

# Declare the associative array
typeset -A ZSH_HIGHLIGHT_STYLES

# Set variant (default: night)
: ${AKARI_VARIANT:=night}

if [[ "$AKARI_VARIANT" == "dawn" ]]; then
  # Dawn palette (light) — dawn alley surface, ink black text
  _lantern='#8A4530'   # colors.lantern
  _ember='#7A3828'     # colors.ember
  _amber='#B07840'     # colors.amber
  _life='#3A5830'      # colors.life
  _night='#304050'     # colors.night
  _muted='#806080'     # colors.muted
  _cyan='#305858'      # ansi.cyan
  _text='#1A1816'      # base.foreground — ink black
  _comment='#4A4642'   # semantic.comment
  _border='#C4BEB6'    # layers.border
else
  # Night palette (dark, default)
  _lantern='#E26A3B'   # colors.lantern
  _ember='#D65A3A'     # colors.ember
  _amber='#D4A05A'     # colors.amber
  _life='#7FAF6A'      # colors.life
  _night='#5A6F82'     # colors.night
  _muted='#7C6A8A'     # colors.muted
  _cyan='#6F8F8A'      # ansi.cyan
  _text='#E6DED3'      # base.foreground
  _comment='#7D8797'   # semantic.comment (distant stone-gray)
  _border='#2E3543'    # layers.border
fi

# Commands
ZSH_HIGHLIGHT_STYLES[command]="fg=$_lantern,bold"          # colors.lantern
ZSH_HIGHLIGHT_STYLES[builtin]="fg=$_lantern,bold"          # colors.lantern
ZSH_HIGHLIGHT_STYLES[alias]="fg=$_lantern"                 # colors.lantern
ZSH_HIGHLIGHT_STYLES[function]="fg=$_lantern"              # colors.lantern

# Errors and unknown
ZSH_HIGHLIGHT_STYLES[unknown-token]="fg=$_comment"         # semantic.comment
ZSH_HIGHLIGHT_STYLES[reserved-word]="fg=$_night"           # colors.night

# Paths and files
ZSH_HIGHLIGHT_STYLES[path]="fg=$_amber,underline"          # colors.amber
ZSH_HIGHLIGHT_STYLES[path_pathseparator]="fg=$_amber"      # colors.amber
ZSH_HIGHLIGHT_STYLES[globbing]="fg=$_comment"              # semantic.comment

# Strings and quotes
ZSH_HIGHLIGHT_STYLES[single-quoted-argument]="fg=$_lantern"  # colors.lantern
ZSH_HIGHLIGHT_STYLES[double-quoted-argument]="fg=$_lantern"  # colors.lantern
ZSH_HIGHLIGHT_STYLES[dollar-quoted-argument]="fg=$_lantern"  # colors.lantern

# Variables and substitutions
ZSH_HIGHLIGHT_STYLES[assign]="fg=$_text"                             # base.foreground
ZSH_HIGHLIGHT_STYLES[named-fd]="fg=$_cyan"                           # ansi.cyan
ZSH_HIGHLIGHT_STYLES[command-substitution]="fg=$_muted"              # colors.muted
ZSH_HIGHLIGHT_STYLES[command-substitution-delimiter]="fg=$_comment"  # semantic.comment
ZSH_HIGHLIGHT_STYLES[process-substitution]="fg=$_muted"              # colors.muted
ZSH_HIGHLIGHT_STYLES[process-substitution-delimiter]="fg=$_comment"  # semantic.comment

# Options and arguments
ZSH_HIGHLIGHT_STYLES[single-hyphen-option]="fg=$_night"    # colors.night
ZSH_HIGHLIGHT_STYLES[double-hyphen-option]="fg=$_night"    # colors.night

# Redirections
ZSH_HIGHLIGHT_STYLES[redirection]="fg=$_comment"           # semantic.comment

# Comments
ZSH_HIGHLIGHT_STYLES[comment]="fg=$_comment"               # semantic.comment

# Default
ZSH_HIGHLIGHT_STYLES[default]="fg=$_text"                  # base.foreground

# zsh-autosuggestions
ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=$_comment"             # semantic.comment

# Cleanup temporary variables
unset _lantern _ember _amber _life _night _muted _cyan _text _comment _border
