---
title: edmv
date: 2023-06-30
repo: terror/edmv
topics: ['Rust', 'CLI', 'Tools', 'Productivity']
lead: Bulk rename files with your favorite editor.
image: edmv.png
---

**edmv** provides a quick way to rename a bunch of files using any editor of
your choosing.

It works by letting you provide valid paths on disk, having them opened up and
editable in a temporary file on separate lines, and then performing any rename
operations for modified lines upon exit.

The tool behaves as if you were running `mv`, and has a few
[integration tests](https://github.com/terror/edmv/blob/master/tests/integration.rs){target="\_blank"}
that might be useful to check out as documentation.

Links: [GitHub](https://github.com/terror/edmv){target="\_blank"},
[Crates.io](https://crates.io/crates/edmv){target="\_blank"}
