#!/usr/bin/env bash
set -ex

function extract_all() {
    # Shared dependencies are extracted by their own canonical drivers.
    dep_extract crates/sys/platform
    dep_extract crates/utils/core-models

    # Intrinsics go into sha3's own `proofs/fstar/intrinsics` dir, extracted
    # transparently (`--interfaces "-**"`) against the core-models Arm64/Avx2
    # models so the `{Arm64,Avx2}_sha3_views` companions can reduce each
    # wrapper to its model.  Touching the sources forces cargo to rebuild the
    # crate under this configuration rather than reuse a cached THIR export.
    touch "$REPO_ROOT/crates/utils/intrinsics/src/"*.rs
    clean_generated_fstar "$SHA3_INTRINSICS_DIR"
    extract crates/utils/intrinsics \
        -C --features simd128,simd256 ";" \
        into -i "-libcrux_core_models::**" \
        --output-dir "$SHA3_INTRINSICS_DIR" \
        fstar --z3rlimit 80 --interfaces "-**"

    dep_extract crates/utils/secrets

    # Only the libcrux-traits items `impl_digest_trait` uses: the
    # `digest::arrayref` `Hash` trait and `HashError`.  `digest::slice::Hash`
    # is not selected: its impl macro reborrows a `&mut [u8; LEN]` out of a
    # `&mut [u8]` via `try_into().map_err(..)?`, which hax cannot model
    # (HAX0010 / HAX0003).
    extract traits \
        into -i "-** +libcrux_traits::digest::arrayref::Hash +libcrux_traits::digest::arrayref::HashError" \
        fstar --z3rlimit 80

    # The sha3 proofs verify against the `Hacspec_sha3.*` reference spec.
    # If cargo considers the crate fresh and writes no THIR export, run
    # `cargo clean -p hacspec_sha3`.
    extract specs/sha3 into -i "+**" fstar --z3rlimit 80

    # Remove generated F* before re-extracting: hax never deletes files for
    # removed modules, and a leftover `.fsti` shadows the fresh `.fst`.  git's
    # ignore rules record which files are generated; `git clean -X` never
    # touches the tracked hand-written modules.
    git clean -Xdfq "$SCRIPT_DIR/proofs/fstar/extraction"

    # Interfaces (.fsti) for the `portable`, `avx2::x4` and `neon` subtrees
    # let consumers (ml-kem, ml-dsa, kmac) verify against sha3's public
    # contracts without the internal Keccak equivalence proofs; sha3's own
    # build checks each `.fst` against its `.fsti`.
    #
    # The SIMD incremental subtrees (`avx2::x4::incremental`,
    # `neon::x2::incremental`) are extracted transparently instead: a
    # generated `.fsti` would drop `noeq` on the `t_KeccakState` record and
    # pull the SIMD lane proofs into every consumer.  Hand-written abstract
    # interfaces for them live in `proofs/fstar/spec`.
    #
    # Shared dependencies are excluded; their canonical drivers write them.
    extract crates/algorithms/sha3 \
        -C --features simd128,simd256 ";" \
        into -i "+** -libcrux_platform::** -libcrux_core_models::** -libcrux_secrets::**" \
        fstar --z3rlimit 80 --interfaces "-** +libcrux_sha3::portable +libcrux_sha3::portable::** +libcrux_sha3::avx2::x4 +libcrux_sha3::avx2::x4::** -libcrux_sha3::avx2::x4::incremental -libcrux_sha3::avx2::x4::incremental::** +libcrux_sha3::neon +libcrux_sha3::neon::** -libcrux_sha3::neon::x2::incremental -libcrux_sha3::neon::x2::incremental::**"

    patch_fstar_extractions
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
    make -C proofs/fstar JOBS=$JOBS "$@"
}

function detect_sed() {
    # GNU sed is required for -i without a backup suffix argument.
    # On Linux, the system sed is GNU sed. On macOS, install gnu-sed
    # via Homebrew and it will be available as gsed.
    if sed --version >/dev/null 2>&1; then
        SED=sed
    elif command -v gsed >/dev/null 2>&1; then
        SED=gsed
    else
        echo "Error: GNU sed is required but not found."
        echo "On macOS, install it with: brew install gnu-sed"
        exit 1
    fi
}

