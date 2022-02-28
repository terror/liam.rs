I recently wrote yet another command-line utility in Rust called
[`present`](https://github.com/terror/present) that lets you interpolate the
standard output of arbitrary commands that get interpreted by the shell into a
markdown file, and I thought I'd share a bit about it here.

### Problem

The main problem I kept running into that pushed me to write this tool was
having to *manually* update the outputs of command line utility help messages in
my project readme documents.

For instance, I wrote a command-line utility called
[`vim-profiler`](https://github.com/terror/vim-profiler) that lets you
profile the startup-time for your installed vim plugins and receive a nicely
formatted output. In the readme document I have a usage section that includes
the output of calling `vp --help`. Each time I make a version change, update
the description, or update the API -- the result of calling `vp --help` changes,
which prompts me to update the readme.

### Solution

Instead of invoking the binary with the `--help` flag, piping the result to
`pbcopy` and then manually pasting the chunk into the appropriate section within
the readme myself, I can now have `present` do all of that for me.

Here's how that looks like:

1. Include the command at the start of a fenced codeblock using the `present`
   prefix

````
## vim-profiler 🕒

...

### Usage

```present cargo run -- --help
```
...
````

2. Run `present` on the markdown file

```bash
$ present --in-place --path readme.md
```

3. View the modified document!

````
## vim-profiler 🕒

...

### Usage

```cargo run -- --help
vim-profiler 0.0.4
A vim profiling tool.

USAGE:
    vp [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -r, --reverse    Display the plugin times in reverse order (fastest first)
    -s, --sys        Show system plugins in the output
    -V, --version    Prints version information
    -v, --verbose    Add informative messages during program execution

OPTIONS:
    -c, --command   <command>      The command to run, e.g vim or neovim [default: vim]
    -n, --count     <count>        The number of plugins to list in the output
    -e, --export    <path>         Export the results to a CSV file
    -f, --file      <file>         A file to open
    -i, --iter      <iter>         The number of iterations
    -p, --plot      <path>         Plot the data and save it to a SVG file
    -x, --precision <precision>    Precision in the output
```
...
````

In practice, I'll add the command in a justfile `all` recipe that I invoke
before I commit any changes, such as shown in this
[justfile](https://github.com/terror/present/blob/master/justfile).

### Moving forward

The project is still in its early stages of development, and there most
certainly exists a few non-trivial bugs to be found and fixed.

Some things on the todo list, in no particular order, include:

> Improve diff output in interactive mode

The project uses the [`similar`](https://github.com/mitsuhiko/similar) crate
to help with diff output in interactive mode, but it can be made nicer by taking
advantage of additional features the crate has to offer.

> Support same-line command interpolation

This isn't that important, but it would be nice to support having backticks
remain on the same line and have the command result get interpolated with the
appropriate newlines. e.g

````
-> ```echo foo```

-> ```echo foo
   foo
   ```
````

> Handle quotes as a single argument

This would let you actually write inline bash scripts, e.g `/bin/bash
-c 'for i in *; do echo "$i"; done'`. As of right now, the program just splits
the entire command string on whitespace, which the shell doesn't like in
certain situations.

This however, for now, can be hacked around by simply including the script in
a justfile or makefile and invoking `just <name>` or `make <name>` within the
markdown file.

> Get rid of `pulldown_cmark`

The project uses the
[`pulldown_cmark`](https://github.com/raphlinus/pulldown-cmark) crate to get full
codeblock ranges within the source, and then hackily turns them into two
separate ranges (start, end) which represent the starting and ending range of
a single codeblock. This could probably be done better, perhaps without depending
on a library for that initial step.

Overall, I get what I need out of the program as it is. Whether or not the
aformentioned todo's provide useful to someone else will most likely have a
non-trivial amount of impact on my motiviation for getting them done.

Feel free to check out the code over on github
[https://github.com/terror/present](https://github.com/terror/present). It is
licensed under the highly permissive
[CC0-1.0](https://en.wikipedia.org/wiki/Creative_Commons_license) license.
