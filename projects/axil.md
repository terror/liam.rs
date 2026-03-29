---
title: axil
date: 2026-03-28
repo: terror/axil
topics: ['Rust', 'CLI', 'Treesitter', 'Tools']
lead: A terminal user-interface for treesitter.
image: axil.png
---

**axil** lets you interactively view the parsed syntax given by
[treesitter](https://tree-sitter.github.io/tree-sitter/){target="\_blank"} for a
file in any supported language.

I initially wrote this as a debugging tool for my treesitter-based
[language server](https://github.com/terror/just-lsp){target="\_blank"} for
[`just`](https://github.com/casey/just){target="\_blank"}, where inspecting and
running queries against syntax trees was a common occurrence.

Among other things, **axil** supports:

- Browsing the full syntax tree in an interactive TUI with vim-style navigation
  (`h`/`j`/`k`/`l`), collapsible nodes, and half-page scrolling.

- Searching for node types with `/`, jumping between matches with `n`/`N`.

- Printing the syntax tree to stdout for non-interactive use, with optional
  `--query` highlighting.

- Mouse events for click-to-navigate and scroll wheel.

Links: [GitHub](https://github.com/terror/axil){target="\_blank"},
[Crates.io](https://crates.io/crates/axilt){target="\_blank"}
