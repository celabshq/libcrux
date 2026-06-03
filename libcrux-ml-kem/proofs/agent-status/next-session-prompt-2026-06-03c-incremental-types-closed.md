# Next session — after Ind_cca.Incremental.{Types,} closure

## What landed this session (see commit log for hash)

Ind_cca.Incremental.fst + Ind_cca.Incremental.Types.fst are VERIFIED (last
two incremental ADMIT_MODULES entries removed). Mechanism — plain-Rust
requires/ensures plus a handful of fstar! scaffolding where Rust can't
express it:

- `polynomial.rs`: len-preservation ensures on `to_bytes`/`vec_to_bytes`/
  `PolynomialRingElement::to_bytes`; `vec_len_bytes` ensures `== K * 512`.
- `incremental/types.rs`:
  - `IncrementalKeyPair` trait decl: `#[requires(bytes.len() >= 64)]` on
    `pk1_bytes`; the **trait impl** carries `#[hax_lib::attributes]` + bare
    requires (incl. a K-dependent pre on `pk2_bytes`) — trait impls CAN
    carry pre/post via the attributes wrapper (memory note corrected).
  - `PublicKey2::{len,deserialize}`, `EncapsState::{num_bytes,to_bytes,
    try_from_bytes,from_bytes}`, `KeyPair::{pk1_bytes,pk2_bytes,to_bytes,
    from_bytes}` requires/ensures; `write` helper specs
    (subtraction-form requires to avoid +! WF overflow).
  - Matrix-loop invariants in `KeyPair::{to,from}_bytes`: MATH-INT form via
    bare `fstar!` (machine-int form fails WF on `i * (K*512)`), plus ground
    `K == 2 || K == 3 || K == 4` requires (K <= 4 leaves the product
    nonlinear → Z3 unknown).
  - 3 admitted PROOF GAPS (`fstar!("admit ()")` + comments): the two From
    instances (core-trait pres forced trivial by `pred:Type0{true ==> pred}`)
    and `to_bytes_compressed` (serialize_vector exact-length contract).
- `incremental.rs`: is_rank/ETA/size-relation requires on all free fns;
  `generate_keypair_serialized` post flipped implies-form → requires-form
  (platform guard discharges); `encapsulate1` ensures bounded state;
  matrix-bound folding after `sample_matrix_A` via the intro-lemma
  incantation (copy it for future sample_matrix_A consumers);
  2 lax fns (B5/B6 raw-bytes bounds gap): `encapsulate2_serialized`,
  `decapsulate_incremental_key`.
- `multiplexing.rs` + `mlkem.rs` macro: requires relays; concrete
  `encapsulate1` (+rand) got `state.len() >= RANK*512+544 &&
  shared_secret.len() >= 32` requires (debug_asserts before guards force
  requires, not Err-conditions).
- Findings section added to proofs/ml_kem_verification_status.md (raw
  16-bit decode → unvalidated coefficients → not F*-panic-free for
  adversarial bytes; debug_assert-before-guard behaviour) — script now
  preserves everything below `<!-- manual-sections-below -->`.

## hax gotchas (memorized, see memory notes)

- `loop_invariant!` needs BARE `fstar!`; `${matrix}.[ i ]` braced form;
  space before `}` after `$K`.
- Trait impls take `#[hax_lib::attributes]` + bare requires/ensures fine;
  `verification_status(lax)` does NOT work there (use body admit()).

## Remaining tasks (priority order)

1. **Findings follow-ups** (needs Karthik/team decision): validate-or-clamp
   on raw-byte deserialization would close B5/B6 lax fns + enable
   decapsulate_incremental_key contract; prefix-form serialize_vector
   contract would close to_bytes_compressed.
2. **pqcp.rs (16 fns) + lib.rs (3 fns)**: still SHELVED per Karthik
   2026-06-03. Flip plan in the 2026-06-03b prompt (traits crate extraction
   stanza + un-exclude impl_kem_trait! + pqcp feature).
3. Neon backend (82 lax) remains the biggest lax bucket (out of scope for
   portable+AVX2 mandate).

## Environment notes

- cargo-hax 0.3.7 (opam switch hax-0.3.7); extraction via
  `python3 hax.py extract` in libcrux-ml-kem (~2.5 min full run).
- Extracted .fst* are GITIGNORED except 18 force-tracked files
  (Polynomial.fsti among them — it changed and must be committed).
- fstar-mcp-logs are shared across parallel agent sessions — never grab the
  newest build log without matching build_id (cost one confused debug loop).
