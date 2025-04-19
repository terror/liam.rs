---
title: notes
date: 2022-11-27
repo: terror/notes
topics: ['Bash', 'Productivity']
lead: Manage a flat directory of notes with fzf, fd and ripgrep.
image: notes.png
---

**notes** enables you to manage a flat directory of notes using fzf, fd, and
ripgrep.

The entire thing is modelled around fzf.

The main command is `enter`, which spawns a fuzzy file finder window on file
names in your notes directory. You can also run a full-text search with ripgrep
using the `search` subcommand.

It's a single bash script, and assumes a few dependencies (as mentioned) are
installed on your system.

Links: [GitHub](https://github.com/terror/notes){target="\_blank"}
