# SHA-3, written the way FIPS 202 writes it

A second specification of SHA-3, next to [`../sha3`](../sha3). The two agree on
every input — the test suite checks that — and they exist for different reasons.

`hacspec_sha3` is the reference that the implementations are verified against,
so it is written in the terms an implementation uses: the state is `[u64; 25]`,
messages are bytes, and the round constants and rotation offsets are tables.
That is the right shape for a proof about `libcrux-sha3`, and the wrong shape
for answering "is this actually what the Standard says?".

This crate is written in the terms [FIPS 202] uses, so it can be read with the
document open:

* the state is the 5-by-5-by-`w` array of bits `A[x, y, z]` of Sec. 3.1, and
  messages are bit strings, not bytes;
* every function is one numbered Algorithm, with the Steps in the Standard's
  order, under the Standard's names;
* nothing is precomputed that the Standard computes: `ρ`'s offsets come out of
  the `(x, y)` walk in Algorithm 2 and `ι`'s round constants out of the `rc`
  LFSR in Algorithm 5, rather than from tables;
* the permutation is generic in the width — `KECCAK-p[b, n_r]` for any of the
  seven `b` of Table 1, not only `b = 1600`.

It is slow, and that is the point: it is meant to be checked line by line
against the PDF, not run.

## Where each section lives

| FIPS 202 | module |
|---|---|
| Sec. 2.3 basic operations; App. B.1 `h2b` / `b2h` (Algorithms 10, 11) | `bits` |
| Sec. 3.1 state, state array, and the two conversions | `state_array` |
| Sec. 3.2 Algorithms 1-6: `θ`, `ρ`, `π`, `χ`, `ι`, and `rc` | `step_mappings` |
| Sec. 3.3 Algorithm 7 `KECCAK-p[b, n_r]`; Sec. 3.4 `KECCAK-f[b]` | `keccak_p` |
| Sec. 4 Algorithm 8 `SPONGE`; Sec. 5.1 Algorithm 9 `pad10*1`; Sec. 5.2 `KECCAK[c]` | `sponge` |
| Sec. 6.1-6.2 the four hash functions and the two XOFs | `sha3` |
| — byte-aligned wrappers, `h2b` in front and `b2h` behind | `bytes` |

The document is [`../sha3/NIST.FIPS.202.pdf`](../sha3/NIST.FIPS.202.pdf).

## What the tests check

```
cargo test -p hacspec_sha3_pedantic --release
```

* the Standard's own worked examples: `h2b(0xA32E, 14)` from Table 5, and the
  byte-aligned padding forms of Table 6 (`M || 0x86`, `M || 0x0680`, …);
* `pad10*1` produces `1 0^j 1` of the length Algorithm 9 promises;
* the published digests of the empty message and of `"abc"`;
* agreement with `hacspec_sha3` on all six functions, at message lengths around
  every rate boundary, and for XOF output longer than the rate;
* Sec. 3.1.2 and 3.1.3 are inverse; `KECCAK-f[b] = KECCAK-p[b, 12 + 2l]` and
  `KECCAK-p[b, n_r]` is the tail of `KECCAK-f[b]` (Sec. 3.4);
* `ρ`'s computed offsets are the ones printed in Table 2.

## Lean extraction

```
cd specs && cargo bin cargo-hax extract hacspec-sha3-pedantic
cd sha3-pedantic/proofs/hacspec-sha3-pedantic/lean && lake build
```

All of it extracts and type-checks (1743 jobs): the six SHA-3 functions, the
five step mappings with `rc`, `KECCAK-p`/`KECCAK-f`, the sponge, `pad10*1` and
the bit-string helpers, plus the four `.pre`/`.spec` pairs that the
`#[hax_lib::requires]` contracts generate. There are no proofs here yet —
`hacspec_sha3` is still the spec the libcrux proofs are written against.

Two gaps in hax's core models are filled by this package itself, in
`Assumptions/FunsExternal.lean` (the file hax seeds and never touches again):

* `RangeInclusive`. CoreModels declares the type but ships no `new` and no
  `Iterator` instance, so `for j in a..=b` did not extract. The model here is
  CoreModels' own `IteratorRange.next` for the half-open `Range` with `<`
  weakened to `≤` -- about twenty lines. It buys back all three of the
  Standard's inclusive loops: "For `j` from 0 to `l`" (Algorithm 6), "For `i`
  from 1 to `t mod 255`" (Algorithm 5), and "For `i_r` from `12+2l-n_r` to
  `12+2l-1`" (Algorithm 7).
  **Caveat:** Rust's `RangeInclusive` carries an `exhausted` flag that the
  modelled type lacks, so the model panics where Rust would yield `A::MAX` and
  stop. Every inclusive range here is small and fixed, so the difference is
  unreachable -- but a proof against this model is a proof about
  non-saturating ranges only.
* `Copy` for `bool`. CoreModels has `marker.Copy` for every integer and
  `clone.Clone` for `Bool`, but not the instance those two determine, so
  `copy_from_slice` on a `[bool]` did not resolve. One line, nothing assumed.
  It buys back Step 3a of Algorithm 5 (`R = 0 || R` as a slice copy) and with
  it the helper that step had been hoisted into.

What remains is not model gaps but limits of the translation itself, so a model
cannot help; each is marked where it happens:

| what the Standard does | what the toolchain needs |
|---|---|
| `SPONGE[f, pad, r]`, parameterised by `f` and `pad` | the components are passed as a *value* (`components: &C`), not only as a type parameter. A trait type parameter determined by nothing but the turbofish loses its instance argument wherever the extraction lifts code out of the function -- every loop included -- and aeneas then either rejects its own output (`ill-formed builtin: invalid number of filtering arguments`) or emits a call of the wrong arity. One value argument avoids all of it |
| `A′[0,0,z] = A′[0,0,z] ⊕ RC[z]` | lane (0,0) is read out, updated and written back: a compound assignment through the nested projection `out.a[0][0][z]` makes aeneas fail with `Unreachable` |
| — | assert messages must be ASCII: a `⊕` in one came out as an invalid escape in the generated Lean, and being a parse error it then cascaded into twenty phantom "unknown constant" reports. `assert_eq!` is also out, since formatting the two operands needs a `core::fmt::Arguments::from_str` CoreModels does not have; plain `assert!(cond, "…")` extracts to a `massert` and is what the "Input:" conditions use |
| `l = log2(w)` | the Table 1 lookup. A `trailing_zeros` model would work, but Table 1 is what the document prints, so this one stays on merit |
| — | `Vec::extend` and `to_vec` have no model -- and unlike `Copy for bool`, `Extend` is not declared in CoreModels at all -- so `concat`, `trunc`, `zeros` and `b2h` `push` in explicit loops |
| — | `X ⊕ Y` is `bits::xor` rather than an inner loop of Algorithm 8. This was forced by the first row and would work inline now; it stays because a named `⊕` on bit strings reads better next to Sec. 2.3 |
| — | the derived `Debug` is `cfg`-gated out; its generated instance does not match the core model. The only `cfg` left in the crate |

[FIPS 202]: https://doi.org/10.6028/NIST.FIPS.202
