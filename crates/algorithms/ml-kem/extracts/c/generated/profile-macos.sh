#!/bin/sh

set -o errexit
set -o nounset

if [ "$#" -lt 1 ]
then
  echo "Usage $0 <program> [arguments...]" 1>&2
  exit 1
fi

PROGRAM="$(realpath "$1")"
shift

OUTPUT="/tmp/cpu_profile_$(whoami)_$(basename "$PROGRAM").trace"
echo "Profiling $PROGRAM into $OUTPUT" 1>&2
# Delete potential previous traces
rm -rf "$OUTPUT"
xcrun xctrace record \
  --template 'CPU Profiler' \
  --no-prompt \
  --output "$OUTPUT" \
  --target-stdout - \
  --launch -- "$PROGRAM" "$@"
open "$OUTPUT"
