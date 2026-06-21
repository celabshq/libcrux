use super::arithmetic::*;
use super::vector_type::*;
use crate::vector::traits::FIELD_ELEMENTS_IN_VECTOR;
use crate::vector::FIELD_MODULUS;
use libcrux_secrets::*;

/// The `compress_*` functions implement the `Compress` function specified in the NIST FIPS
/// 203 standard (Page 18, Expression 4.5), which is defined as:
///
/// ```plaintext
/// Compress_d: ℤq -> ℤ_{2ᵈ}
/// Compress_d(x) = ⌈(2ᵈ/q)·x⌋
/// ```
///
/// Since `⌈x⌋ = ⌊x + 1/2⌋` we have:
///
/// ```plaintext
/// Compress_d(x) = ⌊(2ᵈ/q)·x + 1/2⌋
///               = ⌊(2^{d+1}·x + q) / 2q⌋
/// ```
///
/// For further information about the function implementations, consult the
/// `implementation_notes.pdf` document in this directory.
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
#[hax_lib::fstar::before(r#"[@ "opaque_to_smt"]"#)]
#[cfg_attr(hax, hax_lib::requires(fe < (FIELD_MODULUS as u16)))]
// Post: threshold form AND hacspec integer form.  The hacspec form
// is `v result == ((v fe * 2 * 2^1 + 3329) / (2 * 3329)) % 2^1`
// simplified: `((v fe * 4 + 3329) / 6658) % 2`.  Both forms are
// equivalent on [0, 3329); the bridge lemma uses the hacspec form.
//
// Hacspec-form post discharged via 3-case integer-division enumeration
// on `fe ∈ [0, 3329)` proven inline at the function tail.
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"((833 <= v fe && v fe <= 2496) ==> v result == 1) /\
						    (~(833 <=  v fe && v fe <= 2496) ==> v result == 0) /\
						    (v result == ((v fe * 4 + 3329) / 6658) % 2)"#)))]
