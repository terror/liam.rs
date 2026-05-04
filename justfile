set dotenv-load

export EDITOR := 'nvim'

alias d := dev
alias f := fmt
alias g := generate

default:
  just --list

all: fix-typos generate fmt-docs forbid

[group: 'check']
check-favicon port='https://liam.rs':
  npx realfavicon check {{ port }}

[group: 'dev']
dev:
  cargo run serve

[group: 'fix']
fix-typos:
  typos --write-changes **/*.md

[group: 'format']
fmt:
  cargo fmt --all

[group: 'format']
[working-directory: 'docs']
fmt-docs:
  prettier --write .

[group: 'check']
forbid:
  ./bin/forbid

[group: 'dev']
generate:
  cargo run generate

[group: 'dev']
generate-favicon image:
  npx realfavicon generate {{ image }} \
    favicon/favicon-settings.json \
    favicon/favicon-output.json \
    docs/favicon

  cat favicon-output.json | jq -r '.markups | sort | join("\n")' > favicon/favicon-output.txt

  rm favicon-output.json
