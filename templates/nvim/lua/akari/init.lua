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
  variant = "night", -- "night" or "dawn"
}

function M.setup(opts)
  M.config = vim.tbl_deep_extend("force", M.config, opts or {})
end

function M.load(variant)
  -- Allow variant to be passed directly or use config
  local selected_variant = variant or M.config.variant or "night"

  if vim.g.colors_name then
    vim.cmd("hi clear")
  end

  vim.g.colors_name = "akari"
  vim.o.termguicolors = true

  if vim.fn.exists("syntax_on") then
    vim.cmd("syntax reset")
  end

  local palette_module = require("akari.palette")
  local highlights = require("akari.highlights")

  -- Select the appropriate palette based on variant
  local palette = selected_variant == "dawn" and palette_module.dawn or palette_module.night
  local terminal = palette_module.get_terminal(selected_variant)

  local hl = highlights.setup(palette, M.config)

  if M.config.terminalColors then
    highlights.apply(hl, terminal)
  else
    highlights.apply(hl, {})
  end
end

return M
