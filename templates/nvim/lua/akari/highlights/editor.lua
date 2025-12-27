-- Editor UI highlights

local M = {}

function M.setup(p, config)
  local bg = config.transparent and p.none or p.background

  return {
    -- Basic UI
    Normal = { fg = p.foreground, bg = bg },
    NormalNC = { fg = p.foreground, bg = bg },
    NormalFloat = { fg = p.foreground, bg = p.black },
    FloatBorder = { fg = p.bright_black, bg = p.black },
    FloatTitle = { fg = p.lantern, bg = p.black },

    -- Cursor
    Cursor = { fg = p.cursor_text, bg = p.cursor },
    lCursor = { link = "Cursor" },
    CursorIM = { link = "Cursor" },
    TermCursor = { link = "Cursor" },
    TermCursorNC = { fg = p.foreground, bg = p.bright_black },

    -- Cursor line/column
    CursorLine = { bg = p.black },
    CursorColumn = { bg = p.black },
    ColorColumn = { bg = p.black },

    -- Line numbers
    LineNr = { fg = p.bright_black },
    CursorLineNr = { fg = p.lantern },
    LineNrAbove = { fg = p.bright_black },
    LineNrBelow = { fg = p.bright_black },
    SignColumn = { fg = p.foreground, bg = bg },
    FoldColumn = { fg = p.bright_black, bg = bg },

    -- Selection
    Visual = { bg = p.selection_bg },
    VisualNOS = { bg = p.selection_bg },

    -- Search
    Search = { fg = p.foreground, bg = p.selection_bg },
    IncSearch = { fg = p.cursor_text, bg = p.lantern },
    CurSearch = { fg = p.cursor_text, bg = p.lantern },
    Substitute = { fg = p.cursor_text, bg = p.bright_red },

    -- Statusline
    StatusLine = { fg = p.foreground, bg = p.black },
    StatusLineNC = { fg = p.bright_black, bg = p.black },
    WinBar = { fg = p.foreground, bg = bg },
    WinBarNC = { fg = p.bright_black, bg = bg },

    -- Tabline
    TabLine = { fg = p.bright_black, bg = p.black },
    TabLineFill = { bg = p.black },
    TabLineSel = { fg = p.foreground, bg = bg },

    -- Window separators
    WinSeparator = { fg = p.bright_black },
    VertSplit = { fg = p.bright_black },

    -- Popup menu
    Pmenu = { fg = p.foreground, bg = p.black },
    PmenuSel = { fg = p.selection_fg, bg = p.selection_bg },
    PmenuSbar = { bg = p.black },
    PmenuThumb = { bg = p.bright_black },
    PmenuKind = { fg = p.lantern, bg = p.black },
    PmenuKindSel = { fg = p.lantern, bg = p.selection_bg },
    PmenuExtra = { fg = p.comment, bg = p.black },
    PmenuExtraSel = { fg = p.comment, bg = p.selection_bg },

    -- Messages
    ModeMsg = { fg = p.foreground },
    MsgArea = { fg = p.foreground },
    MoreMsg = { fg = p.green },
    WarningMsg = { fg = p.warning },
    ErrorMsg = { fg = p.error, bold = true },

    -- Folds
    Folded = { fg = p.comment, bg = p.black },

    -- Diff
    DiffAdd = { fg = p.diff_add, bg = p.black },
    DiffChange = { fg = p.diff_change, bg = p.black },
    DiffDelete = { fg = p.diff_delete, bg = p.black },
    DiffText = { fg = p.lantern, bg = p.selection_bg },
    diffAdded = { fg = p.diff_add },
    diffRemoved = { fg = p.diff_delete },
    diffChanged = { fg = p.diff_change },
    diffOldFile = { fg = p.bright_red },
    diffNewFile = { fg = p.bright_green },
    diffFile = { fg = p.bright_blue },
    diffLine = { fg = p.comment },
    diffIndexLine = { fg = p.bright_magenta },

    -- Spelling
    SpellBad = { sp = p.error, undercurl = config.undercurl },
    SpellCap = { sp = p.warning, undercurl = config.undercurl },
    SpellLocal = { sp = p.info, undercurl = config.undercurl },
    SpellRare = { sp = p.hint, undercurl = config.undercurl },

    -- Misc
    Conceal = { fg = p.comment },
    Directory = { fg = p.bright_cyan },
    EndOfBuffer = { fg = p.background },
    MatchParen = { fg = p.lantern, bold = true },
    NonText = { fg = p.bright_black },
    Question = { fg = p.green },
    QuickFixLine = { bg = p.selection_bg },
    SpecialKey = { fg = p.bright_black },
    Title = { fg = p.lantern, bold = true },
    Whitespace = { fg = p.bright_black },
    WildMenu = { fg = p.selection_fg, bg = p.selection_bg },

    -- Diagnostics
    DiagnosticError = { fg = p.error, bold = true },
    DiagnosticWarn = { fg = p.warning },
    DiagnosticInfo = { fg = p.info },
    DiagnosticHint = { fg = p.hint },
    DiagnosticOk = { fg = p.green },

    DiagnosticVirtualTextError = { fg = p.error },
    DiagnosticVirtualTextWarn = { fg = p.warning },
    DiagnosticVirtualTextInfo = { fg = p.info },
    DiagnosticVirtualTextHint = { fg = p.hint },
    DiagnosticVirtualTextOk = { fg = p.green },

    DiagnosticUnderlineError = { sp = p.error, undercurl = config.undercurl },
    DiagnosticUnderlineWarn = { sp = p.warning, undercurl = config.undercurl },
    DiagnosticUnderlineInfo = { sp = p.info, undercurl = config.undercurl },
    DiagnosticUnderlineHint = { sp = p.hint, undercurl = config.undercurl },
    DiagnosticUnderlineOk = { sp = p.green, undercurl = config.undercurl },

    DiagnosticFloatingError = { fg = p.error },
    DiagnosticFloatingWarn = { fg = p.warning },
    DiagnosticFloatingInfo = { fg = p.info },
    DiagnosticFloatingHint = { fg = p.hint },
    DiagnosticFloatingOk = { fg = p.green },

    DiagnosticSignError = { fg = p.error },
    DiagnosticSignWarn = { fg = p.warning },
    DiagnosticSignInfo = { fg = p.info },
    DiagnosticSignHint = { fg = p.hint },
    DiagnosticSignOk = { fg = p.green },

    DiagnosticUnnecessary = { fg = p.comment },
    DiagnosticDeprecated = { strikethrough = true },
  }
end

return M
