# Lean proofs for the portable AES implementation

This directory contains functional-correctness proofs for the bit-sliced
portable AES implementation in `src/platform/portable/aes_core.rs`.

The proofs are written in [Lean 4](https://lean-lang.org) against a model of
the Rust source produced by [hax](https://github.com/cryspen/hax). They use
`hax_mvcgen` (the monadic verification-condition generator shipped with the
hax Lean prelude) to reduce each Rust function to a bitvector goal, and
`bv_decide` to discharge it through a SAT solver.

## Layout

| Path | Contents |
| --- | --- |
| `extraction/` | hax-generated model of the crate. Generated, not checked in. |
| `Utilities.lean` | Bit-sliced state accessors, plus the `rename_auto_n` proof tactic. |
| `cipher/` | Specifications and proofs for the AES round functions. |
| `key_expansion/` | Specifications and proofs for the key-schedule functions. |

The specification is written directly from FIPS-197 in terms of `BitVec 8`
bytes and GF(2^8) arithmetic, independently of the bit-sliced representation
the implementation uses. `Utilities.lean` provides `get_elem` / `set_elem`,
which read and write one logical AES state byte across the eight bit-planes,
so the specifications never mention the bit-slicing.

## Building

The extraction is generated, so it has to be produced before the proofs can
be checked:

```sh
cd crates/algorithms/aes
./hax.py extract --target lean     # writes proofs/lean/extraction/libcrux_aes.lean
cd proofs/lean
lake build
```

`lake` resolves the hax Lean prelude from git at the revision pinned in
`lake-manifest.json`. That revision must agree with the hax engine that
produced the extraction, otherwise the generated names will not resolve.
Both are pinned to `hax-lib-v0.3.7`, matching the `hax-lib` version in the
workspace `Cargo.toml`.

## What is proved

Every theorem below is stated as a Hoare triple about the hax-generated
function, and says that the function returns the value the specification
prescribes. None of them depend on `sorry`.

| Theorem | File | Statement |
| --- | --- | --- |
| `transpose_u8x16_correct` | `cipher/cipher_theorems.lean` | Loading a 16-byte block into the bit-sliced state matches `transposeU8toU16`. |
| `sub_bytes_correct` | `cipher/cipher_theorems.lean` | Every state byte is replaced by its S-box image. |
| `shift_rows_state_correct` | `cipher/cipher_theorems.lean` | ShiftRows permutes the state as specified. |
| `xor_key1_correct` | `cipher/cipher_theorems.lean` | AddRoundKey is the pointwise XOR of state and round key. |
| `get_elem_SBOX_correct` | `cipher/sub_bytes.lean` | The branch-free S-box selector agrees with the FIPS-197 table. |
| `sub_bytes_correct_chunk` | `cipher/sub_bytes.lean` | SubBytes applied to one four-byte column. |
| `transpose_correct` | `cipher/transpose_u8x16.lean` | Reading a byte out of the bit-sliced state agrees with the byte the block-load specification puts there. |
| `shift_row_u16_correct` | `cipher/shift_rows.lean` | The single bit-plane rotation matches its specification. |
| `mix_columns_state_correct` | `cipher/cipher_theorems.lean` | MixColumns multiplies every column by the FIPS-197 matrix over GF(2^8). |
| `mix_columns_state_eq_unrolled` | `cipher/mix_columns.lean` | The generated `mix_columns_state`, a loop over the eight bit-planes, equals its fully unrolled form. |
| `key_expand1_correct` | `key_expansion/key_expansion_step.lean` | The word-level key-schedule XOR cascade is correct. |
| `key_expansion_correct` | `key_expansion/key_expansion_step.lean` | One key-schedule step is the specified prefix-XOR of the previous round key. |
| `aes_keygen_assisti_correct` | `key_expansion/aes_keygen_assist.lean` | The per-bit-plane RotWord/Rcon step is correct. |
| `aes_keygen_assist_correct` | `key_expansion/aes_keygen_assist.lean` | SubWord and RotWord composed with Rcon match the specification. |
| `aes_keygen_assist0_correct`, `aes_keygen_assist1_correct` | `key_expansion/aes_keygen_assist.lean` | The two broadcast variants used by the AES-128 and AES-256 schedules. |

### How the MixColumns proof handles the loop

`mix_columns_state` is the only proved function with a loop. hax models the
Rust `for i in 0..8` as a `fold_range 0 8` carrying the previous column. Since
the bounds are literals, `mix_columns_state_eq_unrolled` unfolds the fold
eight times by rewriting with its defining equation, and shows the result
equal to `mix_columns_state_unrolled`, a hand-written straight-line copy. That
lemma needs only `simp`, not SAT solving. The correctness proof then runs on
the straight-line form. The hand-written copy is never trusted: the theorem is
stated about the generated function, and the copy only appears inside its
proof.

## Trust base

`LibcruxAes.lean` ends with an `#assert_axioms` over every theorem in the
table, and over the GF(2^8) lemmas that nothing in it reaches yet. It fails the build if any of them comes to rest on an axiom outside
what this development accepts, which is:

- Lean's three standard axioms, `propext`, `Classical.choice`, `Quot.sound`;
- the per-call axioms `bv_decide` generates. Each one records that Lean's
  compiled LRAT checker accepted the refutation a SAT solver produced for one
  bitvector goal, so the solver's certificate is checked by compiled code
  rather than by the kernel. Almost every theorem here is discharged this
  way, so this is the method, not an accident.

`get_elem_SBOX_correct`, `xor_key1_correct` and
`mix_columns_state_eq_unrolled` need neither: they hold on the standard
axioms alone.

What the check rules out is `sorry`, which leaves `sorryAx` behind, and
`native_decide`, which asserts the result of an arbitrary compiled `Bool`
computation. Neither makes a build fail on its own, so without the check a
proof could stop proving anything without anybody noticing.

## What is not proved, and why

Two groups of results are deliberately out of scope. Both are blocked on
the same limitation rather than on the proofs themselves: hax's Lean
backend cannot currently model writes through a `&mut [T]` slice.

- **`transpose_u16x8` (storing the state back to bytes).** The function
  writes into a `&mut [u8]`. hax emits
  `rust_primitives.hax.monomorphized_update_at.update_at_usize` for those
  writes, which is only defined for `RustArray`, so the generated model does
  not typecheck. `transpose_u8x16`, which writes into a `&mut [u16; 8]`
  array, is fine and is proved.

- **The `AESState` instance for the portable state,** and with it the
  whole-schedule theorems for AES-128 and AES-256. `store_block` and
  `xor_block` both write through slices, so the trait implementation cannot
  be extracted, and the schedule functions are generic over that trait.

Restoring either needs work in hax rather than in this directory.
