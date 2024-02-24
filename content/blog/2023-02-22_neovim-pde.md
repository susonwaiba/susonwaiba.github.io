+++
title = "Neovim - PDE"
description = ""
date = 2023-02-22
[taxonomies]
tags = ["Editor", "Linux"]
+++

I just love **neovim**.

## Where I come from?

As a beginer windows user, obviously I have tried coding in **Notepad**.
Then I used to use **Notepad++**.
Then I found **sublime-text** and loved **Monokai** theme.

Then I started using **Linux** and **Netbeans** with its rich language support and slowness got me.
Then I started using **Jetbrains IDE**, loved its features then came **vscode**.
It changed the way I used to use text editor. With LSP, a simple editor got power of IDE.
With remote development plugins, now it possible to SSH into remote server and use light clients with access to powerful servers.
**vscode** with **WSL2** started everything then SSH came along.
Then came quarantine, during those days I used to use old laptop as home lab server and working on remote machine was my everyday.

Jetbrains IDE was very slow to index my codebases but vscode was also slow.
vscode also didn't had good support for **PHP** but with **PHP Intelephense** plugin, it is workable.
I was missing one regulary used feature from Jetbrains IDE (ie Goto Implementation) in vscode with plugins.
I mostly work on large codebases and there were numerious type of files, vscode was good for multiple language support. Each and every language had rich support, LSP was that good.

All these years, **vim** was under my nose as editor in server when I am using ssh in terminal I always had vim.
Using vscode in client server and production server was no go due to node dependencies and **vscode-server** installation. Production server should be isolated and only http port should be opened.
I have worked on some servers where one must ssh through various tunnels to finally getting into production.

## Why to configure neovim?

I had to rethink about my code editor choice, then came **neovim** with lua script support, customization and plugins.
All these years after moving from one primary editor to another and testing numerious editors, I knew what I wanted but configuring neovim exectly was so hard at first.
I configured my neovim and was missing so many features that I used to get back to vscode at beginer days.

Then I found **Lunarvim** distro, neovim configuration was like magic.
Everything is configured for you and most of keybinding are sane and workable.
I still suggest beginer to use **Lunarvim** until you want or need to create your own config.
It also uses separate neovim files dir living with your own neovim config. Then one can use both and keep configuring own config until one can migrate.
It allows you to configure that you don't like about it and all updates are maintained by distro maintainers.
I have use neovim for long time to see plugins deprecated and rise of suggestion for similar plugins, plugin manager changes and lots of LSP changes and updates.
With preconfigured neovim distro, there comes numerious features that you use or don't use. And keybindings that you don't remember.

There is concept of **Personal Development Environment (PDE)**. And I got fond of it.
Ultimately as I use my editor all the time to code my projects, it makes sense to code my editor as well or atlease configure it to my liking.
I got my hand issues, typing issues and I am brust typer. So, I configured my neovim according to it.

## Neovim version manager

- bob-bin

## Package manager

- lazy.nvim

## Plugins

- nvim-telescope/telescope.nvim
- voldikss/vim-floaterm
- nvim-treesitter/nvim-treesitter

- theprimeagen/harpoon

- mbbill/undotree

- tpope/vim-fugitive
- lewis6991/gitsigns.nvim
- sindrets/diffview.nvim

- VonHeikemen/lsp-zero.nvim

- williamboman/mason.nvim
- jose-elias-alvarez/null-ls.nvim
- jayp0521/mason-null-ls.nvim
- ray-x/lsp_signature.nvim

- folke/troble.nvim

- github/copilot.vim

- mfussenegger/nvim-dap
- rcarriga/nvim-dap-ui

- numToStr/Comment.nvim
- JoosepAlviste/nvim-ts-context-commentstring

- windwp/nvim-autopairs
- lukas-reineke/indent-blankline.nvim
- tpope/vim-sleuth

- folke/neodev.nvim

- ellisonleao/glow.nvim

- nvim-lualine/lualine.nvim
- j-hui/fidget.nvim
- folke/todo-comments.nvim

- tpope/vim-dadbod
- kristijanhusak/vim-dadbod-ui

