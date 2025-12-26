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
  magenta = "#7C6A8A",
  cyan = "#6F8F8A",
  white = "#E6DED3",

  -- Bright ANSI colors
  bright_black = "#3D4045",
  bright_red = "#D65E55",
  bright_green = "#89B576",
  bright_yellow = "#D8AA6B",
  bright_blue = "#6B7D8F",
  bright_magenta = "#897996",
  bright_cyan = "#7D9A96",
  bright_white = "#E7E0D5",

  -- Semantic colors
  lantern = "#E26A3B",
  ember = "#D65A3A",
  amber = "#D4A05A",
  constant = "#8CA5A1",
  comment = "#7B8C9B",
  path = "#7FAF6A",

  -- Diagnostic
  error = "#D25046",
  warning = "#E26A3B",
  info = "#5A6F82",
  hint = "#7B8C9B",

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
  surface = "#D2CCC5",
  sunken = "#DDD2C9",
  raised = "#E6E1D9",
  border = "#C2BDB6",

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
  bright_black = "#484645",
  bright_red = "#622525",
  bright_green = "#35512C",
  bright_yellow = "#9E6C3A",
  bright_blue = "#2B3A48",
  bright_magenta = "#735673",
  bright_cyan = "#2B4F4F",
  bright_white = "#CDC8C1",

  -- Semantic colors
  lantern = "#8A4530",
  ember = "#7A3828",
  amber = "#B07840",
  constant = "#4F7171",
  comment = "#222D38",
  path = "#3A5830",

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
