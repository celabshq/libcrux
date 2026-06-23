module Hacspec_ml_kem.Serialize
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let v_MAX_BYTES: usize = mk_usize 16384

#push-options "--z3rlimit 150"

/// Converts a set of bytes in `bytes` into a set of bits.
/// This function implements <strong>Algorithm 3</strong> of the NIST FIPS 203
/// standard, which is reproduced below:
/// ```plaintext
/// Input: byte array B ∈ 𝔹ˡ.
/// Output: bit array b ∈ {0,1}⁸ˡ.
/// for (i ← 0; i < l; i++)
///     for(j ← 0; j < 8; j++)
///         b[8i + j] ← B[i] mod 2
///         B[i] ← ⌊B[i]/2⌋
///     end for
/// end for
/// return b
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let bytes_to_bits (v_N v_N8: usize) (bytes: t_Array u8 v_N)
    : Prims.Pure (t_Array bool v_N8)
      (requires v_N <. mk_usize 16384 && v_N8 =. (v_N *! mk_usize 8 <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  Hacspec_ml_kem.Parameters.createi #bool
    v_N8
    #(usize -> bool)
    (fun i ->
        let i:usize = i in
        (((bytes.[ i /! mk_usize 8 <: usize ] <: u8) >>! (i %! mk_usize 8 <: usize) <: u8) &.
          mk_u8 1
          <:
          u8) =.
        mk_u8 1
        <:
        bool)

#pop-options

#push-options "--z3rlimit 150"

/// Converts a bit string `bits` into an array of bytes. This function asserts
/// that `bits.len()` is a multiple of 8.
/// This function implements <strong>Algorithm 2</strong> of the NIST FIPS 203
/// standard, which is reproduced below:
/// ```plaintext
/// Input: bit array b ∈ {0,1}⁸ˡ.
/// Output: byte array B ∈ 𝔹ˡ.
/// B ← (0,...,0)
/// for (i ← 0; i < 8l; i++)
///     B[⌊i/8⌋] ← B[⌊i/8⌋] + b[i]·2^{i} mod 8
/// end for
/// return B
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let bits_to_bytes (v_N v_N8: usize) (bv: t_Array bool v_N8)
    : Prims.Pure (t_Array u8 v_N)
      (requires v_N <. mk_usize 16384 && v_N8 =. (v_N *! mk_usize 8 <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  Hacspec_ml_kem.Parameters.createi #u8
    v_N
    #(usize -> u8)
    (fun i ->
        let i:usize = i in
        (((((((cast (bv.[ mk_usize 8 *! i <: usize ] <: bool) <: u8) |.
                    ((cast (bv.[ (mk_usize 8 *! i <: usize) +! mk_usize 1 <: usize ] <: bool) <: u8) <<!
                      mk_i32 1
                      <:
                      u8)
                    <:
                    u8) |.
                  ((cast (bv.[ (mk_usize 8 *! i <: usize) +! mk_usize 2 <: usize ] <: bool) <: u8) <<!
                    mk_i32 2
                    <:
                    u8)
                  <:
                  u8) |.
                ((cast (bv.[ (mk_usize 8 *! i <: usize) +! mk_usize 3 <: usize ] <: bool) <: u8) <<!
                  mk_i32 3
                  <:
                  u8)
                <:
                u8) |.
              ((cast (bv.[ (mk_usize 8 *! i <: usize) +! mk_usize 4 <: usize ] <: bool) <: u8) <<!
                mk_i32 4
                <:
                u8)
              <:
              u8) |.
            ((cast (bv.[ (mk_usize 8 *! i <: usize) +! mk_usize 5 <: usize ] <: bool) <: u8) <<!
              mk_i32 5
              <:
              u8)
            <:
            u8) |.
          ((cast (bv.[ (mk_usize 8 *! i <: usize) +! mk_usize 6 <: usize ] <: bool) <: u8) <<!
            mk_i32 6
            <:
            u8)
          <:
          u8) |.
        ((cast (bv.[ (mk_usize 8 *! i <: usize) +! mk_usize 7 <: usize ] <: bool) <: u8) <<!
          mk_i32 7
          <:
          u8)
        <:
        u8)

#pop-options

#push-options "--z3rlimit 150"

/// Convert the associated ring element into a vector of
/// `COEFFICIENTS_IN_RING_ELEMENT * bits_per_coefficient`
/// bits, and output this vector as a byte array such that the first 8 bits of
/// the vector represent the first byte of the output, the next 8 bits the
/// next byte of the output, and so on ...
/// N.B.: `byte_encode` is only the inverse of `byte_decode` when:
/// - each ring coefficient can fit into `bits_per_coefficient` (otherwise
///   lossy compression takes place)
/// - `bits_per_coefficient < BITS_PER_COEFFICIENT`, since
///   otherwise when `byte_decode` operates on 12 bits at a time,
///   it is not injective: the values 3329 + 1 and 1 for example both fit into
///   12 bits and map to the same `KyberFieldElement`
/// Otherwise `byte_decode` is not injective and therefore has no left inverse.
/// N.B.: This function asserts that `bits_per_coefficient <= 12`
/// This function implements <strong>Algorithm 4</strong> of the NIST FIPS 203 standard, which is
/// reproduced below:
/// ```plaintext
/// Input: integer array F ∈ ℤₘ²⁵⁶, where m = 2ᵈ if d < 12 and m = q if d = 12.
/// Output: byte array B ∈ 𝔹^{32d}.
/// for(i ← 0; i < 256; i++)
///     a ← F[i]
///     for(j ← 0; j < d; j++)
///         b[i·d + j] ← a mod 2
///         a ← (a − b[i·d + j])/2
///     end for
/// B ← BitsToBytes(b)
/// return B
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let bitvector_from_bounded_ints (v_N v_Nd: usize) (input: t_Array u16 v_N) (d: usize)
    : Prims.Pure (t_Array bool v_Nd)
      (requires
        v_N <. mk_usize 16384 && d <=. Hacspec_ml_kem.Parameters.v_BITS_PER_COEFFICIENT &&
        v_Nd =. (v_N *! d <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  Hacspec_ml_kem.Parameters.createi #bool
    v_Nd
    #(usize -> bool)
    (fun i ->
        let i:usize = i in
        (((input.[ i /! d <: usize ] <: u16) >>! (i %! d <: usize) <: u16) &. mk_u16 1 <: u16) =.
        mk_u16 1
        <:
        bool)

#pop-options

#push-options "--z3rlimit 150"

let byte_encode
      (v_D32 v_D256: usize)
      (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (d: usize)
    : Prims.Pure (t_Array u8 v_D32)
      (requires
        d <=. Hacspec_ml_kem.Parameters.v_BITS_PER_COEFFICIENT &&
        v_D32 =. (mk_usize 32 *! d <: usize) &&
        v_D256 =. (mk_usize 256 *! d <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let (p_raw: t_Array u16 (mk_usize 256)):t_Array u16 (mk_usize 256) =
    Hacspec_ml_kem.Parameters.createi #u16
      (mk_usize 256)
      #(usize -> u16)
      (fun i ->
          let i:usize = i in
          (p.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement).Hacspec_ml_kem.Parameters.f_val)
  in
  let bv:t_Array bool v_D256 = bitvector_from_bounded_ints (mk_usize 256) v_D256 p_raw d in
  bits_to_bytes v_D32 v_D256 bv

#pop-options

#push-options "--z3rlimit 300"

/// Given a series of bytes representing a ring element in `re_bytes`,
/// first convert them into a vector of bits in little-endian order; i.e.
/// the least significant `bits_per_coefficient` of `re_bytes[0]`
/// are the first set of bits in the bitstream.
/// This vector is deserialized into a `Polynomial` structure.
/// The first `bits_per_coefficient` represent the first coefficient of
/// the ring element, the second `bits_per_coefficient` the second coefficient,
/// and so on.
/// N.B.: This function asserts that `bits_per_coefficient <= 12`
/// This function implements <strong>Algorithm 5</strong> of the NIST FIPS 203
/// standard, which is reproduced below:
/// ```plaintext
/// Input: byte array B ∈ 𝔹^{32d}.
/// Output: integer array F ∈ ℤₘ²⁵⁶, where m = 2ᵈ if d < 12 and m = q if d = 12.
/// b ← BytesToBits(B)
/// for (i ← 0; i < 256; i++)
///     F[i] ← ∑(j = 0 to d−1) b[i·d + j] · 2ʲ mod m
/// end for
/// return F
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let bitvector_to_bounded_ints (v_N v_Nd: usize) (input: t_Array bool v_Nd) (d: usize)
    : Prims.Pure (t_Array u16 v_N)
      (requires
        v_N <. mk_usize 16384 && d <=. Hacspec_ml_kem.Parameters.v_BITS_PER_COEFFICIENT &&
        v_Nd =. (v_N *! d <: usize))
      (ensures
        fun result ->
          let result:t_Array u16 v_N = result in
          forall (i: usize).
            b2t (i <. v_N <: bool) ==>
            b2t ((result.[ i ] <: u16) <. (mk_u16 1 <<! d <: u16) <: bool)) =
  let _:Prims.unit = () <: Prims.unit in
  let (result: t_Array u16 v_N):t_Array u16 v_N =
    Hacspec_ml_kem.Parameters.createi #u16
      v_N
      #(usize -> u16)
      (fun i ->
          let i:usize = i in
          let (coefficient: u16):u16 = mk_u16 0 in
          let coefficient:u16 =
            Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
              d
              (fun coefficient j ->
                  let coefficient:u16 = coefficient in
                  let j:usize = j in
                  coefficient <. (mk_u16 1 <<! j <: u16) <: bool)
              coefficient
              (fun coefficient j ->
                  let coefficient:u16 = coefficient in
                  let j:usize = j in
                  if input.[ (i *! d <: usize) +! j <: usize ] <: bool
                  then
                    let coefficient:u16 = coefficient +! (mk_u16 1 <<! j <: u16) in
                    coefficient
                  else coefficient)
          in
          coefficient)
  in
  let _:Prims.unit = () <: Prims.unit in
  result

#pop-options

#push-options "--z3rlimit 150"

let byte_decode_generic (v_N v_N8 v_Nd v_Nd8: usize) (b: t_Array u8 v_Nd) (d: usize)
    : Prims.Pure (t_Array u16 v_N8)
      (requires
        d >. mk_usize 0 && d <=. Hacspec_ml_kem.Parameters.v_BITS_PER_COEFFICIENT &&
        v_N <. (mk_usize 16384 /! d <: usize) &&
        v_N <. (mk_usize 16384 /! mk_usize 8 <: usize) &&
        v_N8 =. (v_N *! mk_usize 8 <: usize) &&
        v_Nd =. (v_N *! d <: usize) &&
        v_Nd8 =. (v_Nd *! mk_usize 8 <: usize))
      (ensures
        fun result ->
          let result:t_Array u16 v_N8 = result in
          forall (i: usize).
            b2t (i <. v_N8 <: bool) ==>
            b2t ((result.[ i ] <: u16) <. (mk_u16 1 <<! d <: u16) <: bool)) =
  let _:Prims.unit = () <: Prims.unit in
  let (bv: t_Array bool v_Nd8):t_Array bool v_Nd8 = bytes_to_bits v_Nd v_Nd8 b in
  bitvector_to_bounded_ints v_N8 v_Nd8 bv d

#pop-options

#push-options "--z3rlimit 300"

let byte_decode (v_D32 v_D256: usize) (b: t_Array u8 v_D32) (d: usize)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (requires
        d >. mk_usize 0 && d <=. Hacspec_ml_kem.Parameters.v_BITS_PER_COEFFICIENT &&
        (Core_models.Slice.impl__len #u8 (b <: t_Slice u8) <: usize) =. (mk_usize 32 *! d <: usize) &&
        v_D32 =. (mk_usize 32 *! d <: usize) &&
        v_D256 =. (mk_usize 256 *! d <: usize))
      (ensures
        fun result ->
          let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) = result in
          forall (i: usize).
            b2t (i <. mk_usize 256 <: bool) ==>
            b2t
            ((result.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
                .Hacspec_ml_kem.Parameters.f_val <.
              (mk_u16 1 <<! d <: u16)
              <:
              bool)) =
  let _:Prims.unit = () <: Prims.unit in
  let decoded:t_Array u16 (mk_usize 256) =
    byte_decode_generic (mk_usize 32) (mk_usize 256) v_D32 v_D256 b d
  in
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun i ->
        let i:usize = i in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new ((decoded.[ i ] <: u16) %!
            Hacspec_ml_kem.Parameters.v_FIELD_MODULUS
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

#pop-options

#push-options "--z3rlimit 150"

/// Encode each polynomial of an NTT vector at 12 bits per coefficient,
/// writing the result into `out`.  Companion to value-returning
/// [`serialize_secret_key`].  Mirrors the existing
/// [`byte_encode_into`] convention: the `_into` form is the canonical
/// primitive; the value-returning form is a thin allocating wrapper.
let serialize_secret_key_into
      (v_RANK: usize)
      (vector: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (out: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        v_RANK <=. mk_usize 4 &&
        (Core_models.Slice.impl__len #u8 out <: usize) =.
        (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let out:t_Slice u8 =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      v_RANK
      (fun out e_i ->
          let out:t_Slice u8 = out in
          let e_i:usize = e_i in
          (Core_models.Slice.impl__len #u8 out <: usize) =.
          (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize)
          <:
          bool)
      out
      (fun out i ->
          let out:t_Slice u8 = out in
          let i:usize = i in
          let encoded:t_Array u8 (mk_usize 384) =
            byte_encode (mk_usize 384)
              (mk_usize 3072)
              (vector.[ i ] <: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
              (mk_usize 12)
          in
          let out:t_Slice u8 =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_range out
              ({
                  Core_models.Ops.Range.f_start
                  =
                  i *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize;
                  Core_models.Ops.Range.f_end
                  =
                  (i +! mk_usize 1 <: usize) *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT
                  <:
                  usize
                }
                <:
                Core_models.Ops.Range.t_Range usize)
              (Core_models.Slice.impl__copy_from_slice #u8
                  (out.[ {
                        Core_models.Ops.Range.f_start
                        =
                        i *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize;
                        Core_models.Ops.Range.f_end
                        =
                        (i +! mk_usize 1 <: usize) *!
                        Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT
                        <:
                        usize
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize ]
                    <:
                    t_Slice u8)
                  (encoded <: t_Slice u8)
                <:
                t_Slice u8)
          in
          out)
  in
  out

#pop-options

#push-options "--z3rlimit 150"

let vector_decode_12_ (v_RANK: usize) (encoded: t_Slice u8)
    : Prims.Pure (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (requires
        v_RANK <=. mk_usize 4 &&
        (Core_models.Slice.impl__len #u8 encoded <: usize) =.
        (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
        (mk_usize 256))
    v_RANK
    #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (fun i ->
        let i:usize = i in
        let start:usize = i *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT in
        let (chunk: t_Array u8 (mk_usize 384)):t_Array u8 (mk_usize 384) =
          Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 384))
            #Core_models.Array.t_TryFromSliceError
            (Core_models.Convert.f_try_into #(t_Slice u8)
                #(t_Array u8 (mk_usize 384))
                #FStar.Tactics.Typeclasses.solve
                (encoded.[ {
                      Core_models.Ops.Range.f_start = start;
                      Core_models.Ops.Range.f_end = start +! mk_usize 384 <: usize
                    }
                    <:
                    Core_models.Ops.Range.t_Range usize ]
                  <:
                  t_Slice u8)
              <:
              Core_models.Result.t_Result (t_Array u8 (mk_usize 384))
                Core_models.Array.t_TryFromSliceError)
        in
        byte_decode (mk_usize 384) (mk_usize 3072) chunk (mk_usize 12))

#pop-options

#push-options "--z3rlimit 150"

let byte_encode_into
      (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (d: usize)
      (out: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (d =. mk_usize 1 || d =. mk_usize 4 || d =. mk_usize 5 || d =. mk_usize 10 ||
        d =. mk_usize 11 ||
        d =. mk_usize 12) &&
        (Core_models.Slice.impl__len #u8 out <: usize) =. (mk_usize 32 *! d <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let out:t_Slice u8 =
    match d <: usize with
    | Rust_primitives.Integers.MkInt 1 ->
      Core_models.Slice.impl__copy_from_slice #u8
        out
        (byte_encode (mk_usize 32) (mk_usize 256) p (mk_usize 1) <: t_Slice u8)
    | Rust_primitives.Integers.MkInt 4 ->
      Core_models.Slice.impl__copy_from_slice #u8
        out
        (byte_encode (mk_usize 128) (mk_usize 1024) p (mk_usize 4) <: t_Slice u8)
    | Rust_primitives.Integers.MkInt 5 ->
      Core_models.Slice.impl__copy_from_slice #u8
        out
        (byte_encode (mk_usize 160) (mk_usize 1280) p (mk_usize 5) <: t_Slice u8)
    | Rust_primitives.Integers.MkInt 10 ->
      Core_models.Slice.impl__copy_from_slice #u8
        out
        (byte_encode (mk_usize 320) (mk_usize 2560) p (mk_usize 10) <: t_Slice u8)
    | Rust_primitives.Integers.MkInt 11 ->
      Core_models.Slice.impl__copy_from_slice #u8
        out
        (byte_encode (mk_usize 352) (mk_usize 2816) p (mk_usize 11) <: t_Slice u8)
    | Rust_primitives.Integers.MkInt 12 ->
      Core_models.Slice.impl__copy_from_slice #u8
        out
        (byte_encode (mk_usize 384) (mk_usize 3072) p (mk_usize 12) <: t_Slice u8)
    | _ ->
      let args:usize = d <: usize in
      let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
        let list = [Core_models.Fmt.Rt.impl__new_display #usize args] in
        FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
        Rust_primitives.Hax.array_of_list 1 list
      in
      out
  in
  out

#pop-options

#push-options "--z3rlimit 150"

let byte_decode_dyn (b: t_Slice u8) (d: usize)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (requires
        (d =. mk_usize 1 || d =. mk_usize 4 || d =. mk_usize 5 || d =. mk_usize 10 ||
        d =. mk_usize 11 ||
        d =. mk_usize 12) &&
        (Core_models.Slice.impl__len #u8 b <: usize) =. (mk_usize 32 *! d <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  match d <: usize with
  | Rust_primitives.Integers.MkInt 1 ->
    byte_decode (mk_usize 32)
      (mk_usize 256)
      (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
          #Core_models.Array.t_TryFromSliceError
          (Core_models.Convert.f_try_into #(t_Slice u8)
              #(t_Array u8 (mk_usize 32))
              #FStar.Tactics.Typeclasses.solve
              b
            <:
            Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
              Core_models.Array.t_TryFromSliceError)
        <:
        t_Array u8 (mk_usize 32))
      (mk_usize 1)
  | Rust_primitives.Integers.MkInt 4 ->
    byte_decode (mk_usize 128)
      (mk_usize 1024)
      (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 128))
          #Core_models.Array.t_TryFromSliceError
          (Core_models.Convert.f_try_into #(t_Slice u8)
              #(t_Array u8 (mk_usize 128))
              #FStar.Tactics.Typeclasses.solve
              b
            <:
            Core_models.Result.t_Result (t_Array u8 (mk_usize 128))
              Core_models.Array.t_TryFromSliceError)
        <:
        t_Array u8 (mk_usize 128))
      (mk_usize 4)
  | Rust_primitives.Integers.MkInt 5 ->
    byte_decode (mk_usize 160)
      (mk_usize 1280)
      (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 160))
          #Core_models.Array.t_TryFromSliceError
          (Core_models.Convert.f_try_into #(t_Slice u8)
              #(t_Array u8 (mk_usize 160))
              #FStar.Tactics.Typeclasses.solve
              b
            <:
            Core_models.Result.t_Result (t_Array u8 (mk_usize 160))
              Core_models.Array.t_TryFromSliceError)
        <:
        t_Array u8 (mk_usize 160))
      (mk_usize 5)
  | Rust_primitives.Integers.MkInt 10 ->
    byte_decode (mk_usize 320)
      (mk_usize 2560)
      (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 320))
          #Core_models.Array.t_TryFromSliceError
          (Core_models.Convert.f_try_into #(t_Slice u8)
              #(t_Array u8 (mk_usize 320))
              #FStar.Tactics.Typeclasses.solve
              b
            <:
            Core_models.Result.t_Result (t_Array u8 (mk_usize 320))
              Core_models.Array.t_TryFromSliceError)
        <:
        t_Array u8 (mk_usize 320))
      (mk_usize 10)
  | Rust_primitives.Integers.MkInt 11 ->
    byte_decode (mk_usize 352)
      (mk_usize 2816)
      (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 352))
          #Core_models.Array.t_TryFromSliceError
          (Core_models.Convert.f_try_into #(t_Slice u8)
              #(t_Array u8 (mk_usize 352))
              #FStar.Tactics.Typeclasses.solve
              b
            <:
            Core_models.Result.t_Result (t_Array u8 (mk_usize 352))
              Core_models.Array.t_TryFromSliceError)
        <:
        t_Array u8 (mk_usize 352))
      (mk_usize 11)
  | Rust_primitives.Integers.MkInt 12 ->
    byte_decode (mk_usize 384)
      (mk_usize 3072)
      (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 384))
          #Core_models.Array.t_TryFromSliceError
          (Core_models.Convert.f_try_into #(t_Slice u8)
              #(t_Array u8 (mk_usize 384))
              #FStar.Tactics.Typeclasses.solve
              b
            <:
            Core_models.Result.t_Result (t_Array u8 (mk_usize 384))
              Core_models.Array.t_TryFromSliceError)
        <:
        t_Array u8 (mk_usize 384))
      (mk_usize 12)
  | _ ->
    let args:usize = d <: usize in
    let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 1) =
      let list = [Core_models.Fmt.Rt.impl__new_display #usize args] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
      Rust_primitives.Hax.array_of_list 1 list
    in
    Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic_fmt (Core_models.Fmt.Rt.impl_1__new_v1
              (mk_usize 1)
              (mk_usize 1)
              (let list = ["unsupported d="] in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                Rust_primitives.Hax.array_of_list 1 list)
              args
            <:
            Core_models.Fmt.t_Arguments)
        <:
        Rust_primitives.Hax.t_Never)

#pop-options

/// Compress a polynomial to 1 bit per coefficient, then serialize to bytes.
/// Corresponds to `compress_then_serialize_message` in the implementation.
/// Used for encoding/decoding the message in K-PKE.
let compress_then_serialize_message
      (re: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array u8 (mk_usize 32) =
  byte_encode (mk_usize 32)
    (mk_usize 256)
    (Hacspec_ml_kem.Compress.compress re (mk_usize 1)
      <:
      t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (mk_usize 1)

/// Deserialize bytes to a polynomial, then decompress from 1 bit per coefficient.
/// Corresponds to `deserialize_then_decompress_message` in the implementation.
let deserialize_then_decompress_message (serialized: t_Array u8 (mk_usize 32))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Compress.decompress (byte_decode (mk_usize 32)
        (mk_usize 256)
        serialized
        (mk_usize 1)
      <:
      t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (mk_usize 1)

/// Serialize a polynomial with 12-bit coefficients (no compression).
/// Corresponds to `serialize_uncompressed_ring_element` in the implementation.
let serialize_uncompressed_ring_element
      (re: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array u8 (mk_usize 384) = byte_encode (mk_usize 384) (mk_usize 3072) re (mk_usize 12)

/// Deserialize bytes to a polynomial with 12-bit coefficients (no decompression).
/// Corresponds to `deserialize_to_uncompressed_ring_element` in the implementation.
let deserialize_to_uncompressed_ring_element (serialized: t_Array u8 (mk_usize 384))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  byte_decode (mk_usize 384) (mk_usize 3072) serialized (mk_usize 12)

#push-options "--z3rlimit 150"

/// Compress each polynomial in u to du bits, then serialize, writing
/// into `out`.  Companion to value-returning [`compress_then_serialize_u`].
/// Mirrors the [`byte_encode_into`] / [`serialize_secret_key_into`]
/// convention: the `_into` form is the canonical primitive.
let compress_then_serialize_u_into
      (v_RANK: usize)
      (u: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (du: usize)
      (out: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        v_RANK <=. mk_usize 4 && (du =. mk_usize 10 || du =. mk_usize 11) &&
        (Core_models.Slice.impl__len #u8 out <: usize) =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *! du
            <:
            usize) /!
          mk_usize 8
          <:
          usize))
      (fun _ -> Prims.l_True) =
  let du_poly_size:usize =
    (Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *! du <: usize) /! mk_usize 8
  in
  let out:t_Slice u8 =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      v_RANK
      (fun out e_i ->
          let out:t_Slice u8 = out in
          let e_i:usize = e_i in
          (Core_models.Slice.impl__len #u8 out <: usize) =.
          (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *! du
              <:
              usize) /!
            mk_usize 8
            <:
            usize)
          <:
          bool)
      out
      (fun out i ->
          let out:t_Slice u8 = out in
          let i:usize = i in
          let out:t_Slice u8 =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_range out
              ({
                  Core_models.Ops.Range.f_start = i *! du_poly_size <: usize;
                  Core_models.Ops.Range.f_end = (i +! mk_usize 1 <: usize) *! du_poly_size <: usize
                }
                <:
                Core_models.Ops.Range.t_Range usize)
              (byte_encode_into (Hacspec_ml_kem.Compress.compress (u.[ i ]
                        <:
                        t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                      du
                    <:
                    t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                  du
                  (out.[ {
                        Core_models.Ops.Range.f_start = i *! du_poly_size <: usize;
                        Core_models.Ops.Range.f_end
                        =
                        (i +! mk_usize 1 <: usize) *! du_poly_size <: usize
                      }
                      <:
                      Core_models.Ops.Range.t_Range usize ]
                    <:
                    t_Slice u8)
                <:
                t_Slice u8)
          in
          out)
  in
  out

#pop-options

#push-options "--z3rlimit 150"

/// Compress each polynomial in u to du bits, then serialize.
/// Corresponds to `compress_then_serialize_ring_element_u` in the implementation.
/// Thin allocating wrapper around [`compress_then_serialize_u_into`].
/// Note: The implementation dispatches on the compression factor (10 or 11).
/// In the spec we use the generic compress + byte_encode path.
let compress_then_serialize_u
      (v_RANK v_U_SIZE: usize)
      (u: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (du: usize)
    : Prims.Pure (t_Array u8 v_U_SIZE)
      (requires
        v_RANK <=. mk_usize 4 && (du =. mk_usize 10 || du =. mk_usize 11) &&
        v_U_SIZE =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *! du
            <:
            usize) /!
          mk_usize 8
          <:
          usize))
      (fun _ -> Prims.l_True) =
  let out:t_Array u8 v_U_SIZE = Rust_primitives.Hax.repeat (mk_u8 0) v_U_SIZE in
  let out:t_Array u8 v_U_SIZE = compress_then_serialize_u_into v_RANK u du out in
  out

#pop-options

#push-options "--z3rlimit 150"

/// Compress v to dv bits, then serialize.
/// Corresponds to `compress_then_serialize_ring_element_v` in the implementation.
let compress_then_serialize_v
      (v_V_SIZE: usize)
      (v: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (dv: usize)
    : Prims.Pure (t_Array u8 v_V_SIZE)
      (requires
        (dv =. mk_usize 4 || dv =. mk_usize 5) &&
        v_V_SIZE =.
        ((Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *! dv <: usize) /! mk_usize 8
          <:
          usize))
      (fun _ -> Prims.l_True) =
  let out:t_Array u8 v_V_SIZE = Rust_primitives.Hax.repeat (mk_u8 0) v_V_SIZE in
  let out:t_Array u8 v_V_SIZE =
    byte_encode_into (Hacspec_ml_kem.Compress.compress v dv
        <:
        t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      dv
      out
  in
  out

#pop-options

#push-options "--z3rlimit 150"

/// Deserialize and decompress u from ciphertext bytes.
/// Corresponds to `deserialize_then_decompress_ring_element_u` in the implementation.
let deserialize_then_decompress_u (v_RANK: usize) (ciphertext: t_Slice u8) (du: usize)
    : Prims.Pure (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (requires
        v_RANK <=. mk_usize 4 && (du =. mk_usize 10 || du =. mk_usize 11) &&
        (Core_models.Slice.impl__len #u8 ciphertext <: usize) =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *! du
            <:
            usize) /!
          mk_usize 8
          <:
          usize))
      (fun _ -> Prims.l_True) =
  let du_poly_size:usize =
    (Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *! du <: usize) /! mk_usize 8
  in
  Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
        (mk_usize 256))
    v_RANK
    #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (fun i ->
        let i:usize = i in
        let start:usize = i *! du_poly_size in
        Hacspec_ml_kem.Compress.decompress (byte_decode_dyn (ciphertext.[ {
                    Core_models.Ops.Range.f_start = start;
                    Core_models.Ops.Range.f_end = start +! du_poly_size <: usize
                  }
                  <:
                  Core_models.Ops.Range.t_Range usize ]
                <:
                t_Slice u8)
              du
            <:
            t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
          du)

#pop-options

#push-options "--z3rlimit 150"

/// Deserialize, decompress, then NTT-transform u from ciphertext bytes.
/// Composes [`deserialize_then_decompress_u`] with [`vector_ntt`].  Used
/// by `ind_cpa::decrypt` and matches the libcrux-impl
/// `deserialize_then_decompress_u` function (which fuses the NTT into
/// the per-element decompress loop).
let deserialize_then_decompress_u_then_ntt (v_RANK: usize) (ciphertext: t_Slice u8) (du: usize)
    : Prims.Pure (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (requires
        v_RANK <=. mk_usize 4 && (du =. mk_usize 10 || du =. mk_usize 11) &&
        (Core_models.Slice.impl__len #u8 ciphertext <: usize) =.
        (((v_RANK *! Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *! du
            <:
            usize) /!
          mk_usize 8
          <:
          usize))
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Ntt.vector_ntt v_RANK
    (deserialize_then_decompress_u v_RANK ciphertext du
      <:
      t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)

#pop-options

#push-options "--z3rlimit 150"

/// Deserialize and decompress v from ciphertext bytes.
/// Corresponds to `deserialize_then_decompress_ring_element_v` in the implementation.
let deserialize_then_decompress_v (serialized: t_Slice u8) (dv: usize)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (requires
        (dv =. mk_usize 4 || dv =. mk_usize 5) &&
        (Core_models.Slice.impl__len #u8 serialized <: usize) =.
        ((Hacspec_ml_kem.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *! dv <: usize) /! mk_usize 8
          <:
          usize))
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Compress.decompress (byte_decode_dyn serialized dv
      <:
      t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    dv

#pop-options

/// Deserialize ring elements from a byte slice, reducing mod q.
/// Corresponds to `deserialize_ring_elements_reduced` in the implementation.
/// This is equivalent to `vector_decode_12` but named to match the implementation.
let deserialize_ring_elements_reduced (v_RANK: usize) (encoded: t_Slice u8)
    : Prims.Pure (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (requires
        v_RANK <=. mk_usize 4 &&
        (Core_models.Slice.impl__len #u8 encoded <: usize) =.
        (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize))
      (fun _ -> Prims.l_True) = vector_decode_12_ v_RANK encoded

/// Serialize a vector of polynomials with 12-bit coefficients.
/// Corresponds to `serialize_secret_key` / `serialize_vector` in the implementation.
let serialize_secret_key
      (v_RANK v_T_SIZE: usize)
      (vector: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    : Prims.Pure (t_Array u8 v_T_SIZE)
      (requires
        v_RANK <=. mk_usize 4 &&
        v_T_SIZE =. (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize))
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Parameters.createi #u8
    v_T_SIZE
    #(usize -> u8)
    (fun k ->
        let k:usize = k in
        let i:usize = k /! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT in
        let j:usize = k %! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT in
        let encoded:t_Array u8 (mk_usize 384) =
          byte_encode (mk_usize 384)
            (mk_usize 3072)
            (vector.[ i ] <: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
            (mk_usize 12)
        in
        encoded.[ j ])

/// Serialize a public key: encode the NTT vector t\u{302} concatenated with the seed ρ.
/// Corresponds to `serialize_public_key` in the implementation\'s `ind_cpa.rs`.
/// Layout: bytes [0, RANK*384) hold byte_encode(t_as_ntt[i], 12) per polynomial,
/// bytes [RANK*384, EK_SIZE) hold seed_for_A[0..32].
let serialize_public_key
      (v_RANK v_EK_SIZE: usize)
      (tt_as_ntt: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (seed_for_A: t_Slice u8)
    : Prims.Pure (t_Array u8 v_EK_SIZE)
      (requires
        v_RANK <=. mk_usize 4 &&
        v_EK_SIZE =.
        ((v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32
          <:
          usize) &&
        (Core_models.Slice.impl__len #u8 seed_for_A <: usize) >=. mk_usize 32)
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Parameters.createi #u8
    v_EK_SIZE
    #(usize -> u8)
    (fun k ->
        let k:usize = k in
        if k <. (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize) <: bool
        then
          let i:usize = k /! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT in
          let j:usize = k %! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT in
          let encoded:t_Array u8 (mk_usize 384) =
            byte_encode (mk_usize 384)
              (mk_usize 3072)
              (tt_as_ntt.[ i ] <: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
              (mk_usize 12)
          in
          encoded.[ j ]
        else
          seed_for_A.[ k -! (v_RANK *! Hacspec_ml_kem.Parameters.v_BYTES_PER_RING_ELEMENT <: usize)
            <:
            usize ]
          <:
          u8)
