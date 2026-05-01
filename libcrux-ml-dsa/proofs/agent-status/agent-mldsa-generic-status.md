# Status — agent-mldsa-generic
Updated: 2026-05-01T15:00:00Z
Sub-task: handoff-2026-05-01-rust-level-ensures.md — convert the three wired ensures from `fstar!(...)` strings to pure-Rust closures with `cfg(hax)` access to `hacspec_ml_dsa`.
ETA: complete; 3/3 modules verifying in ~8s each.
Functions landed: 3 / 3 (generate_key_pair, verify, sign)

## What landed

`libcrux-ml-dsa/Cargo.toml` — added a `cfg(hax)` dep on the spec crate
alongside the existing `core-models` entry, so the spec module is
reachable from hax-extract builds:

```toml
[target.'cfg(hax)'.dependencies]
core-models = { path = "../crates/utils/core-models", version = "0.0.5" }
hacspec_ml_dsa = { path = "../specs/ml-dsa" }
```

`crates/utils/macros/src/lib.rs` — `ml_dsa_parameter_sets` no longer
injects an F*-string `let v_HACSPEC_PARAMS = ...`.  It now emits a
real Rust `const`:

```rust
#[cfg(hax)]
pub(crate) const HACSPEC_PARAMS: hacspec_ml_dsa::MlDsaParams =
    hacspec_ml_dsa::ML_DSA_44;   // (or _65, _87)
```

`libcrux-ml-dsa/src/ml_dsa_generic.rs` — the three wired ensures
(generate_key_pair, sign, verify) are now pure-Rust closures, gated
on `cfg_attr(hax, ...)` so they only enter the compile graph when
`hacspec_ml_dsa` is actually present.  Each calls the matching spec
function with the const-generic args threaded from `HACSPEC_PARAMS`
and the impl's per-variant constants.  Example:

```rust
#[cfg_attr(hax, hax_lib::ensures(|_| {
    let (pk_spec, sk_spec) = hacspec_ml_dsa::keygen_internal::<
        { HACSPEC_PARAMS.k },
        { HACSPEC_PARAMS.l },
        VERIFICATION_KEY_SIZE,
        SIGNING_KEY_SIZE,
    >(&randomness, &HACSPEC_PARAMS);
    future(signing_key).len() == signing_key.len()
        && future(verification_key).len() == verification_key.len()
        && future(signing_key) == &sk_spec[..]
        && future(verification_key) == &pk_spec[..]
}))]
```

Hax extracts these closures to F* of essentially the same shape as
the prior manual strings — slice `==` becomes `=.` (Seq.equal),
`is_ok()` to `Core_models.Result.impl__is_ok`, the const-generic args
inline as `mk_usize <n>`.

## Verification

| Module | Outcome | Time |
|---|---|---|
| `Libcrux_ml_dsa.Ml_dsa_generic.Ml_dsa_44_.fst` | ✅ Verified clean | 8.2s |
| `Libcrux_ml_dsa.Ml_dsa_generic.Ml_dsa_65_.fst` | ✅ Verified clean | 8.9s |
| `Libcrux_ml_dsa.Ml_dsa_generic.Ml_dsa_87_.fst` | ✅ Verified clean | 8.6s |

Bodies still have `hax_lib::fstar!("admit ()")` — wiring only, no
body-side proof yet.

`cargo build --tests` clean.  `cargo test --release --lib`: 20/20
passing.  `RUSTFLAGS="--cfg hax" cargo +nightly check` clean.

## Notes for next agent

- `cfg_attr(hax, hax_lib::ensures(...))` is the gating pattern that
  lets the closure body reference `cfg(hax)`-only items
  (`HACSPEC_PARAMS`, `hacspec_ml_dsa::*`) without breaking debug
  builds (where `hax_lib::ensures` would otherwise type-check the
  closure under `cfg(any(hax_compilation, debug_assertions))`).
- Const-generic args use `{ HACSPEC_PARAMS.k }` const-block syntax;
  hax inlines these to `mk_usize <n>` at extract time.  The
  precondition `K == params.k` becomes a trivial numeric equality
  per variant — F* discharges it from the `HACSPEC_PARAMS`
  definition without ceremony.
- Spec-side `keygen_internal` returns `(pk, sk)` and the impl mutates
  `(signing_key, verification_key)` slices — note the tuple-component
  swap when comparing.