- tamago324/lir.nvim

- folke/which-key.nvim

## Themes

- folke/tokyonight.nvim

- rose-pine/neovim
- catppuccin/nvim

## Autocmd

- highlight on yank

```lua
local highlight_group = vim.api.nvim_create_augroup('YankHighlight', { clear = true })
vim.api.nvim_create_autocmd('TextYankPost', {
	callback = function()
		vim.highlight.on_yank()
	end,
	group = highlight_group,
	pattern = '*',
})
```

- Different indent for file type

```lua
vim.api.nvim_command [[
	autocmd FileType json,js,jsx,javascript,ts,tsx,typescript,typescriptreact,css,less,scss set tabstop=2
]]
```

- auto indent bug fix for PHP

```lua
vim.api.nvim_create_autocmd({ "BufWinEnter" }, {
	callback = function()
		vim.cmd "set autoindent"
	end
})
```

## Keybinding

I use which-key for keymap documentation.

- Save current buffer

```lua
vim.keymap.set("i", "<C-s>", "<Esc>:w<CR>")
vim.keymap.set("n", "<C-s>", ":w<CR>")
```

- lir explorer

```lua
vim.keymap.set("n", "<leader>e", [[<Cmd>execute 'e ' .. expand('%:p:h')<CR>]], { noremap = true, desc = "Explore" })
vim.api.nvim_set_keymap(
	'n',
	'-',
	[[<Cmd>execute 'e ' .. expand('%:p:h')<CR>]],
	{ noremap = true }
)
```

- harpoon

```lua
require("harpoon").setup({
	menu = {
		width = vim.api.nvim_win_get_width(0) - 100,
	}
})

local mark = require("harpoon.mark")
local ui = require("harpoon.ui")

vim.keymap.set("n", "<C-m>", mark.add_file, { desc = "Add to harpoon" })
vim.keymap.set("n", "<C-h>", ui.toggle_quick_menu, { desc = "Harpoon menu" })

vim.keymap.set("n", "<C-j>", function () ui.nav_file(1) end)
vim.keymap.set("n", "<C-k>", function () ui.nav_file(2) end)
vim.keymap.set("n", "<C-l>", function () ui.nav_file(3) end)
```

- Floaterm

```lua
vim.keymap.set("n", "<leader>gg", "<cmd>FloatermNew --width=0.99 --height=0.99 lazygit<CR>", { desc = "Lazygit" })
vim.keymap.set("n", "<leader>E", "<cmd>FloatermNew --width=0.99 --height=0.99 lf<CR>", { desc = "LF" })
vim.keymap.set("n", "<leader>t", "<cmd>FloatermNew --width=0.99 --height=0.99 bash<CR>", { desc = "Terminal" })
```

- Wiki

```lua
vim.keymap.set("n", "<leader>wf", "<cmd>Telescope find_files cwd=~/wiki/<cr>", { desc = "Find File" })
vim.keymap.set("n", "<leader>wg", "<cmd>Telescope live_grep cwd=~/wiki/<cr>", { desc = "Grep" })
vim.keymap.set("n", "<leader>wq", "<cmd>e ~/wiki/quick-note.md<cr>", { desc = "Quick note" })

vim.keymap.set("n", "<leader>wt", string.format("<cmd>e ~/wiki/diary/%s/%s.md<cr>", os.date("%Y"), os.date("%Y-%m-%d")), { desc = "Today note" })
vim.keymap.set("n", "<leader>wT", string.format("<cmd>e ~/wiki/diary/%s/%s.md<cr>", os.date("%Y"), os.date("%Y-%m-%d"), os.time() + 24 * 60 * 60), { desc = "Tomorrow note" })
vim.keymap.set("n", "<leader>wy", string.format("<cmd>e ~/wiki/diary/%s/%s.md<cr>", os.date("%Y"), os.date("%Y-%m-%d"), os.time() - 24 * 60 * 60), { desc = "Yesterday note" })

vim.keymap.set("n", "<leader>wdf", string.format("<cmd>Telescope find_files cwd=~/wiki/diary/%s/<cr>", os.date("%Y")), { desc = "Find diary year" })
vim.keymap.set("n", "<leader>wdF", "<cmd>Telescope find_files cwd=~/wiki/diary/<cr>", { desc = "Find all diary" })
vim.keymap.set("n", "<leader>wdg", string.format("<cmd>Telescope live_grep cwd=~/wiki/diary/%s/<cr>", os.date("%Y")), { desc = "Grep diary year" })
vim.keymap.set("n", "<leader>wdG", "<cmd>Telescope live_grep cwd=~/wiki/diary/<cr>", { desc = "Grep all diary" })
```

