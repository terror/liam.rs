---
title: pyproject
date: 2025-11-21
repo: terror/pyproject
topics: ['Rust', 'Language Servers', 'Linters']
lead: Check your `pyproject.toml` files.
image: pyproject.png
---

**pyproject** is a linter and language server for
[pyproject.toml](https://packaging.python.org/en/latest/guides/writing-pyproject-toml/){target="\_blank"}
files.

I wrote this after encountering inconsistencies in build tool error reporting
(e.g. some tools will let you ship empty licenses directories). I thought it
would be nice to have a single source of truth for
[PEP 621](https://peps.python.org/pep-0621/){target="\_blank"} related checks
and beyond that can run prior to running more expensive workflows.

There are currently over
[30+ rules](https://github.com/terror/pyproject/tree/master/src/rule){target="\_blank"}
that cover syntax validation, schema compliance, project metadata (i.e. name,
version, description, etc), dependencies (i.e. PEP 508 format, version bounds,
deprecations, updates), and lots more. The rule system is designed to be easily
extended with custom rules to fit any projects specific needs.

Links: [GitHub](https://github.com/terror/pyproject){target="\_blank"},
[Crates.io](https://crates.io/crates/pyproject){target="\_blank"}
