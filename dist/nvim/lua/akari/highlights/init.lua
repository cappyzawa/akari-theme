-- Highlight groups aggregator

local M = {}

function M.setup(palette, config)
  local highlights = {}
  local modules = { "editor", "syntax", "treesitter", "lsp" }

  for _, mod_name in ipairs(modules) do
    local ok, mod = pcall(require, "akari.highlights." .. mod_name)
    if ok then
      for hl, spec in pairs(mod.setup(palette, config)) do
        highlights[hl] = spec
      end
    end
  end

  return highlights
end

function M.apply(highlights, terminal_colors)
  for hl, spec in pairs(highlights) do
    vim.api.nvim_set_hl(0, hl, spec)
  end

  for i, color in ipairs(terminal_colors) do
    vim.g["terminal_color_" .. (i - 1)] = color
  end
end

return M
