-- Akari color palette
-- A terminal color palette inspired by Japanese alleys lit by round lanterns.

local M = {}

M.palette = {
  -- Base
  background = "#1C1A17", -- warm gray night alley
  foreground = "#E6DED3", -- soft paper white

  -- UI
  cursor = "#E26A3B", -- lantern glow
  cursor_text = "#1C1A17",
  selection_bg = "#3A3530",
  selection_fg = "#F2ECE4",

  -- ANSI colors
  black = "#12100E",
  red = "#C84C32",
  green = "#7FAF6A", -- brighter life green
  yellow = "#E26A3B", -- lantern (not electric yellow)
  blue = "#5A6F82", -- night air
  magenta = "#7C6A8A", -- muted, non-neon
  cyan = "#6F8F8A",
  white = "#E6DED3",

  -- Bright ANSI colors
  bright_black = "#3A3530",
  bright_red = "#D65A3A",
  bright_green = "#8FC57A",
  bright_yellow = "#F08A5D",
  bright_blue = "#6F879A",
  bright_magenta = "#9A8FB3",
  bright_cyan = "#8FB8B0",
  bright_white = "#F2ECE4",

  -- Semantic colors
  lantern = "#E26A3B", -- primary accent (keyword/function/string)
  constant = "#D65A3A", -- numbers/constants
  comment = "#7A7A75", -- readable warm gray

  -- Diagnostic
  error = "#C84C32",
  warning = "#E26A3B",
  info = "#5A6F82",
  hint = "#7A7A75",

  -- Diff
  diff_add = "#7FAF6A",
  diff_delete = "#C84C32",
  diff_change = "#E26A3B",

  -- None (for transparent)
  none = "NONE",
}

-- Terminal colors (16 ANSI colors)
M.terminal = {
  M.palette.black, -- 0
  M.palette.red, -- 1
  M.palette.green, -- 2
  M.palette.yellow, -- 3
  M.palette.blue, -- 4
  M.palette.magenta, -- 5
  M.palette.cyan, -- 6
  M.palette.white, -- 7
  M.palette.bright_black, -- 8
  M.palette.bright_red, -- 9
  M.palette.bright_green, -- 10
  M.palette.bright_yellow, -- 11
  M.palette.bright_blue, -- 12
  M.palette.bright_magenta, -- 13
  M.palette.bright_cyan, -- 14
  M.palette.bright_white, -- 15
}

return M
