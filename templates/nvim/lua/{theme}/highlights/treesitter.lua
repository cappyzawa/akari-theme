-- Treesitter highlights

local M = {}

function M.setup(p, config)
  return {
    -- Identifiers
    ["@variable"] = { fg = p.base.foreground },
    ["@variable.builtin"] = { fg = p.ansi.bright.red, italic = true },
    ["@variable.parameter"] = { fg = p.base.foreground, italic = true },
    ["@variable.parameter.builtin"] = { fg = p.base.foreground, italic = true },
    ["@variable.member"] = { fg = p.syntax.member },

    -- Constants
    ["@constant"] = { fg = p.syntax.constant },
    ["@constant.builtin"] = { fg = p.syntax.constant },
    ["@constant.macro"] = { fg = p.syntax.constant },

    -- Modules
    ["@module"] = { fg = p.syntax.namespace },
    ["@module.builtin"] = { fg = p.syntax.namespace },

    -- Labels
    ["@label"] = { fg = p.syntax.label },

    -- Strings
    ["@string"] = { fg = p.syntax.string },
    ["@string.documentation"] = { fg = p.syntax.string },
    ["@string.regexp"] = { fg = p.syntax.regexp },
    ["@string.escape"] = { fg = p.syntax.escape },
    ["@string.special"] = { fg = p.syntax.string },
    ["@string.special.symbol"] = { fg = p.ansi.bright.magenta },
    ["@string.special.path"] = { fg = p.syntax.path },
    ["@string.special.url"] = { fg = p.ui.link, underline = true },

    -- Characters
    ["@character"] = { fg = p.syntax.character },
    ["@character.special"] = { fg = p.ansi.bright.magenta },

    -- Booleans
    ["@boolean"] = { fg = p.syntax.constant, bold = true },

    -- Numbers
    ["@number"] = { fg = p.syntax.number },
    ["@number.float"] = { fg = p.syntax.number },

    -- Types
    ["@type"] = { fg = p.syntax.type },
    ["@type.builtin"] = { fg = p.syntax.type },
    ["@type.definition"] = { fg = p.syntax.type },

    -- Attributes
    ["@attribute"] = { fg = p.syntax.attribute },
    ["@attribute.builtin"] = { fg = p.syntax.attribute },

    -- Properties
    ["@property"] = { fg = p.syntax.member },

    -- Functions
    ["@function"] = vim.tbl_extend("force", { fg = p.syntax["function"] }, config.functionStyle),
    ["@function.builtin"] = { fg = p.ansi.bright.magenta },
    ["@function.call"] = vim.tbl_extend("force", { fg = p.syntax["function"] }, config.functionStyle),
    ["@function.macro"] = { fg = p.syntax.macro },
    ["@function.method"] = vim.tbl_extend("force", { fg = p.syntax["function"] }, config.functionStyle),
    ["@function.method.call"] = vim.tbl_extend("force", { fg = p.syntax["function"] }, config.functionStyle),

    -- Constructors
    ["@constructor"] = { fg = p.syntax.type },

    -- Operators
    ["@operator"] = { fg = p.base.foreground },

    -- Keywords
    ["@keyword"] = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    ["@keyword.coroutine"] = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    ["@keyword.function"] = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    ["@keyword.operator"] = { fg = p.syntax.keyword },
    ["@keyword.import"] = { fg = p.syntax.keyword },
    ["@keyword.type"] = { fg = p.syntax.keyword },
    ["@keyword.modifier"] = { fg = p.syntax.keyword },
    ["@keyword.repeat"] = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    ["@keyword.return"] = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    ["@keyword.debug"] = { fg = p.ansi.bright.red },
    ["@keyword.exception"] = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    ["@keyword.conditional"] = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    ["@keyword.conditional.ternary"] = { fg = p.syntax.keyword },
    ["@keyword.directive"] = { fg = p.syntax.macro },
    ["@keyword.directive.define"] = { fg = p.syntax.macro },
    ["@keyword.storage"] = { fg = p.syntax.keyword },

    -- Punctuation
    ["@punctuation.delimiter"] = { fg = p.base.foreground },
    ["@punctuation.bracket"] = { fg = p.base.foreground },
    ["@punctuation.special"] = { fg = p.syntax.punctuation_special },

    -- Comments
    ["@comment"] = vim.tbl_extend("force", { fg = p.syntax.comment }, config.commentStyle),
    ["@comment.documentation"] = vim.tbl_extend("force", { fg = p.syntax.comment }, config.commentStyle),
    ["@comment.error"] = { fg = p.diagnostic.error },
    ["@comment.warning"] = { fg = p.diagnostic.warning },
    ["@comment.note"] = { fg = p.diagnostic.info },
    ["@comment.todo"] = { fg = p.base.background, bg = p.ansi.blue, bold = true },

    -- Markup
    ["@markup.strong"] = { bold = true },
    ["@markup.italic"] = { italic = true },
    ["@markup.strikethrough"] = { strikethrough = true },
    ["@markup.underline"] = { underline = true },
    ["@markup.heading"] = { fg = p.markup.heading_1, bold = true },
    ["@markup.heading.1"] = { fg = p.markup.heading_1, bold = true },
    ["@markup.heading.2"] = { fg = p.markup.heading_2, bold = true },
    ["@markup.heading.3"] = { fg = p.markup.heading_3, bold = true },
    ["@markup.heading.4"] = { fg = p.markup.heading_4 },
    ["@markup.heading.5"] = { fg = p.syntax.comment, bold = true },
    ["@markup.heading.6"] = { fg = p.ansi.blue },
    ["@markup.heading.marker"] = { fg = p.syntax.comment },
    ["@markup.quote"] = { fg = p.syntax.comment, italic = true },
    ["@markup.math"] = { fg = p.ansi.bright.cyan },
    ["@markup.link"] = { fg = p.ansi.cyan },
    ["@markup.link.label"] = { fg = p.ansi.magenta },
    ["@markup.link.url"] = { fg = p.ui.link, underline = true },
    ["@markup.raw"] = { fg = p.markup.raw },
    ["@markup.raw.block"] = { fg = p.markup.raw },
    ["@markup.list"] = { fg = p.ansi.cyan },
    ["@markup.list.numbered"] = { fg = p.markup.list },
    ["@markup.list.unnumbered"] = { fg = p.ansi.cyan },
    ["@markup.list.checked"] = { fg = p.diagnostic.success },
    ["@markup.list.unchecked"] = { fg = p.ui.muted },

    -- Diff
    ["@diff.plus"] = { fg = p.diff.added },
    ["@diff.minus"] = { fg = p.diff.removed },
    ["@diff.delta"] = { fg = p.diff.changed },

    -- Tags (HTML, XML, JSX)
    ["@tag"] = { fg = p.syntax.tag },
    ["@tag.builtin"] = { fg = p.ansi.blue },
    ["@tag.attribute"] = { fg = p.syntax.attribute },
    ["@tag.delimiter"] = { fg = p.base.foreground },

    -- Non-standard captures (for specific languages)
    ["@namespace"] = { fg = p.syntax.namespace },
    ["@symbol"] = { fg = p.ansi.bright.magenta },
    ["@annotation"] = { fg = p.syntax.decorator },
    ["@conceal"] = { fg = p.ui.muted },

    -- Text (legacy captures, kept for compatibility)
    ["@text"] = { fg = p.base.foreground },
    ["@text.strong"] = { bold = true },
    ["@text.emphasis"] = { italic = true },
    ["@text.underline"] = { underline = true },
    ["@text.strike"] = { strikethrough = true },
    ["@text.title"] = { fg = p.markup.heading_1, bold = true },
    ["@text.title.1"] = { fg = p.markup.heading_1, bold = true },
    ["@text.title.2"] = { fg = p.markup.heading_2, bold = true },
    ["@text.title.3"] = { fg = p.markup.heading_3, bold = true },
    ["@text.title.4"] = { fg = p.markup.heading_4 },
    ["@text.title.5"] = { fg = p.syntax.comment, bold = true },
    ["@text.title.6"] = { fg = p.ansi.blue },
    ["@text.literal"] = { fg = p.markup.raw },
    ["@text.uri"] = { fg = p.ui.link, underline = true },
    ["@text.math"] = { fg = p.ansi.bright.cyan },
    ["@text.reference"] = { fg = p.ansi.magenta },
    ["@text.todo"] = { fg = p.base.background, bg = p.ansi.blue, bold = true },
    ["@text.note"] = { fg = p.diagnostic.info },
    ["@text.warning"] = { fg = p.diagnostic.warning },
    ["@text.danger"] = { fg = p.diagnostic.error },
    ["@text.diff.add"] = { fg = p.diff.added },
    ["@text.diff.delete"] = { fg = p.diff.removed },
  }
end

return M
