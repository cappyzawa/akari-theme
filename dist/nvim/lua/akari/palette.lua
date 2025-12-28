-- Akari color palette
-- A terminal color palette inspired by Japanese alleys lit by round lanterns.

local M = {}

-- Night palette (dark theme)
M.night = {
  -- Base
  background = "#10141C",
  foreground = "#E6DED3",

  -- Layers (blue-tinted elevation)
  surface = "#1B222B",
  sunken = "#302121",
  raised = "#1F2630",
  border = "#262F3B",

  -- UI
  cursor = "#E26A3B",
  cursor_text = "#10141C",
  selection_bg = "#2D2925",
  selection_fg = "#E6DED3",
  match_bg = "#3A2522",

  -- ANSI colors
  black = "#0D1016",
  red = "#D25046",
  green = "#7FAF6A",
  yellow = "#D4A05A",
  blue = "#5A6F82",
  magenta = "#8E7BA0",
  cyan = "#6F8F8A",
  white = "#E6DED3",

  -- Bright ANSI colors
  bright_black = "#465676",
  bright_red = "#DE7F77",
  bright_green = "#A1C492",
  bright_yellow = "#E4C397",
  bright_blue = "#8195A8",
  bright_magenta = "#B4A7C0",
  bright_cyan = "#9AB1AD",
  bright_white = "#EFEAE3",

  -- Semantic colors
  lantern = "#E26A3B",
  ember = "#D65A3A",
  amber = "#D4A05A",
  constant = "#8CA6A1",
  comment = "#7E93A6",
  path = "#7FAF6A",
  macro = "#B4A7C0",
  escape = "#B4A7C0",
  regexp = "#A1C492",
  link = "#8195A8",

  -- Diagnostic
  error = "#D25046",
  warning = "#E26A3B",
  info = "#5A6F82",
  hint = "#7E93A6",

  -- Diff
  diff_add = "#7FAF6A",
  diff_delete = "#D25046",
  diff_change = "#D4A05A",

  -- None (for transparent)
  none = "NONE",
}

-- Dawn palette (light theme)
M.dawn = {
  -- Base
  background = "#E4DED6",
  foreground = "#1A1816",

  -- Layers
  surface = "#CFC4B6",
  sunken = "#DDD2C9",
  raised = "#D9D1C6",
  border = "#CABEAE",

  -- UI
  cursor = "#8A4530",
  cursor_text = "#E4DED6",
  selection_bg = "#D7C5B1",
  selection_fg = "#1A1816",
  match_bg = "#D2BFB5",

  -- ANSI colors
  black = "#1A1816",
  red = "#6A2828",
  green = "#3A5830",
  yellow = "#B07840",
  blue = "#304050",
  magenta = "#806080",
  cyan = "#305858",
  white = "#E4DED6",

  -- Bright ANSI colors
  bright_black = "#514B45",
  bright_red = "#3E1717",
  bright_green = "#20301A",
  bright_yellow = "#78522C",
  bright_blue = "#131A20",
  bright_magenta = "#543F54",
  bright_cyan = "#152727",
  bright_white = "#D0C5B7",

  -- Semantic colors
  lantern = "#8A4530",
  ember = "#7A3828",
  amber = "#B07840",
  constant = "#447C7C",
  comment = "#222D38",
  path = "#3A5830",
  macro = "#543F54",
  escape = "#543F54",
  regexp = "#20301A",
  link = "#131A20",

  -- Diagnostic
  error = "#6A2828",
  warning = "#8A4530",
  info = "#304050",
  hint = "#100E0D",

  -- Diff
  diff_add = "#314B29",
  diff_delete = "#6A2828",
  diff_change = "#9E6C3A",

  -- None (for transparent)
  none = "NONE",
}

-- Default to night palette for backwards compatibility
M.palette = M.night

-- Get terminal colors for a specific variant
function M.get_terminal(variant)
  local p = variant == "dawn" and M.dawn or M.night
  return {
    p.black, -- 0
    p.red, -- 1
    p.green, -- 2
    p.yellow, -- 3
    p.blue, -- 4
    p.magenta, -- 5
    p.cyan, -- 6
    p.white, -- 7
    p.bright_black, -- 8
    p.bright_red, -- 9
    p.bright_green, -- 10
    p.bright_yellow, -- 11
    p.bright_blue, -- 12
    p.bright_magenta, -- 13
    p.bright_cyan, -- 14
    p.bright_white, -- 15
  }
end

-- Terminal colors (16 ANSI colors) - default to night
M.terminal = M.get_terminal("night")

return M
