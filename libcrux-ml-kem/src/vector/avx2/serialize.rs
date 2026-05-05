use super::*;
use crate::vector::portable::PortableVector;

#[inline(always)]
// FOLLOW-UP (sprint 2026-05-10): the `prove_forall_nat_pointwise` tactic
// below at i=1 fails Z3 deterministically with "incomplete quantifiers"
// (uses ~1.5 of 80 rlimit, then gives up).  Earlier verified runs on
// this branch passed via hint replay; once those hints are invalidated
// the proof doesn't go through from scratch.  Reverted to `lax` so the
// rest of the module typechecks; restore to `panic_free` after the
// tactic is stabilised (replace with explicit per-lane lemma, or split
// the assertion into per-lane cases that don't need quantifier
// instantiation).  Tracked in proofs/agent-status/sprint-2026-05-10-status.md.
#[hax_lib::fstar::verification_status(lax)]
#[hax_lib::fstar::options("--ext context_pruning --compat_pre_core 0")]
#[hax_lib::requires(fstar!(r#"forall i. i % 16 >= 1 ==> vector i == 0"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i. bit_vec_of_int_t_array $result 8 i == $vector (i * 16)"#))]
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

    hax_lib::fstar!(
        r#"
let bits_packed' = BitVec.Intrinsics.mm_movemask_epi8_bv msbs in
  assert (forall (i: nat{i < 16}). bits_packed' i = $vector ((i / 1) * 16 + i % 1))
      by (
         Tactics.Utils.prove_forall_nat_pointwise (fun _ -> 
           Tactics.compute ();
           Tactics.smt_sync ()
         )
      )
"#
    );

    // Now that every element is either 0xFF or 0x00, we just extract the most
    // significant bit from each element and collate them into two bytes.
    let bits_packed = mm_movemask_epi8(msbs);

    let result = [bits_packed as u8, (bits_packed >> 8) as u8];

    hax_lib::fstar!(
        r#"
assert (forall (i: nat {i < 8}). get_bit ($bits_packed >>! (mk_i32 8) <: i32) (sz i) == get_bit $bits_packed (sz (i + 8)))
"#
    );

    result
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 2)]
#[hax_lib::ensures(|coefficients| fstar!(
        r#"forall (i:nat{i < 256}).
      $coefficients i
    = ( if i % 16 >= 1 then 0
        else let j = (i / 16) * 1 + i % 16 in
             bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 2)) 8 j))
"#
))]
#[hax_lib::fstar::before("#restart-solver")]
pub(crate) fn deserialize_1(bytes: &[u8]) -> Vec256 {
    #[hax_lib::ensures(|coefficients| fstar!(
        r#"forall (i:nat{i < 256}).
      $coefficients i
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
      $coefficients i
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
        mm256_srli_epi16::<15>(coefficients_in_msb)
    }

    deserialize_1_u8s(bytes[0], bytes[1])
}

/// `mm256_concat_pairs_n(n, x)` is then a sequence of 32 bits packets
/// of the shape `0b0…0b₁…bₙa₁…aₙ`, if `x` is a sequence of pairs of
/// 16 bits, of the shape `(0b0…0a₁…aₙ, 0b0…0b₁…bₙ)` (where the last
/// `n` bits are non-zero).
#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_concat_pairs_n}")]
#[inline(always)]
fn mm256_concat_pairs_n(n: u8, x: Vec256) -> Vec256 {
    let n = 1 << n;
    mm256_madd_epi16(
        x,
        mm256_set_epi16(n, 1, n, 1, n, 1, n, 1, n, 1, n, 1, n, 1, n, 1),
    )
}

