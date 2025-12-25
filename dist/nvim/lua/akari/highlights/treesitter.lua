-- Treesitter highlights

local M = {}

function M.setup(p, config)
  return {
    -- Identifiers
    ["@variable"] = { fg = p.foreground },
    ["@variable.builtin"] = { fg = p.bright_red, italic = true },
    ["@variable.parameter"] = { fg = p.foreground, italic = true },
    ["@variable.parameter.builtin"] = { fg = p.foreground, italic = true },
    ["@variable.member"] = { fg = p.foreground },

    -- Constants
    ["@constant"] = { fg = p.constant },
    ["@constant.builtin"] = { fg = p.constant },
    ["@constant.macro"] = { fg = p.constant },

    -- Modules
    ["@module"] = { fg = p.amber },
    ["@module.builtin"] = { fg = p.amber },

    -- Labels
    ["@label"] = { fg = p.amber },

    -- Strings
    ["@string"] = { fg = p.green },
    ["@string.documentation"] = { fg = p.green },
    ["@string.regexp"] = { fg = p.bright_green },
    ["@string.escape"] = { fg = p.bright_magenta },
    ["@string.special"] = { fg = p.green },
    ["@string.special.symbol"] = { fg = p.bright_magenta },
    ["@string.special.path"] = { fg = p.bright_green },
    ["@string.special.url"] = { fg = p.bright_blue, underline = true },

    -- Characters
    ["@character"] = { fg = p.lantern },
    ["@character.special"] = { fg = p.bright_magenta },

    -- Booleans
    ["@boolean"] = { fg = p.constant, bold = true },

    -- Numbers
    ["@number"] = { fg = p.constant },
    ["@number.float"] = { fg = p.constant },

    -- Types
    ["@type"] = { fg = p.amber },
    ["@type.builtin"] = { fg = p.yellow },
    ["@type.definition"] = { fg = p.amber },

    -- Attributes
    ["@attribute"] = { fg = p.amber },
    ["@attribute.builtin"] = { fg = p.amber },

    -- Properties
    ["@property"] = { fg = p.foreground },

    -- Functions
    ["@function"] = vim.tbl_extend("force", { fg = p.magenta }, config.functionStyle),
    ["@function.builtin"] = { fg = p.bright_magenta },
    ["@function.call"] = vim.tbl_extend("force", { fg = p.magenta }, config.functionStyle),
    ["@function.macro"] = { fg = p.bright_magenta },
    ["@function.method"] = vim.tbl_extend("force", { fg = p.magenta }, config.functionStyle),
    ["@function.method.call"] = vim.tbl_extend("force", { fg = p.magenta }, config.functionStyle),

    -- Constructors
    ["@constructor"] = { fg = p.amber },

    -- Operators
    ["@operator"] = { fg = p.foreground },

    -- Keywords
    ["@keyword"] = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    ["@keyword.coroutine"] = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    ["@keyword.function"] = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    ["@keyword.operator"] = { fg = p.lantern },
    ["@keyword.import"] = { fg = p.lantern },
    ["@keyword.type"] = { fg = p.lantern },
    ["@keyword.modifier"] = { fg = p.lantern },
    ["@keyword.repeat"] = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    ["@keyword.return"] = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    ["@keyword.debug"] = { fg = p.bright_red },
    ["@keyword.exception"] = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    ["@keyword.conditional"] = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    ["@keyword.conditional.ternary"] = { fg = p.lantern },
    ["@keyword.directive"] = { fg = p.bright_magenta },
    ["@keyword.directive.define"] = { fg = p.bright_magenta },
    ["@keyword.storage"] = { fg = p.lantern },

    -- Punctuation
    ["@punctuation.delimiter"] = { fg = p.foreground },
    ["@punctuation.bracket"] = { fg = p.foreground },
    ["@punctuation.special"] = { fg = p.amber },

    -- Comments
    ["@comment"] = vim.tbl_extend("force", { fg = p.comment }, config.commentStyle),
    ["@comment.documentation"] = vim.tbl_extend("force", { fg = p.comment }, config.commentStyle),
    ["@comment.error"] = { fg = p.error },
    ["@comment.warning"] = { fg = p.warning },
    ["@comment.note"] = { fg = p.info },
    ["@comment.todo"] = { fg = p.background, bg = p.blue, bold = true },

    -- Markup
    ["@markup.strong"] = { bold = true },
    ["@markup.italic"] = { italic = true },
    ["@markup.strikethrough"] = { strikethrough = true },
    ["@markup.underline"] = { underline = true },
    ["@markup.heading"] = { fg = p.lantern, bold = true },
    ["@markup.heading.1"] = { fg = p.lantern, bold = true },
    ["@markup.heading.2"] = { fg = p.amber, bold = true },
    ["@markup.heading.3"] = { fg = p.ember, bold = true },
    ["@markup.heading.4"] = { fg = p.amber },
    ["@markup.heading.5"] = { fg = p.comment, bold = true },
    ["@markup.heading.6"] = { fg = p.blue },
    ["@markup.heading.marker"] = { fg = p.comment },
    ["@markup.quote"] = { fg = p.comment, italic = true },
    ["@markup.math"] = { fg = p.bright_cyan },
    ["@markup.link"] = { fg = p.cyan },
    ["@markup.link.label"] = { fg = p.magenta },
    ["@markup.link.url"] = { fg = p.bright_blue, underline = true },
    ["@markup.raw"] = { fg = p.bright_green },
    ["@markup.raw.block"] = { fg = p.bright_green },
    ["@markup.list"] = { fg = p.cyan },
    ["@markup.list.numbered"] = { fg = p.lantern },
    ["@markup.list.unnumbered"] = { fg = p.cyan },
    ["@markup.list.checked"] = { fg = p.green },
    ["@markup.list.unchecked"] = { fg = p.comment },

    -- Diff
    ["@diff.plus"] = { fg = p.diff_add },
    ["@diff.minus"] = { fg = p.diff_delete },
    ["@diff.delta"] = { fg = p.diff_change },

    -- Tags (HTML, XML, JSX)
    ["@tag"] = { fg = p.lantern },
    ["@tag.builtin"] = { fg = p.blue },
    ["@tag.attribute"] = { fg = p.amber },
    ["@tag.delimiter"] = { fg = p.foreground },

    -- Non-standard captures (for specific languages)
    ["@namespace"] = { fg = p.amber },
    ["@symbol"] = { fg = p.bright_magenta },
    ["@annotation"] = { fg = p.bright_yellow },
    ["@conceal"] = { fg = p.comment },

    -- Text (legacy captures, kept for compatibility)
    ["@text"] = { fg = p.foreground },
    ["@text.strong"] = { bold = true },
    ["@text.emphasis"] = { italic = true },
    ["@text.underline"] = { underline = true },
    ["@text.strike"] = { strikethrough = true },
    ["@text.title"] = { fg = p.lantern, bold = true },
    ["@text.title.1"] = { fg = p.lantern, bold = true },
    ["@text.title.2"] = { fg = p.amber, bold = true },
    ["@text.title.3"] = { fg = p.ember, bold = true },
    ["@text.title.4"] = { fg = p.amber },
    ["@text.title.5"] = { fg = p.comment, bold = true },
    ["@text.title.6"] = { fg = p.blue },
    ["@text.literal"] = { fg = p.bright_green },
    ["@text.uri"] = { fg = p.bright_blue, underline = true },
    ["@text.math"] = { fg = p.bright_cyan },
    ["@text.reference"] = { fg = p.magenta },
    ["@text.todo"] = { fg = p.background, bg = p.blue, bold = true },
    ["@text.note"] = { fg = p.info },
    ["@text.warning"] = { fg = p.warning },
    ["@text.danger"] = { fg = p.error },
    ["@text.diff.add"] = { fg = p.diff_add },
    ["@text.diff.delete"] = { fg = p.diff_delete },
  }
end

return M
