-- LSP semantic token highlights

local M = {}

function M.setup(p, config)
  return {
    -- LSP semantic token types
    ["@lsp.type.class"] = { link = "@type" },
    ["@lsp.type.comment"] = { link = "@comment" },
    ["@lsp.type.decorator"] = { fg = p.syntax.decorator },
    ["@lsp.type.enum"] = { link = "@type" },
    ["@lsp.type.enumMember"] = { fg = p.syntax.constant },
    ["@lsp.type.event"] = { fg = p.ansi.bright.yellow },
    ["@lsp.type.function"] = { link = "@function" },
    ["@lsp.type.interface"] = { fg = p.ansi.bright.cyan },
    ["@lsp.type.keyword"] = { link = "@keyword" },
    ["@lsp.type.macro"] = { link = "@function.macro" },
    ["@lsp.type.method"] = { link = "@function.method" },
    ["@lsp.type.modifier"] = { link = "@keyword.modifier" },
    ["@lsp.type.namespace"] = { link = "@module" },
    ["@lsp.type.number"] = { link = "@number" },
    ["@lsp.type.operator"] = { link = "@operator" },
    ["@lsp.type.parameter"] = { link = "@variable.parameter" },
    ["@lsp.type.property"] = { link = "@property" },
    ["@lsp.type.regexp"] = { link = "@string.regexp" },
    ["@lsp.type.string"] = { link = "@string" },
    ["@lsp.type.struct"] = { link = "@type" },
    ["@lsp.type.type"] = { link = "@type" },
    ["@lsp.type.typeParameter"] = { fg = p.ansi.cyan, italic = true },
    ["@lsp.type.variable"] = {}, -- Use treesitter highlight

    -- LSP semantic token modifiers
    ["@lsp.mod.abstract"] = { italic = true },
    ["@lsp.mod.async"] = { italic = true },
    ["@lsp.mod.declaration"] = {},
    ["@lsp.mod.defaultLibrary"] = { fg = p.ansi.bright.yellow },
    ["@lsp.mod.definition"] = {},
    ["@lsp.mod.deprecated"] = { strikethrough = true },
    ["@lsp.mod.documentation"] = {},
    ["@lsp.mod.modification"] = {},
    ["@lsp.mod.readonly"] = { fg = p.syntax.constant },
    ["@lsp.mod.static"] = { italic = true },

    -- LSP combined type.modifier
    ["@lsp.typemod.class.declaration"] = { link = "@type" },
    ["@lsp.typemod.enum.declaration"] = { link = "@type" },
    ["@lsp.typemod.function.declaration"] = { link = "@function" },
    ["@lsp.typemod.function.defaultLibrary"] = { link = "@function.builtin" },
    ["@lsp.typemod.interface.declaration"] = { fg = p.ansi.bright.cyan },
    ["@lsp.typemod.keyword.async"] = vim.tbl_extend("force", { fg = p.syntax.keyword }, config.keywordStyle),
    ["@lsp.typemod.macro.defaultLibrary"] = { link = "@function.macro" },
    ["@lsp.typemod.method.declaration"] = { link = "@function.method" },
    ["@lsp.typemod.method.defaultLibrary"] = { link = "@function.builtin" },
    ["@lsp.typemod.namespace.declaration"] = { link = "@module" },
    ["@lsp.typemod.operator.injected"] = { link = "@operator" },
    ["@lsp.typemod.parameter.declaration"] = { link = "@variable.parameter" },
    ["@lsp.typemod.property.declaration"] = { link = "@property" },
    ["@lsp.typemod.property.readonly"] = { fg = p.syntax.constant },
    ["@lsp.typemod.string.injected"] = { link = "@string" },
    ["@lsp.typemod.struct.declaration"] = { link = "@type" },
    ["@lsp.typemod.type.declaration"] = { link = "@type" },
    ["@lsp.typemod.type.defaultLibrary"] = { link = "@type.builtin" },
    ["@lsp.typemod.typeAlias.declaration"] = { link = "@type.definition" },
    ["@lsp.typemod.variable.callable"] = { link = "@function" },
    ["@lsp.typemod.variable.declaration"] = { link = "@variable" },
    ["@lsp.typemod.variable.defaultLibrary"] = { link = "@variable.builtin" },
    ["@lsp.typemod.variable.global"] = { fg = p.base.foreground },
    ["@lsp.typemod.variable.injected"] = { link = "@variable" },
    ["@lsp.typemod.variable.readonly"] = { fg = p.syntax.constant },
    ["@lsp.typemod.variable.static"] = { fg = p.base.foreground, italic = true },

    -- LSP reference highlights
    LspReferenceText = { bg = p.ui.selection_bg },
    LspReferenceRead = { bg = p.ui.selection_bg },
    LspReferenceWrite = { bg = p.ui.selection_bg },

    -- LSP signature help
    LspSignatureActiveParameter = { fg = p.ui.accent, bold = true },

    -- LSP codelens
    LspCodeLens = { fg = p.ui.muted },
    LspCodeLensSeparator = { fg = p.ansi.bright.black },

    -- LSP inlay hints
    LspInlayHint = { fg = p.ui.muted },
  }
end

return M
