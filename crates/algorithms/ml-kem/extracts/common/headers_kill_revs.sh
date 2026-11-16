#!/bin/bash
# This script removes specific revision number lines from files that
# have them, as well as removing the `code_gen.txt` file that also
# stores only this information. Its purpose is to prepare two
# extractions for diffing of extraction results independent of
# concrete tool revs used for extraction.
#
# Run this from the repository root.

grep -rl -e "This code was generated with the following revisions:" "$1" | xargs -d '\n' sed -i -f crates/algorithms/ml-kem/extracts/common/header_kill_revs.sed
rm -f "$1/code_gen.txt"
rm -f "$1/header.txt"
