#!/usr/bin/env bash

#   ./c.sh --no-glue --no-unrolling \
#    --no-karamel_include --cpp17 $no_charon

set -ex
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
# mlkem_root=$(realpath ../../)
repo_root=$(realpath ../)

portable_only=0
no_hacl=0
no_charon=0
clean=0
config=$extract_root/extract.yaml
out=generated
glue=$EURYDICE_HOME/include/eurydice_glue.h
features_mlkem="${features} --no-default-features --features=mlkem768"
features_mldsa="${features} --no-default-features --features=mldsa65"
eurydice_glue=1
karamel_include=1
unrolling=16
format=1
cpp17=-fc++17-compat

# Parse command line arguments.
all_args=("$@")
while [ $# -gt 0 ]; do
    case "$1" in
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
    
    cd $repo_root/libcrux-ml-dsa
    echo "Running charon (ML-DSA) ..."
    RUSTFLAGS="--cfg eurydice" $CHARON_HOME/bin/charon cargo \
             --rustc-arg="-Cdebug-assertions=no" \
             --preset eurydice \
             --remove-associated-types '*' \
             --include 'core::num::*::BITS' --include 'core::num::*::MAX' \
             $flags $features_mldsa

    
    # rm -rf $repo_root/libcrux_ml_kem.llbc $repo_root/libcrux_sha3.llbc $repo_root/libcrux_secrets.llbc
    # echo "Running charon (secrets) ..."
    # (cd $repo_root/secrets && RUSTFLAGS="--cfg eurydice" $CHARON_HOME/bin/charon --remove-associated-types '*' --translate-all-methods)
    if ! [[ -f $repo_root/libcrux_ml_kem.llbc || -f $repo_root/libcrux_ml_dsa.llbc ]]; then
        echo "😱😱😱 You are the victim of this bug: https://hacspec.zulipchat.com/#narrow/stream/433829-Circus/topic/charon.20declines.20to.20generate.20an.20llbc.20file"
        echo "Suggestion: rm -rf $repo_root/target or cargo clean"
        exit 1
    fi
    # # Because of a Charon bug we have to clean the sha3 crate.
    # cargo clean -p libcrux-sha3
    # echo "Running charon (sha3) ..."
    # (cd $repo_root/libcrux-sha3 && RUSTFLAGS="--cfg eurydice" $CHARON_HOME/bin/charon --remove-associated-types '*' --rustc-arg=-Cdebug-assertions=no)
    # if ! [[ -f $repo_root/libcrux_sha3.llbc ]]; then
    #     echo "😱😱😱 You are the victim of this bug: https://hacspec.zulipchat.com/#narrow/stream/433829-Circus/topic/charon.20declines.20to.20generate.20an.20llbc.20file"
    #     echo "Suggestion: rm -rf $repo_root/target or cargo clean"
    #     exit 1
    # fi
    # echo "Running charon (ml-kem) ..."
    # RUSTFLAGS="--cfg eurydice" $CHARON_HOME/bin/charon --remove-associated-types '*' --rustc-arg=-Cdebug-assertions=no $features
else
    echo "Skipping charon"
fi

cd $extract_root
mkdir -p $out
cd $out

# Clean only when requesting it.
# Note that we can not extract for all platforms on any platform right now.
# Make sure to keep files from other platforms.
if [[ "$clean" = 1 ]]; then
    rm -rf libcrux_*.c libcrux_*.h
    rm -rf internal/*.h
fi

# Write out infos about the used tools
[[ -z "$CHARON_REV" && -d $CHARON_HOME/.git ]] && export CHARON_REV=$(git -C $CHARON_HOME rev-parse HEAD)
[[ -z "$EURYDICE_REV" && -d $EURYDICE_HOME/.git ]] && export EURYDICE_REV=$(git -C $EURYDICE_HOME rev-parse HEAD)
[[ -z "$KRML_REV" && -d $KRML_HOME/.git ]] && export KRML_REV=$(git -C $KRML_HOME rev-parse HEAD)
[[ -z "$LIBCRUX_REV" ]] && export LIBCRUX_REV=$(git rev-parse HEAD)
if [[ -z "$FSTAR_REV" ]]; then
    if [[ -d $FSTAR_HOME/.git ]]; then
        export FSTAR_REV=$(git -C $FSTAR_HOME rev-parse HEAD)
    else
        export FSTAR_REV=$(fstar.exe --version | grep commit | sed 's/commit=\(.*\)/\1/')
    fi
fi
rm -f code_gen.txt
echo "This code was generated with the following revisions:" >> code_gen.txt
echo -n "Charon: " >> code_gen.txt
echo "$CHARON_REV" >> code_gen.txt
echo -n "Eurydice: " >> code_gen.txt
echo "$EURYDICE_REV" >> code_gen.txt
echo -n "Karamel: " >> code_gen.txt
echo "$KRML_REV" >> code_gen.txt
echo -n "F*: " >> code_gen.txt
echo "$FSTAR_REV" >> code_gen.txt
echo -n "Libcrux: " >> code_gen.txt
echo "$LIBCRUX_REV" >> code_gen.txt

# # Generate header
cat spdx-header.txt > header.txt
sed -e 's/^/ * /' code_gen.txt >> header.txt
echo " */" >> header.txt

#  --log "*" --debug checker
$EURYDICE_HOME/eurydice \
    --debug "-dast" \
    --config "$config" -funroll-loops 0 \
    --header header.txt \
    $cpp17 \
    "$repo_root/libcrux_secrets.llbc" "$repo_root/libcrux_sha3.llbc" "$repo_root/libcrux_ml_kem.llbc" "$repo_root/libcrux_ml_dsa.llbc" --keep-going