pub(crate) fn compress_message_coefficient(fe: U16) -> U8 {
    // The approach used here is inspired by:
    // https://github.com/cloudflare/circl/blob/main/pke/kyber/internal/common/poly.go#L150

    // If 833 <= fe <= 2496,
    // then -832 <= shifted <= 831
    let shifted: I16 = 1664.classify() - (fe.as_i16());
    hax_lib::fstar!(r#"assert (v $shifted == 1664 - v $fe)"#);

    // If shifted < 0, then
    // (shifted >> 15) ^ shifted = flip_bits(shifted) = -shifted - 1, and so
    // if -832 <= shifted < 0 then 0 < shifted_positive <= 831
    //
    // If shifted >= 0 then
    // (shifted >> 15) ^ shifted = shifted, and so
    // if 0 <= shifted <= 831 then 0 <= shifted_positive <= 831
    let mask = shifted >> 15;

    hax_lib::fstar!(
        "assert (v $mask = v $shifted / pow2 15);
        assert (if v $shifted < 0 then $mask = ones else $mask = zero)"
    );

    let shifted_to_positive: I16 = mask ^ shifted;

    hax_lib::fstar!(
        "logxor_lemma $shifted $mask;
        assert (v $shifted < 0 ==> v $shifted_to_positive = v (lognot $shifted));
        neg_equiv_lemma $shifted;
        assert (v (lognot $shifted) = -(v $shifted) -1);
        assert (v $shifted >= 0 ==> v $shifted_to_positive = v ($mask `logxor` $shifted));
        assert (v $shifted >= 0 ==> $mask = zero);
        assert (v $shifted >= 0 ==> $mask ^. $shifted = $shifted);
        assert (v $shifted >= 0 ==> v $shifted_to_positive = v $shifted);
        assert ($shifted_to_positive >=. mk_i16 0)"
    );

    let shifted_positive_in_range: I16 = shifted_to_positive - 832;

    hax_lib::fstar!(
        "assert (1664 - v $fe >= 0 ==> v $shifted_positive_in_range == 832 - v $fe);
        assert (1664 - v $fe < 0 ==> v $shifted_positive_in_range == -2497 + v $fe)"
    );

    // If x <= 831, then x - 832 <= -1, and so x - 832 < 0, which means
    // the most significant bit of shifted_positive_in_range will be 1.
    let r0: I16 = shifted_positive_in_range >> 15;
    let r1: I16 = r0 & 1i16;
    let res = r1.as_u8();

    hax_lib::fstar!(
        r#"assert (v $r0 = v $shifted_positive_in_range / pow2 15);
        assert (if v $shifted_positive_in_range < 0 then $r0 = ones else $r0 = zero);
        logand_lemma (mk_i16 1) $r0;
        assert (if v $shifted_positive_in_range < 0 then $r1 = mk_i16 1 else $r1 = mk_i16 0);
        assert ((v $fe >= 833 && v $fe <= 2496) ==> $r1 = mk_i16 1);
        assert (v $fe < 833 ==> $r1 = mk_i16 0);
        assert (v $fe > 2496 ==> $r1 = mk_i16 0);
        assert (v $res = v $r1)"#
    );

    // Hacspec-form post `v res == ((v fe * 4 + 3329) / 6658) % 2` —
    // discharged by 3-case integer-division enumeration on
    // `fe ∈ [0, 3329)`:
    //   fe ∈ [0, 832]:    4*fe+3329 ∈ [3329, 6657],   quot 0, mod 2 = 0
    //   fe ∈ [833, 2496]: 4*fe+3329 ∈ [6661, 13313],  quot 1, mod 2 = 1
    //   fe ∈ [2497, 3328]: 4*fe+3329 ∈ [13317, 16641], quot 2, mod 2 = 0
    hax_lib::fstar!(
        r#"assert (v $fe < 833 ==>
            (v $fe * 4 + 3329) >= 3329 /\ (v $fe * 4 + 3329) < 6658);
        assert (v $fe < 833 ==>
            (v $fe * 4 + 3329) / 6658 == 0);
        assert (v $fe < 833 ==> ((v $fe * 4 + 3329) / 6658) % 2 == 0);
        assert ((v $fe >= 833 && v $fe <= 2496) ==>
            (v $fe * 4 + 3329) >= 6658 /\ (v $fe * 4 + 3329) < 13316);
        assert ((v $fe >= 833 && v $fe <= 2496) ==>
            (v $fe * 4 + 3329) / 6658 == 1);
        assert ((v $fe >= 833 && v $fe <= 2496) ==>
            ((v $fe * 4 + 3329) / 6658) % 2 == 1);
        assert (v $fe > 2496 ==>
            (v $fe * 4 + 3329) >= 13316 /\ (v $fe * 4 + 3329) < 19974);
        assert (v $fe > 2496 ==>
            (v $fe * 4 + 3329) / 6658 == 2);
        assert (v $fe > 2496 ==> ((v $fe * 4 + 3329) / 6658) % 2 == 0)"#
    );
    res
}

#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
#[hax_lib::fstar::before(r#"[@ "opaque_to_smt"]"#)]
#[cfg_attr(hax,
    hax_lib::requires(
        (coefficient_bits == 4 ||
         coefficient_bits == 5 ||
         coefficient_bits == 10 ||
         coefficient_bits == 11) &&
         fe < (FIELD_MODULUS as u16)))]
