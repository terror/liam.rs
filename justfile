set dotenv-load

export EDITOR := 'nvim'
export ESH_AWK := '/opt/homebrew/bin/gawk'

alias d := dev
alias f := fmt
alias g := generate

default:
  just --list

all: forbid fix-typos generate fmt

[group: 'check']
check:
  uv run ruff check
  shellcheck ./bin/*

[group: 'check']
check-favicon port='https://liam.rs':
  npx realfavicon check {{ port }}

[group: 'dev']
dev:
  python3 -m http.server --directory ./docs

[group: 'dev']
dev-deps:
  brew install just prettier typos-cli shellcheck uv
  curl https://raw.githubusercontent.com/jirutka/esh/master/esh > /usr/local/bin/esh

[group: 'fix']
fix-typos:
  typos --write-changes **/*.md

[group: 'format']
fmt-js:
  prettier --write .

[group: 'format']
fmt-python:
  uv run ruff check --select I --fix && uv run ruff format

[group: 'format']
fmt-rust:
  cargo fmt --all

[group: 'format']
fmt: fmt-js fmt-python fmt-rust

[group: 'check']
forbid:
  ./bin/forbid

[group: 'dev']
generate:
  ./bin/sync-post-timestamps
  ./bin/generate-index
  ./bin/generate-projects.py

[group: 'dev']
generate-favicon image:
  npx realfavicon generate {{ image }} favicon/favicon-settings.json favicon/favicon-output.json docs/favicon
  cat favicon-output.json | jq -r '.markups | sort | join("\n")' > favicon/favicon-output.txt
  rm favicon-output.json

[group: 'dev']
watch:
  ./bin/watch
