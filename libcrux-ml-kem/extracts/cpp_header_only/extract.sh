#!/bin/bash

set -e

SED=$(which gsed &>/dev/null && echo gsed || echo sed)

no_clean=0
no_extract=0
no_charon=

# Parse command line arguments.
all_args=("$@")
while [ $# -gt 0 ]; do
  case "$1" in
  --no-clean) no_clean=1 ;;
  --no-extract) no_extract=1 ;;
  --no-charon) no_charon=--no-charon ;;
  esac
  shift
done

# Extract the C code
if [[ "$no_clean" = 0 ]]; then
  cargo clean
fi

if [[ "$no_extract" = 0 ]]; then
  ./c.sh --mlkem768 --no-glue --no-unrolling \
    --no-karamel_include --cpp17 $no_charon
fi

if [[ -n "$BORINGSSL_HOME" ]]; then
  echo "Copying the files into $BORINGSSL_HOME/third_party/libcrux/"

  cp generated/libcrux_*.h $BORINGSSL_HOME/third_party/libcrux/
  cp generated/code_gen.txt $BORINGSSL_HOME/third_party/libcrux/
  cp -r generated/intrinsics $BORINGSSL_HOME/third_party/libcrux/

  # We use special files here.
  cp generated/boring/eurydice_glue.h $BORINGSSL_HOME/third_party/libcrux/
  cp -r generated/boring/karamel $BORINGSSL_HOME/third_party/libcrux/
fi
