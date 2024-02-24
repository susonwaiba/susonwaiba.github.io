+++
title = "Arch minimal setup"
description = "While distro hopping, you stumble on Arch and you stick with arch. When I moved from Window to Linux few years ago, I saw significant less memory usage and overall responsive system with same hardware. While moving from Ubuntu to Arch, I saw similar difference. Ubuntu is not that bloated but I just don't want to use snapd for packages."
date = 2023-01-18
[taxonomies]
tags = ["Linux"]
+++

While distro hopping, you stumble on Arch and you stick with arch.
When I moved from **Window** to **Linux** few years ago, I saw significant less memory usage and overall responsive system with same hardware.
While moving from **Ubuntu** to **Arch**, I saw similar difference. Ubuntu is not that bloated but I just don't want to use snapd for packages.
Arch Package Repository and Arch User Package Repository is another reason to love Arch.

## How I setup my Arch?

Arch doesn't have GUI installation, working with manual terminal based installation is hard work.
`archinstall` python tool is very useful for initial setup. It can install all desktop environment with all the dependencies.
I usually go with **AwesomeWM** as it has minimal footprint.
Then I install basic packages like **Vim**, **Firefox** and **ansible**.
I have ansible setup for automatic packages installation.
Then I login to Github using Firefox and download zip package of my ansible config repository.
Then with single setup command, ansible will dcrypt and restore my secrets like pem files, start installation of packages and clone all my configs from private github repositories.
I manage all my configs in their own repositories and push clean changes.
I also have parallel ansible setup for Ubuntu.

## Basic packages

- alacritty
- kitty
- lxappearance
- pavucontrol
- arandr
- zip unzip
- openssh
- git
- htop
- neofetch
- vim
- yay
- bob-bin to manage neovim + `ripgrep`, `fd` and `xclip`
- tmux
- lazygit
- lf
- pcmanfm

## Keybinding

**AwesomeWM** comes with predefined vim-motion keys.

## Package manager

I like `pacman`.

Similarly, `yay` is then obiously choice as it has almost same flags.

## Fonts

- noto-fonts
- noto-fonts-emoji
- noto-fonts-extra
- otf-cascadia-code-nerd for terminal and neovim

## Window manager

**AwesomeWM** with X server.
Lua configuration.
Dynamic tiling window manager.

GTK and theme.

- pcmanfm and lf
- pavucontrol
- htop

## Start menu

- dmenu

## Panel widgets

**awesome-wm-widgets**

- Clock widget
- Date and Calender popup widget
- Network usage widget

## Multi monitor setup

`arandr` for GUI client to manage monitor position.

## Terminal

- kitty supports font ligatures
- alacritty

## Browser

- Firefox
- Chrome

## Notes

**Neovim** is my choice for editor so, I have keybindings to setup wiki using markdown text files.
I have `~/wiki` dir setup for markdown files and hosted via private github repo.
With neovim I have setup scratchpad, new note, search notes via telescope and even diary notes create keybinding to create new file.

## Docker

As a software developer, I use docker for most of the stateful services setup and development.
Along with docker, I also setup language compilers and interpertrs for some languages that I work daily.

All my config are in github repository: [https://github.com/susonwaiba/.dotfiles](https://github.com/susonwaiba/.dotfiles)