// Post: bounds AND hacspec integer form.  The hacspec form is
// `v result == ((v fe * 2 * pow2 D + 3329) / 6658) % pow2 D` where
// D = v coefficient_bits.  This is the exact NIST FIPS 203
// Compress_d formula; the impl uses a Barrett-style approximation
// (magic constant 10_321_340 >> 35 ≈ 1/3329).  The Barrett spine equals
// the exact rounded division via the shared scalar identity
// `Hacspec_ml_kem.Commute.Chunk.lemma_compress_d_barrett_eq`.
#[cfg_attr(hax,
     hax_lib::ensures(
     |result| fstar!(r#"v result >= 0 /\ v result < pow2 (v $coefficient_bits) /\
                        v result == ((v $fe * 2 * pow2 (v $coefficient_bits) + 3329) / 6658)
                                    % pow2 (v $coefficient_bits)"#)))]
pub(crate) fn compress_ciphertext_coefficient(coefficient_bits: u8, fe: U16) -> FieldElement {
    // hax_debug_assert!(
    //     coefficient_bits == 4
    //         || coefficient_bits == 5
    //         || coefficient_bits == 10
    //         || coefficient_bits == 11
    // );
    // hax_debug_assert!(fe <= (FIELD_MODULUS as u16));

    hax_lib::fstar!(
        r#"assert_norm (pow2 4 == 16);
        assert_norm (pow2 5 == 32);
        assert_norm (pow2 10 == 1024);
        assert_norm (pow2 11 == 2048);
        assert_norm (pow2 35 == 34359738368)"#
    );
    hax_lib::fstar!(
        r#"assert (pow2 (v $coefficient_bits) <= 2048 /\ pow2 (v $coefficient_bits) >= 1)"#
    );

    // This has to be constant time due to:
    // https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/ldX0ThYJuBo/m/ovODsdY7AwAJ
    let mut compressed = (fe.as_u64()) << coefficient_bits;
    hax_lib::fstar!(
        r#"assert (v (Libcrux_secrets.Int.f_as_u64 #u16 #FStar.Tactics.Typeclasses.solve $fe <: u64)
            == v $fe);
        FStar.Math.Lemmas.lemma_mult_le_right (pow2 (v $coefficient_bits)) (v $fe) 3328;
        assert (v $fe * pow2 (v $coefficient_bits) <= 3328 * 2048);
        assert (v $compressed == (v $fe * pow2 (v $coefficient_bits)) % pow2 64);
        FStar.Math.Lemmas.small_mod (v $fe * pow2 (v $coefficient_bits)) (pow2 64);
        assert (v $compressed == v $fe * pow2 (v $coefficient_bits))"#
    );
    compressed += 1664 as u64;
    hax_lib::fstar!(r#"assert (v $compressed == v $fe * pow2 (v $coefficient_bits) + 1664)"#);

    compressed *= 10_321_340;
    hax_lib::fstar!(
        r#"assert (v $compressed == (v $fe * pow2 (v $coefficient_bits) + 1664) * 10321340)"#
    );
    compressed >>= 35;
    hax_lib::fstar!(
        r#"assert (v $compressed ==
            ((v $fe * pow2 (v $coefficient_bits) + 1664) * 10321340) / pow2 35);
        FStar.Math.Lemmas.lemma_div_le ((v $fe * pow2 (v $coefficient_bits) + 1664) * 10321340)
          ((3328 * 2048 + 1664) * 10321340) (pow2 35);
        assert (v $compressed < pow2 32);
        assert (v (Libcrux_secrets.Int.f_as_u32 #u64 #FStar.Tactics.Typeclasses.solve $compressed
                <: u32) == v $compressed)"#
    );

    let result = get_n_least_significant_bits(coefficient_bits, compressed.as_u32()).as_i16();
    hax_lib::fstar!(
        r#"FStar.Math.Lemmas.lemma_mod_lt
            (((v $fe * pow2 (v $coefficient_bits) + 1664) * 10321340) / pow2 35)
            (pow2 (v $coefficient_bits));
        assert (v $result == (((v $fe * pow2 (v $coefficient_bits) + 1664) * 10321340) / pow2 35)
                             % pow2 (v $coefficient_bits));
        Hacspec_ml_kem.Commute.Chunk.lemma_compress_d_barrett_eq (v $fe) (v $coefficient_bits);
        assert (v $result == ((v $fe * 2 * pow2 (v $coefficient_bits) + 3329) / 6658)
                             % pow2 (v $coefficient_bits))"#
    );
    result
}

#[inline(always)]
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        r#"
let compress_message_coefficient_range_helper (fe: u16) : Lemma
  (requires fe <. (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS) <: u16))
  (ensures v (cast (compress_message_coefficient fe) <: i16) >= 0 /\
    v (cast (compress_message_coefficient fe) <: i16) < 2) =
  assert (v (cast (compress_message_coefficient fe) <: i16) >= 0 /\
    v (cast (compress_message_coefficient fe) <: i16) < 2)

