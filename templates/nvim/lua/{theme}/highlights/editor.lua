-- Editor UI highlights

local M = {}

function M.setup(p, config)
  local bg = config.transparent and "NONE" or p.base.background

  return {
    -- Basic UI
    Normal = { fg = p.base.foreground, bg = bg },
    NormalNC = { fg = p.base.foreground, bg = bg },
    NormalFloat = { fg = p.base.foreground, bg = p.ansi.black },
    FloatBorder = { fg = p.ansi.bright.black, bg = p.ansi.black },
    FloatTitle = { fg = p.ui.accent, bg = p.ansi.black },

    -- Cursor
    Cursor = { fg = p.ui.cursor_text, bg = p.ui.cursor },
    lCursor = { link = "Cursor" },
    CursorIM = { link = "Cursor" },
    TermCursor = { link = "Cursor" },
    TermCursorNC = { fg = p.base.foreground, bg = p.ansi.bright.black },

    -- Cursor line/column
    CursorLine = { bg = p.ansi.black },
    CursorColumn = { bg = p.ansi.black },
    ColorColumn = { bg = p.ansi.black },

    -- Line numbers
    LineNr = { fg = p.ansi.bright.black },
    CursorLineNr = { fg = p.ui.accent },
    LineNrAbove = { fg = p.ansi.bright.black },
    LineNrBelow = { fg = p.ansi.bright.black },
    SignColumn = { fg = p.base.foreground, bg = bg },
    FoldColumn = { fg = p.ansi.bright.black, bg = bg },

    -- Selection
    Visual = { bg = p.ui.selection_bg },
    VisualNOS = { bg = p.ui.selection_bg },

    -- Search
    Search = { fg = p.base.foreground, bg = p.ui.selection_bg },
    IncSearch = { fg = p.ui.on_accent, bg = p.ui.accent },
    CurSearch = { fg = p.ui.on_accent, bg = p.ui.accent },
    Substitute = { fg = p.ui.cursor_text, bg = p.ansi.bright.red },

    -- Statusline
    StatusLine = { fg = p.base.foreground, bg = p.ansi.black },
    StatusLineNC = { fg = p.ansi.bright.black, bg = p.ansi.black },
    WinBar = { fg = p.base.foreground, bg = bg },
    WinBarNC = { fg = p.ansi.bright.black, bg = bg },

    -- Tabline
    TabLine = { fg = p.ansi.bright.black, bg = p.ansi.black },
    TabLineFill = { bg = p.ansi.black },
    TabLineSel = { fg = p.base.foreground, bg = bg },

    -- Window separators
    WinSeparator = { fg = p.ansi.bright.black },
    VertSplit = { fg = p.ansi.bright.black },

    -- Popup menu
    Pmenu = { fg = p.base.foreground, bg = p.ansi.black },
    PmenuSel = { fg = p.ui.selection_fg, bg = p.ui.selection_bg },
    PmenuSbar = { bg = p.ansi.black },
    PmenuThumb = { bg = p.ansi.bright.black },
    PmenuKind = { fg = p.ui.accent, bg = p.ansi.black },
    PmenuKindSel = { fg = p.ui.accent, bg = p.ui.selection_bg },
    PmenuExtra = { fg = p.ui.muted, bg = p.ansi.black },
    PmenuExtraSel = { fg = p.ui.muted, bg = p.ui.selection_bg },

    -- Messages
    ModeMsg = { fg = p.base.foreground },
    MsgArea = { fg = p.base.foreground },
    MoreMsg = { fg = p.ansi.green },
    WarningMsg = { fg = p.diagnostic.warning },
    ErrorMsg = { fg = p.diagnostic.error, bold = true },

    -- Folds
    Folded = { fg = p.ui.muted, bg = p.ansi.black },

    -- Diff
    DiffAdd = { fg = p.diff.added, bg = p.ansi.black },
    DiffChange = { fg = p.diff.changed, bg = p.ansi.black },
    DiffDelete = { fg = p.diff.removed, bg = p.ansi.black },
    DiffText = { fg = p.diff.changed, bg = p.ui.selection_bg },
    diffAdded = { fg = p.diff.added },
    diffRemoved = { fg = p.diff.removed },
    diffChanged = { fg = p.diff.changed },
    diffOldFile = { fg = p.ansi.bright.red },
    diffNewFile = { fg = p.ansi.bright.green },
    diffFile = { fg = p.ansi.bright.blue },
    diffLine = { fg = p.ui.muted },
    diffIndexLine = { fg = p.ansi.bright.magenta },

    -- Spelling
    SpellBad = { sp = p.diagnostic.error, undercurl = config.undercurl },
    SpellCap = { sp = p.diagnostic.warning, undercurl = config.undercurl },
    SpellLocal = { sp = p.diagnostic.info, undercurl = config.undercurl },
    SpellRare = { sp = p.diagnostic.hint, undercurl = config.undercurl },

    -- Misc
    Conceal = { fg = p.ui.muted },
    Directory = { fg = p.ansi.bright.cyan },
    EndOfBuffer = { fg = p.base.background },
    MatchParen = { fg = p.ui.accent, bold = true },
    NonText = { fg = p.ansi.bright.black },
    Question = { fg = p.ansi.green },
    QuickFixLine = { bg = p.ui.selection_bg },
    SpecialKey = { fg = p.ansi.bright.black },
    Title = { fg = p.markup.heading_1, bold = true },
    Whitespace = { fg = p.ansi.bright.black },
    WildMenu = { fg = p.ui.selection_fg, bg = p.ui.selection_bg },

    -- Diagnostics
    DiagnosticError = { fg = p.diagnostic.error, bold = true },
    DiagnosticWarn = { fg = p.diagnostic.warning },
    DiagnosticInfo = { fg = p.diagnostic.info },
    DiagnosticHint = { fg = p.diagnostic.hint },
    DiagnosticOk = { fg = p.diagnostic.success },

    DiagnosticVirtualTextError = { fg = p.diagnostic.error },
    DiagnosticVirtualTextWarn = { fg = p.diagnostic.warning },
    DiagnosticVirtualTextInfo = { fg = p.diagnostic.info },
    DiagnosticVirtualTextHint = { fg = p.diagnostic.hint },
    DiagnosticVirtualTextOk = { fg = p.diagnostic.success },

    DiagnosticUnderlineError = { sp = p.diagnostic.error, undercurl = config.undercurl },
    DiagnosticUnderlineWarn = { sp = p.diagnostic.warning, undercurl = config.undercurl },
    DiagnosticUnderlineInfo = { sp = p.diagnostic.info, undercurl = config.undercurl },
    DiagnosticUnderlineHint = { sp = p.diagnostic.hint, undercurl = config.undercurl },
    DiagnosticUnderlineOk = { sp = p.diagnostic.success, undercurl = config.undercurl },

    DiagnosticFloatingError = { fg = p.diagnostic.error },
    DiagnosticFloatingWarn = { fg = p.diagnostic.warning },
    DiagnosticFloatingInfo = { fg = p.diagnostic.info },
    DiagnosticFloatingHint = { fg = p.diagnostic.hint },
    DiagnosticFloatingOk = { fg = p.diagnostic.success },

    DiagnosticSignError = { fg = p.diagnostic.error },
    DiagnosticSignWarn = { fg = p.diagnostic.warning },
    DiagnosticSignInfo = { fg = p.diagnostic.info },
    DiagnosticSignHint = { fg = p.diagnostic.hint },
    DiagnosticSignOk = { fg = p.diagnostic.success },

    DiagnosticUnnecessary = { fg = p.ui.muted },
    DiagnosticDeprecated = { strikethrough = true },
  }
end

return M