function init_vars() {
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    SCRIPT_NAME="$(basename "${BASH_SOURCE[0]}")"
    SCRIPT_PATH="${SCRIPT_DIR}/${SCRIPT_NAME}"
    REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
    # sha3 extracts its own copy of the intrinsics modules into a dedicated
    # directory.  FINDLIBS (Makefile.generic) puts every workspace crate's
    # `proofs/fstar/extraction` on every crate's include path, so a copy there
    # would collide with other crates' copies; `proofs/fstar/intrinsics` is
    # added only to sha3's include path, via FSTAR_INCLUDE_DIRS_EXTRA.
    SHA3_INTRINSICS_DIR="$SCRIPT_DIR/proofs/fstar/intrinsics"

    detect_sed

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

function patch_fstar_extractions() {
    go_to "crates/algorithms/sha3"
    local target_dir="proofs/fstar/extraction"

    # The Squeeze2/Squeeze4 trait impls need their supertrait instance
    # supplied explicitly.  Each patch is guarded on the module existing.
    for f in Libcrux_sha3.Simd.Arm64.Store.fst Libcrux_sha3.Simd.Arm64.fst; do
        [ -f "$target_dir/$f" ] && $SED -i '/f_squeeze2_pre/i\    _super_i0 = FStar.Tactics.Typeclasses.solve;' "$target_dir/$f"
    done
    for f in Libcrux_sha3.Simd.Avx2.Store.fst Libcrux_sha3.Simd.Avx2.fst; do
        [ -f "$target_dir/$f" ] && $SED -i '/f_squeeze4_pre/i\    _super_i0 = FStar.Tactics.Typeclasses.solve;' "$target_dir/$f"
    done

    # The incremental KeccakState wrappers hold a generic KeccakState over an
    # opaque SIMD vector record (Vec256 on AVX2 X4, uint64x2_t on Neon X2)
    # that has no decidable equality.  hax has no source-level `noeq`
    # attribute, so mark both wrappers noeq here.
    for f in Libcrux_sha3.Avx2.X4.Incremental.fst Libcrux_sha3.Neon.X2.Incremental.fst; do
        [ -f "$target_dir/$f" ] && $SED -i 's/^type t_KeccakState =/noeq type t_KeccakState =/' "$target_dir/$f"
    done

    # The multi-block squeeze composers (impl__squeeze_first_three_blocks ..
    # impl__squeeze_first_five_blocks, the last decls of the module) exclude the
    # `Arm64_sha3_views` companion SMTPats from their context.  `fstar::options`
    # is dropped on inherent-impl methods, so the options are inserted here.
    local simd128f="$target_dir/Libcrux_sha3.Generic_keccak.Simd128.fst"
    if [ -f "$simd128f" ] && grep -q '^let impl__squeeze_first_three_blocks' "$simd128f"; then
        SQZ_OPTS="#push-options \"--fuel 0 --ifuel 1 --z3rlimit 400 --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane -Libcrux_intrinsics.Arm64_sha3_views'\"" \
            perl -i -pe 'print "$ENV{SQZ_OPTS}\n\n" if /^let impl__squeeze_first_three_blocks$/' "$simd128f"
        printf '\n#pop-options\n' >> "$simd128f"
    fi

    # Same for the Simd256 (X4) squeeze composers and the `Avx2_sha3_views`
    # companion; the inner push inherits the impl block's options.
    local simd256f="$target_dir/Libcrux_sha3.Generic_keccak.Simd256.fst"
    if [ -f "$simd256f" ] && grep -q '^let impl__squeeze_first_three_blocks' "$simd256f"; then
        SQZ_OPTS="#push-options \"--fuel 0 --ifuel 1 --z3rlimit 400 --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane -Libcrux_intrinsics.Avx2_sha3_views'\"" \
            perl -i -pe 'print "$ENV{SQZ_OPTS}\n\n" if /^let impl__squeeze_first_three_blocks$/' "$simd256f"
        printf '\n#pop-options\n' >> "$simd256f"
    fi
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

# Run a shared dependency's canonical `hax.py extract` (idempotent: it skips
# when already extracted), so shared trees have a single writer.
function dep_extract() {
    local dep="$1"   # e.g. crates/sys/platform
    msg "$BLUE" "dep_extract ${BOLD}$dep${RESET}"
    python3 "$REPO_ROOT/$dep/hax.py" extract || {
        msg "$RED" "dep extraction failed for ${BOLD}$dep${RESET}"
        exit 1
    }
}

# Remove generated .fst/.fsti from a directory holding only generated files:
# hax never deletes a `.fsti` when a module stops emitting one, and a leftover
# `.fsti` shadows the fresh `.fst`.
function clean_generated_fstar() {
    local dir="$1"
    [ -d "$dir" ] && rm -f "$dir"/*.fst "$dir"/*.fsti
    return 0
}

function extract_to_lean() {
    TARGET="$1"
    shift 1

    msg "$BLUE" "extract (lean) ${BOLD}$TARGET${RESET}"
    go_to "$TARGET"
    cargo hax "$@" || {
        msg "$RED" "lean extraction failed for ${BOLD}$TARGET${RESET}"
        exit 1
    }
}

function extract_all_lean() {
    extract_to_lean crates/sys/platform \
        into -i "+:** -**::x86::init::cpuid -**::x86::init::cpuid_count" \
        lean

    extract_to_lean crates/utils/core-models into lean

    extract_to_lean crates/utils/intrinsics \
        into -i "-libcrux_core_models::**" \
        lean

    extract_to_lean crates/utils/secrets \
        into -i "+**" \
        lean

    extract_to_lean crates/algorithms/sha3 \
        into -i "+**" \
        -i "-**::avx2::**" \
        -i "-**::neon::**" \
        -i "-**::simd128::**" \
        -i "-**::simd256::**" \
        lean

    patch_lean_extractions
}

function patch_lean_extractions() {
    # Add dependency imports that hax does not emit automatically.
    go_to "crates/algorithms/sha3"
    local sha3="proofs/lean/extraction/libcrux_sha3.lean"
    $SED -i'' -e '/^import Hax$/a\
import Stubs\
import extraction.libcrux_intrinsics' "$sha3"

    # Replace all generated proof tactics with sorry.
    $SED -i '' 's/by hax_construct_pure <;> bv_decide/by sorry/g' "$sha3"
    $SED -i '' 's/by hax_mvcgen \[[^]]*\] <;> bv_decide/by sorry/g' "$sha3"
    $SED -i '' 's/by hax_construct_pure <;> rfl/by sorry/g' "$sha3"

    # slices_same_len: replace monadic body with a pure Prop
    # (hax_lib.prop.constructors.forall can't synthesize pureP for this predicate).
    python3 -c "
import re, sys
t = open(sys.argv[1]).read()
t = re.sub(
    r'(def slices_same_len \(N : usize\) \(slices : \(RustArray \(RustSlice u8\) N\)\) :\n    RustM hax_lib\.prop\.Prop) := do\n.*?RustM hax_lib\.prop\.Prop\)\)\)',
    r\"\"\"\1 :=
  pure (∀ (i j : Fin N.toNat), slices.toVec[i].val.size = slices.toVec[j].val.size)\"\"\",
    t, flags=re.DOTALL)
open(sys.argv[1],'w').write(t)
" "$sha3"

    # Intrinsic stubs should be irreducible, not @[spec].
    go_to "crates/utils/intrinsics"
    local intrinsics="proofs/lean/extraction/libcrux_intrinsics.lean"
    $SED -i'' -e 's/^@\[spec\]$/@[irreducible]/' "$intrinsics"
}

function help() {
    echo "Libcrux script to extract Rust to F* and Lean via hax."
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Comands:"
    echo ""
    grep '[#]>' "$SCRIPT_PATH" | $SED 's/[)] #[>]/\t/g'
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
        extract_lean) #> Extract Lean code for the proofs.
            extract_all_lean
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