#[hax_lib::fstar::options("--ext context_pruning --split_queries always")]
#[hax_lib::requires(
    fstar!(
        r#"forall (i: nat{i < 256}). i % 16 < 4 || $vector i = 0"#
    )
)]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 64}). bit_vec_of_int_t_array $r 8 i == $vector ((i/4) * 16 + i%4)"#))]
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
    let combined =
        mm256_permutevar8x32_epi32(adjacent_8_combined, mm256_set_epi32(0, 0, 0, 0, 0, 0, 4, 0));
    let combined = mm256_castsi256_si128(combined);

    // ... so that we can read them out in one go.
    mm_storeu_bytes_si128(&mut serialized, combined);

    hax_lib::fstar!(
        r#"
assert (forall (i: nat{i < 64}). $combined i == bit_vec_of_int_t_array serialized 8 i);
  introduce forall (i: nat {i < 64}). $combined i = vector ((i / 4) * 16 + i % 4)
  with assert_norm (BitVec.Utils.forall64 (fun i -> $combined i = $vector ((i / 4) * 16 + i % 4)));
  assert (forall (i: nat{i < 64}). bit_vec_of_int_t_array serialized 8 i == $vector ((i / 4) * 16 + i % 4))
"#
    );

    serialized[0..8].try_into().unwrap()
}

#[inline(always)]
#[hax_lib::requires(bytes.len() == 8)]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  $result i = (if i % 16 >= 4 then 0
               else let j = (i / 16) * 4 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 8)) 8 j)"#))]
#[hax_lib::fstar::before("#restart-solver")]
pub(crate) fn deserialize_4(bytes: &[u8]) -> Vec256 {
    #[hax_lib::ensures(|coefficients| fstar!(
        r#"forall (i:nat{i < 256}).
      $coefficients i
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
      $coefficients i
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
        mm256_and_si256(coefficients_in_lsb, mm256_set1_epi16((1 << 4) - 1))
    }

    deserialize_4_u8s(
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    )
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 5 || vector i = 0"#))]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 80}). bit_vec_of_int_t_array r 8 i == vector ((i/5) * 16 + i%5)"#))]
pub(crate) fn serialize_5(vector: Vec256) -> [u8; 10] {
    #[inline(always)]
    #[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
    #[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 5 || vector i = 0"#))]
    #[hax_lib::ensures(|(lower_8, upper_8)| fstar!(
        r#"
         forall (i: nat{i < 80}).
           vector ((i/5) * 16 + i%5) == (if i < 40 then $lower_8 i else $upper_8 (i - 40))
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
        let adjacent_4_combined = mm256_sllv_epi32(
            adjacent_2_combined,
            mm256_set_epi32(0, 22, 0, 22, 0, 22, 0, 22),
        );
        let adjacent_4_combined = mm256_srli_epi64::<22>(adjacent_4_combined);

        // Shuffle to bring the bits into a contiguous form, then shift up
        // by 12 in 32-bit lanes, view as 64-bit lanes, shift down by 12 to
        // pack adjacent 4-combined into adjacent 8-combined.
        // Equivalent to `mm256_shuffle_epi32::<0b00_00_10_00>` but expressed
        // via `mm256_shuffle_epi8`, which has a `BitVec.Intrinsics` spec
        // usable by `assert_norm`.  In each 128-bit half, places 32-bit
        // lane 2 into lane 1; lanes 0/2/3 retain old lane 0 (will be
        // masked off by the next sllv/srli pair).
        let adjacent_8_combined = mm256_shuffle_epi8(
            adjacent_4_combined,
            mm256_set_epi8(
                3, 2, 1, 0, 3, 2, 1, 0, 11, 10, 9, 8, 3, 2, 1, 0, 3, 2, 1, 0, 3, 2, 1, 0, 11, 10, 9,
                8, 3, 2, 1, 0,
            ),
        );
        let adjacent_8_combined = mm256_sllv_epi32(
            adjacent_8_combined,
            mm256_set_epi32(0, 0, 0, 12, 0, 0, 0, 12),
        );
        let adjacent_8_combined = mm256_srli_epi64::<12>(adjacent_8_combined);

        // We now have 40 bits starting at position 0 in the lower 128-bit lane, ...
        let lower_8 = mm256_castsi256_si128(adjacent_8_combined);
        // ... and the second 40 bits at position 0 in the upper 128-bit lane
        let upper_8 = mm256_extracti128_si256::<1>(adjacent_8_combined);

        hax_lib::fstar!(
            r#"
    introduce forall (i:nat{i < 40}). lower_8_ i = vector ((i / 5) * 16 + i % 5)
    with assert_norm (BitVec.Utils.forall_n 40 (fun i -> lower_8_ i = vector ((i / 5) * 16 + i % 5)));
    introduce forall (i:nat{i < 40}). upper_8_ i = vector (128 + (i / 5) * 16 + i % 5)
    with assert_norm (BitVec.Utils.forall_n 40 (fun i -> upper_8_ i = vector (128 + (i / 5) * 16 + i % 5)))
    "#
        );
        (lower_8, upper_8)
    }

    let mut serialized = [0u8; 32];
    let (lower_8, upper_8) = serialize_5_vec(vector);
    mm_storeu_bytes_si128(&mut serialized[0..16], lower_8);
    mm_storeu_bytes_si128(&mut serialized[5..21], upper_8);

    serialized[0..10].try_into().unwrap()
}

