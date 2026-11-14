#!/usr/bin/env bash

contest_id=${PWD##*/}
contest_json=$(oj-api get-contest "https://atcoder.jp/contests/$contest_id" 2>/dev/null) || exit 1

(
  set -o pipefail

  printf '%s\n' "$contest_json" |
    jq -r '.result.problems[] | [(.context.alphabet | ascii_downcase), .url] | @tsv' |
    {
      failed=0
      printf 'Downloaded:\n'
      while IFS=$'\t' read -r problem url; do
        if oj download "$url" -d "testcases/$problem/samples" </dev/null >/dev/null 2>&1; then
          printf '  %s\n' "$problem"
        else
          failed=1
        fi
      done

      (( failed==0 ))
    }
) || exit 1

