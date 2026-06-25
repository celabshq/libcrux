#!/usr/bin/env bash
set -e

function extract_all() {
    extract crates/sys/platform \
        into -i "+:** -**::x86::init::cpuid -**::x86::init::cpuid_count" \
        fstar --z3rlimit 80 --interfaces "+**"
    
    extract crates/utils/core-models into fstar

    rename_core_models_files crates/utils/core-models

    extract crates/utils/intrinsics \
        -C --features simd128,simd256 ";" \
        into -i "-core_models::**" \
        fstar --z3rlimit 80
    
    extract libcrux-ml-dsa \
        -C --features simd128,simd256 ";" \
        into -i "+**" \
             -i "-libcrux_ml_dsa::hash_functions::portable::*" \
             -i "-libcrux_ml_dsa::hash_functions::simd256::*" \
             -i "-libcrux_ml_dsa::hash_functions::neon::*" \
             -i "+:libcrux_ml_dsa::hash_functions::*::*" \
             -i "-**::types::non_hax_impls::**" \
        fstar --z3rlimit 80

    patch_matrix_typeclass libcrux-ml-dsa
}

# Post-extraction patch: hax extracts `ntt_dot_accumulate`'s inner fold
# body with a tuple-state lambda `(a_as_ntt, result)` whose later
# `update_at_usize result i (add_to_ring_element ... (result.[ i ] <:
# T) ...)` line trips F*'s `Index` typeclass tactic under the heavy
# ambient context.  Documented fix (sprint3.5-as1-attempt.md): in the
# inner-fold body lambda only, rename binder `(result: t_Slice ...)`
# to `(out: ...)` and add an explicit `(out <: t_Slice ...).[ i ]`
# ascription on the indexing inside the `add_to_ring_element` call.
# The patch is stored as a unified diff and applied with `git apply
# --3way` so small line shifts (e.g. from spec annotation edits
# upstream) don't break it; `--check` first lets us skip cleanly when
# already applied.
function patch_matrix_typeclass() {
    TARGET="$1"
    shift 1
    go_to "$TARGET"

    local target_file="proofs/fstar/extraction/Libcrux_ml_dsa.Matrix.fst"
    local patch_file="proofs/fstar/extraction/Libcrux_ml_dsa.Matrix.typeclass.patch"

    if [ ! -f "$target_file" ]; then
        msg "$BLUE" "patch_matrix_typeclass: ${BOLD}$target_file${RESET} not found, skipping"
        return
    fi
    if [ ! -f "$patch_file" ]; then
        msg "$BLUE" "patch_matrix_typeclass: ${BOLD}$patch_file${RESET} not found, skipping"
        return
    fi

    # Already applied?  Detect via the patched binder name.
    if grep -q '^                  (out: t_Slice (Libcrux_ml_dsa\.Polynomial\.t_PolynomialRingElement v_SIMDUnit)) =$' "$target_file"; then
        msg "$BLUE" "patch_matrix_typeclass: already applied, skipping"
        return
    fi

    # Plain `git apply` (no `--3way` — Matrix.fst is gitignored, not in
    # the index, so 3-way fallback can't resolve).  `--recount` lets the
    # hunk header line counts drift if upstream Rust edits change Matrix.fst's
    # extraction shape slightly.
    if git apply --recount "$patch_file" 2>&1; then
        msg "$BLUE" "patched ${BOLD}$target_file${RESET} (ntt_dot_accumulate typeclass workaround)"
    else
        msg "$RED" "patch_matrix_typeclass: ${BOLD}git apply${RESET} failed; refresh ${BOLD}$patch_file${RESET} or apply manually"
        # Don't `exit 1` here — a stale patch shouldn't block extraction.
        # Downstream `make check Matrix.fst` will surface the typeclass
        # error if the patch didn't apply.
    fi
}

function prove() {
    case "$1" in
        --admit)
            shift 1
            export OTHERFLAGS="--admit_smt_queries true";;
        *);;
    esac
    go_to "libcrux-ml-dsa"

    # Detect parallel-job count portably (Linux uses `nproc --all`; macOS
    # uses `sysctl -n hw.ncpu`; other systems get a hard-coded fallback).
    JOBS="${JOBS:-$(nproc --all 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)}"

    # Save full make output for post-mortem inspection; analogous to
    # libcrux-ml-kem/hax.py's `verification_result.txt`.
    local output_file="${VERIFICATION_OUTPUT:-verification_result.txt}"
    msg "$BLUE" "Running F* verification (output saved to ${BOLD}${output_file}${RESET})"

    # `make -k` keeps going after a failed target so the per-module
    # summary covers every module rather than stopping at the first
    # error.  PIPESTATUS[0] preserves make's real exit code through the
    # `tee`.
    set +e
    make -k -C proofs/fstar/extraction -j $JOBS "$@" 2>&1 | tee "$output_file"
    local rc=${PIPESTATUS[0]}
    set -e

    summarize_verification "$output_file"
    return $rc
}

