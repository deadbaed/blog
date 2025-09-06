+++
title = "Neovim LSP config for TypeScript (vtsls) with Yarn Plug'N'Play"
date = 2025-09-07T01:15:38+02:00[Europe/Paris]
uuid = "dd677927-3f6a-4d72-bad7-e9f1d81770aa"
description = "What is Plug'N'Play in Yarn, how to setup neovim to work with a TypeScript project using the `vtsls` LSP"
+++

I am starting a new project with Yarn. The last time I used it new projects defaulted to use v1, which uses the old way of installing packages in `node_modules`, like `npm` does.

When I created my project and upgraded to use the latest version of Yarn (v4 as I am writing this), I was surpised to see that neovim could not find TypeScript types in my code! VSCodium was not able to find types either. I started to research what was going on.

## Plug And Play

Yarn has multiple modes to install packages, the latest and default one is called [Plug'N'Play](https://yarnpkg.com/features/pnp) (PNP). Instead of installing packages inside a `node_modules` folder, it will instead download them in a cache, and create a special file named `.pnp.cjs`, defining what packages are resolved.

From what I understand, you give this file to the node runtime, and it will know where to load packages from its cache.

### Ghost dependencies

The main issue I had so far is the [ghost dependencies protection](https://yarnpkg.com/features/pnp#ghost-dependencies-protection) which requires packages to precisely define all their dependencies, something which can be tricky to do correctly.

If a package does not declare their dependencies correctly and it breaks at runtime, you have multiple options:
1. Declare in your project which packages need some extra information with a `.yarnrc.yml` file, see the [Yarn documentation](https://yarnpkg.com/features/pnp#how-can-i-fix-ghost-dependencies)
2. Fix the packages upstream directly, but you need to be ready to spend some time doing that (which can be hard to do in some cases)
3. Disable the PNP mode entirely, by using [another linker](https://yarnpkg.com/features/linkers) like `pnpm` or going back to the `node_modules` way.

PNP is still recommended to use, because it has an optimized way of managing installed packages across projects, and a good protection against [unaccounted dependencies at runtime](https://yarnpkg.com/features/pnp#ghost-dependencies-protection) (at the cost of having all your packages declaring their dependencies correctly).

### Editor support for TypeScript and others

By default, tools like LSPs and text editors will look for packages in the `node_modules` folder, because it is the way node always worked so far.

In PNP mode however, since the `.pnp.cjs` file tells where to go instead of `node_modules`, many tools have no idea how to load that file out of the box.

Yarn has a concept of [editor SDKs](https://yarnpkg.com/getting-started/editor-sdks) which will help your editor find your packages. I will cover neovim, since it requires a bit of work to get it working.

## Neovim setup

### Zip files

Yarn seems to heavily use zip files to store packages, and stores zip files inside other zip files. There is a plugin named [vim-rzip](https://github.com/lbrayner/vim-rzip) which brings this capability to neovim. Simply install it with your favorite package manager, with [lazy.nvim](https://lazy.folke.io) I installed it like this:

```lua
require("lazy").setup({
  spec = {
    { "lbrayner/vim-rzip" }
  }
})
```

### Editor SDK generation

To generate the Yarn editor SDK, a CLI tool is provided: Install it in inside your project with `yarn add -D @yarnpkg/sdks`, and run `yarn sdks base` to generate the base SDKs. For neovim this is enough, if you need some integrations can be [additionally generated](https://yarnpkg.com/cli/sdks/default#examples).

The location of the editor SDK is in `.yarn/sdks`. From what I understand this folder should be committed in version control. 

### Integration with the `vtsls` LSP

I use the `vtsls` LSP wrapper over `ts_ls`, because it seems to be the only one working with the [VueJS LSP](https://github.com/vuejs/language-tools/wiki/Neovim/a6764ad4f57d4a9a5cf88040a792034d861d7ca1#configuration).

`vtsls` allows to provide the location of the TypeScript library and compiler ([config reference](https://github.com/yioneko/vtsls/blob/ae8aea1cf71acd25f10f5122b91d9bf29bdce675/packages/service/configuration.schema.json#L7)), required to use Yarn with PNP.

I found a config snippet doing [exactly what I wanted](https://github.com/parksb/dotfiles/blob/2ae492bdd4827e065ab448fb4d8315736f418f1e/nvim/lua/plugins/langs/typescript.lua#L18). When using neovim 0.11, the lua code will look like this to configure `vtsls`:

```lua
vim.lsp.config("vtsls", {
  settings = {
    vtsls = {},
    typescript = {},
  },
  before_init = function(_, config)
    local yarnPnpFile = vim.fs.find({ ".pnp.cjs" }, { upward = true })[1]
    if yarnPnpFile then
      config.settings.typescript.tsdk = vim.fs.dirname(yarnPnpFile) .. "/.yarn/sdks/typescript/lib"
      config.settings.vtsls.autoUseWorkspaceTsdk = true
    end
  end,
})
```

Thank you very much [Simon Park](https://parksb.github.io) for sharing your dotfiles! It saved me a lot of time.

Now when opening a TypeScript file, the LSP should find your packages and do its work normally.

## Future

Hopefully the [Go port of TypeScript](https://github.com/microsoft/typescript-go) will implement the automatic detection of packages with Yarn PNP, so that in the future no additional setup will be required when working with projects using Yarn and its PNP mode.