/// We cannot model `mm256_inserti128_si256` on its own: it produces a
/// Vec256 where the upper 128 bits are undefined. Thus
/// `mm256_inserti128_si256` is not pure.
///
/// Luckily, we always call `mm256_castsi128_si256` right after
/// `mm256_inserti128_si256`: this composition sets the upper bits,
/// making the whole computation pure again.
#[inline(always)]
#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm256_si256_from_two_si128 as ${mm256_si256_from_two_si128}}"
)]
fn mm256_si256_from_two_si128(lower: Vec128, upper: Vec128) -> Vec256 {
    mm256_inserti128_si256::<1>(mm256_castsi128_si256(lower), upper)
}

#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"Seq.length bytes == 10"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  $result i = (if i % 16 >= 5 then 0
               else let j = (i / 16) * 5 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 10)) 8 j)"#))]
pub(crate) fn deserialize_5(bytes: &[u8]) -> Vec256 {
    let coefficients = mm_set_epi8(
        bytes[9] as i8,
        bytes[8] as i8,
        bytes[8] as i8,
        bytes[7] as i8,
        bytes[7] as i8,
        bytes[6] as i8,
        bytes[6] as i8,
        bytes[5] as i8,
        bytes[4] as i8,
        bytes[3] as i8,
        bytes[3] as i8,
        bytes[2] as i8,
        bytes[2] as i8,
        bytes[1] as i8,
        bytes[1] as i8,
        bytes[0] as i8,
    );

    let coefficients_loaded = mm256_si256_from_two_si128(coefficients, coefficients);

    let coefficients = mm256_shuffle_epi8(
        coefficients_loaded,
        mm256_set_epi8(
            15, 14, 15, 14, 13, 12, 13, 12, 11, 10, 11, 10, 9, 8, 9, 8, 7, 6, 7, 6, 5, 4, 5, 4, 3,
            2, 3, 2, 1, 0, 1, 0,
        ),
    );

    let coefficients = mm256_mullo_epi16(
        coefficients,
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
    mm256_srli_epi16::<11>(coefficients)
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 10 || vector i = 0"#))]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 160}). bit_vec_of_int_t_array r 8 i == vector ((i/10) * 16 + i%10)"#))]
pub(crate) fn serialize_10(vector: Vec256) -> [u8; 20] {
    #[hax_lib::fstar::options("--ext context_pruning --split_queries always")]
    #[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 10 || vector i = 0"#))]
    #[hax_lib::ensures(|(lower_8, upper_8)| fstar!(
        r#"
         forall (i: nat{i < 160}).
           vector ((i/10) * 16 + i%10) == (if i < 80 then $lower_8 i else $upper_8 (i - 80))
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
        let adjacent_4_combined = mm256_sllv_epi32(
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
        let adjacent_4_combined = mm256_srli_epi64::<12>(adjacent_4_combined);

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
        hax_lib::fstar!(
            r#"
    introduce forall (i:nat{i < 80}). lower_8_ i = vector ((i / 10) * 16 + i % 10)
    with assert_norm (BitVec.Utils.forall_n 80 (fun i -> lower_8_ i = vector ((i / 10) * 16 + i % 10)));
    introduce forall (i:nat{i < 80}). upper_8_ i = vector (128 + (i / 10) * 16 + i % 10)
    with assert_norm (BitVec.Utils.forall_n 80 (fun i -> upper_8_ i = vector (128 + (i / 10) * 16 + i % 10)))
    "#
        );
        (lower_8, upper_8)
    }

    let (lower_8, upper_8) = serialize_10_vec(vector);

    let mut serialized = [0u8; 32];
    mm_storeu_bytes_si128(&mut serialized[0..16], lower_8);
    mm_storeu_bytes_si128(&mut serialized[10..26], upper_8);

    serialized[0..20].try_into().unwrap()
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Seq.length bytes == 20"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  $result i = (if i % 16 >= 10 then 0
               else let j = (i / 16) * 10 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 20)) 8 j)"#))]
