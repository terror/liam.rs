---
title: present
date: 2022-02-24
repo: terror/present
topics: ['Rust', 'Markdown', 'CLI', 'Tools', 'Productivity']
lead: A script interpolation engine for markdown documents
image: present.png
---

**present** is a tool that lets you interpolate the standard output of arbitrary
scripts that get interpreted by the shell into your markdown documents.

It provides a nice way to automatically update sections of your markdown
documents that might be the standard output of a command, such as command-line
utility help outputs or benchmarks.

The main feature enables users to append a suffix to the initial tag of markdown
code-blocks using the format `present <command>`. After executing the present
binary on the document, the `<command>` will subsequently be run, and the
resulting standard output will be interpolated within the same code-block.

Links: [GitHub](https://github.com/terror/present),
[Crates.io](https://crates.io/crates/present)