#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
(* Per-lane compression fact, proven in clean context so the loop body
   stays light: the i16 lane written by compress_1 at index i equals
   f_as_i16 #u8 (compress_message_coefficient (f_as_u16 #i16 fe)), whose
   value is the hacspec compress_d _ 1 integer form on v fe. *)
let compress_1_lane_eq (fe: i16) : Lemma
  (requires v fe >= 0 /\ v fe < 3329)
  (ensures
    (let r:i16 =
       Libcrux_secrets.Int.f_as_i16 #u8 #FStar.Tactics.Typeclasses.solve
         (compress_message_coefficient (Libcrux_secrets.Int.f_as_u16 #i16
                 #FStar.Tactics.Typeclasses.solve fe
               <: u16)
           <: u8)
     in
     v r >= 0 /\ v r < 2 /\ v r == ((v fe * 4 + 3329) / 6658) % 2)) =
  let feu:u16 = Libcrux_secrets.Int.f_as_u16 #i16 #FStar.Tactics.Typeclasses.solve fe in
  assert (v feu == v fe);
  assert (feu <. (cast (Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS) <: u16));
  let c:u8 = compress_message_coefficient feu in
  assert (v c == ((v feu * 4 + 3329) / 6658) % 2);
  compress_message_coefficient_range_helper feu;
  assert (v c >= 0 /\ v c < 2);
  assert (v (Libcrux_secrets.Int.f_as_i16 #u8 #FStar.Tactics.Typeclasses.solve c <: i16) == v c)
#pop-options
"#
    )
)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 0 --z3rlimit 200")]
#[hax_lib::requires(fstar!(r#"forall (i:nat). i < 16 ==> v (Seq.index ${a}.f_elements i) >= 0 /\
    v (Seq.index ${a}.f_elements i) < 3329"#))]
// Hacspec-form post: `v result[i] == ((v a[i] * 4 + 3329) / 6658) % 2`
// (hacspec `compress_d _ 1` in integer form).  The per-lane value is
// discharged by the clean-context helper `compress_1_lane_eq`; the
// loop carries a snapshot `a_orig` so the value is stated on the
// original input lane.
#[hax_lib::ensures(|result| fstar!(r#"forall (i:nat). i < 16 ==>
    (let res_i = v (${result}.f_elements.[ sz i ] <: i16) in
     let a_i   = v (Seq.index ${a}.f_elements i) in
     res_i >= 0 /\ res_i < 2 /\
     res_i == ((a_i * 4 + 3329) / 6658) % 2)"#))]
