#!/usr/bin/env bash

set -e
set -o pipefail

if [[ -z "$CHARON_HOME" ]]; then
    echo "Please set CHARON_HOME to the Charon directory" 1>&2
    exit 1
fi
if [[ -z "$EURYDICE_HOME" ]]; then
    echo "Please set EURYDICE_HOME to the Eurydice directory" 1>&2
    exit 1
fi
if [[ -z "$KRML_HOME" ]]; then
    echo "Please set KRML_HOME to the KaRaMeL directory" 1>&2
    exit 1
fi

extract_root=$(pwd)
script_path=$(realpath "$0")
# mlkem_root=$(realpath ../../)
repo_root=$(realpath ../)

portable_only=0
no_hacl=0
no_charon=0
clean=0
both=1
config=$extract_root/extract-c.yaml
out=c
glue=$EURYDICE_HOME/include/eurydice_glue.h
features_mlkem="${features} --no-default-features --features=mlkem512 --features=mlkem768 --features=mlkem1024"
features_mldsa="${features} --no-default-features --features=mldsa65 --features=mldsa44 --features=mldsa87"
eurydice_glue=1
karamel_include=1
unrolling=16
format=1
cpp17=

# Run Eurydice for a single output directory.
# Arguments: <out_dir> <config_path> [<cpp17_flag>]
run_extraction() {
    local out_dir="$1"
    local config_path="$2"
    local cpp17_arg="${3:-}"

    cd "$extract_root"
    mkdir -p "$out_dir"
    cd "$out_dir"

    if [[ "$clean" = 1 ]]; then
        rm -rf libcrux_*.c libcrux_*.h
        rm -rf internal/*.h
    fi

    rm -f code_gen.txt
    echo "This code was generated with the following revisions:" >> code_gen.txt
    echo -n "Charon: "   >> code_gen.txt; echo "$CHARON_REV"   >> code_gen.txt
    echo -n "Eurydice: " >> code_gen.txt; echo "$EURYDICE_REV" >> code_gen.txt
    echo -n "Karamel: "  >> code_gen.txt; echo "$KRML_REV"     >> code_gen.txt
    echo -n "F*: "       >> code_gen.txt; echo "$FSTAR_REV"    >> code_gen.txt
    echo -n "Libcrux: "  >> code_gen.txt; echo "$LIBCRUX_REV"  >> code_gen.txt

    cat spdx-header.txt > header.txt
    sed -e 's/^/ * /' code_gen.txt >> header.txt
    echo " */" >> header.txt

    #  --log "*" --debug checker
    $EURYDICE_HOME/eurydice \
        --debug "-dast" \
        --config "$config_path" -funroll-loops $unrolling \
        --header header.txt \
        $cpp17_arg \
        "$repo_root/libcrux_secrets.llbc" "$repo_root/libcrux_sha3.llbc" \
        "$repo_root/libcrux_ml_kem.llbc"  "$repo_root/libcrux_ml_dsa.llbc" \
        --keep-going
}

# Parse command line arguments.
all_args=("$@")
while [ $# -gt 0 ]; do
    case "$1" in
        --header-only)
            config="$extract_root/extract-c-header.yaml"
            out=c-header-only
            cpp17=-fc++17-compat
            both=0
            ;;
        --c-only) both=0 ;;
        -p | --portable) portable_only=1 ;;
        --no-hacl) no_hacl=1 ;;
        --no-charon) no_charon=1 ;;
        -c | --clean) clean=1 ;;
        --config) config="$2"; shift ;;
        --out) out="$2"; shift ;;
        --glue) glue="$2"; shift ;;
        --no-glue) eurydice_glue=0 ;;
        --no-karamel_include) karamel_include=0 ;;
        --no-unrolling) unrolling=0 ;;
        --no-format) format=0 ;;
        --cpp17) cpp17=-fc++17-compat ;;
    esac
    shift
done

# we will cd to a subdirectory later. We need to resolve paths, because relative paths won't bw valid anymore.
glue=$(realpath "$glue")
config=$(realpath "$config")

if [[ "$portable_only" = 1 ]]; then
    export LIBCRUX_DISABLE_SIMD256=1
    export LIBCRUX_DISABLE_SIMD128=1
fi

if [[ "$clean" = 1 ]]; then
    pushd $repo_root
    cargo clean
    popd