pub(crate) fn deserialize_10(bytes: &[u8]) -> Vec256 {
    #[inline(always)]
    #[hax_lib::ensures(|coefficients| fstar!(r#"
forall (i: nat {i < 256}).
      $coefficients i
    = ( if i % 16 >= 10 then 0
        else let j = (i / 16) * 10 + i % 16 in
             if i < 128 then $lower_coefficients0 j else $upper_coefficients0 (j - 32)))
"#))]
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

        let coefficients = mm256_si256_from_two_si128(lower_coefficients, upper_coefficients);

        let coefficients = mm256_mullo_epi16(
            coefficients,
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
        let coefficients = mm256_srli_epi16::<6>(coefficients);
        // Here I can prove this `and` is not useful
        let coefficients = mm256_and_si256(coefficients, mm256_set1_epi16((1 << 10) - 1));
        hax_lib::fstar!(
            r#"
assert_norm(BitVec.Utils.forall256 (fun i -> 
      $coefficients i
    = ( if i % 16 < 10
        then let j = (i / 16) * 10 + i % 16 in
             if i < 128 then $lower_coefficients0 j else $upper_coefficients0 (j - 32)
        else 0)))
"#
        );
        coefficients
    }

    let lower_coefficients = &bytes[0..16];
    let upper_coefficients = &bytes[4..20];
    deserialize_10_vec(
        mm_loadu_si128(lower_coefficients),
        mm_loadu_si128(upper_coefficients),
    )
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --z3rlimit 200")]
#[hax_lib::fstar::before(
    r#"
(* Axiom: `mm256_storeu_si256_i16 output vector` writes the 16 i16 lanes of
   `vector` (per `vec256_as_i16x16`) into a length-16 output slice, returning
   that slice.  The intrinsic's `val` in `Libcrux_intrinsics.Avx2_extract`
   currently only states length preservation; this strengthening makes the
   bit-level content of the result available for AVX2 ↔ portable bridging.
   Local axiom for `Vector.Avx2.Serialize`; mirror in any other consumer.
   No SMTPat — invoked manually in the body. *)
