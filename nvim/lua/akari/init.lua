-- Akari colorscheme for Neovim
-- A terminal color palette inspired by Japanese alleys lit by round lanterns.

local M = {}

M.config = {
  undercurl = true,
  commentStyle = { italic = true },
  functionStyle = {},
  keywordStyle = {},
  transparent = false,
  terminalColors = true,
}

function M.setup(opts)
  M.config = vim.tbl_deep_extend("force", M.config, opts or {})
end

function M.load()
  if vim.g.colors_name then
    vim.cmd("hi clear")
  end

  vim.g.colors_name = "akari"
  vim.o.termguicolors = true

  if vim.fn.exists("syntax_on") then
    vim.cmd("syntax reset")
  end

  local palette = require("akari.palette")
  local highlights = require("akari.highlights")

  local hl = highlights.setup(palette.palette, M.config)

  if M.config.terminalColors then
    highlights.apply(hl, palette.terminal)
  else
    highlights.apply(hl, {})
  end
end

return M
