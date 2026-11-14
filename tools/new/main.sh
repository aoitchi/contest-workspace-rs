#!/usr/bin/env bash

if (( $# < 2 )); then
  printf 'Usage: new <contest-id> <problem> ...\n' >&2
  exit 2
fi

contest_id=$1
shift
if [[ ! "$contest_id" =~ ^[a-zA-Z0-9][a-zA-Z0-9_-]*$ ]]; then
  printf 'Invalid contest ID: %s\n' "$contest_id" >&2
  exit 2
fi

if [[ -e "$contest_id" ]]; then
  printf '%s already exists\n' "$contest_id" >&2
  exit 1
fi

for problem in "$@"; do
  if [[ ! "$problem" =~ ^[a-zA-Z0-9_-]+$ ]]; then
    printf 'Invalid problem name: %s\n' "$problem" >&2
    exit 2
  fi
done

mkdir -p "$contest_id/src/bin" || exit 1
script_path=$(realpath "${BASH_SOURCE[0]}") || exit 1
script_dir=$(dirname "$script_path") || exit 1
sed "s/@CONTEST@/$contest_id/g" "$script_dir/templates/atcoder/Cargo.toml" > "$contest_id/Cargo.toml" || exit 1
printf 'Created files:\n'
for problem in "$@"; do
  install -m 644 "$script_dir/templates/atcoder/main.rs" "$contest_id/src/bin/$problem.rs" || exit 1
  printf '  %s\n' "$contest_id/src/bin/$problem.rs"
done
