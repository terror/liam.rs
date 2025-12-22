I recently ran the following command to get my top 5 most used
[Bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell){target="_blank"}) commands:

```bash
$ history | awk '{print $2}' | sort | uniq -c | sort -nr | head -5
```

and here is what came up:

```
20793 lt
10124 lv
9328 v
8686 j
7243 gst
```

These are, for the most part -- ordinary, everyday commands:

- `lt`: alias for `tree -L 1`
- `v`: alias for `nvim`
- `j`: alias for `just`, [the fabulous command runner](https://github.com/casey/just){target="_blank"}
- `gst`: alias for `git status`

What I assume isn't so obvious is `lv`, which stands for 'last vim'. `lv` is an
alias for the command `nvim -c "normal '0" -c bd1`, which opens the last opened
file at the last cursor position in a new Neovim buffer.

To break the command down a bit:

- `nvim`: the Neovim binary
- `-c <cmd>`: Execute `<cmd>` after config and first file
- `normal '0`: The last set cursor position and file
- `bd1`: Delete the first buffer

I use [Neovim](https://en.wikipedia.org/wiki/Vim_(text_editor){target="_blank"}#Neovim) alot,
and sometimes I quit the editor to run some arbitrary Bash command. Without
having to search for the previously opened file, I invoke `lv`, which gets me
back to where I was at.

From bash to vim and back, in a flash.
