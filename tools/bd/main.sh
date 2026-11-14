#!/usr/bin/env bash

if (( $# != 1 )); then
  printf 'Usage: bd <problem>\n' >&2
  exit 2
fi

problem=$1

build_json=$(cargo build --release --bin="$problem" --message-format=json-render-diagnostics) || exit 1
library_manifest=$(
  printf '%s\n' "$build_json" |
    jq -er '
      select(
        .reason == "compiler-artifact"
        and .target.kind == ["lib"]
        and .target.name == "cplib"
      )
      | .manifest_path
      | select(. != null)
    '
) || exit 1
library_dir=$(dirname "$library_manifest") || exit 1

bundled_code=$(bundle-rs --lib "$library_dir" "src/bin/$problem.rs") || exit 1
printf '%s\n' "$bundled_code" | wl-copy --type text/plain || exit 1