let mm256_storeu_si256_i16_post_axiom
      (output: t_Slice i16) (vector: Libcrux_intrinsics.Avx2_extract.t_Vec256)
    : Lemma
      (requires Core_models.Slice.impl__len #i16 output == mk_usize 16)
      (ensures
        (Libcrux_intrinsics.Avx2_extract.mm256_storeu_si256_i16 output vector
         <: Seq.seq i16)
        == (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 vector <: Seq.seq i16))
  = admit ()

(* Axiom: `mm256_loadu_si256_i16 input` from a length-16 i16 slice produces
   the Vec256 whose `vec256_as_i16x16` decomposition equals the input.
   No SMTPat — see comment above. *)
let mm256_loadu_si256_i16_post_axiom (input: t_Slice i16)
    : Lemma
      (requires Core_models.Slice.impl__len #i16 input == mk_usize 16)
      (ensures
        (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16
           (Libcrux_intrinsics.Avx2_extract.mm256_loadu_si256_i16 input)
         <: Seq.seq i16)
        == (input <: Seq.seq i16))
  = admit ()

(* Lane-bound bridge.  Same proof as `vector/avx2.rs`'s before-block helper;
   redefined locally because `Vector.Avx2.Serialize` is checked before
   `Vector.Avx2` and so cannot import it. *)
let lemma_vec256_lane_bounded_local
      (vec: Libcrux_intrinsics.Avx2_extract.t_Vec256) (n: nat{n > 0 /\ n <= 16}) (i: nat{i < 16})
    : Lemma
      (requires forall (b: nat{b < 16}). b >= n ==> vec (i * 16 + b) == 0)
      (ensures
        Rust_primitives.BitVectors.bounded
          (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 vec) i) n)
  = let arr = Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 vec in
    let lane = Seq.index arr i in
    let aux (b: usize{v b < 16}) : Lemma (v b > n ==> get_bit lane b == 0)
      = if v b > n then begin
          Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
            vec 16 (i * 16 + v b);
          Math.Lemmas.lemma_mod_plus (v b) i 16;
          Math.Lemmas.lemma_div_plus (v b) i 16
        end
        else ()
    in
    Classical.forall_intro aux;
    Rust_primitives.BitVectors.lemma_get_bit_bounded' lane n
"#
)]
#[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 11 || vector i = 0"#))]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 176}). bit_vec_of_int_t_array r 8 i == vector ((i/11) * 16 + i%11)"#))]
pub(crate) fn serialize_11(vector: Vec256) -> [u8; 22] {
    let mut array = [0i16; 16];
    mm256_storeu_si256_i16(&mut array, vector);
    hax_lib::fstar!(
        r#"
mm256_storeu_si256_i16_post_axiom
  (Rust_primitives.Hax.repeat (mk_i16 0) (mk_usize 16) <: t_Slice i16)
  ${vector};
introduce forall (j: nat). j < 16 ==>
    Rust_primitives.BitVectors.bounded (Seq.index array j) 11
with introduce j < 16 ==>
    Rust_primitives.BitVectors.bounded (Seq.index array j) 11
with _. lemma_vec256_lane_bounded_local ${vector} 11 j
"#
    );
    let input = PortableVector::from_i16_array(&array);
    let result = PortableVector::serialize_11(input);
    hax_lib::fstar!(
        r#"
introduce forall (i: nat{i < 176}).
    bit_vec_of_int_t_array result 8 i == ${vector} ((i / 11) * 16 + i % 11)
with begin
  Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
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
  $result i = (if i % 16 >= 11 then 0
               else let j = (i / 16) * 11 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 22)) 8 j)"#))]
