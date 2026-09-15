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

## Not extracted

There are no `hax` annotations here and nothing in `proofs/`. This crate is for
reading; `hacspec_sha3` remains the spec that F* and Lean see.

[FIPS 202]: https://doi.org/10.6028/NIST.FIPS.202
