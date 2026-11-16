#!/bin/bash

set -e

cwd=$(cd "$(dirname "$0")"; pwd -P)

docker=0
build=0
test=0
extract=1

while [ $# -gt 0 ]; do
    case "$1" in
        --docker) docker=1 ;;
        --no-extract) extract=0 ;;
        --build) build=1;;
        --test) test=1;;
        *) echo "Unknown argument"; exit 2 ;;
    esac
    shift
done

# Extract
if (( docker == 1 && extract == 1 )); then
    echo "Extracting with docker ..."
    sudo docker pull ghcr.io/cryspen/libcrux-c:latest
    sudo docker run -v "$PWD":/home/user/libcrux \
        --rm ghcr.io/cryspen/libcrux-c:latest bash \
        -c "$cwd/libcrux-ml-kem/extracts/extract-all.sh && \
            cd $cwd/crates/algorithms/ml-dsa && ./boring.sh"
elif (( extract == 1 )); then
    echo "Extracting locally ..."
    echo "  ML-KEM ..."
    "$cwd"/libcrux-ml-kem/extracts/extract-all.sh
    echo "  ML-DSA ..."
    (cd "$cwd"/crates/algorithms/ml-dsa && ./boring.sh)
fi

if (( build == 0 && test == 0 )); then
    echo "Run build and tests with --build and --test"
fi

# Build & test

# ML-KEM
ml_kem=(
    libcrux-ml-kem/extracts/c/generated
    libcrux-ml-kem/extracts/c_header_only/generated
    libcrux-ml-kem/extracts/cpp_header_only/generated
)

build_mlkem() {
    cd "$cwd/$1"
    LIBCRUX_BENCHMARKS=1 CC=clang-19 CXX=clang++-19 cmake -B build -G "Ninja Multi-Config"
    cmake --build build
    cd "$cwd"
}

test_mlkem() {
    cd "$cwd/$1"
    ./build/Debug/ml_kem_test
    cd "$cwd"
}

if (( build == 1 )); then
    for path in "${ml_kem[@]}"; do
        echo "--------------------------------"
        echo "Building: $path"
        build_mlkem "$path"
    done
fi

if (( test == 1 )); then
    for path in "${ml_kem[@]}"; do
        echo "--------------------------------"
        echo "Testing: $path"
        test_mlkem "$path"
    done
fi

# ML-DSA
if (( build == 1 )); then
    echo "--------------------------------"
    echo "Building: ML-DSA"
    cd "$cwd"/crates/algorithms/ml-dsa/cg
    CC=clang-18 CXX=clang++-18 cmake -B build -G "Ninja Multi-Config"
    cmake --build build
    cd "$cwd"
fi

if (( test == 1 )); then
    echo "--------------------------------"
    echo "Testing: ML-DSA"
    cd $cwd/crates/algorithms/ml-dsa/cg
    ./build/Debug/ml_dsa_test
    cd "$cwd"
fi
