-- Standard syntax highlights

local M = {}

function M.setup(p, config)
  return {
    -- Comments
    Comment = vim.tbl_extend("force", { fg = p.syntax.comment }, config.commentStyle),

    -- Constants
    Constant = { fg = p.syntax.constant },
    String = { fg = p.syntax.string },
    Character = { fg = p.syntax.character },
    Number = { fg = p.syntax.number },
    Boolean = { fg = p.syntax.constant, bold = true },
    Float = { fg = p.syntax.number },

    -- Identifiers
    Identifier = { fg = p.base.foreground },
    Function = vim.tbl_extend("force", { fg = p.syntax["function"] }, config.functionStyle),

    -- Statements
    Statement = { fg = p.syntax.keyword },
    Conditional = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    Repeat = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    Label = { fg = p.syntax.label },
    Operator = { fg = p.base.foreground },
    Keyword = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    Exception = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),

    -- Preprocessor
    PreProc = { fg = p.syntax.macro },
    Include = { fg = p.syntax.keyword },
    Define = { fg = p.syntax.macro },
    Macro = { fg = p.syntax.macro },
    PreCondit = { fg = p.syntax.macro },

    -- Types
    Type = { fg = p.syntax.type },
    StorageClass = { fg = p.syntax.keyword },
    Structure = { fg = p.syntax.type },
    Typedef = { fg = p.syntax.type },

    -- Special
    Special = { fg = p.ansi.bright.yellow },
    SpecialChar = { fg = p.syntax.escape },
    Tag = { fg = p.syntax.tag },
    Delimiter = { fg = p.base.foreground },
    SpecialComment = { fg = p.syntax.comment, italic = true },
    Debug = { fg = p.ansi.bright.red },

    -- Underlined
    Underlined = { underline = true },

    -- Ignore
    Ignore = { fg = p.ui.muted },

    -- Error
    Error = { fg = p.diagnostic.error, bold = true },

    -- Todo
    Todo = { fg = p.base.background, bg = p.ansi.blue, bold = true },

    -- Markup (for markdown, etc.)
    htmlH1 = { fg = p.markup.heading_1, bold = true },
    htmlH2 = { fg = p.ansi.bright.yellow, bold = true },
    htmlH3 = { fg = p.markup.heading_3 },
    htmlH4 = { fg = p.ansi.bright.yellow },
    htmlH5 = { fg = p.markup.heading_4 },
    htmlH6 = { fg = p.ansi.bright.yellow },
    htmlBold = { bold = true },
    htmlItalic = { italic = true },
    htmlLink = { fg = p.ui.link, underline = true },
    htmlTag = { fg = p.syntax.tag },
    htmlTagName = { fg = p.syntax.tag },
    htmlEndTag = { fg = p.syntax.tag },
    htmlArg = { fg = p.ansi.bright.cyan },
    htmlSpecialChar = { fg = p.syntax.escape },

    markdownH1 = { fg = p.markup.heading_1, bold = true },
    markdownH2 = { fg = p.ansi.bright.yellow, bold = true },
    markdownH3 = { fg = p.markup.heading_3 },
    markdownH4 = { fg = p.ansi.bright.yellow },
    markdownH5 = { fg = p.markup.heading_4 },
    markdownH6 = { fg = p.ansi.bright.yellow },
    markdownCode = { fg = p.markup.raw },
    markdownCodeBlock = { fg = p.markup.raw },
    markdownCodeDelimiter = { fg = p.markup.raw },
    markdownBlockquote = { fg = p.syntax.comment, italic = true },
    markdownListMarker = { fg = p.ansi.cyan },
    markdownOrderedListMarker = { fg = p.ansi.cyan },
    markdownRule = { fg = p.ansi.bright.black },
    markdownHeadingRule = { fg = p.ansi.bright.black },
    markdownUrlDelimiter = { fg = p.base.foreground },
    markdownLinkDelimiter = { fg = p.base.foreground },
    markdownLinkTextDelimiter = { fg = p.base.foreground },
    markdownHeadingDelimiter = { fg = p.markup.heading_1 },
    markdownUrl = { fg = p.ui.link, underline = true },
    markdownUrlTitleDelimiter = { fg = p.base.foreground },
    markdownLinkText = { fg = p.ansi.cyan },
    markdownIdDeclaration = { fg = p.ansi.cyan },
    markdownBold = { bold = true },
    markdownItalic = { italic = true },
    markdownBoldItalic = { bold = true, italic = true },
  }
end

return M