fi

# TODO: add LIBCRUX_ENABLE_SIMD128=1 LIBCRUX_ENABLE_SIMD256=1 charon invocations
if [[ "$no_charon" = 0 ]]; then
    pushd $repo_root
    cargo clean
    popd
    rm -rf $repo_root/libcrux_ml_kem.llbc $repo_root/libcrux_sha3.llbc $repo_root/libcrux_ml_dsa.llbc

    flags="-- "
    if [[ $(uname -m) == "arm64" ]]; then
       flags+="--target=x86_64-apple-darwin "
    fi

    cd $repo_root/crates/utils/secrets
    echo "Running charon (secrets) ..."
    RUSTFLAGS="--cfg eurydice" $CHARON_HOME/bin/charon cargo \
             --rustc-arg="-Cdebug-assertions=no" \
             --preset eurydice \
             --remove-associated-types '*' \
             --include 'core::num::*::BITS' --include 'core::num::*::MAX' $flags

    cd $repo_root/crates/algorithms/sha3
    echo "Running charon (SHA3) ..."
    RUSTFLAGS="--cfg eurydice" $CHARON_HOME/bin/charon cargo \
             --rustc-arg="-Cdebug-assertions=no" \
              --preset eurydice \
             --remove-associated-types '*' \
             --include 'core::num::*::BITS' --include 'core::num::*::MAX' $flags

    cd $repo_root/libcrux-ml-kem
    echo "Running charon (ML-KEM) ..."
    RUSTFLAGS="--cfg eurydice" $CHARON_HOME/bin/charon cargo \
             --rustc-arg="-Cdebug-assertions=no" \
             --preset eurydice \
             --include 'core::num::*::BITS' --include 'core::num::*::MAX' $flags $features_mlkem

    cd $repo_root/crates/algorithms/ml-dsa
    echo "Running charon (ML-DSA) ..."
    RUSTFLAGS="--cfg eurydice" $CHARON_HOME/bin/charon cargo \
             --rustc-arg="-Cdebug-assertions=no" \
             --preset eurydice \
             --remove-associated-types '*' \
             --include 'core::num::*::BITS' --include 'core::num::*::MAX' \
             $flags $features_mldsa

    if ! [[ -f $repo_root/libcrux_ml_kem.llbc || -f $repo_root/libcrux_ml_dsa.llbc ]]; then
        echo "😱😱😱 You are the victim of this bug: https://hacspec.zulipchat.com/#narrow/stream/433829-Circus/topic/charon.20declines.20to.20generate.20an.20llbc.20file"
        echo "Suggestion: rm -rf $repo_root/target or cargo clean"
        exit 1
    fi
else
    echo "Skipping charon"
fi

# Compute toolchain provenance once; run_extraction writes it into each output dir.
[[ -z "$CHARON_REV"   && -d $CHARON_HOME/.git   ]] && export CHARON_REV=$(git   -C $CHARON_HOME   rev-parse HEAD)
[[ -z "$EURYDICE_REV" && -d $EURYDICE_HOME/.git ]] && export EURYDICE_REV=$(git -C $EURYDICE_HOME rev-parse HEAD)
[[ -z "$KRML_REV"     && -d $KRML_HOME/.git     ]] && export KRML_REV=$(git     -C $KRML_HOME     rev-parse HEAD)
[[ -z "$LIBCRUX_REV"  ]] && export LIBCRUX_REV=$(git rev-parse HEAD)
if [[ -z "$FSTAR_REV" ]]; then
    if [[ -d $FSTAR_HOME/.git ]]; then
        export FSTAR_REV=$(git -C $FSTAR_HOME rev-parse HEAD)
    else
        export FSTAR_REV=$(fstar.exe --version | grep commit | sed 's/commit=\(.*\)/\1/')
    fi
fi

if [[ "$both" = 1 ]]; then
    # Run each extraction as a separate sequential invocation so they get a
    # clean environment. Charon has already run above, so pass --no-charon
    # for both passes to avoid running it again.
    cd "$extract_root"
    bash "$script_path" "${all_args[@]}" --c-only      --no-charon
    cd "$extract_root"
    bash "$script_path" "${all_args[@]}" --header-only --no-charon
else
    run_extraction "$out" "$config" "$cpp17"
fi
