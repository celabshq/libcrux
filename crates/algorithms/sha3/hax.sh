#!/usr/bin/env bash
set -ex

function extract_all() {
    # AVX2 flip COMPLETE: no `pre_core_models_*` RUSTFLAG.  lib.rs now routes
    # BOTH backends to the REAL core-models intrinsics: arm64 ->
    # `Libcrux_intrinsics.Arm64` (via the `Arm64_sha3_views` companion), avx2 ->
    # `Libcrux_intrinsics.Avx2` (via the `Avx2_sha3_views` companion).  Both are
    # differentially-tested; the hand-written `{arm64,avx2}_extract.rs` bit_vec
    # stubs are no longer on sha3's path (retired once aes/ml-kem also flip).

    # Uniform shared deps via their canonical scripts (single source of truth;
    # idempotent).  They are content-invariant to pre_core_models, so a canonical
    # config serves sha3 too.  This also UNIFIES sha3's secrets: it previously
    # omitted `--interfaces "-**"`, drifting from ml-kem/ml-dsa; the canonical
    # secrets script always extracts transparently (`--interfaces "-**"`).
    dep_extract crates/sys/platform
    dep_extract crates/utils/core-models

    # Extract intrinsics into sha3's OWN dedicated intrinsics dir (--output-dir),
    # so the shared crates/utils/intrinsics tree is never clobbered.  Both
    # backends now route to the REAL core-models `Libcrux_intrinsics.{Arm64,Avx2}`
    # (no `pre_core_models_*` flag).  `--interfaces "-**"` keeps BOTH real modules
    # TRANSPARENT (no `.fsti`) so the `{Arm64,Avx2}_sha3_views` op-facts can
    # `reveal`/reduce each `e_*` wrapper to its `Neon.*` / `Avx2.*` model.
    #
    # Force a rebuild of the intrinsics crate (touch its sources) so the
    # per-ISA variant is regenerated even if a prior extraction in this working
    # tree built it under a DIFFERENT config: hax reuses the cached THIR when
    # cargo thinks the crate is fresh, so without this touch sha3 can silently
    # pick up another crate's `Arm64.fst`/`Avx2.fst` (cross-crate cargo-freshness
    # flip). Harmless in single-crate CI (one extra intrinsics recompile).
    touch "$REPO_ROOT/crates/utils/intrinsics/src/"*.rs
    clean_generated_fstar "$SHA3_INTRINSICS_DIR"
    extract crates/utils/intrinsics \
        -C --features simd128,simd256 ";" \
        into -i "-libcrux_core_models::**" \
        --output-dir "$SHA3_INTRINSICS_DIR" \
        fstar --z3rlimit 80 --interfaces "-**"

    dep_extract crates/utils/secrets

    # Minimal libcrux-traits surface needed by sha3's
    # `impl_digest_trait` module: only the `digest::arrayref` oneshot
    # `Hash<OUTPUT_LEN>` trait and its `HashError` enum.  Selecting the
    # two items by name (rather than the whole `arrayref` module) keeps
    # the extraction from dragging in `DigestIncremental`,
    # `DigestIncrementalBase`, `Hasher` and the aead/kem/ecdh surface.
    #
    # The `digest::slice::Hash` trait is deliberately NOT selected: its
    # impl macro reborrows a `&mut [u8; LEN]` out of a `&mut [u8]` via
    # `try_into().map_err(..)?`, which hax 0.3.7 cannot model (HAX0010
    # / HAX0003 in the `DirectAndMut` context).  The `slice` impls in
    # `impl_digest_trait.rs` are excluded for the same reason.
    extract traits \
        into -i "-** +libcrux_traits::digest::arrayref::Hash +libcrux_traits::digest::arrayref::HashError" \
        fstar --z3rlimit 80

    # Extract the sha3 reference spec (crate `hacspec_sha3`).  The sha3 proofs
    # verify against `Hacspec_sha3.*` (Sha3/Sponge/Keccak_f) — a real dependency of
    # Libcrux_sha3.Generic_keccak.* / Libcrux_sha3.fst — so it MUST be re-extracted
    # here.  This step was missing; libcrux-ml-kem/hax.py and libcrux-ml-dsa/hax.sh
    # already extract their own specs (specs/sha3/hax.sh is the standalone equivalent).
    # NOTE: if cargo reports the crate fresh and skips it (writes no THIR export ->
    # hax panics with a NotFound), force a rebuild with `cargo clean -p hacspec_sha3`.
    extract specs/sha3 into -i "+**" fstar --z3rlimit 80

    # Remove stale generated F* before re-extracting.  hax never deletes files
    # for modules that were removed/renamed, nor old interface (*.fsti) files
    # when a module stops emitting one.  A leftover *.fsti silently SHADOWS the
    # freshly generated *.fst (F* prefers the interface), hiding definitions
    # behind opaque `val`s -> spurious "incomplete quantifiers" proof failures.
    # (This is exactly what broke the SIMD load/store proofs: 7 stale
    # Libcrux_sha3.*.fsti hid get_ij/set_ij.)  The 4 hand-written proof modules
    # (no Rust source) are restored from git after extraction.
    rm -f "$SCRIPT_DIR/proofs/fstar/extraction"/*.fst \
          "$SCRIPT_DIR/proofs/fstar/extraction"/*.fsti

    # Generate ABSTRACT interfaces (.fsti) for the `portable` module subtree so
    # external consumers (ml-kem / ml-dsa / kmac) verify against sha3's PUBLIC
    # function contracts (`sha512`/`shake256`/... : `== Hacspec_sha3.Sponge.keccak`;
    # incremental: length/state posts) WITHOUT dragging in sha3's internal Keccak
    # equivalence proof cone (Generic_keccak.Portable, EquivImplSpec.*,
    # Hacspec_sha3.Sponge.Lemmas). sha3's OWN build still verifies each `.fst`
    # against its `.fsti`; the interface is the trust/verification boundary. The
    # concrete `t_KeccakState` record is kept in the interface (consumers + the
    # internal incremental `Bundle` need `.f_state`), which hax places in the
    # `.fsti` and omits from the `.fst` automatically.
    #
    # EXCEPTION — the SIMD *incremental* subtrees (`avx2::x4::incremental`,
    # `neon::x2::incremental`) are DELIBERATELY excluded from `--interfaces`, so
    # hax extracts them TRANSPARENTLY (concrete `.fst`, no generated `.fsti`).  A
    # hax-GENERATED concrete `.fsti` for these is broken: it drops `noeq` on the
    # `BitVec`-field `t_KeccakState` record (F* Error 19, decidable-equality) and
    # drags the heavy `Simd.{Avx2,Arm64}` lane cone into every consumer's
    # `.fsti.checked`.  Instead a hand-written ABSTRACT `.fsti` (`val
    # t_KeccakState : Type0`, length-only posts — ca6ea1dac technique) is provided
    # for each and restored in patch_fstar_extractions; consumers (ml-kem/ml-dsa/
    # kmac) depend only on that LIGHT interface, never the SIMD cone.  The wrapper
    # state is never read field-wise by any sha3-internal module, so abstracting
    # it is transparent to sha3 (whose own build verifies the concrete `.fst`
    # implements the abstract `.fsti`).
    extract crates/algorithms/sha3 \
        -C --features simd128,simd256 ";" \
        into -i "+**" \
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
    # sha3 carries its OWN copy of the shared intrinsics modules
    # (Libcrux_intrinsics.{Avx2_extract,Arm64_extract}), extracted via --output-dir
    # into this DEDICATED subdir (not the main `extraction` tree).  FINDLIBS
    # (Makefile.generic) auto-includes every workspace crate's
    # `proofs/fstar/extraction` on every other crate's path, so a per-algorithm
    # intrinsics copy in `extraction/` would collide (same module names, DIFFERENT
    # content) on sibling paths.  A `proofs/fstar/intrinsics` sibling dir is NOT
    # auto-discovered by FINDLIBS, so it is added ONLY to sha3's own include path
    # via FSTAR_INCLUDE_DIRS_EXTRA (`../intrinsics`).  The shared
    # crates/utils/intrinsics tree is likewise excluded in Makefile.generic.
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

    # Restore the hand-written proof modules removed by the pre-extract clean.
    # These have no Rust source (hax does not regenerate them); Proof_utils.fst
    # is hand-written and intentionally shadows any hax-generated version.
    git checkout -- \
        "$target_dir/Libcrux_sha3.Proof_utils.fst" \
        "$target_dir/Libcrux_sha3.Proof_utils.Lemmas.fst" \
        "$target_dir/Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst" \
        "$target_dir/Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst" \
        "$target_dir/Libcrux_sha3.Avx2.X4.Incremental.fsti" \
        "$target_dir/Libcrux_sha3.Neon.X2.Incremental.fsti" 2>/dev/null || true
    # hax emits Core_models.Array.from_fn which has the wrong type;
    # replace with Rust_primitives.Slice.array_from_fn and supply the
    # extra implicit #(usize -> u8) that array_from_fn requires.
    $SED -i'' \
        -e 's/Core_models\.Array\.from_fn/Rust_primitives.Slice.array_from_fn/g' \
        -e '/array_from_fn/{n;/v_PARALLEL_LANES/{a\    #(usize -> u8)
}}' \
        "$target_dir"/Libcrux_sha3.Generic_keccak.Xof.fst

    # --- SIMD patches: guarded for the main-merge PORTABLE-only phase ---
    # The proofs-branch store/load-split placed the Squeeze2/Squeeze4 trait
    # impls in `Simd.{Arm64,Avx2}.Store` submodules; origin/main flattened the
    # SIMD backends back to a plain `simd/{arm64,avx2}.rs`, so those submodule
    # files are not produced. The sha3 SIMD store_block proofs are DEFERRED
    # (see proofs/verification_status.md); guard each SIMD patch on file
    # existence so PORTABLE extraction succeeds against main's flat SIMD.
    for f in Libcrux_sha3.Simd.Arm64.Store.fst Libcrux_sha3.Simd.Arm64.fst; do
        [ -f "$target_dir/$f" ] && $SED -i '/f_squeeze2_pre/i\    _super_i0 = FStar.Tactics.Typeclasses.solve;' "$target_dir/$f"
    done
    for f in Libcrux_sha3.Simd.Avx2.Store.fst Libcrux_sha3.Simd.Avx2.fst; do
        [ -f "$target_dir/$f" ] && $SED -i '/f_squeeze4_pre/i\    _super_i0 = FStar.Tactics.Typeclasses.solve;' "$target_dir/$f"
    done

    # The incremental KeccakState wrappers hold a generic KeccakState
    # over an opaque SIMD vector record (Vec256 on AVX2 X4, uint64x2_t
    # on Neon X2) that has no decidable equality.  hax 0.3.7 has no
    # source-level `noeq` attribute and the F* backend does not detect
    # non-eqtype records, so mark both wrappers noeq here.  Guarded (main
    # may not emit these under its flat SIMD layout).
    for f in Libcrux_sha3.Avx2.X4.Incremental.fst Libcrux_sha3.Neon.X2.Incremental.fst; do
        [ -f "$target_dir/$f" ] && $SED -i 's/^type t_KeccakState =/noeq type t_KeccakState =/' "$target_dir/$f"
    done

    # core-models flip: the multi-block squeeze composers (impl__squeeze_first_three_blocks,
    # impl__squeeze_first_five_blocks) saturate cold from the Arm64_sha3_views companion
    # SMTPat cascade (composer pollution, same as the Store composers). `fstar::options`
    # is illegal on an inherent-impl method (anon const) and is dropped on the impl block,
    # so wrap the two decls (three_blocks .. five_blocks, the last decls) in a
    # companion-excluding #push-options / #pop-options here.
    local simd128f="$target_dir/Libcrux_sha3.Generic_keccak.Simd128.fst"
    if [ -f "$simd128f" ] && grep -q '^let impl__squeeze_first_three_blocks' "$simd128f"; then
        SQZ_OPTS="#push-options \"--fuel 0 --ifuel 1 --z3rlimit 400 --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane -Libcrux_intrinsics.Arm64_sha3_views'\"" \
            perl -i -pe 'print "$ENV{SQZ_OPTS}\n\n" if /^let impl__squeeze_first_three_blocks$/' "$simd128f"
        printf '\n#pop-options\n' >> "$simd128f"
    fi

    # AVX2 flip analog: the Simd256 (X4) squeeze composers cascade the same way
    # from the `Avx2_sha3_views` companion's `get_lane_u64_post` SMTPat.  The
    # inner push inherits the impl-block's `--split_queries always` (from
    # `_keccak_state_impl4_opts`) and just adds the companion exclusion; scoped
    # to three_blocks..five_blocks (the last two decls -> pop at EOF).
    local simd256f="$target_dir/Libcrux_sha3.Generic_keccak.Simd256.fst"
    if [ -f "$simd256f" ] && grep -q '^let impl__squeeze_first_three_blocks' "$simd256f"; then
        SQZ_OPTS="#push-options \"--fuel 0 --ifuel 1 --z3rlimit 400 --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane -Libcrux_intrinsics.Avx2_sha3_views'\"" \
            perl -i -pe 'print "$ENV{SQZ_OPTS}\n\n" if /^let impl__squeeze_first_three_blocks$/' "$simd256f"
        printf '\n#pop-options\n' >> "$simd256f"
    fi

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

# Invoke a canonical per-dependency extraction script (single source of truth
# for that uniform shared dep; idempotent — skips if already extracted).  Keeps
# the shared platform/core-models/secrets trees from flip-flopping between
# per-algorithm configs (feedback_shared_coremodels_extraction_contamination).
# NOTE: the dep scripts do NOT set `--cfg pre_core_models`, and these deps are
# content-invariant to it (empirically verified), so a canonical config serves
# sha3 too even though sha3 extracts its own code under pre_core_models.
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
