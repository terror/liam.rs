[`just`](https://github.com/casey/just){target="_blank"} is a command runner written in Rust.
Commands, or _recipes_ in `just` lingo, are populated in a file called 'justfile',
and are expressed in a make-like syntax.

I've been using `just` as my command runner for all of my projects,
big or small, and find it extremely useful to have a file in the projects root
that contains every command needed to interact with the project.

There are quite a few features baked into this program that make it unique from
similar projects, which I find nice to have when crafting software. This post
simply documents the few I use most often.

### Contents

- [Aliases](#aliases)
- [Fuzzy searching recipes](#fuzzy-searching-recipes)
- [Recipes in arbitrary languages](#recipes-in-arbitrary-languages)
- [Taking advantage of the default recipe](#taking-advantage-of-the-default-recipe)
- [Recipe dependencies](#recipe-dependencies)
- [Format your justfile](#format-your-justfile)
- [Documenting recipes](#documenting-recipes)

### Aliases

Recipes can have aliases, or alternative names which refer to the original
recipe that can be used with `just` to invoke the original recipe.

For instance, if you have a recipe called `run`, you can alias it to `r` like
so:

```
alias r := run

run:
  cargo run
```

and invoke it with `just r`.

Somewhat related side note: consider aliasing the `just` command to `j`, it'll
save you some typing if you frequently execute commands with `just`.

### Fuzzy searching recipes

One feature I discovered recently is `just --choose`, which lets you fuzzy
search through recipes defined in your justfile. You can also specify a
chooser by passing it into `just --chooser`, else it will default to
using [`fzf`](https://github.com/junegunn/fzf){target="_blank"}.

There is work planned to support recipe previews and command line arguments for
recipes that take them when using the default chooser.

However, with the help of `just --summary`, `just --show` and a bit of
[Python](https://en.wikipedia.org/wiki/Python_(programming_language)){target="_blank"}, here's how you can get to look:

[![asciicast](https://asciinema.org/a/eNi5cZw4BZLcplHq4Ae1aN8nN.svg){target="_blank"}](https://asciinema.org/a/eNi5cZw4BZLcplHq4Ae1aN8nN)

### Recipes in arbitrary languages

`just` recipes can be expressed in another language by adding a shebang at the
top of the recipe, as they get executed as scripts by the shell.

For instance, we can write a small script in Python that lists the contents
of the current directory, and put it in a recipe:

```python
ls:
  #!/usr/bin/env python3
  import os
  print(*os.listdir())
```

### Taking advantage of the default recipe

`just` allows you to specify a `default` recipe that gets executed when invoking
`just` without any arguments in a directory that contains a justfile (or a
sub-directory in which a parent directory contains a justfile).

For instance:

```
default:
  just --list
```

I usually use this feature to list out all recipes that are available in the
justfile however this could be used for anything that merits being quickly
invoked with a single command.

### Recipe dependencies

`just` recipes can contain 'dependencies', or other recipes that should be ran
_before_ a given recipe.

For example, let's say you have a 'build' recipe and want to run tests -- specified
in a 'test' recipe -- each time before executing the contents of the recipe, you
can do so by making 'test' a dependency of 'build', like so:

```
default:
  just --list

build: test # this is a dependency
  cargo build

test:
  cargo test
```

This also comes in handy when you want to run multiple recipes in sequence,
`just` allows for a very expressive syntax via dependencies:

```
default:
  just --list

all: build test fmt

build:
  cargo build

test:
  cargo test

fmt:
  cargo +nightly fmt
```

### Format your justfile

As of recently, `just` supports justfile formatting via the `--fmt` command.

It currently requires the `--unstable` flag, like so:

```bash
$ just --fmt --unstable
```

Invoking the command above can turn a justfile that looks like this:

```
default:
	just --list

build:
		cargo build

test:
	cargo test

clippy:
			cargo clippy --all-targets --all-features
```

into this:

```
default:
  just --list

build:
  cargo build

test:
  cargo test

clippy:
  cargo clippy --all-targets --all-features
```

This can be useful if you have many people working on your project and recipes
get added or modified in your projects justfile on the regular.

### Documenting recipes

`just` allows for comments above recipes that get appeared when invoking `just --list`.

For example, here is a sample justfile with a `run` recipe that contains a
comment above it:

```
# Run the project
run:
  cargo run
```

invoking `just --list` will yield the following output:

```
Available recipes:
    run # Run the project
```

This is extremely useful for documenting obscure commands, especially when
working in teams where people come and go on a project frequently.

### Fin

I personally think any project could benefit greatly by having a single source
of truth in regards to project specific commands, and `just` provides a simple
way to set that up.

Have a look over at the [official readme](https://github.com/casey/just){target="_blank"} document
on GitHub for more information and further elaboration on the features mentioned
here.
