export ESH_AWK := '/opt/homebrew/bin/gawk'

default:
  just --list

alias d := dev
alias f := fmt
alias g := gen

all: gen fix-typos fmt

check:
  uv run ruff check

dev:
  python3 -m http.server 8000 --directory ./docs

dev-deps:
  brew install just prettier uv
  cargo install typos-cli
  cargo install --path crates/watcher
  curl https://raw.githubusercontent.com/jirutka/esh/master/esh > /usr/local/bin/esh

fix-typos:
  typos --write-changes

fmt:
  ruff check --select I --fix && ruff format
  prettier --write .

gen:
  ./bin/last-modified
  ./bin/generate-index
  uv run ./bin/generate-projects
  ./bin/forbid

watch:
  ./bin/kill-server
  python3 -m http.server 8000 --directory ./docs &
  watcher ./posts ./bin/regenerate-post
