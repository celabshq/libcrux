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

## UPDATE (same session, later): validate-on-decode LANDED (Karthik: "let's do A")

B5/B6 are CLOSED — encapsulate2_serialized + decapsulate_incremental_key
are fully verified. Mechanism:
- polynomial.rs: runtime checkers {vector,poly,polyvec,matrix}
  _within_field_bound with check-to-spec bridge ensures (ground 16-lane
  conjunction + targeted reveal_opaque of is_i16b_array_opaque at the leaf;
  forall-accumulating loop invariants above; intro-lemma folds into the
  opaque atoms). All verified at default rlimit 80, first try.
- types.rs: Error::InvalidInput variant; validation in
  EncapsState::try_from_bytes + KeyPair::from_bytes (conditional Ok-ensures
  carrying the bounds); EncapsState::from_bytes (unwrap variant) REMOVED;
  new annotated KeyPair::into_unpacked (From instance delegates, its admit
  stays — Core_models From-pre still forced trivial).
- incremental.rs: both fns un-laxed; encapsulate2_serialized returns
  Result now (chain: platform macro -> multiplexing -> concrete public API
  returns Result<Ciphertext2, Error>) — **C/eurydice bindings need
  regeneration**; decapsulate_incremental_key routes via into_unpacked.
- tests/self.rs: encapsulate2(...).unwrap() — all 21 self-tests + 6 KATs
  pass (round-trip of honest data unaffected by validation).
- Crate: Lax 90 -> 88 (9.2%); incremental row at its 3 structural admits
  (From x2 + to_bytes_compressed).
- fstar-proxy wedged twice mid-session (tool calls not reaching its log);
  direct `make -j2` fallback used once, logged to /tmp. Events use UTC —
  don't compare against local-time `date` naively.

## Remaining tasks (priority order)

1. **to_bytes_compressed admit**: needs prefix-form serialize_vector
   contract (out.len() >= ranked + prefix ensures) — cascades into verified
   ind_cpa/Serialize consumers; do as its own sprint.
2. **From-instance admits (2)**: upstream hax fix — relax
   `pred:Type0{true ==> pred}` in Core_models.Convert to allow nontrivial
   instance pres; propose to hax team.
3. **pqcp.rs (16 fns) + lib.rs (3 fns)**: still SHELVED per Karthik
   2026-06-03. Flip plan in the 2026-06-03b prompt (traits crate extraction
   stanza + un-exclude impl_kem_trait! + pqcp feature).
4. Neon backend (82 lax) remains the biggest lax bucket (out of scope for
   portable+AVX2 mandate).

## Environment notes

- cargo-hax 0.3.7 (opam switch hax-0.3.7); extraction via
  `python3 hax.py extract` in libcrux-ml-kem (~2.5 min full run).
- Extracted .fst* are GITIGNORED except 18 force-tracked files
  (Polynomial.fsti among them — it changed and must be committed).
- fstar-mcp-logs are shared across parallel agent sessions — never grab the
  newest build log without matching build_id (cost one confused debug loop).
