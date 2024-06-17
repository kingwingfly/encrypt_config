#! /bin/bash
set -e

export TERM=xterm-256color

# Statements waiting to be executed
statements=(
    ".github/scripts/test.sh"
    "cargo publish -p $1 --dry-run"
)

# loop echo and executing statements
for statement in "${statements[@]}"; do
    echo "$(tput setaf 3)$statement$(tput sgr0)"
    eval $statement
    echo
done