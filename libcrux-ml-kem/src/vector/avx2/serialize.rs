use super::*;
use crate::vector::portable::PortableVector;

// This function needs no precondition: the body's first operation
// `mm256_slli_epi16::<15>` discards bits 1..15 of every lane, so the post
// holds for ARBITRARY input.
#[inline(always)]
#[hax_lib::fstar::options(
    "--ext context_pruning --compat_pre_core 0 --split_queries always --z3rlimit 400"
)]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 16}). bit_vec_of_int_t_array $result 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $vector (i * 16)"#))]
// Bring the ml-kem srli i16-view SMTPat into scope (it lives outside
// Avx2_extract so sha3's interface stays lean).
#[hax_lib::fstar::before(
    r#"open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views"#
)]
pub(crate) fn serialize_1(vector: Vec256) -> [u8; 2] {
    // Suppose |vector| is laid out as follows (superscript number indicates the
    // corresponding bit is duplicated that many times):
    //
    // 0¹⁵a₀ 0¹⁵b₀ 0¹⁵c₀ 0¹⁵d₀ | 0¹⁵e₀ 0¹⁵f₀ 0¹⁵g₀ 0¹⁵h₀ | ...
    //
    // We care only about the least significant bit in each lane,
    // move it to the most significant position to make it easier to work with.
    // |vector| now becomes:
    //
    // a₀0¹⁵ b₀0¹⁵ c₀0¹⁵ d₀0¹⁵ | e₀0¹⁵ f₀0¹⁵ g₀0¹⁵ h₀0¹⁵ | ↩
    // i₀0¹⁵ j₀0¹⁵ k₀0¹⁵ l₀0¹⁵ | m₀0¹⁵ n₀0¹⁵ o₀0¹⁵ p₀0¹⁵
    let lsb_to_msb = mm256_slli_epi16::<15>(vector);

    // Get the first 8 16-bit elements ...
    let low_msbs = mm256_castsi256_si128(lsb_to_msb);

    // ... and the next 8 16-bit elements ...
    let high_msbs = mm256_extracti128_si256::<1>(lsb_to_msb);

    // ... and then pack them into 8-bit values using signed saturation.
    // This function packs all the |low_msbs|, and then the high ones.
    //
    //
    // low_msbs =  a₀0¹⁵ b₀0¹⁵ c₀0¹⁵ d₀0¹⁵ | e₀0¹⁵ f₀0¹⁵ g₀0¹⁵ h₀0¹⁵
    // high_msbs = i₀0¹⁵ j₀0¹⁵ k₀0¹⁵ l₀0¹⁵ | m₀0¹⁵ n₀0¹⁵ o₀0¹⁵ p₀0¹⁵
    //
    // We shifted by 15 above to take advantage of the signed saturation performed
    // by mm_packs_epi16:
    //
    // - if the sign bit of the 16-bit element being packed is 1, the
    // corresponding 8-bit element in |msbs| will be 0xFF.
    // - if the sign bit of the 16-bit element being packed is 0, the
    // corresponding 8-bit element in |msbs| will be 0.
    //
    // Thus, if, for example, a₀ = 1, e₀ = 1, and p₀ = 1, and every other bit
    // is 0, after packing into 8 bit value, |msbs| will look like:
    //
    // 0xFF 0x00 0x00 0x00 | 0xFF 0x00 0x00 0x00 | 0x00 0x00 0x00 0x00 | 0x00 0x00 0x00 0xFF
    let msbs = mm_packs_epi16(low_msbs, high_msbs);

    // Now that every element is either 0xFF or 0x00, we just extract the most
    // significant bit from each element and collate them into two bytes.
    let bits_packed = mm_movemask_epi8(msbs);

    let result = [bits_packed as u8, (bits_packed >> 8) as u8];

    proof!(
        r#"
assert (Seq.index $result 0 == (cast ($bits_packed <: i32) <: u8));
assert (Seq.index $result 1 == (cast ($bits_packed >>! mk_i32 8 <: i32) <: u8));
introduce forall (i: nat{i < 16}).
    Rust_primitives.BitVectors.bit_vec_of_int_t_array $result 8 i ==
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $vector (i * 16)
with Libcrux_intrinsics.Avx2_ml_kem_views.lemma_serialize_1_bits $vector $result i
"#
    );

    result
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 2)]
#[hax_lib::ensures(|coefficients| fstar!(
        r#"forall (i:nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $coefficients i
    = ( if i % 16 >= 1 then 0
        else let j = (i / 16) * 1 + i % 16 in
             bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 2)) 8 j))
"#
))]
#[hax_lib::fstar::before("#restart-solver")]
pub(crate) fn deserialize_1(bytes: &[u8]) -> Vec256 {
    #[hax_lib::ensures(|coefficients| fstar!(
        r#"forall (i:nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $coefficients i
    = ( if i % 16 >= 1 then 0
        else let j = (i / 16) * 1 + i % 16 in
             if i < 128 then get_bit $a (sz j) else get_bit $b (sz (j - 8)))
"#
    ))]
    #[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]
    #[inline(always)]
    pub(crate) fn deserialize_1_u8s(a: u8, b: u8) -> Vec256 {
        deserialize_1_i16s(a as i16, b as i16)
    }

    #[hax_lib::ensures(|coefficients| fstar!(
        r#"forall (i:nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $coefficients i
    = ( if i % 16 >= 1 then 0
        else let j = (i / 16) * 1 + i % 16 in
             if i < 128 then get_bit $a (sz j) else get_bit $b (sz (j - 8)))
"#
    ))]
    #[inline(always)]
    #[hax_lib::fstar::options("--ext context_pruning")]
    #[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]
    pub(crate) fn deserialize_1_i16s(a: i16, b: i16) -> Vec256 {
        // We need to take each bit from the 2 bytes of input and put them
        // into their own 16-bit lane. Ideally, we'd load the two bytes into the vector,
        // duplicate them, and right-shift the 0th element by 0 bits,
        // the first element by 1 bit, the second by 2 bits and so on before AND-ing
        // with 0x1 to leave only the least signifinicant bit.
        // But since |_mm256_srlv_epi16| does not exist, so we have to resort to a
        // workaround.
        //
        // Rather than shifting each element by a different amount, we'll multiply
        // each element by a value such that the bit we're interested in becomes the most
        // significant bit.
        // The coefficients are loaded as follows:
        let coefficients = mm256_set_epi16(b, b, b, b, b, b, b, b, a, a, a, a, a, a, a, a);

        // And this vector, when multiplied with the previous one, ensures that the
        // bit we'd like to keep in each lane becomes the most significant bit upon
        // multiplication.
        let coefficients_in_msb = mm256_mullo_epi16(
            coefficients,
            mm256_set_epi16(
                1 << 8,
                1 << 9,
                1 << 10,
                1 << 11,
                1 << 12,
                1 << 13,
                1 << 14,
                -32768,
                1 << 8,
                1 << 9,
                1 << 10,
                1 << 11,
                1 << 12,
                1 << 13,
                1 << 14,
                -32768,
            ),
        );

        // Now that they're all in the most significant bit position, shift them
        // down to the least significant bit.
        let result = mm256_srli_epi16::<15>(coefficients_in_msb);
        proof!(
            r#"
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i
    = ( if i % 16 >= 1 then 0
        else let j = (i / 16) * 1 + i % 16 in
             if i < 128 then get_bit $a (sz j) else get_bit $b (sz (j - 8)))
with Libcrux_intrinsics.Avx2_ml_kem_views.lemma_deserialize_1_bits $a $b i
"#
        );
        result
    }

    deserialize_1_u8s(bytes[0], bytes[1])
}

