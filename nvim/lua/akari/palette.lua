-- Akari color palette
-- A terminal color palette inspired by Japanese alleys lit by round lanterns.

local M = {}

-- Night palette (dark theme)
M.night = {
  -- Base
  background = "#171B22", -- navy-charcoal night air
  foreground = "#E6DED3", -- soft paper white

  -- Layers (blue-tinted elevation)
  surface = "#1E232C", -- slightly elevated
  sunken = "#251D1A", -- cursorline (lantern-tinted warmth)
  raised = "#242836", -- popup/menu
  border = "#2E3543", -- separators

  -- UI
  cursor = "#E26A3B", -- lantern glow
  cursor_text = "#171B22",
  selection_bg = "#2D2518", -- amber-tinted warmth
  selection_fg = "#F2ECE4",
  match_bg = "#332A2A", -- hint of lantern warmth

  -- ANSI colors
  black = "#12100E",
  red = "#C84C32",
  green = "#7FAF6A", -- brighter life green
  yellow = "#D4A05A", -- amber (warm but not lantern)
  blue = "#5A6F82", -- night air
  magenta = "#7C6A8A", -- muted, non-neon
  cyan = "#6F8F8A",
  white = "#E6DED3",

  -- Bright ANSI colors
  bright_black = "#2E3543", -- matches border for consistency
  bright_red = "#D65A3A",
  bright_green = "#8FC57A",
  bright_yellow = "#F08A5D",
  bright_blue = "#6F879A",
  bright_magenta = "#9A8FB3",
  bright_cyan = "#8FB8B0",
  bright_white = "#F2ECE4",

  -- Semantic colors
  lantern = "#E26A3B", -- primary accent (keyword/function/string)
  ember = "#D65A3A", -- numbers/constants
  amber = "#D4A05A", -- paths/strings
  comment = "#7D8797", -- distant stone-gray, night humidity

  -- Diagnostic
  error = "#C84C32",
  warning = "#E26A3B",
  info = "#5A6F82",
  hint = "#7D8797", -- matches comment

  -- Diff
  diff_add = "#7FAF6A",
  diff_delete = "#C84C32",
  diff_change = "#E26A3B",

  -- None (for transparent)
  none = "NONE",
}

-- Dawn palette (light theme)
M.dawn = {
  -- Base
  background = "#E4DED6", -- dawn alley surface (warm paper)
  foreground = "#1A1816", -- ink black (stone pavement)

  -- Layers
  surface = "#D4CEC6", -- slightly elevated
  sunken = "#DCD4CA", -- cursorline (recessed)
  raised = "#EDE7DF", -- popup/menu
  border = "#C4BEB6", -- separators

  -- UI
  cursor = "#8A4530", -- fading lantern
  cursor_text = "#E4DED6",
  selection_bg = "#D0C4B8",
  selection_fg = "#1A1816",
  match_bg = "#D8C8B8", -- hint of lantern warmth

  -- ANSI colors
  black = "#1A1816",
  red = "#B04030",
  green = "#3A5830", -- plants catching first light
  yellow = "#B07840", -- warm yellow in morning light
  blue = "#304050", -- quiet morning air
  magenta = "#806080", -- distant, unobtrusive
  cyan = "#305858",
  white = "#F4F0EA",

  -- Bright ANSI colors
  bright_black = "#3A3E48",
  bright_red = "#C04838",
  bright_green = "#70A868",
  bright_yellow = "#D09060",
  bright_blue = "#4060A0",
  bright_magenta = "#705868",
  bright_cyan = "#306868",
  bright_white = "#0A0808",

  -- Semantic colors
  lantern = "#8A4530", -- fading lantern
  ember = "#7A3828", -- cooling embers
  amber = "#B07840", -- paths/strings
  comment = "#4A4642", -- readable dark gray

  -- Diagnostic
  error = "#B04030",
  warning = "#8A4530",
  info = "#304050",
  hint = "#5A5855",

  -- Diff
  diff_add = "#507848",
  diff_delete = "#B04030",
  diff_change = "#C08050",

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
