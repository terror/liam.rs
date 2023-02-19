export ESH_AWK := '/usr/local/bin/gawk'

default:
  just --list

alias d := dev
alias f := fmt
alias g := gen

all: gen fix-typos fmt

gen:
  ./bin/last-modified
  ./gen.sh
  ./bin/forbid

dev:
  open -a 'Google Chrome' ./docs/index.html

fix-typos:
  typos --write-changes

fmt:
  prettier --write .

dev-deps:
  brew install just prettier
  cargo install typos-cli
  curl https://raw.githubusercontent.com/jirutka/esh/master/esh > /usr/local/bin/esh