/// `mm256_concat_pairs_n(n, x)` is then a sequence of 32 bits packets
/// of the shape `0b0…0b₁…bₙa₁…aₙ`, if `x` is a sequence of pairs of
/// 16 bits, of the shape `(0b0…0a₁…aₙ, 0b0…0b₁…bₙ)` (where the last
/// `n` bits are non-zero).
// The bit-concatenation post is PROVEN by
// `Concat_pairs_theory.lemma_concat_pairs_bits` (no assumed facts).
// `n` is unshadowed to `sh` so both stay nameable from the `fstar!` antiquotes.
#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"1 <= v $n /\ v $n <= 12 /\
  (forall (l: nat{l < 256}). l % 16 >= v $n ==>
     Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $x l = 0)"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i ==
    (if i % 32 < v $n
     then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $x ((i / 32) * 32 + i % 32)
     else if i % 32 < 2 * v $n
     then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $x ((i / 32) * 32 + 16 + (i % 32 - v $n))
     else 0)"#))]
fn mm256_concat_pairs_n(n: u8, x: Vec256) -> Vec256 {
    let sh = 1 << n;
    let result = mm256_madd_epi16(
        x,
        mm256_set_epi16(sh, 1, sh, 1, sh, 1, sh, 1, sh, 1, sh, 1, sh, 1, sh, 1),
    );
    proof!(
        r#"
FStar.Math.Lemmas.pow2_le_compat 12 (v $n);
assert_norm (pow2 12 == 4096);
assert_norm (pow2 16 == 65536);
FStar.Math.Lemmas.small_mod (pow2 (v $n)) (pow2 16);
assert (v $sh == pow2 (v $n));
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i ==
      (if i % 32 < v $n
       then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $x ((i / 32) * 32 + i % 32)
       else if i % 32 < 2 * v $n
       then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $x ((i / 32) * 32 + 16 + (i % 32 - v $n))
       else 0)
with Libcrux_ml_kem.Vector.Avx2.Concat_pairs_theory.lemma_concat_pairs_bits $n $sh $x i
"#
    );
    result
}

// The rlimit matches serialize_5/10/12; serialize_4 was the only width riding the
// module default (80), which starves the final store/gather composition query
// (measured: 80.000 canceled -> 99/400 succeeded, no other change).
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
#[hax_lib::requires(
    fstar!(
        r#"forall (i: nat{i < 256}). i % 16 < 4 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $vector i = 0"#
    )
)]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 64}). bit_vec_of_int_t_array $r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $vector ((i/4) * 16 + i%4)"#))]
#[inline(always)]
pub(crate) fn serialize_4(vector: Vec256) -> [u8; 8] {
    let mut serialized = [0u8; 16];

    // If |vector| is laid out as follows:
    //
    // 0x000A 0x000B 0x000C 0x000D | 0x000E 0x000F 0x000G 0x000H | ....
    //
    // |adjacent_2_combined| will be laid out as a series of 32-bit integeres,
    // as follows:
    //
    // 0x00_00_00_BA 0x00_00_00_DC | 0x00_00_00_FE 0x00_00_00_HG | ...
    let adjacent_2_combined = mm256_concat_pairs_n(4, vector);

    // Recall that |adjacent_2_combined| goes as follows:
    //
    // 0x00_00_00_BA 0x00_00_00_DC | 0x00_00_00_FE 0x00_00_00_HG | ...
    //
    // Out of this, we only need the first byte, the 4th byte, the 8th byte
    // and so on from the bottom and the top 128 bits.
    let adjacent_8_combined = mm256_shuffle_epi8(
        adjacent_2_combined,
        mm256_set_epi8(
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 12, 8, 4, 0, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, 12, 8, 4, 0,
        ),
    );

    // |adjacent_8_combined| looks like this:
    //
    // 0: 0xHG_FE_DC_BA 1: 0x00_00_00_00 | 2: 0x00_00_00_00 3: 0x00_00_00_00 | 4: 0xPO_NM_LK_JI ....
    //
    // We put the element at 4 after the element at 0 ...
    // NB `combined256` is unshadowed from `combined`: a rebound name drops its
    // earlier let-equation, and the gather lemma needs the whole chain in scope.
    let combined256 =
        mm256_permutevar8x32_epi32(adjacent_8_combined, mm256_set_epi32(0, 0, 0, 0, 0, 0, 4, 0));
    let combined = mm256_castsi256_si128(combined256);

    // ... so that we can read them out in one go.
    #[cfg(hax)]
    let serialized_pre = serialized;
    mm_storeu_bytes_si128(&mut serialized, combined);

    proof!(
        r#"
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm_storeu_bytes_si128 $serialized_pre $combined;
introduce forall (i: nat{i < 64}).
    Rust_primitives.BitVectors.bit_vec_of_int_t_array $serialized 8 i ==
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $vector ((i / 4) * 16 + i % 4)
with Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_serialize_4_gather_bits
       $vector $adjacent_2_combined i
"#
    );

    serialized[0..8].try_into().unwrap()
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 8)]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i = (if i % 16 >= 4 then 0
               else let j = (i / 16) * 4 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 8)) 8 j)"#))]
