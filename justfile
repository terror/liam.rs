alias d := dev
alias f := fmt
alias g := gen

default:
  just --list

all: gen fix-typos fmt

gen:
  ./gen.sh

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
