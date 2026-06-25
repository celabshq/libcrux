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
mlkem_root=$(realpath ../../)
repo_root=$(realpath ../../../)

portable_only=0
no_hacl=0
no_charon=0
clean=0
config=$extract_root/extract.yaml
out=generated
glue=$EURYDICE_HOME/include/eurydice_glue.h
features=""
eurydice_glue=1
karamel_include=1
unrolling=16
format=1
cpp17=

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
        --mlkem768) features="${features} --cargo-arg=--no-default-features --cargo-arg=--features=mlkem768" ;;
        --mlkem768-1024) features="${features} --cargo-arg=--no-default-features --cargo-arg=--features=mlkem768 --cargo-arg=--features=mlkem1024" ;;
        --kyber768) features="${features} --cargo-arg=--features=kyber" ;;
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
    cargo clean
fi

# TODO: add LIBCRUX_ENABLE_SIMD128=1 LIBCRUX_ENABLE_SIMD256=1 charon invocations
if [[ "$no_charon" = 0 ]]; then
    cargo clean -p libcrux-sha3
    rm -rf $repo_root/libcrux_ml_kem.llbc $repo_root/libcrux_sha3.llbc $repo_root/libcrux_secrets.llbc

    flags=
    if [[ $(uname -m) == "arm64" ]]; then
        flags+="-- --target=x86_64-apple-darwin"
    fi

    echo "Running charon (all) ..."
    RUSTFLAGS="-Cdebug-assertions=no --cfg eurydice" $CHARON_HOME/bin/charon cargo \
       $features \
      --preset eurydice \
      --include 'libcrux_sha3' \
      --include 'libcrux_secrets' \
      --start-from libcrux_ml_kem --start-from libcrux_sha3 \
      --include 'core::num::*::BITS' --include 'core::num::*::MAX' $flags
    if ! [[ -f $repo_root/libcrux_ml_kem.llbc ]]; then
        echo "😱😱😱 You are the victim of this bug: https://hacspec.zulipchat.com/#narrow/stream/433829-Circus/topic/charon.20declines.20to.20generate.20an.20llbc.20file"
        echo "Suggestion: rm -rf $repo_root/target or cargo clean"
        exit 1
    fi
else
    echo "Skipping charon"
fi

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
if [[ -z "$FSTAR_REV" && -d $FSTAR_HOME/.git ]]; then
    export FSTAR_REV=$(git -C $FSTAR_HOME rev-parse HEAD)
else
    export FSTAR_REV=$(fstar.exe --version | grep commit | sed 's/commit=\(.*\)/\1/')
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

# Generate header
cat spdx-header.txt > header.txt
sed -e 's/^/ * /' code_gen.txt >> header.txt
echo " */" >> header.txt

# Run eurydice to extract the C code
echo "Running eurydice ..."
echo $EURYDICE_HOME/eurydice \
    --config "$config" -funroll-loops $unrolling \
    --header header.txt $cpp17 \
    "$repo_root/libcrux_ml_kem.llbc" --keep-going

$EURYDICE_HOME/eurydice \
    --config "$config" -funroll-loops $unrolling \
    --header header.txt $cpp17 \
    "$repo_root/libcrux_ml_kem.llbc" --keep-going

if [[ "$eurydice_glue" = 1 ]]; then
    cp "$glue" .
fi

if [[ "$karamel_include" = 1 ]]; then
    echo "Copying karamel/include ..."
    mkdir -p karamel
    cp -R "$KRML_HOME/include" karamel/
fi

if [[ "$format" = 1 ]]; then
    find . -type f -name '*.c' -and -not -path '*_deps*' -exec clang-format-18 --style=Google -i "{}" \;
    find . -type f -name '*.h' -and -not -path '*_deps*' -exec clang-format-18 --style=Google -i "{}" \;
    find . -type f -name '*.cc' -and -not -path '*_deps*' -exec clang-format-18 --style=Google -i "{}" \;
    if [ -d "internal" ]; then
        clang-format-18 --style=Google -i internal/*.h
    fi
    clang-format-18 --style=Google -i intrinsics/*.h
fi
