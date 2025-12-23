[treesitter](https://github.com/tree-sitter/tree-sitter){target="_blank"} is a tool that can
build a concrete syntax tree for a source file and incrementally update the tree
as the file is modified.

Recently, I've been experimenting more and more with treesitter's
[query DSL](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries){target="_blank"}
and an abstraction layer in the form of a
[neovim plugin](https://github.com/nvim-treesitter/nvim-treesitter){target="_blank"}, and found a
configuration option to be particularly useful when editing code: _incremental
selection_.

When you write a treesitter parser, you must write a grammar that treesitter can
read, which it then uses to generate the parser. This grammar specifies the
named syntax nodes in the tree and how they relate to other nodes.

For instance, if we head over to the
[syntax tree playground](https://tree-sitter.github.io/tree-sitter/7-playground.html){target="_blank"}
and parse rust source code that looks like this:

```rust
fn main() {
  println!("Hello, world!");
}
```

we get the following syntax tree output:

```
source_file [0, 0] - [3, 0]
  function_item [0, 0] - [2, 1]
    name: identifier [0, 3] - [0, 7]
    parameters: parameters [0, 7] - [0, 9]
    body: block [0, 10] - [2, 1]
      macro_invocation [1, 2] - [1, 27]
        macro: identifier [1, 2] - [1, 9]
        token_tree [1, 10] - [1, 27]
          string_literal [1, 11] - [1, 26]
```

The neovim plugin written as an abstraction layer for treesitter usage within
neovim is able to parse your source file as you type. That is, it's able to
compute a syntax tree that resembles the one shown above, on the fly.

As you hover over a certain part of a file, it is able to detect which node
you're currently at in the tree.

The plugin includes a configuration option which allows for the setting of
keybindings for initializing a node selection and incrementing a selected node
range -- configured like so:

```lua
incremental_selection = {
  enable = true,
  keymaps = {
    init_selection = "<cr>",
    node_incremental = "grn",
    scope_incremental = "grc",
    node_decremental = "grm",
  }
}
```

Incremental selection has improved the rapidity in which I can yank or replace
blocks of text in treesitter parsed source files, as expanding a node selection
with a keybinding is super fast and usually results in the exact block of text I
want to modify.

Here's a quick demo of it in action:

[![asciicast](https://asciinema.org/a/507405.svg)](https://asciinema.org/a/507405){target="_blank"}

A reference of my full neovim configuration can be found on github, in my
[dotfiles](https://github.com/terror/dotfiles){target="_blank"} repo. Be sure to check it out!
