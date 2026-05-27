#!/usr/bin/env bash
set -ex

function extract_all() {
    extract crates/sys/platform \
        into -i "+:** -**::x86::init::cpuid -**::x86::init::cpuid_count" \
        fstar --z3rlimit 80 --interfaces "+**"
    
    extract crates/utils/core-models into fstar
    rename_core_models_files crates/utils/core-models

    extract crates/utils/intrinsics \
        -C --features simd128,simd256 ";" \
        into -i "-core_models::**" \
        fstar --z3rlimit 80 --interfaces "+**"

    extract crates/utils/secrets \
        into -i "+**" \
        fstar --z3rlimit 80
    
    extract crates/algorithms/sha3 \
        -C --features simd128,simd256 ";" \
        into -i "+**" \
        fstar --z3rlimit 80
}

function prove() {
    case "$1" in
        --admit)
            shift 1
            export OTHERFLAGS="--admit_smt_queries true";;
        *);;
    esac
    go_to "crates/algorithms/sha3"
    JOBS="${JOBS:-$(nproc --all)}"
    JOBS="${JOBS:-4}"
    make -C proofs/fstar/extraction -j $JOBS "$@"
}

function init_vars() {
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    SCRIPT_NAME="$(basename "${BASH_SOURCE[0]}")"
    SCRIPT_PATH="${SCRIPT_DIR}/${SCRIPT_NAME}"

    if [ -t 1 ]; then
        BLUE='\033[34m'
        GREEN='\033[32m'
        BOLD='\033[1m'
        RESET='\033[0m'
    else
        BLUE=''
        GREEN=''
        BOLD=''
        RESET=''
    fi
}

function go_to() {
    ROOT="$SCRIPT_DIR/../../.."
    cd "$ROOT"
    cd "$1"
}

function msg() {
    echo -e "$1[$SCRIPT_NAME]$RESET $2"
}

function rename_core_models_files() {
    TARGET="$1"
    shift 1
    go_to "$TARGET"

    local target_dir="proofs/fstar/extraction"
    find "$target_dir" -type f -name "Core_models*" | while read -r file; do
        dir_path="${file%/*}"
        filename="${file##*/}"
        new_filename="Libcrux_core_models${filename#Core_models}"
        mv "$file" "$dir_path/$new_filename"
    done
    find "$target_dir" -type f \( -name "*.fst" -o -name "*.fsti" \) -exec sed -i'' \
        -e 's/module Core_models/module Libcrux_core_models/g' \
        {} +
}

function rename_core_models_uses() {
    local target_dir="proofs/fstar/extraction"
    find "$target_dir" -type f \( -name "*.fst" -o -name "*.fsti" \) -exec sed -i'' \
        -e 's/Core_models\.Abstractions/Libcrux_core_models.Abstractions/g' \
        -e 's/Core_models\.Core_arch/Libcrux_core_models.Core_arch/g' \
        {} +
}

function extract() {
    TARGET="$1"
    shift 1

    msg "$BLUE" "extract ${BOLD}$TARGET${RESET}"
    go_to "$TARGET"
    cargo hax "$@" || {
        msg "$RED" "extract extraction failed for ${BOLD}$1${RESET}"
        exit 1
    }
    rename_core_models_uses
}

function help() {
    echo "Libcrux script to extract Rust to F* via hax."
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Comands:"
    echo ""
    grep '[#]>' "$SCRIPT_PATH" | sed 's/[)] #[>]/\t/g'
    echo ""
}

function cli() {
    if [ -z "$1" ]; then
        help
        exit 1
    fi
    # Check if an argument was provided

    case "$1" in
        --help) #> Show help message
            help;;
        extract) #> Extract the F* code for the proofs.
            extract_all
            msg "$GREEN" "done"
            ;;
        prove) #> Run F*. This typechecks the extracted code. To lax-typecheck use --admit.
            shift 1
            prove "$@";;
        extract+prove) #> Equivalent to extracting and proving.
            shift 1
            extract_all
            prove "$@";;
        *)
            echo "Invalid option: $1"
            help
            exit 1;;
    esac
}

init_vars
cli "$@"
