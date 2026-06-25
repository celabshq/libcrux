#!/bin/bash

set -e

SED=$(which gsed &>/dev/null && echo gsed || echo sed)

no_clean=0

# Parse command line arguments.
all_args=("$@")
while [ $# -gt 0 ]; do
    case "$1" in
    --no-clean) no_clean=1 ;;
    esac
    shift
done

# Extract the C code
if [[ "$no_clean" = 0 ]]; then
    # It's enough to clean sha3 to work around the charon bug.
    cargo clean -p libcrux-sha3
fi

./c.sh --config cg.yaml --out cg --mldsa65\
    --no-glue --no-unrolling --no-karamel_include --cpp17

if [[ -n "$BORINGSSL_HOME" ]]; then
    echo "Copying the files into $BORINGSSL_HOME/third_party/libcrux/"

    cp cg/libcrux_*.h $BORINGSSL_HOME/third_party/libcrux/
    cp cg/code_gen.txt $BORINGSSL_HOME/third_party/libcrux/
    cp -r cg/intrinsics $BORINGSSL_HOME/third_party/libcrux/

    # We use special files here.
    cp cg/boring/eurydice_glue.h $BORINGSSL_HOME/third_party/libcrux/
    cp -r cg/boring/karamel $BORINGSSL_HOME/third_party/libcrux/

    libcrux_rev=$(git rev-parse HEAD)
    echo "libcrux: $libcrux_rev" >> $BORINGSSL_HOME/third_party/libcrux/code_gen.txt
fi
