# Next session — after Mlkem*.Incremental closure (76959deab)

## What landed this session (committed 76959deab)

Mlkem{512,768,1024}.Incremental are FULLY VERIFIED; removed from
ADMIT_MODULES. Mechanism: plain-Rust spec annotations only (Karthik mandate:
no `fstar!` strings when Rust expressions suffice):

- `types.rs`: `#[hax_lib::attributes]` on `impl PublicKey1` and the
  `KeyPair` impl + value posts: `PublicKey1::len() == 64`;
  `KeyPair::num_bytes() == 64 + PK2_LEN + K*512 + 32 + K*K*512`
  (requires `K <= 4 && PK2_LEN <= 1536`).
  GOTCHA: bare `#[hax_lib::ensures]` on impl methods fails under the hax
  driver (E0401 outer-generics / "const _ needs a name") even though plain
  cargo check passes — the `#[hax_lib::attributes]` impl wrapper + bare
  `#[ensures]` inside is mandatory.
- `mlkem.rs impl_incr_key_size!`: `pk1_len == 64`,
  `pk2_len == RANKED_BYTES_PER_RING_ELEMENT`, and `generate_key_pair` post =
  length-preservation `&` `implies(len >= 64 + RANKED + RANK*512 + 32 +
  RANK*RANK*512, result.is_ok())` — this discharges the
  `from_seed`/`generate` unwrap obligations (unwrap requires
  `Core_models.Result.impl__is_ok`).
- Same post on `multiplexing::generate_keypair` and
  `impl_incr_platform!::generate_keypair_serialized` (bodies VERIFIED in
  Ind_cca.Incremental.{Avx2,Portable,Multiplexing}); the generic
  `ind_cca/incremental.rs::generate_keypair_serialized` carries it admitted
  (module still in ADMIT_MODULES — to_bytes/from_bytes byte-layout proofs
  are the remaining milestone-#0 scope there).

Builds: d4480ac7 (3 targets), b932b686 (chain bodies) — real query stats,
0 failures. Extraction diff was confined to the 13-file incremental subtree
(md5-diff verified). Full-crate `make all` gate PASSED clean (build
449a6c5a, 12.4 min, 0 failed modules).

Also landed same session: adc9be2c4 (status-script `expansion_targets` —
mlkem.rs 36 fns Unverified → 131 PF + 3 Math, crate Unverified 5.9% → 2.1%,
panic-safe 84.3%) and 0102447b2 (single tracked status file =
proofs/ml_kem_verification_status.md; old verification_status.md renamed,
live doc references updated).

## Remaining tasks (in priority order)

1. **Ind_cca.Incremental.fst + Types.fst** (45-fn "incremental" row, the
   only incremental admits left): open obligations are f_pk1_bytes
   `bytes.len()>=64` precondition, KeyPair::to_bytes/from_bytes byte-layout,
   EncapsState to/from_bytes. The Ok-condition/len-preservation posts added
   to the generic generate_keypair_serialized in 76959deab are admitted
   there — proving to_bytes closes them.
2. **pqcp.rs (16 fns) + lib.rs (3 fns)**: SHELVED per Karthik 2026-06-03
   ("Let's ignore pqcp and lib for now"). Findings kept for the record:
   - pqcp fns expand into `mlkem{512,768,1024}::pqcp` under
     `#[cfg(all(not(eurydice), feature = "pqcp"))]`; `pqcp` is NOT in the
     hax.py extraction feature list, so they are cfg'd out entirely.
   - Every pqcp fn calls `<MlKem{512,768,1024} as
     libcrux_traits::kem::arrayref::Kem>::{keygen,encaps,decaps}`. That
     trait impl lives in lib.rs's `impl_kem_trait!` and is
     `#[hax_lib::exclude]`d; lib.rs's 3 "fns" are exactly those methods.
   - libcrux-traits (= `<repo>/traits`, v0.0.6) has NO proofs dir — never
     extracted by any crate.
   Flip plan (~1 session, per feedback_extraction_first): (a) add a
   `cargo hax into` stanza for `../../traits` to libcrux-ml-kem/hax.py +
   include dir in the extraction Makefile (scope: kem::{arrayref,owned}
   hierarchy; the `slice::impl_trait!` expansion in lib.rs is already
   `#[cfg(not(hax))]`); (b) remove `#[hax_lib::exclude]` from the
   impl_kem_trait! impl block and rediscover why it was excluded (plain
   extraction of trait impls works; ensures on trait-impl methods does NOT —
   see reference_hax_ensures_trait_impl_methods); (c) add `pqcp` to the
   feature list. Payoff: 19 fns Unverified → PF.

## Environment notes

- `libcrux-ml-kem/verification_result.txt` still has the OTHER session's
  uncommitted modifications — left untouched again; ask Karthik.
- /tmp backups of probe files: /tmp/Mlkem512.Incremental.{fst,fsti}.bak,
  /tmp/Types.fsti.bak — stale (probe superseded by re-extraction), ignore.
- cargo-hax 0.3.7; all F* via fstar_build MCP; `--admit_except` takes ONE
  name; rm tainted .checked + finish with full no-admit builds.
