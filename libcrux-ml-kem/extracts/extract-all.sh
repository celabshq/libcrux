#!/usr/bin/env bash

set -e

cwd=$(cd $(dirname $0); pwd -P)

(cd $cwd/c_header_only && ./extract.sh "$@")
(cd $cwd/cpp_header_only && ./extract.sh "$@")
(cd $cwd/c && ./extract.sh "$@")
