#!/usr/bin/env bash

if (( $# != 1 )); then
  printf 'Usage: ck <problem>\n' >&2
  exit 2
fi

problem=$1
build_json=$(cargo build --release --bin="$problem" --message-format=json-render-diagnostics) || exit 1
test_command=$(
  printf '%s\n' "$build_json" |
    jq -er --arg problem "$problem" '
      select(
        .reason == "compiler-artifact"
        and .target.kind == ["bin"]
        and .target.name == $problem
      )
      | .executable
      | select(. != null)
      | @sh
    '
) || exit 1

exec oj test -c "$test_command" -d "testcases/$problem/samples"