pub(crate) fn compress_1(mut a: PortableVector) -> PortableVector {
    hax_lib::fstar!(
        "assert (forall (i:nat). i < 16 ==> (cast (${a}.f_elements.[ sz i ]) <: u16) <.
        (cast ($FIELD_MODULUS) <: u16))"
    );

    #[cfg(hax)]
    let a_orig = a;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"(v $i < 16 ==> (forall (j:nat). (j >= v $i /\ j < 16) ==>
            Seq.index ${a}.f_elements j == Seq.index ${a_orig}.f_elements j /\
            v (cast (${a}.f_elements.[ sz j ]) <: u16) < v (cast ($FIELD_MODULUS) <: u16))) /\
            (forall (j:nat). j < v $i ==> v (${a}.f_elements.[ sz j ] <: i16) >= 0 /\
                v (${a}.f_elements.[ sz j ] <: i16) < 2 /\
                v (${a}.f_elements.[ sz j ] <: i16) ==
                ((v (Seq.index ${a_orig}.f_elements j) * 4 + 3329) / 6658) % 2)"#
            )
        });

        hax_lib::fstar!(
            r#"assert (Seq.index ${a}.f_elements (v $i) == Seq.index ${a_orig}.f_elements (v $i));
            assert (v (${a}.f_elements.[ $i ] <: i16) >= 0 /\ v (${a}.f_elements.[ $i ] <: i16) < 3329);
            compress_1_lane_eq (${a}.f_elements.[ $i ] <: i16);
            compress_message_coefficient_range_helper
              (Libcrux_secrets.Int.f_as_u16 #i16 #FStar.Tactics.Typeclasses.solve
                 (${a}.f_elements.[ $i ] <: i16) <: u16)"#
        );

        a.elements[i] = compress_message_coefficient(a.elements[i].as_u16()).as_i16();

        hax_lib::fstar!(
            r#"assert (v (${a}.f_elements.[ $i ] <: i16) >= 0 /\
            v (${a}.f_elements.[ $i ] <: i16) < 2);
            assert (v (${a}.f_elements.[ $i ] <: i16) ==
                ((v (Seq.index ${a_orig}.f_elements (v $i)) * 4 + 3329) / 6658) % 2)"#
        );
        // Frame: untouched elements still equal the original and satisfy the bound
        hax_lib::fstar!(
            r#"assert (forall (j: nat). (j > v $i /\ j < 16) ==>
                Seq.index ${a}.f_elements j == Seq.index ${a_orig}.f_elements j /\
                v (cast (${a}.f_elements.[ sz j ]) <: u16) < v (cast ($FIELD_MODULUS) <: u16))"#
        );
    }

    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 16 ==> v (${a}.f_elements.[ sz i ] <: i16) >= 0 /\
        v (${a}.f_elements.[ sz i ] <: i16) < 2)"#
    );

    a
}

#[inline(always)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 0 --z3rlimit 500")]
#[hax_lib::requires(fstar!(r#"(v $COEFFICIENT_BITS == 4 \/
        v $COEFFICIENT_BITS == 5 \/
        v $COEFFICIENT_BITS == 10 \/
        v $COEFFICIENT_BITS == 11) /\
    (forall (i:nat). i < 16 ==>
        (v (Seq.index ${a}.f_elements i) >= 0 /\
         v (Seq.index ${a}.f_elements i) < 3329))"#))]
// Hacspec-form post: `v result[i] == ((v a[i] * 2 * 2^D + 3329) / 6658) % 2^D`
// (hacspec `compress_d _ D` in integer form).  Discharged by composing
// the now-functional leaf `compress_ciphertext_coefficient` post (which
// equals the Barrett spine via `lemma_compress_d_barrett_eq`) through a
// snapshot-carrying loop invariant.
#[hax_lib::ensures(|result| fstar!(r#"forall (i:nat). i < 16 ==>
    (let res_i = v (${result}.f_elements.[ sz i ] <: i16) in
     let a_i   = v (Seq.index ${a}.f_elements i) in
     res_i >= 0 /\ res_i < pow2 (v $COEFFICIENT_BITS) /\
     res_i == ((a_i * 2 * pow2 (v $COEFFICIENT_BITS) + 3329) / 6658)
              % pow2 (v $COEFFICIENT_BITS))"#))]