# Print a one-shot summary of an F* `make` log: per-module CHECK/ADMIT
# tags, F* errors, and make-level failures.  Mirrors the
# `Verification Summary` block emitted by libcrux-ml-kem/hax.py so the
# two crates' workflows are interchangeable.
function summarize_verification() {
    local output_file="$1"
    if [ ! -f "$output_file" ]; then
        msg "$BLUE" "No verification output file at ${BOLD}${output_file}${RESET}; skipping summary."
        return
    fi

    # Strip ANSI escape codes so the grep patterns don't have to deal
    # with the `\033[NN;...m` colour markers the Makefile emits.
    local clean
    clean=$($SED 's/\x1b\[[0-9;]*m//g' "$output_file")

    # Tags emitted by fstar-helpers/Makefile.generic at the start of each
    # module run: `[CHECK] <module>` or `[ADMIT] <module>`.
    local n_check
    local n_admit
    n_check=$(printf '%s\n' "$clean" | grep -c '^\[CHECK\]' || true)
    n_admit=$(printf '%s\n' "$clean" | grep -c '^\[ADMIT\]' || true)

    # F* prints `Verified module: X` (or `Verified i'face …: X`) when a
    # module is fully discharged.
    local n_verified
    n_verified=$(printf '%s\n' "$clean" | grep -cE "^Verified (module|i'face)" || true)

    # `* Error N at FILE(L,C-L,C):` is F*'s typecheck error format.
    local n_errors
    n_errors=$(printf '%s\n' "$clean" | grep -cE '^\* Error [0-9]+ at' || true)

    # `*** [.../X.checked] Error Y` is make's signal that a target died.
    local n_make_fail
    n_make_fail=$(printf '%s\n' "$clean" | grep -cE '^\*\*\* \[.*\.checked\]' || true)

    echo
    echo "======================================================================"
    echo "  F* Verification Summary"
    echo "======================================================================"
    printf "  Modules invoked:        %d  ([CHECK]=%d  [ADMIT]=%d)\n" \
        "$((n_check + n_admit))" "$n_check" "$n_admit"
    printf "  Modules verified by F*: %d\n" "$n_verified"
    printf "  F* errors reported:     %d\n" "$n_errors"
    printf "  Make-level failures:    %d\n" "$n_make_fail"

    if [ "$n_errors" -gt 0 ]; then
        echo
        echo "  Error locations (first 10):"
        printf '%s\n' "$clean" | grep -E '^\* Error [0-9]+ at' | head -10 \
            | $SED 's/^/    /'
    fi

    if [ "$n_make_fail" -gt 0 ]; then
        echo
        echo "  Failed make targets (first 10):"
        printf '%s\n' "$clean" | grep -E '^\*\*\* \[.*\.checked\]' | head -10 \
            | $SED 's/^/    /'
    fi

    echo
    printf "  Full output:            %s\n" "$output_file"
    echo "======================================================================"
}

function init_vars() {
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    SCRIPT_NAME="$(basename "${BASH_SOURCE[0]}")"
    SCRIPT_PATH="${SCRIPT_DIR}/${SCRIPT_NAME}"

    # GNU sed (handles `-i''` for in-place edits without a backup suffix).
    # On Linux distros, `sed` is GNU sed; on macOS, `sed` is BSD sed which
    # interprets `-i''` differently, so we prefer `gsed` (Homebrew package
    # `gnu-sed`) when it's on PATH.
    if command -v gsed >/dev/null 2>&1; then
        SED=gsed
    else
        SED=sed
    fi

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
    ROOT="$SCRIPT_DIR/.."
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
    find "$target_dir" -type f \( -name "*.fst" -o -name "*.fsti" \) -exec "$SED" -i'' \
        -e 's/module Core_models/module Libcrux_core_models/g' \
        {} +
}

function rename_core_models_uses() {
    local target_dir="proofs/fstar/extraction"
    find "$target_dir" -type f \( -name "*.fst" -o -name "*.fsti" \) -exec "$SED" -i'' \
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
    grep '[#]>' "$SCRIPT_PATH" | "$SED" 's/[)] #[>]/\t/g'
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
