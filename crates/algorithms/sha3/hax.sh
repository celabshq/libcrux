#!/usr/bin/env bash
set -ex

function extract_all() {
    # `--cfg pre_core_models` routes the AVX2 backend to
    # `avx2_extract.rs` (the bit_vec stub), mirroring the arm64
    # pattern.  Without it, hax pulls in the full
    # `core_models::arch::x86::*` chain (Bitvec/Funarr) which we do
    # not need for the SHA-3 proofs.
    export RUSTFLAGS="${RUSTFLAGS:-} --cfg pre_core_models"

    extract crates/sys/platform \
        into -i "+:** -**::x86::init::cpuid -**::x86::init::cpuid_count" \
        fstar --z3rlimit 80 --interfaces "+**"

    extract crates/utils/core-models into fstar

    extract crates/utils/intrinsics \
        -C --features simd128,simd256 ";" \
        into -i "-libcrux_core_models::**" \
        fstar --z3rlimit 80 --interfaces "+**"

    extract crates/utils/secrets \
        into -i "+**" \
        fstar --z3rlimit 80

    # Minimal libcrux-traits surface needed by sha3's
    # `impl_digest_trait` module: only the `digest::arrayref` oneshot
    # `Hash<OUTPUT_LEN>` trait and its `HashError` enum.  Selecting the
    # two items by name (rather than the whole `arrayref` module) keeps
    # the extraction from dragging in `DigestIncremental`,
    # `DigestIncrementalBase`, `Hasher` and the aead/kem/ecdh surface.
    extract traits \
        into -i "-** +libcrux_traits::digest::arrayref::Hash +libcrux_traits::digest::arrayref::HashError" \
        fstar --z3rlimit 80

    extract crates/algorithms/sha3 \
        -C --features simd128,simd256 ";" \
        into -i "+**" \
        -i "-**::neon::x2::**" \
        fstar --z3rlimit 80

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
    # hax emits Core_models.Array.from_fn which has the wrong type;
    # replace with Rust_primitives.Slice.array_from_fn and supply the
    # extra implicit #(usize -> u8) that array_from_fn requires.
    $SED -i'' \
        -e 's/Core_models\.Array\.from_fn/Rust_primitives.Slice.array_from_fn/g' \
        -e '/array_from_fn/{n;/v_PARALLEL_LANES/{a\    #(usize -> u8)
}}' \
        "$target_dir"/Libcrux_sha3.Generic_keccak.Xof.fst

    # hax omits the _super_i0 (KeccakItem superclass) field in the
    # Squeeze2 trait instance; insert it so F* can resolve the class.
    # Post load/store-split (2026-05-06) the Squeeze2/Squeeze4 impls
    # moved from the parent `Simd.Arm64`/`Simd.Avx2` into the
    # `.Store` submodules.
    $SED -i '/f_squeeze2_pre/i\    _super_i0 = FStar.Tactics.Typeclasses.solve;' \
        "$target_dir"/Libcrux_sha3.Simd.Arm64.Store.fst

    # Same omission in the AVX2 Squeeze4 trait instance.
    $SED -i '/f_squeeze4_pre/i\    _super_i0 = FStar.Tactics.Typeclasses.solve;' \
        "$target_dir"/Libcrux_sha3.Simd.Avx2.Store.fst

    # The AVX2 X4.Incremental.t_KeccakState wraps an opaque Vec256 record
    # that has no decidable equality.  Mark the wrapper noeq.
    $SED -i 's/^type t_KeccakState =/noeq type t_KeccakState =/' \
        "$target_dir"/Libcrux_sha3.Avx2.X4.Incremental.fst

    # Note: per-u64-lane SMTPat lemma admits (lemma_mm256_*_u64x4)
    # are now injected directly from avx2_extract.rs via
    # `#[hax_lib::fstar::after(...)]` on each intrinsic.  No patch
    # needed here.
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
