# Akari Dawn — zsh-syntax-highlighting configuration
# A color palette inspired by Japanese alleys lit by round lanterns.

# Declare the associative array
typeset -A ZSH_HIGHLIGHT_STYLES

# Palette
_lantern='#8A4530'
_ember='#7A3828'
_amber='#B07840'
_life='#3A5830'
_night='#304050'
_muted='#806080'
_cyan='#305858'
_constant='#406868'
_path='#70A868'
_text='#1A1816'
_comment='#4A4642'
_border='#C4BEB6'

# Commands
ZSH_HIGHLIGHT_STYLES[command]="fg=$_lantern,bold"
ZSH_HIGHLIGHT_STYLES[builtin]="fg=$_lantern,bold"
ZSH_HIGHLIGHT_STYLES[alias]="fg=$_lantern"
ZSH_HIGHLIGHT_STYLES[function]="fg=$_muted"

# Errors and unknown
ZSH_HIGHLIGHT_STYLES[unknown-token]="fg=$_comment"
ZSH_HIGHLIGHT_STYLES[reserved-word]="fg=$_night"

# Paths and files
ZSH_HIGHLIGHT_STYLES[path]="fg=$_path,underline"
ZSH_HIGHLIGHT_STYLES[path_pathseparator]="fg=$_path"
ZSH_HIGHLIGHT_STYLES[globbing]="fg=$_comment"

# Strings and quotes
ZSH_HIGHLIGHT_STYLES[single-quoted-argument]="fg=$_life"
ZSH_HIGHLIGHT_STYLES[double-quoted-argument]="fg=$_life"
ZSH_HIGHLIGHT_STYLES[dollar-quoted-argument]="fg=$_life"

# Variables and substitutions
ZSH_HIGHLIGHT_STYLES[assign]="fg=$_text"
ZSH_HIGHLIGHT_STYLES[named-fd]="fg=$_cyan"
ZSH_HIGHLIGHT_STYLES[command-substitution]="fg=$_muted"
ZSH_HIGHLIGHT_STYLES[command-substitution-delimiter]="fg=$_comment"
ZSH_HIGHLIGHT_STYLES[process-substitution]="fg=$_muted"
ZSH_HIGHLIGHT_STYLES[process-substitution-delimiter]="fg=$_comment"

# Options and arguments
ZSH_HIGHLIGHT_STYLES[single-hyphen-option]="fg=$_night"
ZSH_HIGHLIGHT_STYLES[double-hyphen-option]="fg=$_night"

# Redirections
ZSH_HIGHLIGHT_STYLES[redirection]="fg=$_comment"

# Comments
ZSH_HIGHLIGHT_STYLES[comment]="fg=$_comment"

# Default
ZSH_HIGHLIGHT_STYLES[default]="fg=$_text"

# zsh-autosuggestions
ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=$_comment"

# Cleanup temporary variables
unset _lantern _ember _amber _life _night _muted _cyan _constant _path _text _comment _border
