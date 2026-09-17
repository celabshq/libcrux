#!/usr/bin/env bash
set -e

function extract_all() {
    # Uniform shared deps via their canonical scripts (single source of truth;
    # idempotent — skip if already extracted).  They stay in their own crate dirs
    # and are auto-included by Makefile.generic's dependencies().
    dep_extract crates/sys/platform
    dep_extract crates/utils/core-models

    # Extract intrinsics into ml-dsa's OWN dedicated intrinsics dir (--output-dir),
    # so the shared crates/utils/intrinsics tree is never clobbered by ml-dsa's
    # new-core-models mapping (non-pre_core_models => real `Libcrux_intrinsics.Avx2`,
    # the cross-crate flip vs ml-kem's/sha3's `Avx2_extract` stub).
    #
    # `--interfaces` is SPLIT per module (mirror ml-kem hax.py):
    #  * Avx2 (migrated to core-models) stays TRANSPARENT — consumers use the
    #    `.fst` op BODIES (which route through the `Libcrux_core_models` crate
    #    extracted above) so `reveal_opaque` can unfold `mm256_OP = e_mm256_OP`.
    #  * Arm64_extract (still the hand-written bit_vec stub) MUST get a `.fsti`:
    #    arm64_extract.rs uses `fstar::replace(interface, ...)` blocks; under
    #    transparent extraction they have no `.fsti` to land in and flood the
    #    `.fst` AFTER the impl blocks -> use-before-def (Error 72) + dup val
    #    (Error 47).  (ml-dsa is AVX2-only so it never checks Arm64_extract, but
    #    keep the config correct so a re-extract can't regenerate the defect.)
    #
    # Force a rebuild of the intrinsics crate (touch its sources) so the non-pcm
    # variant is regenerated even if a prior extraction in this working tree built
    # it under pre_core_models (ml-kem/sha3 -> `Avx2_extract`): hax reuses the
    # cached THIR when cargo thinks the crate is fresh, so without this touch
    # ml-dsa can silently pick up an `Avx2_extract.fst` instead of its own real
    # `Avx2.fst` (the cross-crate cargo-freshness flip). Harmless in single-crate CI.
    touch "$REPO_ROOT/crates/utils/intrinsics/src/"*.rs
    clean_generated_fstar "$ML_DSA_INTRINSICS_DIR"
    # ml-dsa is AVX2-only: nothing under proofs/fstar/{extraction,spec} references
    # `Libcrux_intrinsics.Arm64_extract` (verified 2026-08-07 — removing both files
    # left the stale set at 91 modules, unchanged, with no missing-module error).
    # Extracting it anyway cost ml-dsa's trust ledger 18 unreferenced `assume val`
    # obligations (9 `vecN_as_*_axiom` + 9 derive-`Copy` tc instances).  Excluding it
    # here drops those at zero proof cost; the NEON consumers live in ml-kem
    # (`Vector.Neon.*`) and sha3 (`Simd.Arm64.*`), which extract it themselves.
    # With arm64_extract gone, the interface allowlist is vestigial -> plain "-**",
    # which is what ml-dsa wants for Avx2 anyway (transparent .fst, bodies visible
    # for reveal_opaque).
    extract crates/utils/intrinsics \
        -C --features simd128,simd256 ";" \
        into -i "-libcrux_core_models::** -libcrux_intrinsics::arm64_extract::**" \
        --output-dir "$ML_DSA_INTRINSICS_DIR" \
        fstar --z3rlimit 80 \
        --interfaces "-**"

    # Uniform shared secrets dep via its canonical script (transparent
    # `--interfaces "-**"`; see feedback_postmerge_audit_order).
    dep_extract crates/utils/secrets

    # Extract the ml-dsa reference spec (crate `hacspec_ml_dsa`).  The
    # hand-written Spec.MLDSA.Math.fsti and Hacspec_ml_dsa.Commute.Chunk
    # depend on these `Hacspec_ml_dsa.*` modules; without this step the
    # ml-dsa proof build is RED (Error 72: module Hacspec_ml_dsa could
    # not be resolved).  Mirrors libcrux-ml-kem/hax.py's spec extraction.
    # NOTE: if cargo reports the crate is fresh and skips it (writes no
    # THIR export -> hax panics with a NotFound in run_command), force a
    # rebuild with `cargo clean -p hacspec_ml_dsa` before re-running.
    # `--z3rlimit 80` matches the sha3 and ml-kem spec passes: this sweep also
    # re-emits shared spec crates, and an unset flag writes the module header at
    # hax's default (15), so the file flip-flopped on cross-crate extraction.
    extract specs/ml-dsa into -i "+**" fstar --z3rlimit 80

    # G3 module-trust mirror (annotation_lint V6 / trust_ledger --check): every
    # `-libcrux_ml_dsa::…` module dropped from F* extraction below carries a
    # machine-readable reason here (an absent module is worse than an admitted one).
    # Reasons use the shared category vocabulary (reason_ok); the V6 lint enforces
    # the bijection {exclusion tokens} == {annotations}. (Reasons live here, above the
    # `\`-continued command, because a trailing comment would break the continuation.)
    # trusted-module: -libcrux_ml_dsa::hash_functions::portable::* : trusted-extern: SHA3 hash backend verified in the sha3 crate (only the trait signature is re-extracted)
    # trusted-module: -libcrux_ml_dsa::hash_functions::simd256::* : trusted-extern: SHA3 hash backend verified in the sha3 crate (only the trait signature is re-extracted)
    # trusted-module: -libcrux_ml_dsa::hash_functions::neon::* : trusted-extern: SHA3 hash backend verified in the sha3 crate (only the trait signature is re-extracted)
    # `-libcrux_{platform,core_models,secrets}::**`: canonically owned by their
    # own hax.py (run above).  A `+**` sweep would re-emit them under THIS
    # crate's backend flags (different `--interfaces`/`--z3rlimit` => different
    # module header and interface split), making the shared tree depend on which
    # algorithm extracted last.
    extract libcrux-ml-dsa \
        -C --features simd128,simd256 ";" \
        into -i "+** -libcrux_platform::** -libcrux_core_models::** -libcrux_secrets::**" \
             -i "-libcrux_ml_dsa::hash_functions::portable::*" \
             -i "-libcrux_ml_dsa::hash_functions::simd256::*" \
             -i "-libcrux_ml_dsa::hash_functions::neon::*" \
             -i "+:libcrux_ml_dsa::hash_functions::*::*" \
             -i "-**::types::non_hax_impls::**" \
        fstar --z3rlimit 80

    patch_matrix_typeclass libcrux-ml-dsa
    patch_arithmetic_typeclass libcrux-ml-dsa
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

# Post-extraction patch: `decompose_vector` takes two `&mut` slices (`low`,
# `high`), which hax extracts as an outer fold over tuple state `(high, low)`.
# Inside the body F*'s `Index` typeclass tactic resolves `.[ ]` on the FIRST
# binder (`high`, and the immutable `t`) but FAILS on the SECOND binder
# (`low.[ i ]`) under the strengthened loop invariant's subtyping context
# (Error 228 "Tactic failed"; same tuple-state root cause as
# `patch_matrix_typeclass`).  hax emits only an element-type ascription, not
# the receiver ascription the tactic needs, so we add `(low <: t_Slice ...)`
# here.  Scoped via an awk range on the top-level `let` to the
# `decompose_vector` decl ONLY, so the unrelated `low.[ i ]` in `make_hint`
# is left alone.  The rewrite is idempotent (the patched text contains no
# `low.[ i ]` substring), so re-running on an already-patched file is a no-op.
function patch_arithmetic_typeclass() {
    TARGET="$1"
    shift 1
    go_to "$TARGET"

    local target_file="proofs/fstar/extraction/Libcrux_ml_dsa.Arithmetic.fst"
    if [ ! -f "$target_file" ]; then
        msg "$BLUE" "patch_arithmetic_typeclass: ${BOLD}$target_file${RESET} not found, skipping"
        return
    fi

    if ! grep -q 'low\.\[ i \]' "$target_file"; then
        msg "$BLUE" "patch_arithmetic_typeclass: no un-ascribed low.[ i ] found, skipping"
        return
    fi

    local tmp="${target_file}.tc.tmp"
    awk '
      /^let / { indv = ($0 ~ /^let decompose_vector/) ? 1 : 0 }
      indv { gsub(/low\.\[ i \]/, "(low <: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)).[ i ]") }
      { print }
    ' "$target_file" > "$tmp" && mv "$tmp" "$target_file"
    msg "$BLUE" "patched ${BOLD}$target_file${RESET} (decompose_vector tuple-state Index workaround)"
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
    REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
    # ml-dsa carries its OWN copy of the shared intrinsics modules
    # (Libcrux_intrinsics.{Avx2,Arm64_extract}), extracted via --output-dir into
    # this DEDICATED subdir (not the main `extraction` tree).  Rationale: the
    # Makefile.generic FINDLIBS auto-includes every workspace crate's
    # `proofs/fstar/extraction` on every other crate's path, so a per-algorithm
    # intrinsics copy in `extraction/` would collide (same module names, DIFFERENT
    # content — ml-dsa's real Avx2 vs ml-kem's/sha3's Avx2_extract stub) on
    # sibling paths (F* silently picks the alphabetically-last one).  A
    # `proofs/fstar/intrinsics` sibling dir is NOT auto-discovered by FINDLIBS, so
    # it is added ONLY to ml-dsa's own include path via FSTAR_INCLUDE_DIRS_EXTRA
    # (`../intrinsics`), like the existing `../spec` / commute dirs.  The shared
    # crates/utils/intrinsics tree is likewise excluded in Makefile.generic.
    ML_DSA_INTRINSICS_DIR="$SCRIPT_DIR/proofs/fstar/intrinsics"

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

function extract() {
    TARGET="$1"
    shift 1

    msg "$BLUE" "extract ${BOLD}$TARGET${RESET}"
    go_to "$TARGET"
    cargo hax "$@" || {
        msg "$RED" "extract extraction failed for ${BOLD}$1${RESET}"
        exit 1
    }
}

# Invoke a canonical per-dependency extraction script (single source of truth
# for that uniform shared dep; idempotent — skips if already extracted).  Keeps
# the shared platform/core-models/secrets trees from flip-flopping between
# per-algorithm configs (feedback_shared_coremodels_extraction_contamination).
function dep_extract() {
    local dep="$1"   # e.g. crates/sys/platform
    msg "$BLUE" "dep_extract ${BOLD}$dep${RESET}"
    python3 "$REPO_ROOT/$dep/hax.py" extract || {
        msg "$RED" "dep extraction failed for ${BOLD}$dep${RESET}"
        exit 1
    }
}

# Remove generated .fst/.fsti from an extraction dir BEFORE re-extracting.  hax
# extracts incrementally and NEVER deletes a .fsti when a module stops emitting
# one; a leftover .fsti then silently SHADOWS the fresh .fst (the stale-.fsti
# contamination that broke the SHA-3 SIMD proofs).  The dedicated intrinsics dir
# holds only generated files, so removing all is safe.
function clean_generated_fstar() {
    local dir="$1"
    [ -d "$dir" ] && rm -f "$dir"/*.fst "$dir"/*.fsti
    return 0
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
