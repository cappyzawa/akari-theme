# zsh-syntax-highlighting configuration
# Color scheme: Akari (Japanese alleys lit by round lanterns)

# Declare the associative array
typeset -A ZSH_HIGHLIGHT_STYLES

# Akari palette
_lantern='#E26A3B'     # round lantern light
_ember='#D65A3A'       # heat / material
_life='#7FAF6A'        # brighter life green
_night='#5A6F82'       # night air
_muted='#7C6A8A'       # muted purple
_cyan='#6F8F8A'
_text='#E6DED3'        # soft paper white
_comment='#7A7A75'     # readable warm gray
_overlay='#3A3530'     # selection / overlay

# Commands
ZSH_HIGHLIGHT_STYLES[command]="fg=$_lantern,bold"
ZSH_HIGHLIGHT_STYLES[builtin]="fg=$_lantern,bold"
ZSH_HIGHLIGHT_STYLES[alias]="fg=$_lantern"
ZSH_HIGHLIGHT_STYLES[function]="fg=$_lantern"

# Errors and unknown
ZSH_HIGHLIGHT_STYLES[unknown-token]="fg=$_comment"
ZSH_HIGHLIGHT_STYLES[reserved-word]="fg=$_night"

# Paths and files
ZSH_HIGHLIGHT_STYLES[path]="fg=$_lantern,underline"
ZSH_HIGHLIGHT_STYLES[path_pathseparator]="fg=$_lantern"
ZSH_HIGHLIGHT_STYLES[globbing]="fg=$_comment"

# Strings and quotes
ZSH_HIGHLIGHT_STYLES[single-quoted-argument]="fg=$_lantern"
ZSH_HIGHLIGHT_STYLES[double-quoted-argument]="fg=$_lantern"
ZSH_HIGHLIGHT_STYLES[dollar-quoted-argument]="fg=$_lantern"

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
unset _lantern _ember _life _night _muted _cyan _text _comment _overlay
