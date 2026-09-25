# Akari Night — zsh-syntax-highlighting configuration
# A terminal color palette inspired by Japanese alleys lit by round lanterns.

# Declare the associative array
typeset -A ZSH_HIGHLIGHT_STYLES

# Palette
_keyword='#E26A3B'
_function='#8E7BA0'
_builtin='#D65A3A'
_string='#7FAF6A'
_variable='#E6DED3'
_cyan='#6F8F8A'
_path='#7FAF6A'
_text='#E6DED3'
_comment='#9BABB9'
_muted='#9BABB9'

# Commands
ZSH_HIGHLIGHT_STYLES[command]="fg=$_function,bold"
ZSH_HIGHLIGHT_STYLES[builtin]="fg=$_builtin,bold"
ZSH_HIGHLIGHT_STYLES[alias]="fg=$_function"
ZSH_HIGHLIGHT_STYLES[function]="fg=$_function"

# Errors and unknown
ZSH_HIGHLIGHT_STYLES[unknown-token]="fg=$_comment"
ZSH_HIGHLIGHT_STYLES[reserved-word]="fg=$_keyword"

# Paths and files
ZSH_HIGHLIGHT_STYLES[path]="fg=$_path,underline"
ZSH_HIGHLIGHT_STYLES[path_pathseparator]="fg=$_path"
ZSH_HIGHLIGHT_STYLES[globbing]="fg=$_muted"

# Strings and quotes
ZSH_HIGHLIGHT_STYLES[single-quoted-argument]="fg=$_string"
ZSH_HIGHLIGHT_STYLES[double-quoted-argument]="fg=$_string"
ZSH_HIGHLIGHT_STYLES[dollar-quoted-argument]="fg=$_string"

# Variables and substitutions
ZSH_HIGHLIGHT_STYLES[assign]="fg=$_variable"
ZSH_HIGHLIGHT_STYLES[named-fd]="fg=$_cyan"
ZSH_HIGHLIGHT_STYLES[command-substitution]="fg=$_function"
ZSH_HIGHLIGHT_STYLES[command-substitution-delimiter]="fg=$_comment"
ZSH_HIGHLIGHT_STYLES[process-substitution]="fg=$_function"
ZSH_HIGHLIGHT_STYLES[process-substitution-delimiter]="fg=$_comment"

# Options and arguments
ZSH_HIGHLIGHT_STYLES[single-hyphen-option]="fg=$_variable"
ZSH_HIGHLIGHT_STYLES[double-hyphen-option]="fg=$_variable"

# Redirections
ZSH_HIGHLIGHT_STYLES[redirection]="fg=$_comment"

# Comments
ZSH_HIGHLIGHT_STYLES[comment]="fg=$_comment"

# Default
ZSH_HIGHLIGHT_STYLES[default]="fg=$_text"

# zsh-autosuggestions
ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=$_muted"

# Cleanup temporary variables
unset _keyword _function _builtin _string _variable _cyan _path _text _comment _muted
