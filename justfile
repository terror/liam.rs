export ESH_AWK := '/opt/homebrew/bin/gawk'

default:
  just --list

alias d := dev
alias f := fmt
alias g := gen

all: gen fix-typos fmt

gen:
  ./bin/last-modified
  ./bin/generate-index
  ./bin/generate-projects
  ./bin/forbid

dev:
  python3 -m http.server 8000 --directory ./docs

fix-typos:
  typos --write-changes

fmt:
  yapf --in-place --recursive . && isort .
  prettier --write .

dev-deps:
  brew install just prettier
  cargo install typos-cli
  curl https://raw.githubusercontent.com/jirutka/esh/master/esh > /usr/local/bin/esh
