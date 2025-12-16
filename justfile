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
  python3 -m http.server 8000 --directory ./docs

[group: 'dev']
dev-deps:
  brew install just prettier typos-cli shellcheck uv
  cargo install --path crates/watcher
  curl https://raw.githubusercontent.com/jirutka/esh/master/esh > /usr/local/bin/esh

[group: 'fix']
fix-typos:
  typos --write-changes **/*.md

[group: 'format']
fmt:
  uv run ruff check --select I --fix && uv run ruff format
  prettier --write .

[group: 'check']
forbid:
  ./bin/forbid

[group: 'dev']
generate:
  ./bin/last-modified
  ./bin/generate-index
  ./bin/generate-projects.py

[group: 'dev']
generate-favicon image:
  npx realfavicon generate {{ image }} favicon-settings.json favicon-output.json docs/favicon
  cat favicon-output.json | jq -r '.markups | sort | join("\n")' > favicon-output.txt
  rm favicon-output.json

[group: 'dev']
watch:
  ./bin/kill-server
  python3 -m http.server 8000 --directory ./docs &
  watcher ./posts ./bin/regenerate-post
