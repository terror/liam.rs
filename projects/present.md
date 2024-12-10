---
title: present
date: 2022-02-24
repo: terror/present
topics: ['Rust', 'Markdown', 'CLI', 'Tools', 'Productivity']
lead: A script interpolation engine for markdown documents.
image: present.png
---

**present** is a tool that lets you interpolate the standard output of arbitrary
scripts that get interpreted by the shell into your markdown documents.

It offers a nice way to automatically update sections of your markdown documents
that might be the standard output of a command, such as command-line utility
help outputs or benchmarks.

Simply add a suffix to the initial tag of markdown code-blocks using the format
`present <command>`. When you execute the present binary on your document, it
will run the specified `<command>`, and insert the resulting standard output
within the same code-block.

Links: [GitHub](https://github.com/terror/present){target="\_blank"},
[Crates.io](https://crates.io/crates/present){target="\_blank"}