#[hax_lib::fstar::before("#restart-solver")]
pub(crate) fn deserialize_4(bytes: &[u8]) -> Vec256 {
    #[hax_lib::ensures(|coefficients| fstar!(
        r#"forall (i:nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $coefficients i
    = ( if i % 16 < 4
        then let j = (i / 16) * 4 + i % 16 in
             (match i / 32 with
             | 0 -> get_bit $b0 (sz j)
             | 1 -> get_bit $b1 (sz (j - 8))
             | 2 -> get_bit $b2 (sz (j - 16))
             | 3 -> get_bit $b3 (sz (j - 24))
             | 4 -> get_bit $b4 (sz (j - 32))
             | 5 -> get_bit $b5 (sz (j - 40))
             | 6 -> get_bit $b6 (sz (j - 48))
             | 7 -> get_bit $b7 (sz (j - 56)))
        else 0)
"#
    ))]
    #[inline(always)]
    #[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]
    fn deserialize_4_u8s(b0: u8, b1: u8, b2: u8, b3: u8, b4: u8, b5: u8, b6: u8, b7: u8) -> Vec256 {
        deserialize_4_i16s(
            b0 as i16, b1 as i16, b2 as i16, b3 as i16, b4 as i16, b5 as i16, b6 as i16, b7 as i16,
        )
    }

    #[hax_lib::ensures(|coefficients| fstar!(
        r#"forall (i:nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $coefficients i
    = ( if i % 16 < 4
        then let j = (i / 16) * 4 + i % 16 in
             (match i / 32 with
             | 0 -> get_bit $b0 (sz j)
             | 1 -> get_bit $b1 (sz (j - 8))
             | 2 -> get_bit $b2 (sz (j - 16))
             | 3 -> get_bit $b3 (sz (j - 24))
             | 4 -> get_bit $b4 (sz (j - 32))
             | 5 -> get_bit $b5 (sz (j - 40))
             | 6 -> get_bit $b6 (sz (j - 48))
             | 7 -> get_bit $b7 (sz (j - 56)))
        else 0)
"#
    ))]
    #[inline(always)]
    #[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]
    fn deserialize_4_i16s(
        b0: i16,
        b1: i16,
        b2: i16,
        b3: i16,
        b4: i16,
        b5: i16,
        b6: i16,
        b7: i16,
    ) -> Vec256 {
        // Every 4 bits from each byte of input should be put into its own 16-bit lane.
        // Since |_mm256_srlv_epi16| does not exist, we have to resort to a workaround.
        //
        // Rather than shifting each element by a different amount, we'll multiply
        // each element by a value such that the bits we're interested in become the most
        // significant bits (of an 8-bit value).
        let coefficients = mm256_set_epi16(
            // In this lane, the 4 bits we need to put are already the most
            // significant bits of |bytes[7]| (that is, b7).
            b7,
            // In this lane, the 4 bits we need to put are the least significant bits,
            // so we need to shift the 4 least-significant bits of |b7| to the
            // most significant bits (of an 8-bit value).
            b7, // and so on ...
            b6, b6, b5, b5, b4, b4, b3, b3, b2, b2, b1, b1, b0, b0,
        );
        let coefficients_in_msb = mm256_mullo_epi16(
            coefficients,
            mm256_set_epi16(
                // These constants are chosen to shift the bits of the values
                // that we loaded into |coefficients|.
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
            ),
        );

        // Once the 4-bit coefficients are in the most significant positions (of
        // an 8-bit value), shift them all down by 4.
        let coefficients_in_lsb = mm256_srli_epi16::<4>(coefficients_in_msb);

        // Zero the remaining bits.
        let result = mm256_and_si256(coefficients_in_lsb, mm256_set1_epi16((1 << 4) - 1));
        proof!(
            r#"
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i
    = ( if i % 16 < 4
        then let j = (i / 16) * 4 + i % 16 in
             (match i / 32 with
             | 0 -> get_bit $b0 (sz j)
             | 1 -> get_bit $b1 (sz (j - 8))
             | 2 -> get_bit $b2 (sz (j - 16))
             | 3 -> get_bit $b3 (sz (j - 24))
             | 4 -> get_bit $b4 (sz (j - 32))
             | 5 -> get_bit $b5 (sz (j - 40))
             | 6 -> get_bit $b6 (sz (j - 48))
             | 7 -> get_bit $b7 (sz (j - 56)))
        else 0)
with Libcrux_ml_kem.Vector.Avx2.Unpack_theory.lemma_deserialize_4_bits $b0 $b1 $b2 $b3 $b4 $b5
       $b6 $b7 i
"#
        );
        result
    }

    deserialize_4_u8s(
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    )
}