pub(crate) fn compress<const COEFFICIENT_BITS: i32>(mut a: PortableVector) -> PortableVector {
    hax_lib::fstar!(
        "assert (v (cast ($COEFFICIENT_BITS) <: u8) == v $COEFFICIENT_BITS);
        assert (v (cast ($COEFFICIENT_BITS) <: u32) == v $COEFFICIENT_BITS);
        assert (v (cast ($FIELD_MODULUS) <: u16) == 3329)"
    );
    hax_lib::fstar!(
        "assert (forall (i:nat). i < 16 ==>
            (cast (${a}.f_elements.[ sz i ]) <: u16) <.
            (cast ($FIELD_MODULUS) <: u16))"
    );

    #[cfg(hax)]
    let a_orig = a;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"((forall (j:nat). (j >= v $i /\ j < 16) ==>
                      Seq.index ${a}.f_elements j == Seq.index ${a_orig}.f_elements j /\
                      v (cast (${a}.f_elements.[ sz j ]) <: u16) < v (cast ($FIELD_MODULUS) <: u16))) /\
                    (forall (j:nat). j < v $i ==> v (${a}.f_elements.[ sz j ] <: i16) >= 0 /\
                     v (${a}.f_elements.[ sz j ] <: i16) < pow2 (v $COEFFICIENT_BITS) /\
                     v (${a}.f_elements.[ sz j ] <: i16) ==
                     ((v (Seq.index ${a_orig}.f_elements j) * 2 * pow2 (v $COEFFICIENT_BITS) + 3329)
                       / 6658) % pow2 (v $COEFFICIENT_BITS))"#
            )
        });

        hax_lib::fstar!(
            r#"assert (Seq.index ${a}.f_elements (v $i) == Seq.index ${a_orig}.f_elements (v $i));
            assert (v (cast (${a}.f_elements.[ $i ]) <: u16) == v (Seq.index ${a_orig}.f_elements (v $i)))"#
        );

        a.elements[i] =
            compress_ciphertext_coefficient(COEFFICIENT_BITS as u8, a.elements[i].as_u16())
                .as_i16();

        hax_lib::fstar!(
            r#"assert (v (${a}.f_elements.[ $i ] <: i16) >= 0 /\
            v (${a}.f_elements.[ $i ] <: i16) < pow2 (v $COEFFICIENT_BITS));
            assert (v (${a}.f_elements.[ $i ] <: i16) ==
                ((v (Seq.index ${a_orig}.f_elements (v $i)) * 2 * pow2 (v $COEFFICIENT_BITS) + 3329)
                  / 6658) % pow2 (v $COEFFICIENT_BITS))"#
        );
        // Frame: untouched elements still equal the original and satisfy the bound
        hax_lib::fstar!(
            r#"assert (forall (j: nat). (j > v $i /\ j < 16) ==>
                Seq.index ${a}.f_elements j == Seq.index ${a_orig}.f_elements j /\
                v (cast (${a}.f_elements.[ sz j ]) <: u16) <
                v (cast ($FIELD_MODULUS) <: u16))"#
        );
    }
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 16 ==>
                    (v (${a}.f_elements.[ sz i ] <: i16) >= 0 /\
                     v (${a}.f_elements.[ sz i ] <: i16) < pow2 (v $COEFFICIENT_BITS)))"#
    );

    a
}

#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"forall i. let x = Seq.index ${a}.f_elements i in
                                        (x == mk_i16 0 \/ x == mk_i16 1)"#))]
// Hacspec-form post: per-lane integer equation matching
// `Hacspec_ml_kem.Compress.decompress_d fe 1` = `(2*fe*3329 + 2) / 4`.
// On input ∈ {0, 1}: decompress_d 0 1 = 0, decompress_d 1 1 = 1665.
#[hax_lib::ensures(|result| fstar!(r#"forall (i:nat). i < 16 ==>
        (let res_i = v (Seq.index ${result}.f_elements i) in
         let a_i   = v (Seq.index ${a}.f_elements i) in
         (res_i == 0 \/ res_i == 1665) /\
         res_i == (2 * a_i * 3329 + 2) / 4)"#))]
