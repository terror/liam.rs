set dotenv-load

export EDITOR := 'nvim'

alias d := dev
alias f := fmt
alias g := generate

default:
  just --list

all: fix-typos generate fmt forbid check

[group: 'check']
check:
  cargo fmt --all -- --check
  prettier --check .
  shellcheck ./bin/*
  uv run ruff check .
  uv run ruff format --check .

[group: 'check']
check-favicon port='https://liam.rs':
  npx realfavicon check {{ port }}

[group: 'dev']
dev:
  cargo run serve

[group: 'dev']
dev-deps:
  brew install \
    just \
    prettier \
    shellcheck \
    typos-cli \
    uv

  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

[group: 'fix']
fix-typos:
  typos --write-changes **/*.md

[group: 'format']
fmt:
  cargo fmt --all
  prettier --write .
  uv run ruff check --select I --fix && uv run ruff format

[group: 'check']
forbid:
  ./bin/forbid

[group: 'dev']
generate:
  ./bin/sync-post-timestamps
  cargo run -p generator

[group: 'dev']
generate-favicon image:
  npx realfavicon generate {{ image }} \
    favicon/favicon-settings.json \
    favicon/favicon-output.json \
    docs/favicon

  cat favicon-output.json | jq -r '.markups | sort | join("\n")' > favicon/favicon-output.txt

  rm favicon-output.json

[group: 'dev']
watch:
  cargo run -p generator -- serve
