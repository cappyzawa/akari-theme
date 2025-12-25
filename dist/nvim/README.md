# Akari Neovim Theme

> [!IMPORTANT]
> This repository is a read-only mirror.
> Issues, pull requests, and stars should go to [cappyzawa/akari-theme](https://github.com/cappyzawa/akari-theme).

Neovim colorscheme inspired by Japanese alleys lit by round lanterns.

## Installation

### lazy.nvim

```lua
{
    "cappyzawa/akari-nvim",
    config = function()
        require("akari").setup({ variant = "night" })
        vim.cmd.colorscheme("akari")
    end,
}
```

To pin a specific version:

```lua
{
    "cappyzawa/akari-nvim",
    tag = "v0.9.0",
    config = function()
        require("akari").setup({ variant = "night" })
        vim.cmd.colorscheme("akari")
    end,
}
```

### vim-plug

```vim
Plug 'cappyzawa/akari-nvim'
```

Then in your config:

```lua
lua require("akari").setup({ variant = "night" })
colorscheme akari
```

### dein.vim

```vim
call dein#add('cappyzawa/akari-nvim')
```

Then in your config:

```lua
lua require("akari").setup({ variant = "night" })
colorscheme akari
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