#[inline(always)]
pub(crate) fn decompress_1(a: PortableVector) -> PortableVector {
    let z = zero();

    hax_lib::fstar!("assert(forall i. Seq.index ${z}.f_elements i == mk_i16 0)");
    hax_lib::fstar!(
        r#"assert(forall i. let x = Seq.index ${a}.f_elements i in
                                      ((0 - v x) == 0 \/ (0 - v x) == -1))"#
    );
    hax_lib::fstar!(
        r#"assert(forall i. i < 16 ==>
                                      Spec.Utils.is_intb (pow2 15 - 1)
                                        (0 - v (Seq.index ${a}.f_elements i)))"#
    );

    let s = sub(z, &a);

    // Per-lane pairing: a_i == 0 ⇒ s_i == 0; a_i == 1 ⇒ s_i == -1.
    hax_lib::fstar!(
        r#"assert(forall i. Seq.index ${s}.f_elements i == mk_i16 0 \/
                                      Seq.index ${s}.f_elements i == mk_i16 (-1));
        assert (forall (i:nat). i < 16 ==>
             (let a_i = v (Seq.index ${a}.f_elements i) in
              let s_i = v (Seq.index ${s}.f_elements i) in
              (a_i == 0 ==> s_i == 0) /\
              (a_i == 1 ==> s_i == -1)))"#
    );

    let res = bitwise_and_with_constant(s, 1665);

    // Per-lane pairing: s_i == 0 ⇒ res_i == 0; s_i == -1 ⇒ res_i == 1665.
    // bitwise_and_with_constant's spec gives res_i == s_i & 1665.
    hax_lib::fstar!(
        r#"assert(forall i. Seq.index ${res}.f_elements i == mk_i16 0 \/
                                      Seq.index ${res}.f_elements i == mk_i16 1665);
        assert (forall (i:nat). i < 16 ==>
             (let a_i = v (Seq.index ${a}.f_elements i) in
              let res_i = v (Seq.index ${res}.f_elements i) in
              (a_i == 0 ==> res_i == 0) /\
              (a_i == 1 ==> res_i == 1665)))"#
    );

    // Hacspec form: res_i == (2*a_i*3329 + 2) / 4 for a_i ∈ {0, 1}.
    //   a_i = 0: (0 + 2)/4 = 0  (res = 0)
    //   a_i = 1: (6658 + 2)/4 = 6660/4 = 1665  (res = 1665)
    hax_lib::fstar!(
        r#"assert (forall (i:nat). i < 16 ==>
             (let a_i = v (Seq.index ${a}.f_elements i) in
              (a_i == 0 ==> (2 * a_i * 3329 + 2) / 4 == 0) /\
              (a_i == 1 ==> (2 * a_i * 3329 + 2) / 4 == 1665)));
        assert (forall (i:nat). i < 16 ==>
             (let a_i = v (Seq.index ${a}.f_elements i) in
              let res_i = v (Seq.index ${res}.f_elements i) in
              (res_i == 0 \/ res_i == 1665) /\
              res_i == (2 * a_i * 3329 + 2) / 4))"#
    );

    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --ext context_pruning")]
#[hax_lib::requires(fstar!(r#"(v $COEFFICIENT_BITS == 4 \/
        v $COEFFICIENT_BITS == 5 \/
        v $COEFFICIENT_BITS == 10 \/
        v $COEFFICIENT_BITS == 11) /\
    (forall (i:nat). i < 16 ==> v (Seq.index ${a}.f_elements i) >= 0 /\
        v (Seq.index ${a}.f_elements i) < pow2 (v $COEFFICIENT_BITS))"#))]
// Hacspec-form post: `v result[i] == (2*v a[i]*3329 + pow2 D) / pow2 (D+1)`,
// which is exactly `Hacspec_ml_kem.Compress.decompress_d fe D` when fe
// has val = v a[i] (fe in [0, 2^D) guarantees val < 2^D < 3329 so the
// u16 mod-3329 normalisation is a no-op).  The exact value is plain
// integer division (no Barrett); carried through a snapshot loop invariant.
#[hax_lib::ensures(|result| fstar!(r#"forall (i:nat). i < 16 ==>
        (let res_i = v (Seq.index ${result}.f_elements i) in
         let a_i   = v (Seq.index ${a}.f_elements i) in
         res_i >= 0 /\ res_i < v $FIELD_MODULUS /\
         res_i == (2 * a_i * 3329 + pow2 (v $COEFFICIENT_BITS))
                  / pow2 (v $COEFFICIENT_BITS + 1))"#))]
