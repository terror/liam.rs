---
title: edmv
date: 2023-05-12
repo: terror/edmv
topics: ['Rust', 'CLI', 'Tools', 'Productivity']
lead: A project scaffolding utility
image: skeleton.png
---

**skeleton** is a tool that makes it easier to start new projects.

It stores your commonly used project specific files in one place, letting you
easily apply them to new projects.

In essence, a `template` is a file ending in `.skeleton` with a front-matter and
content. The front-matter is structured
[YAML](https://en.wikipedia.org/wiki/YAML?useskin=vector) with effect and free
variables.

An `effect` variable is pre-defined to perform some action, e.g. `filename` is
used when applying a template and writing it to its appropriate location,
`command` is used to apply a command post-write, etc.

A `free` variable is used to substitute into the templates content, you can also
specify whether or not to be interactively prompted for these types of variables
when applying templates.

`skeleton` is a re-implementation and improvement of a Python program I wrote a
while back called `bp`, which you can find [here](https://github.com/terror/bp).

Links: [GitHub](https://github.com/terror/skeleton)
