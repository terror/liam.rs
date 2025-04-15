export ESH_AWK := '/opt/homebrew/bin/gawk'

default:
  just --list

alias d := dev
alias f := fmt
alias g := generate

all: generate fix-typos fmt

check:
  uv run ruff check

check-favicon port='https://liam.rs':
  npx realfavicon check {{ port }}

dev:
  python3 -m http.server 8000 --directory ./docs

dev-deps:
  brew install just prettier uv
  cargo install typos-cli
  cargo install --path crates/watcher
  curl https://raw.githubusercontent.com/jirutka/esh/master/esh > /usr/local/bin/esh

fix-typos:
  typos --write-changes **/*.md

fmt:
  uv run ruff check --select I --fix && uv run ruff format
  prettier --write .

generate:
  ./bin/last-modified
  ./bin/generate-index
  uv run ./bin/generate-projects.py
  ./bin/forbid
  just fmt

generate-favicon image:
  npx realfavicon generate {{ image }} favicon-settings.json favicon-output.json docs/favicon
  cat favicon-output.json | jq -r '.markups | sort | join("\n")' > favicon-output.txt
  rm favicon-output.json

watch:
  ./bin/kill-server
  python3 -m http.server 8000 --directory ./docs &
  watcher ./posts ./bin/regenerate-post