#[inline(always)]
// proof-residence: locked(clean-context) — the store-glue composition saturates
// on this module's accumulated solver state; the restart is the documented
// first probe for module-context-only saturation (skill 7 step 0.5).
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 5 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector i = 0"#))]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 80}). bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector ((i/5) * 16 + i%5)"#))]
pub(crate) fn serialize_5(vector: Vec256) -> [u8; 10] {
    #[inline(always)]
    #[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
    #[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 5 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector i = 0"#))]
    #[hax_lib::ensures(|(lower_8, upper_8)| fstar!(
        r#"
         forall (i: nat{i < 80}).
           Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector ((i/5) * 16 + i%5) ==
             (if i < 40 then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower_8 i
              else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper_8 (i - 40))
      )
    "#
    ))]
    fn serialize_5_vec(vector: Vec256) -> (Vec128, Vec128) {
        // If |vector| is laid out as follows (superscript number indicates the
        // corresponding bit is duplicated that many times):
        //
        // 0¹¹a₄a₃a₂a₁a₀ 0¹¹b₄b₃b₂b₁b₀ 0¹¹c₄c₃c₂c₁c₀ 0¹¹d₄d₃d₂d₁d₀ | ↩
        // 0¹¹e₄e₃e₂e₁e₀ 0¹¹f₄f₃f₂f₁f₀ 0¹¹g₄g₃g₂g₁g₀ 0¹¹h₄h₃h₂h₁h₀ | ↩
        //
        // |adjacent_2_combined| will be laid out as a series of 32-bit integers,
        // as follows:
        //
        // 0²²b₄b₃b₂b₁b₀a₄a₃a₂a₁a₀ 0²²d₄d₃d₂d₁d₀c₄c₃c₂c₁c₀ | ↩
        // 0²²f₄f₃f₂f₁f₀e₄e₃e₂e₁e₀ 0²²h₄h₃h₂h₁h₀g₄g₃g₂g₁g₀ | ↩
        // ....
        let adjacent_2_combined = mm256_concat_pairs_n(5, vector);

        // Shifting up by 22, then back down by 22, viewing as 64-bit lanes,
        // packs adjacent 2-combined into adjacent 4-combined.
        // NB the SIMD locals in this chain are unshadowed (`_shifted` /
        // `_shuffled` / `_combined`): a rebound name drops its earlier
        // let-equation, and the gather lemmas need the whole chain in scope.
        let adjacent_4_shifted = mm256_sllv_epi32(
            adjacent_2_combined,
            mm256_set_epi32(0, 22, 0, 22, 0, 22, 0, 22),
        );
        let adjacent_4_combined = mm256_srli_epi64::<22>(adjacent_4_shifted);

        // Shuffle to bring the bits into a contiguous form, then shift up
        // by 12 in 32-bit lanes, view as 64-bit lanes, shift down by 12 to
        // pack adjacent 4-combined into adjacent 8-combined.
        // Equivalent to `mm256_shuffle_epi32::<0b00_00_10_00>` but expressed
        // via `mm256_shuffle_epi8`, which has a `BitVec.Intrinsics` spec
        // usable by `assert_norm`.  In each 128-bit half, places 32-bit
        // lane 2 into lane 1; lanes 0/2/3 retain old lane 0 (will be
        // masked off by the next sllv/srli pair).
        let adjacent_8_shuffled = mm256_shuffle_epi8(
            adjacent_4_combined,
            mm256_set_epi8(
                3, 2, 1, 0, 3, 2, 1, 0, 11, 10, 9, 8, 3, 2, 1, 0, 3, 2, 1, 0, 3, 2, 1, 0, 11, 10,
                9, 8, 3, 2, 1, 0,
            ),
        );
        let adjacent_8_shifted = mm256_sllv_epi32(
            adjacent_8_shuffled,
            mm256_set_epi32(0, 0, 0, 12, 0, 0, 0, 12),
        );
        let adjacent_8_combined = mm256_srli_epi64::<12>(adjacent_8_shifted);

        // We now have 40 bits starting at position 0 in the lower 128-bit lane, ...
        let lower_8 = mm256_castsi256_si128(adjacent_8_combined);
        // ... and the second 40 bits at position 0 in the upper 128-bit lane
        let upper_8 = mm256_extracti128_si256::<1>(adjacent_8_combined);

        proof!(
            r#"
introduce forall (i: nat{i < 80}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $vector ((i / 5) * 16 + i % 5) ==
      (if i < 40
       then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower_8 i
       else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper_8 (i - 40))
with (if i < 40
      then Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_serialize_5_lower_bits
             $vector $adjacent_2_combined i
      else (FStar.Math.Lemmas.lemma_div_plus (i - 40) 8 5;
            FStar.Math.Lemmas.lemma_mod_plus (i - 40) 8 5;
            Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_serialize_5_upper_bits
              $vector $adjacent_2_combined (i - 40)))
"#
        );
        (lower_8, upper_8)
    }

    let mut serialized = [0u8; 32];
    let (lower_8, upper_8) = serialize_5_vec(vector);
    // The two stores OVERLAP: the second clobbers bytes [5,16) of the first.
    // That is sound because only the low 40 bits of `lower_8` are live, and
    // they sit in bytes [0,5).  `lemma_store_glue_two_writes` consumes the
    // whole store spine in clean context — supplying its per-byte frame facts
    // HERE instead drags `update_at_range`, the two slice reads and the
    // unbounded `forall (j: nat)` of `lemma_index_update_at_range` into this
    // function's WP, which saturates at 400.000 even with `#restart-solver`.
    #[cfg(hax)]
    let ser0 = serialized;
    mm_storeu_bytes_si128(&mut serialized[0..16], lower_8);
    #[cfg(hax)]
    let ser1 = serialized;
    mm_storeu_bytes_si128(&mut serialized[5..21], upper_8);

    proof!(
        r#"
let o1 = Libcrux_intrinsics.Avx2.mm_storeu_bytes_si128
           ((${ser0}).[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                           Core_models.Ops.Range.f_end = mk_usize 16 }
                         <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8)
           ${lower_8} in
let o2 = Libcrux_intrinsics.Avx2.mm_storeu_bytes_si128
           ((${ser1}).[ ({ Core_models.Ops.Range.f_start = mk_usize 5;
                           Core_models.Ops.Range.f_end = mk_usize 21 }
                         <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8)
           ${upper_8} in
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm_storeu_bytes_si128
  ((${ser0}).[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                  Core_models.Ops.Range.f_end = mk_usize 16 }
                <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8) ${lower_8};
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm_storeu_bytes_si128
  ((${ser1}).[ ({ Core_models.Ops.Range.f_start = mk_usize 5;
                  Core_models.Ops.Range.f_end = mk_usize 21 }
                <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8) ${upper_8};
Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_store_glue_two_writes
  ${ser0} ${ser1} ${serialized} o1 o2 ${lower_8} ${upper_8} 5
  ({ Core_models.Ops.Range.f_start = mk_usize 0;
     Core_models.Ops.Range.f_end = mk_usize 16 } <: Core_models.Ops.Range.t_Range usize)
  ({ Core_models.Ops.Range.f_start = mk_usize 5;
     Core_models.Ops.Range.f_end = mk_usize 21 } <: Core_models.Ops.Range.t_Range usize);
introduce forall (i: nat{i < 80}).
    Rust_primitives.BitVectors.bit_vec_of_int_t_array ${serialized} 8 i ==
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${vector} ((i / 5) * 16 + i % 5)
with ()
"#
    );

    serialized[0..10].try_into().unwrap()
}

/// The 128 + 128 -> 256 concatenation: `mm256_castsi128_si256` zero-extends
/// into the low half and `mm256_inserti128_si256::<1>` replaces the high
/// 128-bit lane.
///
/// Over core-models both ops have models, so the contract below is proven
/// directly from the actual code.
#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --z3rlimit 300")]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 256}).
  Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $r i ==
    (if i < 128 then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower i
     else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper (i - 128))"#))]
fn mm256_si256_from_two_si128(lower: Vec128, upper: Vec128) -> Vec256 {
    let result = mm256_inserti128_si256::<1>(mm256_castsi128_si256(lower), upper);
    proof!(
        r#"
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i ==
      (if i < 128 then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower i
       else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper (i - 128))
with Libcrux_ml_kem.Vector.Avx2.Unpack_theory.lemma_bv_bit_si256_from_two_si128
       $lower $upper i
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 800")]
#[hax_lib::requires(fstar!(r#"Seq.length bytes == 10"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i = (if i % 16 >= 5 then 0
               else let j = (i / 16) * 5 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 10)) 8 j)"#))]
pub(crate) fn deserialize_5(bytes: &[u8]) -> Vec256 {
    // Inner helper takes a Vec128 directly (no Seq.index lookups in the closure),
    // so the assert_norm inside reduces over a free Vec128 rather than the chain
    // mm_set_epi8(bytes[i]…). Spec is keyed on c-bit positions; the byte-level
    // bridge is at the outer level via mm_set_epi8's spec.
    #[inline(always)]
    #[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
    #[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]
    #[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i =
        (if i % 16 >= 5 then 0
         else let shift_inv = ((i / 16) % 2) * 5 + (((i / 16) % 8) / 2) * 2 in
              let j = i + shift_inv in
              let byte_pos = j / 8 in
              let c_byte =
                if byte_pos < 16
                then (byte_pos / 4) * 2 + (byte_pos % 2)
                else ((byte_pos - 16) / 4) * 2 + ((byte_pos - 16) % 2) + 8 in
              Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $c (c_byte * 8 + j % 8))"#))]
    fn deserialize_5_vec(c: Vec128) -> Vec256 {
        let coefficients_loaded = mm256_si256_from_two_si128(c, c);

        let shuffled = mm256_shuffle_epi8(
            coefficients_loaded,
            mm256_set_epi8(
                15, 14, 15, 14, 13, 12, 13, 12, 11, 10, 11, 10, 9, 8, 9, 8, 7, 6, 7, 6, 5, 4, 5, 4,
                3, 2, 3, 2, 1, 0, 1, 0,
            ),
        );

        let scaled = mm256_mullo_epi16(
            shuffled,
            mm256_set_epi16(
                1 << 0,
                1 << 5,
                1 << 2,
                1 << 7,
                1 << 4,
                1 << 9,
                1 << 6,
                1 << 11,
                1 << 0,
                1 << 5,
                1 << 2,
                1 << 7,
                1 << 4,
                1 << 9,
                1 << 6,
                1 << 11,
            ),
        );
        let result = mm256_srli_epi16::<11>(scaled);
        proof!(
            r#"
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${result} i =
      (if i % 16 >= 5 then 0
       else let shift_inv = ((i / 16) % 2) * 5 + (((i / 16) % 8) / 2) * 2 in
            let j = i + shift_inv in
            let byte_pos = j / 8 in
            let c_byte =
              if byte_pos < 16
              then (byte_pos / 4) * 2 + (byte_pos % 2)
              else ((byte_pos - 16) / 4) * 2 + ((byte_pos - 16) % 2) + 8 in
            Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${c} (c_byte * 8 + j % 8))
with Libcrux_ml_kem.Vector.Avx2.Unpack_theory.lemma_deserialize_5_bits
       ${c} ${coefficients_loaded} i
"#
        );
        result
    }

    // The ten source bytes are bound to locals so the byte-bridge lemma can
    // name them: it pins them to `Seq.index bytes k` by EQUATIONAL requires,
    // which the caller discharges from these let-equations, so no slice
    // reasoning enters the lemma's own context.
    let b0 = bytes[0];
    let b1 = bytes[1];
    let b2 = bytes[2];
    let b3 = bytes[3];
    let b4 = bytes[4];
    let b5 = bytes[5];
    let b6 = bytes[6];
    let b7 = bytes[7];
    let b8 = bytes[8];
    let b9 = bytes[9];
    let coefficients = mm_set_epi8(
        b9 as i8, b8 as i8, b8 as i8, b7 as i8, b7 as i8, b6 as i8, b6 as i8, b5 as i8, b4 as i8,
        b3 as i8, b3 as i8, b2 as i8, b2 as i8, b1 as i8, b1 as i8, b0 as i8,
    );
    let result = deserialize_5_vec(coefficients);
    proof!(
        r#"
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${result} i =
      (if i % 16 >= 5 then 0
       else let j = (i / 16) * 5 + i % 16 in
            Rust_primitives.BitVectors.bit_vec_of_int_t_array
              (${bytes} <: t_Array u8 (sz 10)) 8 j)
with Libcrux_ml_kem.Vector.Avx2.Unpack_theory.lemma_deserialize_5_outer_bits
       ${bytes} ${b0} ${b1} ${b2} ${b3} ${b4} ${b5} ${b6} ${b7} ${b8} ${b9} ${result} i
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 10 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector i = 0"#))]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 160}). bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector ((i/10) * 16 + i%10)"#))]
pub(crate) fn serialize_10(vector: Vec256) -> [u8; 20] {
    #[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
    #[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 10 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector i = 0"#))]
    #[hax_lib::ensures(|(lower_8, upper_8)| fstar!(
        r#"
         forall (i: nat{i < 160}).
           Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector ((i/10) * 16 + i%10) ==
             (if i < 80 then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower_8 i
              else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper_8 (i - 80))
      )
    "#
    ))]
    fn serialize_10_vec(vector: Vec256) -> (Vec128, Vec128) {
        // If |vector| is laid out as follows (superscript number indicates the
        // corresponding bit is duplicated that many times):
        //
        // 0⁶a₉a₈a₇a₆a₅a₄a₃a₂a₁a₀ 0⁶b₉b₈b₇b₆b₅b₄b₃b₂b₁b₀ 0⁶c₉c₈c₇c₆c₅c₄c₃c₂c₁c₀ 0⁶d₉d₈d₇d₆d₅d₄d₃d₂d₁d₀ | ↩
        // 0⁶e₉e₈e₇e₆e₅e₄e₃e₂e₁e₀ 0⁶f₉f₈f₇f₆f₅f₄f₃f₂f₁f₀ 0⁶g₉g₈g₇g₆g₅g₄g₃g₂g₁g₀ 0⁶h₉h₈h₇h₆h₅h₄h₃h₂h₁h₀ | ↩
        // ...
        //
        // |adjacent_2_combined| will be laid out as a series of 32-bit integers,
        // as follows:
        //
        // 0¹²b₉b₈b₇b₆b₅b₄b₃b₂b₁b₀a₉a₈a₇a₆a₅a₄a₃a₂a₁a₀ 0¹²d₉d₈d₇d₆d₅d₄d₃d₂d₁d₀c₉c₈c₇c₆c₅c₄c₃c₂c₁c₀ | ↩
        // 0¹²f₉f₈f₇f₆f₅f₄f₃f₂f₁f₀e₉e₈e₇e₆e₅e₄e₃e₂e₁e₀ 0¹²h₉h₈h₇h₆h₅h₄h₃h₂h₁h₀g₉g₈g₇g₆g₅g₄g₃g₂g₁g₀ | ↩
        // ....
        let adjacent_2_combined = mm256_concat_pairs_n(10, vector);

        // Shifting up the values at the even indices by 12, we get:
        //
        // b₉b₈b₇b₆b₅b₄b₃b₂b₁b₀a₉a₈a₇a₆a₅a₄a₃a₂a₁a₀0¹² 0¹²d₉d₈d₇d₆d₅d₄d₃d₂d₁d₀c₉c₈c₇c₆c₅c₄c₃c₂c₁c₀ | ↩
        // f₉f₈f₇f₆f₅f₄f₃f₂f₁f₀e₉e₈e₇e₆e₅e₄e₃e₂e₁e₀0¹² 0¹²h₉h₈h₇h₆h₅h₄h₃h₂h₁h₀g₉g₈g₇g₆g₅g₄g₃g₂g₁g₀ | ↩
        // ...
        // NB `adjacent_4_shifted` is unshadowed from the `srli` result below: a
        // rebound name drops its earlier let-equation, and the gather lemmas
        // need the whole chain in scope.
        let adjacent_4_shifted = mm256_sllv_epi32(
            adjacent_2_combined,
            mm256_set_epi32(0, 12, 0, 12, 0, 12, 0, 12),
        );

        // Viewing this as a set of 64-bit integers we get:
        //
        // 0¹²d₉d₈d₇d₆d₅d₄d₃d₂d₁d₀c₉c₈c₇c₆c₅c₄c₃c₂c₁c₀b₉b₈b₇b₆b₅b₄b₃b₂b₁b₀a₉a₈a₇a₆a₅a₄a₃a₂a₁a₀0¹²  | ↩
        // 0¹²h₉h₈h₇h₆h₅h₄h₃h₂h₁h₀g₉g₈g₇g₆g₅g₄g₃g₂g₁g₀f₉f₈f₇f₆f₅f₄f₃f₂f₁f₀e₉e₈e₇e₆e₅e₄e₃e₂e₁e₀0¹²  | ↩
        // ...
        //
        // Shifting down by 12 gives us:
        //
        // 0²⁴d₉d₈d₇d₆d₅d₄d₃d₂d₁d₀c₉c₈c₇c₆c₅c₄c₃c₂c₁c₀b₉b₈b₇b₆b₅b₄b₃b₂b₁b₀a₉a₈a₇a₆a₅a₄a₃a₂a₁a₀ | ↩
        // 0²⁴h₉h₈h₇h₆h₅h₄h₃h₂h₁h₀g₉g₈g₇g₆g₅g₄g₃g₂g₁g₀f₉f₈f₇f₆f₅f₄f₃f₂f₁f₀e₉e₈e₇e₆e₅e₄e₃e₂e₁e₀ | ↩
        // ...
        let adjacent_4_combined = mm256_srli_epi64::<12>(adjacent_4_shifted);

        // |adjacent_4_combined|, when the bottom and top 128 bit-lanes are grouped
        // into bytes, looks like:
        //
        // 0₇0₆0₅B₄B₃B₂B₁B₀ | ↩
        // 0₁₅0₁₄0₁₃B₁₂B₁₁B₁₀B₉B₈ | ↩
        //
        // In each 128-bit lane, we want to put bytes 8, 9, 10, 11, 12 after
        // bytes 0, 1, 2, 3 to allow for sequential reading.
        let adjacent_8_combined = mm256_shuffle_epi8(
            adjacent_4_combined,
            mm256_set_epi8(
                -1, -1, -1, -1, -1, -1, 12, 11, 10, 9, 8, 4, 3, 2, 1, 0, -1, -1, -1, -1, -1, -1,
                12, 11, 10, 9, 8, 4, 3, 2, 1, 0,
            ),
        );
        // We now have 64 bits starting at position 0 in the lower 128-bit lane, ...
        let lower_8 = mm256_castsi256_si128(adjacent_8_combined);
        // and 64 bits starting at position 0 in the upper 128-bit lane.
        let upper_8 = mm256_extracti128_si256::<1>(adjacent_8_combined);
        proof!(
            r#"
introduce forall (i: nat{i < 160}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $vector ((i / 10) * 16 + i % 10) ==
      (if i < 80
       then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower_8 i
       else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper_8 (i - 80))
with (if i < 80
      then Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_serialize_10_lower_bits
             $vector $adjacent_2_combined i
      else (FStar.Math.Lemmas.lemma_div_plus (i - 80) 8 10;
            FStar.Math.Lemmas.lemma_mod_plus (i - 80) 8 10;
            Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_serialize_10_upper_bits
              $vector $adjacent_2_combined (i - 80)))
"#
        );
        (lower_8, upper_8)
    }

    let (lower_8, upper_8) = serialize_10_vec(vector);

    let mut serialized = [0u8; 32];
    // Same two-overlapping-store shape as serialize_5, at off = 10: the second
    // store clobbers bytes [10,16) of the first, sound because only the low 80
    // bits of `lower_8` are live and they sit in bytes [0,10).
    // `lemma_store_glue_two_writes` consumes the whole spine in clean context.
    #[cfg(hax)]
    let ser0 = serialized;
    mm_storeu_bytes_si128(&mut serialized[0..16], lower_8);
    #[cfg(hax)]
    let ser1 = serialized;
    mm_storeu_bytes_si128(&mut serialized[10..26], upper_8);

    proof!(
        r#"
let o1 = Libcrux_intrinsics.Avx2.mm_storeu_bytes_si128
           ((${ser0}).[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                           Core_models.Ops.Range.f_end = mk_usize 16 }
                         <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8)
           ${lower_8} in
let o2 = Libcrux_intrinsics.Avx2.mm_storeu_bytes_si128
           ((${ser1}).[ ({ Core_models.Ops.Range.f_start = mk_usize 10;
                           Core_models.Ops.Range.f_end = mk_usize 26 }
                         <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8)
           ${upper_8} in
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm_storeu_bytes_si128
  ((${ser0}).[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                  Core_models.Ops.Range.f_end = mk_usize 16 }
                <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8) ${lower_8};
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm_storeu_bytes_si128
  ((${ser1}).[ ({ Core_models.Ops.Range.f_start = mk_usize 10;
                  Core_models.Ops.Range.f_end = mk_usize 26 }
                <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8) ${upper_8};
Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_store_glue_two_writes
  ${ser0} ${ser1} ${serialized} o1 o2 ${lower_8} ${upper_8} 10
  ({ Core_models.Ops.Range.f_start = mk_usize 0;
     Core_models.Ops.Range.f_end = mk_usize 16 } <: Core_models.Ops.Range.t_Range usize)
  ({ Core_models.Ops.Range.f_start = mk_usize 10;
     Core_models.Ops.Range.f_end = mk_usize 26 } <: Core_models.Ops.Range.t_Range usize);
introduce forall (i: nat{i < 160}).
    Rust_primitives.BitVectors.bit_vec_of_int_t_array ${serialized} 8 i ==
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${vector} ((i / 10) * 16 + i % 10)
with ()
"#
    );

    serialized[0..20].try_into().unwrap()
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"Seq.length bytes == 20"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i = (if i % 16 >= 10 then 0
               else let j = (i / 16) * 10 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 20)) 8 j)"#))]
pub(crate) fn deserialize_10(bytes: &[u8]) -> Vec256 {
    #[inline(always)]
    #[hax_lib::ensures(|coefficients| fstar!(r#"
forall (i: nat {i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $coefficients i
    = ( if i % 16 >= 10 then 0
        else let j = (i / 16) * 10 + i % 16 in
             if i < 128 then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower_coefficients0 j
             else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper_coefficients0 (j - 32)))
"#))]
    #[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
    #[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]
    fn deserialize_10_vec(lower_coefficients0: Vec128, upper_coefficients0: Vec128) -> Vec256 {
        let lower_coefficients = mm_shuffle_epi8(
            lower_coefficients0,
            mm_set_epi8(9, 8, 8, 7, 7, 6, 6, 5, 4, 3, 3, 2, 2, 1, 1, 0),
        );
        let upper_coefficients = mm_shuffle_epi8(
            upper_coefficients0,
            mm_set_epi8(15, 14, 14, 13, 13, 12, 12, 11, 10, 9, 9, 8, 8, 7, 7, 6),
        );

        let concatenated = mm256_si256_from_two_si128(lower_coefficients, upper_coefficients);

        let scaled = mm256_mullo_epi16(
            concatenated,
            mm256_set_epi16(
                1 << 0,
                1 << 2,
                1 << 4,
                1 << 6,
                1 << 0,
                1 << 2,
                1 << 4,
                1 << 6,
                1 << 0,
                1 << 2,
                1 << 4,
                1 << 6,
                1 << 0,
                1 << 2,
                1 << 4,
                1 << 6,
            ),
        );
        let shifted = mm256_srli_epi16::<6>(scaled);
        // Here I can prove this `and` is not useful
        let coefficients = mm256_and_si256(shifted, mm256_set1_epi16((1 << 10) - 1));
        proof!(
            r#"
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${coefficients} i
    = ( if i % 16 >= 10 then 0
        else let j = (i / 16) * 10 + i % 16 in
             if i < 128 then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${lower_coefficients0} j
             else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${upper_coefficients0} (j - 32))
with Libcrux_ml_kem.Vector.Avx2.Unpack_theory.lemma_deserialize_10_bits
       ${lower_coefficients0} ${upper_coefficients0} ${concatenated} i
"#
        );
        coefficients
    }

    let lower_bytes = &bytes[0..16];
    let upper_bytes = &bytes[4..20];
    let lower_coefficients = mm_loadu_si128(lower_bytes);
    let upper_coefficients = mm_loadu_si128(upper_bytes);
    let result = deserialize_10_vec(lower_coefficients, upper_coefficients);
    proof!(
        r#"
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${result} i
    = ( if i % 16 >= 10 then 0
        else let j = (i / 16) * 10 + i % 16 in
             Rust_primitives.BitVectors.bit_vec_of_int_t_array
               (${bytes} <: t_Array u8 (sz 20)) 8 j)
with (if i % 16 < 10
      then (let j = (i / 16) * 10 + i % 16 in
            if i < 128
            then Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit_mm_loadu_si128 ${lower_bytes} j
            else (FStar.Math.Lemmas.lemma_div_plus (j - 32) 4 8;
                  FStar.Math.Lemmas.lemma_mod_plus (j - 32) 4 8;
                  Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit_mm_loadu_si128
                    ${upper_bytes} (j - 32))))
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --z3rlimit 200")]
// serialize_11's lane-bound bridge is the canonical `lemma_vec256_lane_bounded`
// in the companion Libcrux_ml_kem.Vector.Avx2_theory (a companion is checked
// before both Serialize and Vector.Avx2, so no local copy is needed).
#[hax_lib::fstar::before(r#"open Libcrux_ml_kem.Vector.Avx2_theory"#)]
#[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 11 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector i = 0"#))]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 176}). bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector ((i/11) * 16 + i%11)"#))]
pub(crate) fn serialize_11(vector: Vec256) -> [u8; 22] {
    let mut array = [0i16; 16];
    #[cfg(hax)]
    let array0 = array;
    mm256_storeu_si256_i16(&mut array, vector);
    proof!(
        r#"
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_storeu_si256_i16 ($array0 <: t_Slice i16) ${vector};
assert (array == Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${vector});
introduce forall (j: nat). j < 16 ==>
    Rust_primitives.BitVectors.bounded (Seq.index array j) 11
with introduce j < 16 ==>
    Rust_primitives.BitVectors.bounded (Seq.index array j) 11
with _. Libcrux_ml_kem.Vector.Avx2_theory.lemma_vec256_lane_bounded ${vector} 11 j
"#
    );
    let input = PortableVector::from_i16_array(&array);
    let result = PortableVector::serialize_11(input);
    proof!(
        r#"
introduce forall (i: nat{i < 176}).
    bit_vec_of_int_t_array result 8 i ==
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${vector} ((i / 11) * 16 + i % 11)
with begin
  Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
    ${vector} 11 i
end
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --z3rlimit 200")]
#[hax_lib::requires(fstar!(r#"Seq.length bytes == 22"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i = (if i % 16 >= 11 then 0
               else let j = (i / 16) * 11 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 22)) 8 j)"#))]
pub(crate) fn deserialize_11(bytes: &[u8]) -> Vec256 {
    let output = PortableVector::deserialize_11(bytes);
    let array = PortableVector::to_i16_array(output);
    let result = mm256_loadu_si256_i16(&array);
    proof!(
        r#"
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm256_loadu_si256_i16 ($array <: t_Slice i16);
assert (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result == $array);
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit result i =
      (if i % 16 >= 11 then 0
       else let j = (i / 16) * 11 + i % 16 in
            bit_vec_of_int_t_array (${bytes} <: t_Array _ (sz 22)) 8 j)
with begin
  if i % 16 >= 11 then begin
    Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
      result 16 i;
    ()
  end else begin
    Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
      result 11 ((i / 16) * 11 + i % 16)
  end
end
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 12 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector i = 0"#))]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 192}). bit_vec_of_int_t_array r 8 i == Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector ((i/12) * 16 + i%12)"#))]
pub(crate) fn serialize_12(vector: Vec256) -> [u8; 24] {
    #[inline(always)]
    #[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
    #[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 12 || Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector i = 0"#))]
    #[hax_lib::ensures(|(lower_8, upper_8)| fstar!(
        r#"
         forall (i: nat{i < 192}).
           Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit vector ((i/12) * 16 + i%12) ==
             (if i < 96 then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower_8 i
              else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper_8 (i - 96))
      )
    "#
    ))]
    fn serialize_12_vec(vector: Vec256) -> (Vec128, Vec128) {
        let adjacent_2_combined = mm256_concat_pairs_n(12, vector);
        // NB `adjacent_4_shifted` is unshadowed from the `srli` result below: a
        // rebound name drops its earlier let-equation, and the gather lemmas
        // need the whole chain in scope.
        let adjacent_4_shifted =
            mm256_sllv_epi32(adjacent_2_combined, mm256_set_epi32(0, 8, 0, 8, 0, 8, 0, 8));
        let adjacent_4_combined = mm256_srli_epi64::<8>(adjacent_4_shifted);

        let adjacent_8_combined = mm256_shuffle_epi8(
            adjacent_4_combined,
            mm256_set_epi8(
                -1, -1, -1, -1, 13, 12, 11, 10, 9, 8, 5, 4, 3, 2, 1, 0, -1, -1, -1, -1, 13, 12, 11,
                10, 9, 8, 5, 4, 3, 2, 1, 0,
            ),
        );

        let lower_8 = mm256_castsi256_si128(adjacent_8_combined);
        let upper_8 = mm256_extracti128_si256::<1>(adjacent_8_combined);
        proof!(
            r#"
introduce forall (i: nat{i < 192}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $vector ((i / 12) * 16 + i % 12) ==
      (if i < 96
       then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower_8 i
       else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper_8 (i - 96))
with (if i < 96
      then Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_serialize_12_lower_bits
             $vector $adjacent_2_combined i
      else (FStar.Math.Lemmas.lemma_div_plus (i - 96) 8 12;
            FStar.Math.Lemmas.lemma_mod_plus (i - 96) 8 12;
            Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_serialize_12_upper_bits
              $vector $adjacent_2_combined (i - 96)))
"#
        );
        (lower_8, upper_8)
    }

    let mut serialized = [0u8; 32];
    let (lower_8, upper_8) = serialize_12_vec(vector);
    // Same two-overlapping-store shape as serialize_5, at off = 12: the second
    // store clobbers bytes [12,16) of the first, sound because only the low 96
    // bits of `lower_8` are live and they sit in bytes [0,12).
    #[cfg(hax)]
    let ser0 = serialized;
    mm_storeu_bytes_si128(&mut serialized[0..16], lower_8);
    #[cfg(hax)]
    let ser1 = serialized;
    mm_storeu_bytes_si128(&mut serialized[12..28], upper_8);

    proof!(
        r#"
let o1 = Libcrux_intrinsics.Avx2.mm_storeu_bytes_si128
           ((${ser0}).[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                           Core_models.Ops.Range.f_end = mk_usize 16 }
                         <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8)
           ${lower_8} in
let o2 = Libcrux_intrinsics.Avx2.mm_storeu_bytes_si128
           ((${ser1}).[ ({ Core_models.Ops.Range.f_start = mk_usize 12;
                           Core_models.Ops.Range.f_end = mk_usize 28 }
                         <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8)
           ${upper_8} in
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm_storeu_bytes_si128
  ((${ser0}).[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                  Core_models.Ops.Range.f_end = mk_usize 16 }
                <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8) ${lower_8};
Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm_storeu_bytes_si128
  ((${ser1}).[ ({ Core_models.Ops.Range.f_start = mk_usize 12;
                  Core_models.Ops.Range.f_end = mk_usize 28 }
                <: Core_models.Ops.Range.t_Range usize) ] <: t_Slice u8) ${upper_8};
Libcrux_ml_kem.Vector.Avx2.Byteperm_theory.lemma_store_glue_two_writes
  ${ser0} ${ser1} ${serialized} o1 o2 ${lower_8} ${upper_8} 12
  ({ Core_models.Ops.Range.f_start = mk_usize 0;
     Core_models.Ops.Range.f_end = mk_usize 16 } <: Core_models.Ops.Range.t_Range usize)
  ({ Core_models.Ops.Range.f_start = mk_usize 12;
     Core_models.Ops.Range.f_end = mk_usize 28 } <: Core_models.Ops.Range.t_Range usize);
introduce forall (i: nat{i < 192}).
    Rust_primitives.BitVectors.bit_vec_of_int_t_array ${serialized} 8 i ==
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${vector} ((i / 12) * 16 + i % 12)
with ()
"#
    );

    serialized[0..24].try_into().unwrap()
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"Seq.length bytes == 24"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $result i = (if i % 16 >= 12 then 0
               else let j = (i / 16) * 12 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 24)) 8 j)"#))]
pub(crate) fn deserialize_12(bytes: &[u8]) -> Vec256 {
    #[inline(always)]
    #[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
    #[hax_lib::ensures(|coefficients| fstar!(r#"
forall (i: nat {i < 256}).
      Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $coefficients i
    = ( if i % 16 >= 12 then 0
        else let j = (i / 16) * 12 + i % 16 in
             if i < 128 then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $lower_coefficients0 j
             else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit $upper_coefficients0 (j - 64)))
"#))]
    #[hax_lib::fstar::before(r#"[@@"opaque_to_smt"]"#)]
    fn deserialize_12_vec(lower_coefficients0: Vec128, upper_coefficients0: Vec128) -> Vec256 {
        let lower_coefficients = mm_shuffle_epi8(
            lower_coefficients0,
            mm_set_epi8(11, 10, 10, 9, 8, 7, 7, 6, 5, 4, 4, 3, 2, 1, 1, 0),
        );
        let upper_coefficients = mm_shuffle_epi8(
            upper_coefficients0,
            mm_set_epi8(15, 14, 14, 13, 12, 11, 11, 10, 9, 8, 8, 7, 6, 5, 5, 4),
        );

        let concatenated = mm256_si256_from_two_si128(lower_coefficients, upper_coefficients);

        let scaled = mm256_mullo_epi16(
            concatenated,
            mm256_set_epi16(
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
                1 << 0,
                1 << 4,
            ),
        );
        let shifted = mm256_srli_epi16::<4>(scaled);
        let coefficients = mm256_and_si256(shifted, mm256_set1_epi16((1 << 12) - 1));
        proof!(
            r#"
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${coefficients} i
    = ( if i % 16 >= 12 then 0
        else let j = (i / 16) * 12 + i % 16 in
             if i < 128 then Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${lower_coefficients0} j
             else Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${upper_coefficients0} (j - 64))
with Libcrux_ml_kem.Vector.Avx2.Unpack_theory.lemma_deserialize_12_bits
       ${lower_coefficients0} ${upper_coefficients0} ${concatenated} i
"#
        );
        coefficients
    }
    let lower_bytes = &bytes[0..16];
    let upper_bytes = &bytes[8..24];
    let lower_coefficients = mm_loadu_si128(lower_bytes);
    let upper_coefficients = mm_loadu_si128(upper_bytes);
    let result = deserialize_12_vec(lower_coefficients, upper_coefficients);
    proof!(
        r#"
introduce forall (i: nat{i < 256}).
    Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${result} i
    = ( if i % 16 >= 12 then 0
        else let j = (i / 16) * 12 + i % 16 in
             Rust_primitives.BitVectors.bit_vec_of_int_t_array
               (${bytes} <: t_Array u8 (sz 24)) 8 j)
with (if i % 16 < 12
      then (let j = (i / 16) * 12 + i % 16 in
            if i < 128
            then Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit_mm_loadu_si128 ${lower_bytes} j
            else (FStar.Math.Lemmas.lemma_div_plus (j - 64) 8 8;
                  FStar.Math.Lemmas.lemma_mod_plus (j - 64) 8 8;
                  Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit_mm_loadu_si128
                    ${upper_bytes} (j - 64))))
"#
    );
    result
}
