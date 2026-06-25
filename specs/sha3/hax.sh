#!/usr/bin/env bash
set -ex

function extract_all() {
    extract specs/sha3 \
        into -i "+**" \
        fstar --z3rlimit 80
}

function extract_all_lean() {
    extract_to_lean specs/sha3 \
        into -i "+**" \
        lean

    patch_lean_extractions
}

function prove() {
    case "$1" in
        --admit)
            shift 1
            export OTHERFLAGS="--admit_smt_queries true";;
        *);;
    esac
    go_to "specs/sha3"
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
    ROOT="$SCRIPT_DIR/../.."
    cd "$ROOT"
    cd "$1"
}

function msg() {
    echo -e "$1[$SCRIPT_NAME]$RESET $2"
}

function extract() {
    TARGET="$1"
    shift 1

    msg "$BLUE" "extract ${BOLD}$TARGET${RESET}"
    go_to "$TARGET"
    cargo hax "$@" || {
        msg "$RED" "extract extraction failed for ${BOLD}$TARGET${RESET}"
        exit 1
    }
}

function extract_to_lean() {
    TARGET="$1"
    shift 1

    msg "$BLUE" "extract (lean) ${BOLD}$TARGET${RESET}"
    go_to "$TARGET"
    cargo hax "$@" || {
        msg "$BLUE" "hax reported warnings for ${BOLD}$TARGET${RESET} (continuing)"
    }
}

function patch_lean_extractions() {
    go_to "specs/sha3"
    local sha3="proofs/lean/extraction/hacspec_sha3.lean"

    # Add Stubs import.
    sed -i '' '/^import Hax$/a\
import Stubs' "$sha3"

    # Replace all generated proof tactics with sorry.
    sed -i '' 's/by hax_construct_pure <;> bv_decide/by sorry/g' "$sha3"
    sed -i '' 's/by hax_mvcgen \[[^]]*\] <;> bv_decide/by sorry/g' "$sha3"
    sed -i '' 's/by hax_construct_pure <;> rfl/by sorry/g' "$sha3"

    # squeeze_state: the output parameter is RustSlice u8 but update_at_usize
    # is only defined for RustArray. Rename the slice calls to update_at_usize_slice.
    # Inside squeeze_state, `output` is a RustSlice, so all update_at_usize on it
    # need the slice variant.
    python3 -c "
import re, sys
t = open(sys.argv[1]).read()
# In squeeze_state, update_at_usize is called on 'output' which is a RustSlice.
# The Hax library only defines update_at_usize for RustArray, so we rename
# these calls to the slice-specific version defined in Stubs.lean.
# We match inside the squeeze_state definition (between 'def squeeze_state' and
# the next 'end' or 'def') and replace all occurrences.
def patch_squeeze(m):
    body = m.group(0)
    return body.replace(
        'rust_primitives.hax.monomorphized_update_at.update_at_usize',
        'rust_primitives.hax.monomorphized_update_at.update_at_usize_slice')
t = re.sub(
    r'def squeeze_state.*?(?=\nset_option|\ndef )',
    patch_squeeze,
    t, flags=re.DOTALL)
open(sys.argv[1],'w').write(t)
" "$sha3"

    # Remove sorry'd @[hax_spec] definitions that block mvcgen from using
    # our proven @[spec] triples. The hax-generated specs have
    # `ensures := fun _ => pure True` which is uninformative.
    python3 -c "
import re, sys
t = open(sys.argv[1]).read()
# Remove all @[hax_spec] attributes — the generated specs have sorry proofs
# and block mvcgen from using our proven @[spec] triples.
t = t.replace('@[hax_spec]', '-- @[hax_spec] -- removed by patch')
open(sys.argv[1],'w').write(t)
" "$sha3"

    # Remove broken createi definition (our Stubs.lean provides it).
    python3 -c "
import re, sys
t = open(sys.argv[1]).read()
t = re.sub(
    r'--  Utility function to create.*?end hacspec_sha3',
    'end hacspec_sha3',
    t, flags=re.DOTALL)
open(sys.argv[1],'w').write(t)
" "$sha3"
}

function help() {
    echo "Hacspec SHA3 script to extract Rust to F* and Lean via hax."
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo ""
    grep '[#]>' "$SCRIPT_PATH" | sed 's/[)] #[>]/\t/g'
    echo ""
}

function cli() {
    if [ -z "$1" ]; then
        help
        exit 1
    fi

    case "$1" in
        --help) #> Show help message
            help;;
        extract) #> Extract the F* code for the proofs.
            extract_all
            msg "$GREEN" "done"
            ;;
        extract_lean) #> Extract Lean code for the proofs.
            extract_all_lean
            msg "$GREEN" "done"
            ;;
        prove) #> Run F*. This typechecks the extracted code. To lax-typecheck use --admit.
            shift 1
            prove "$@";;
        *)
            echo "Invalid option: $1"
            help
            exit 1;;
    esac
}

init_vars
cli "$@"
