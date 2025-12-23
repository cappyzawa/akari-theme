# Akari Neovim Themes

Neovim colorscheme inspired by Japanese alleys lit by round lanterns.

## Installation

Using [lazy.nvim](https://github.com/folke/lazy.nvim):

```lua
{
    "cappyzawa/akari-theme",
    config = function()
        require("akari").setup({ variant = "night" })
        vim.cmd.colorscheme("akari")
    end,
}
```

Or for dawn:

```lua
{
    "cappyzawa/akari-theme",
    config = function()
        require("akari").setup({ variant = "dawn" })
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

- **night** (default) - Dark theme with lantern-lit atmosphere
- **dawn** - Light theme with morning warmth
