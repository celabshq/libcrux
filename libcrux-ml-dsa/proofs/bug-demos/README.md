# Bug-fix demonstrations — `ml-dsa-proofs` lane

In-place demonstrations on the **below-trait** lane (`ml-dsa-proofs`).
The standalone version of every demo lives at
`~/libcrux-ml-dsa-bug-demos/`.

| PR | Bug class | In‑place FIXED verifies | In‑place BUGGY fails | Report |
| --- | --- | :---: | :---: | --- |
| [1395](https://github.com/cryspen/libcrux/pull/1395) | Functional spec — AVX2 reduce skipped 28/32 lanes | ✅ (118 s) | ✅ (Error 19 — `lemma_reduce_lane_commute` precondition) | [`PR-1395.md`](PR-1395.md) |
| [1347](https://github.com/cryspen/libcrux/pull/1347) | Functional spec — γ₁ doubled | n/a (above-trait module) | n/a | see `~/libcrux-ml-dsa-above-trait/libcrux-ml-dsa/proofs/bug-demos/PR-1347.md` |
| [1348](https://github.com/cryspen/libcrux/pull/1348) | Panic-freedom — HintBitUnpack OOB | n/a (above-trait module) | n/a | see `~/libcrux-ml-dsa-above-trait/libcrux-ml-dsa/proofs/bug-demos/PR-1348.md` |

PR 1395's production location (`src/simd/avx2.rs::reduce_with_proof`)
is below the trait, which this branch verifies. PRs 1347 and 1348
live in above-trait modules (`ml_dsa_generic.rs` and
`encoding/signature.rs`); their in-place demos belong on the
`ml-dsa-above-trait` lane.

## Reproduce PR 1395 in-place

```sh
cd ~/libcrux-ml-dsa-proofs/libcrux-ml-dsa/proofs/fstar/extraction
fstar.exe <flags> Libcrux_ml_dsa.Simd.Avx2.fst   # see /tmp/pr1395-fixed.log for the full invocation
# observe: Verified module: Libcrux_ml_dsa.Simd.Avx2
#          All verification conditions discharged successfully
#          TOTAL TIME 118033 ms
# Then apply the 1-line buggy diff documented in PR-1395.md and rerun;
# observe Error 19 at line (315,37-315,62) — Assertion failed.
```
