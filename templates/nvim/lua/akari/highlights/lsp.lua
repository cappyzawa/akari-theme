-- LSP semantic token highlights

local M = {}

function M.setup(p, config)
  return {
    -- LSP semantic token types
    ["@lsp.type.class"] = { link = "@type" },
    ["@lsp.type.comment"] = { link = "@comment" },
    ["@lsp.type.decorator"] = { fg = p.bright_yellow },
    ["@lsp.type.enum"] = { link = "@type" },
    ["@lsp.type.enumMember"] = { fg = p.constant },
    ["@lsp.type.event"] = { fg = p.bright_yellow },
    ["@lsp.type.function"] = { link = "@function" },
    ["@lsp.type.interface"] = { fg = p.bright_cyan },
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
    ["@lsp.type.typeParameter"] = { fg = p.cyan, italic = true },
    ["@lsp.type.variable"] = {}, -- Use treesitter highlight

    -- LSP semantic token modifiers
    ["@lsp.mod.abstract"] = { italic = true },
    ["@lsp.mod.async"] = { italic = true },
    ["@lsp.mod.declaration"] = {},
    ["@lsp.mod.defaultLibrary"] = { fg = p.bright_yellow },
    ["@lsp.mod.definition"] = {},
    ["@lsp.mod.deprecated"] = { strikethrough = true },
    ["@lsp.mod.documentation"] = {},
    ["@lsp.mod.modification"] = {},
    ["@lsp.mod.readonly"] = { fg = p.constant },
    ["@lsp.mod.static"] = { italic = true },

    -- LSP combined type.modifier
    ["@lsp.typemod.class.declaration"] = { link = "@type" },
    ["@lsp.typemod.enum.declaration"] = { link = "@type" },
    ["@lsp.typemod.function.declaration"] = { link = "@function" },
    ["@lsp.typemod.function.defaultLibrary"] = { link = "@function.builtin" },
    ["@lsp.typemod.interface.declaration"] = { fg = p.bright_cyan },
    ["@lsp.typemod.keyword.async"] = vim.tbl_extend("force", { fg = p.lantern }, config.keywordStyle),
    ["@lsp.typemod.macro.defaultLibrary"] = { link = "@function.macro" },
    ["@lsp.typemod.method.declaration"] = { link = "@function.method" },
    ["@lsp.typemod.method.defaultLibrary"] = { link = "@function.builtin" },
    ["@lsp.typemod.namespace.declaration"] = { link = "@module" },
    ["@lsp.typemod.operator.injected"] = { link = "@operator" },
    ["@lsp.typemod.parameter.declaration"] = { link = "@variable.parameter" },
    ["@lsp.typemod.property.declaration"] = { link = "@property" },
    ["@lsp.typemod.property.readonly"] = { fg = p.constant },
    ["@lsp.typemod.string.injected"] = { link = "@string" },
    ["@lsp.typemod.struct.declaration"] = { link = "@type" },
    ["@lsp.typemod.type.declaration"] = { link = "@type" },
    ["@lsp.typemod.type.defaultLibrary"] = { link = "@type.builtin" },
    ["@lsp.typemod.typeAlias.declaration"] = { link = "@type.definition" },
    ["@lsp.typemod.variable.callable"] = { link = "@function" },
    ["@lsp.typemod.variable.declaration"] = { link = "@variable" },
    ["@lsp.typemod.variable.defaultLibrary"] = { link = "@variable.builtin" },
    ["@lsp.typemod.variable.global"] = { fg = p.foreground },
    ["@lsp.typemod.variable.injected"] = { link = "@variable" },
    ["@lsp.typemod.variable.readonly"] = { fg = p.constant },
    ["@lsp.typemod.variable.static"] = { fg = p.foreground, italic = true },

    -- LSP reference highlights
    LspReferenceText = { bg = p.selection_bg },
    LspReferenceRead = { bg = p.selection_bg },
    LspReferenceWrite = { bg = p.selection_bg },

    -- LSP signature help
    LspSignatureActiveParameter = { fg = p.lantern, bold = true },

    -- LSP codelens
    LspCodeLens = { fg = p.comment },
    LspCodeLensSeparator = { fg = p.bright_black },

    -- LSP inlay hints
    LspInlayHint = { fg = p.comment },
  }
end

return M
