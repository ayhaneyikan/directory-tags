#!/bin/bash

# UPDATE WITH YOUR PATH
PROJECT_DIR=$HOME/dev/rust/directory-tags
BINARY_PATH=$PROJECT_DIR/target/release/cdt


# Wraps cdt binary and cd's to the given directory
function cdt() {
  # cdt returns
  #  2 when cd is necessary
  target=$($BINARY_PATH "$@")
  if [ $? -eq 2 ]; then
    cd "$target"
  else
    echo "$target"
  fi
}


# cdt autocomplete function
function _cdt_complete() {
  local cur prev opts
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD-1]}"

  # Only suggest tags when:
  # - we're not completing an option (starts with -)
  # - and no options like --add or --list-tags are present
  if [[ "$cur" != -* ]]; then
    # Run cdt --list-tags and collect lines
    if tags=$($BINARY_PATH --list-tags 2>/dev/null); then
      COMPREPLY=( $(compgen -W "$tags" -- "$cur") )
    fi
  fi
}

complete -F _cdt_complete cdt