pub(crate) fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(
    mut a: PortableVector,
) -> PortableVector {
    hax_lib::fstar!(
        "assert_norm (pow2 1 == 2);
        assert_norm (pow2 4 == 16);
        assert_norm (pow2 5 == 32);
        assert_norm (pow2 10 == 1024);
        assert_norm (pow2 11 == 2048)"
    );

    #[cfg(hax)]
    let a_orig = a;

    for i in 0..FIELD_ELEMENTS_IN_VECTOR {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"(v $i < 16 ==> (forall (j:nat). (j >= v $i /\ j < 16) ==>
            Seq.index ${a}.f_elements j == Seq.index ${a_orig}.f_elements j /\
            v (Seq.index ${a}.f_elements j) >= 0 /\  v (Seq.index ${a}.f_elements j) < pow2 (v $COEFFICIENT_BITS))) /\
            (forall (j:nat). j < v $i ==>
                (v (Seq.index ${a}.f_elements j) >= 0 /\ v (Seq.index ${a}.f_elements j) < v $FIELD_MODULUS /\
                 v (Seq.index ${a}.f_elements j) ==
                 (2 * v (Seq.index ${a_orig}.f_elements j) * 3329 + pow2 (v $COEFFICIENT_BITS))
                 / pow2 (v $COEFFICIENT_BITS + 1)))"#
            )
        });
        hax_lib::fstar!(
            "assert (Seq.index ${a}.f_elements (v $i) == Seq.index ${a_orig}.f_elements (v $i));
          assert (v (${a}.f_elements.[ $i ] <: i16) < pow2 11);
          assert (v (${a}.f_elements.[ $i ] <: i16) ==
            v (cast (${a}.f_elements.[ $i ] <: i16) <: i32));
          assert (v ($FIELD_MODULUS <: i16) ==
            v (cast ($FIELD_MODULUS <: i16) <: i32));
          assert (v ((cast (${a}.f_elements.[ $i ] <: i16) <: i32) *!
            (cast ($FIELD_MODULUS <: i16) <: i32)) ==
              v (cast (${a}.f_elements.[ $i ] <: i16) <: i32) *
                v (cast ($FIELD_MODULUS <: i16) <: i32))"
        );

        let mut decompressed = a.elements[i].as_i32() * FIELD_MODULUS.classify().as_i32();

        hax_lib::fstar!(
            "assert (v ($decompressed <<! mk_i32 1) == v $decompressed * 2);
          assert (v (mk_i32 1 <<! $COEFFICIENT_BITS) == pow2 (v $COEFFICIENT_BITS));
          assert (v (($decompressed <<! mk_i32 1) +! (mk_i32 1 <<! $COEFFICIENT_BITS)) ==
            v ($decompressed <<! mk_i32 1) + v (mk_i32 1 <<! $COEFFICIENT_BITS))"
        );

        decompressed = (decompressed << 1) + (1i32 << COEFFICIENT_BITS);

        hax_lib::fstar!(
            "assert (v ($COEFFICIENT_BITS +! mk_i32 1) == v $COEFFICIENT_BITS + 1);
          assert (v ($decompressed >>! ($COEFFICIENT_BITS +! mk_i32 1 <: i32)) ==
            v $decompressed / pow2 (v $COEFFICIENT_BITS + 1))"
        );

        decompressed = decompressed >> (COEFFICIENT_BITS + 1);

        hax_lib::fstar!(
            "assert (v $decompressed < v $FIELD_MODULUS);
          assert (v (cast $decompressed <: i16) < v $FIELD_MODULUS);
          assert (v (cast $decompressed <: i16) ==
            (2 * v (Seq.index ${a_orig}.f_elements (v $i)) * 3329 + pow2 (v $COEFFICIENT_BITS))
            / pow2 (v $COEFFICIENT_BITS + 1))"
        );

        a.elements[i] = decompressed.as_i16();

        // Frame: untouched elements still equal the original input
        hax_lib::fstar!(
            r#"assert (forall (j: nat). (j > v $i /\ j < 16) ==>
                Seq.index ${a}.f_elements j == Seq.index ${a_orig}.f_elements j)"#
        );
    }

    a
}