pub(crate) fn deserialize_11(bytes: &[u8]) -> Vec256 {
    let output = PortableVector::deserialize_11(bytes);
    let array = PortableVector::to_i16_array(output);
    let result = mm256_loadu_si256_i16(&array);
    hax_lib::fstar!(
        r#"
mm256_loadu_si256_i16_post_axiom (array <: t_Slice i16);
introduce forall (i: nat{i < 256}).
    result i =
      (if i % 16 >= 11 then 0
       else let j = (i / 16) * 11 + i % 16 in
            bit_vec_of_int_t_array (${bytes} <: t_Array _ (sz 22)) 8 j)
with begin
  if i % 16 >= 11 then begin
    Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
      result 16 i;
    ()
  end else begin
    Libcrux_intrinsics.Avx2_extract.bit_vec_of_int_t_array_vec256_as_i16x16_lemma
      result 11 ((i / 16) * 11 + i % 16)
  end
end
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 12 || vector i = 0"#))]
#[hax_lib::ensures(|r| fstar!(r#"forall (i: nat{i < 192}). bit_vec_of_int_t_array r 8 i == vector ((i/12) * 16 + i%12)"#))]
pub(crate) fn serialize_12(vector: Vec256) -> [u8; 24] {
    #[inline(always)]
    #[hax_lib::fstar::options("--ext context_pruning --split_queries always")]
    #[hax_lib::requires(fstar!(r#"forall (i: nat{i < 256}). i % 16 < 12 || vector i = 0"#))]
    #[hax_lib::ensures(|(lower_8, upper_8)| fstar!(
        r#"
         forall (i: nat{i < 192}).
           vector ((i/12) * 16 + i%12) == (if i < 96 then $lower_8 i else $upper_8 (i - 96))
      )
    "#
    ))]
    fn serialize_12_vec(vector: Vec256) -> (Vec128, Vec128) {
        let adjacent_2_combined = mm256_concat_pairs_n(12, vector);
        let adjacent_4_combined =
            mm256_sllv_epi32(adjacent_2_combined, mm256_set_epi32(0, 8, 0, 8, 0, 8, 0, 8));
        let adjacent_4_combined = mm256_srli_epi64::<8>(adjacent_4_combined);

        let adjacent_8_combined = mm256_shuffle_epi8(
            adjacent_4_combined,
            mm256_set_epi8(
                -1, -1, -1, -1, 13, 12, 11, 10, 9, 8, 5, 4, 3, 2, 1, 0, -1, -1, -1, -1, 13, 12, 11,
                10, 9, 8, 5, 4, 3, 2, 1, 0,
            ),
        );

        let lower_8 = mm256_castsi256_si128(adjacent_8_combined);
        let upper_8 = mm256_extracti128_si256::<1>(adjacent_8_combined);
        hax_lib::fstar!(
            r#"
    introduce forall (i:nat{i < 96}). lower_8_ i = vector ((i / 12) * 16 + i % 12)
    with assert_norm (BitVec.Utils.forall_n 96 (fun i -> lower_8_ i = vector ((i / 12) * 16 + i % 12)));
    introduce forall (i:nat{i < 96}). upper_8_ i = vector (128 + (i / 12) * 16 + i % 12)
    with assert_norm (BitVec.Utils.forall_n 96 (fun i -> upper_8_ i = vector (128 + (i / 12) * 16 + i % 12)))
    "#
        );
        (lower_8, upper_8)
    }

    let mut serialized = [0u8; 32];
    let (lower_8, upper_8) = serialize_12_vec(vector);
    mm_storeu_bytes_si128(&mut serialized[0..16], lower_8);
    mm_storeu_bytes_si128(&mut serialized[12..28], upper_8);

    serialized[0..24].try_into().unwrap()
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Seq.length bytes == 24"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 256}).
  $result i = (if i % 16 >= 12 then 0
               else let j = (i / 16) * 12 + i % 16 in
                     bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 24)) 8 j)"#))]
pub(crate) fn deserialize_12(bytes: &[u8]) -> Vec256 {
    #[inline(always)]
    #[hax_lib::ensures(|coefficients| fstar!(r#"
forall (i: nat {i < 256}).
      $coefficients i
    = ( if i % 16 >= 12 then 0
        else let j = (i / 16) * 12 + i % 16 in
             if i < 128 then $lower_coefficients0 j else $upper_coefficients0 (j - 64)))
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

        let coefficients = mm256_si256_from_two_si128(lower_coefficients, upper_coefficients);

        let coefficients = mm256_mullo_epi16(
            coefficients,
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
        let coefficients = mm256_srli_epi16::<4>(coefficients);
        let coefficients = mm256_and_si256(coefficients, mm256_set1_epi16((1 << 12) - 1));
        hax_lib::fstar!(
            r#"
assert_norm(BitVec.Utils.forall256 (fun i -> 
      $coefficients i
    = ( if i % 16 < 12
        then let j = (i / 16) * 12 + i % 16 in
             if i < 128 then $lower_coefficients0 j else $upper_coefficients0 (j - 64)
        else 0)))
"#
        );
        coefficients
    }
    let lower_coefficients = mm_loadu_si128(&bytes[0..16]);
    let upper_coefficients = mm_loadu_si128(&bytes[8..24]);
    deserialize_12_vec(lower_coefficients, upper_coefficients)
}
