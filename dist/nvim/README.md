# Akari Neovim Theme

Neovim colorscheme for Akari. A terminal color palette inspired by Japanese alleys lit by round lanterns.

## Installation

### lazy.nvim

Point lazy.nvim at this directory:

```lua
{
    dir = "/path/to/this/directory",
    config = function()
        require("akari").setup({ variant = "night" })
        vim.cmd.colorscheme("akari")
    end,
}
```

## Options

```lua
require("akari").setup({
    variant = "night", -- "night" or "dawn"
})
```

## Variants

- **night** (default) - A dark palette inspired by Japanese alleys lit by round lanterns.
- **dawn** - A light palette inspired by Japanese alleys lit by round lanterns.
