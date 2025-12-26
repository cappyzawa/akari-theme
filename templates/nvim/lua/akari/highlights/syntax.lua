-- Standard syntax highlights

local M = {}

function M.setup(p, config)
  return {
    -- Comments
    Comment = vim.tbl_extend("force", { fg = p.comment }, config.commentStyle),

    -- Constants
    Constant = { fg = p.constant },
    String = { fg = p.green },
    Character = { fg = p.lantern },
    Number = { fg = p.constant },
    Boolean = { fg = p.constant, bold = true },
    Float = { fg = p.constant },

    -- Identifiers
    Identifier = { fg = p.foreground },
    Function = vim.tbl_extend("force", { fg = p.magenta }, config.functionStyle),

    -- Statements
    Statement = { fg = p.lantern },
    Conditional = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    Repeat = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    Label = { fg = p.amber },
    Operator = { fg = p.foreground },
    Keyword = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    Exception = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),

    -- Preprocessor
    PreProc = { fg = p.bright_magenta },
    Include = { fg = p.lantern },
    Define = { fg = p.bright_magenta },
    Macro = { fg = p.bright_magenta },
    PreCondit = { fg = p.bright_magenta },

    -- Types
    Type = { fg = p.amber },
    StorageClass = { fg = p.lantern },
    Structure = { fg = p.amber },
    Typedef = { fg = p.amber },

    -- Special
    Special = { fg = p.bright_yellow },
    SpecialChar = { fg = p.bright_magenta },
    Tag = { fg = p.lantern },
    Delimiter = { fg = p.foreground },
    SpecialComment = { fg = p.comment, italic = true },
    Debug = { fg = p.bright_red },

    -- Underlined
    Underlined = { underline = true },

    -- Ignore
    Ignore = { fg = p.comment },

    -- Error
    Error = { fg = p.error, bold = true },

    -- Todo
    Todo = { fg = p.background, bg = p.blue, bold = true },

    -- Markup (for markdown, etc.)
    htmlH1 = { fg = p.lantern, bold = true },
    htmlH2 = { fg = p.bright_yellow, bold = true },
    htmlH3 = { fg = p.lantern },
    htmlH4 = { fg = p.bright_yellow },
    htmlH5 = { fg = p.lantern },
    htmlH6 = { fg = p.bright_yellow },
    htmlBold = { bold = true },
    htmlItalic = { italic = true },
    htmlLink = { fg = p.bright_blue, underline = true },
    htmlTag = { fg = p.lantern },
    htmlTagName = { fg = p.lantern },
    htmlEndTag = { fg = p.lantern },
    htmlArg = { fg = p.bright_cyan },
    htmlSpecialChar = { fg = p.bright_magenta },

    markdownH1 = { fg = p.lantern, bold = true },
    markdownH2 = { fg = p.bright_yellow, bold = true },
    markdownH3 = { fg = p.lantern },
    markdownH4 = { fg = p.bright_yellow },
    markdownH5 = { fg = p.lantern },
    markdownH6 = { fg = p.bright_yellow },
    markdownCode = { fg = p.bright_green },
    markdownCodeBlock = { fg = p.bright_green },
    markdownCodeDelimiter = { fg = p.bright_green },
    markdownBlockquote = { fg = p.comment, italic = true },
    markdownListMarker = { fg = p.cyan },
    markdownOrderedListMarker = { fg = p.cyan },
    markdownRule = { fg = p.bright_black },
    markdownHeadingRule = { fg = p.bright_black },
    markdownUrlDelimiter = { fg = p.foreground },
    markdownLinkDelimiter = { fg = p.foreground },
    markdownLinkTextDelimiter = { fg = p.foreground },
    markdownHeadingDelimiter = { fg = p.lantern },
    markdownUrl = { fg = p.bright_blue, underline = true },
    markdownUrlTitleDelimiter = { fg = p.foreground },
    markdownLinkText = { fg = p.cyan },
    markdownIdDeclaration = { fg = p.cyan },
    markdownBold = { bold = true },
    markdownItalic = { italic = true },
    markdownBoldItalic = { bold = true, italic = true },
  }
end

return M