- Custom build key

```lua
vim.keymap.set("n", "<Space>b", ':! cd md_to_json && cargo run<CR>', { desc = "Build" })
```

## LSP

I use **Mason** to manage LSP, DAP, Linter and Formatter related plugins.
I use **lsp-zero** to auto configure all LSP.
**cmp** is another thing that need to be configured with LSP.
I use **mason-null-ls** to help with formatter configuration.

I use **PHP** with **phpactor** and it doesnot have formatter so, I use **null-ls** to configure **phpcsfixer** to format PHP files.

```lua
require('mason.settings').set({
	ui = {
		border = 'rounded'
	}
})

local lsp = require('lsp-zero')

lsp.preset('recommended')

lsp.ensure_installed({})

lsp.nvim_workspace()

local cmp = require('cmp')
local cmp_select = { behavior = cmp.SelectBehavior.Select }
local cmp_mappings = lsp.defaults.cmp_mappings({
	['<C-p>'] = cmp.mapping.select_prev_item(cmp_select),
	['<C-n>'] = cmp.mapping.select_next_item(cmp_select),
	['<C-y>'] = cmp.mapping.confirm({ select = true }),
	['<C-Space'] = cmp.mapping.complete(),
})

lsp.setup_nvim_cmp({
	mapping = cmp_mappings,
})

lsp.set_preferences({
	suggest_lsp_servers = true,
	setup_servers_on_start = true,
	set_lsp_keymaps = false,
	configure_diagnostics = true,
	cmp_capabilities = true,
	manage_nvim_cmp = true,
	call_servers = 'local',
	sign_icons = {
		error = '✘',
		warn = '▲',
		hint = '⚑',
		info = ''
	}
})

lsp.on_attach(function(client, bufnr)
	if client.name == "phpcsfixer" then
		client.server_capabilities.documentFormattingProvider = false -- 0.8 and later
	end
    local nmap = function(keys, func, desc)
		vim.keymap.set('n', keys, func, { buffer = bufnr, desc = desc })
	end

	nmap("<leader>lf", function() vim.lsp.buf.format({ async = true }) end, "Format")

    -- other things...
end)

lsp.setup()

vim.diagnostic.config({
	virtual_text = true,
	signs = true,
	update_in_insert = false,
	underline = true,
	severity_sort = false,
	float = true,
})

local mason_nullls = require("mason-null-ls")

mason_nullls.setup({
	debug = true,
	automatic_installation = true,
	automatic_setup = true,
})

local null_ls = require("null-ls");
null_ls.setup({
	debug = true,
	sources = {
		null_ls.builtins.formatting.phpcsfixer.with({
			filetypes = { "html", "php", "phtml" },
		}),
	}
})

mason_nullls.setup_handlers()

require "lsp_signature".setup({})
```

DAP can also be configured also with LSP.

```lua
local dap = require('dap')
dap.adapters.php = {
	type = 'executable',
	command = 'node',
	args = { os.getenv("HOME") .. "/.local/share/nvim/mason/packages/php-debug-adapter/extension/out/phpDebug.js" }
}

dap.configurations.php = {
	{
		type = 'php',
		request = 'launch',
		name = 'Listen for Xdebug',
		hostname = 'fpm',
		port = 9000
	}
}

require("dapui").setup()
```

Trouble

```lua
vim.keymap.set("n", "<leader>xx", "<cmd>TroubleToggle<cr>",
	{ silent = true, noremap = true }
)
```

All my config are in github repository: [https://github.com/susonwaiba/.dotfiles](https://github.com/susonwaiba/.dotfiles)
