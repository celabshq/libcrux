-- Correctness proofs for the portable (bit-sliced) AES implementation.
--
-- See README.md for what each theorem says and for the results that are
-- still out of scope.
import AxiomGuard
import cipher_theorems
import key_expansion_step
import aes_keygen_assist

-- Everything below has to rest on the trust base described in README.md:
-- Lean's three standard axioms, plus the per-call axioms `bv_decide` emits
-- for its SAT certificates. `sorry` and `native_decide` are rejected.
-- Building this file is what checks that, so a proof cannot quietly stop
-- proving anything.
open libcrux_aes.platform.portable.aes_core in
#assert_axioms
  transpose_u8x16_correct,
  sub_bytes_correct,
  get_elem_SBOX_correct,
  sub_bytes_correct_chunk,
  shift_rows_state_correct,
  shift_row_u16_correct,
  mix_columns_state_correct,
  mix_columns_state_eq_unrolled,
  xor_key1_correct,
  key_expand1_correct,
  key_expansion_correct,
  aes_keygen_assisti_correct,
  aes_keygen_assist_correct,
  aes_keygen_assist0_correct,
  aes_keygen_assist1_correct,
  transpose_correct

-- GF(2^8) lemmas that no theorem above reaches.
#assert_axioms
  GF8.xtime_bv_eq_xtime,
  GF8.xtime_bv_bv
