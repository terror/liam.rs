## liam.rs

[![build](https://img.shields.io/github/actions/workflow/status/terror/liam.rs/ci.yaml?branch=master&style=flat&labelColor=1d1d1d&color=424242&logo=GitHub%20Actions&logoColor=white&label=build)](https://github.com/terror/liam.rs/actions/workflows/ci.yaml)

Source code for my personal website built with Rust and
[MiniJinja](https://github.com/mitsuhiko/minijinja).

<img width="800" alt="demo" src="screenshot.png" />

## Development

If you notice any typos feel free to submit a PR. All site content is in the
/posts directory.

Fork this repository and clone your fork:

```bash
git clone https://github.com/{username}/liam.rs.git
```

Make changes and generate static assets:

```bash
./bin/sync-post-timestamps
cargo run -p generator -- generate
```

Serve the site locally with automatic browser refresh:

```bash
cargo run -p generator -- serve
```

**n.b.** This requires you to have [Pandoc](https://pandoc.org/) installed on
your system.
