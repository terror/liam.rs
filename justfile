set dotenv-load

default:
  just --list

alias f := fmt
alias r := run
alias t := test

all: build test clippy fmt-check

[group: 'misc']
build:
  cargo build

[group: 'check']
check:
  cargo check

[group: 'check']
check-favicon port='https://liam.rs':
  npx realfavicon check {{ port }}

[group: 'check']
ci: test clippy forbid
  cargo fmt --all -- --check
  cargo update --locked --package generator

[group: 'check']
clippy:
  cargo clippy --all --all-targets

[group: 'format']
fmt:
  cargo fmt --all

[group: 'format']
fmt-check:
  cargo fmt --all -- --check

[group: 'check']
forbid:
  ./bin/forbid

[group: 'dev']
generate-favicon image:
  npx realfavicon generate {{ image }} \
    favicon/favicon-settings.json \
    favicon/favicon-output.json \
    docs/favicon

  cat favicon-output.json | jq -r '.markups | sort | join("\n")' > favicon/favicon-output.txt

  rm favicon-output.json

[group: 'dev']
run *args:
  cargo run {{ args }}

[group: 'test']
test:
  cargo test

[group: 'fix']
typos:
  typos --write-changes **/*.md

[group: 'format']
[working-directory: 'docs']
web-format:
  prettier --write .
