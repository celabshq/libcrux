module Hacspec_ml_kem.Sampling
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

type t_BadRejectionSamplingRandomnessError =
  | BadRejectionSamplingRandomnessError : t_BadRejectionSamplingRandomnessError

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl': Core_models.Fmt.t_Debug t_BadRejectionSamplingRandomnessError

unfold
let impl = impl'

/// If `bytes` contains a set of uniformly random bytes, this function
/// uniformly samples a ring element `â` that is treated as being the NTT representation
/// of the corresponding polynomial `a`.
/// Since rejection sampling is used, it is possible the supplied bytes are
/// not enough to sample the element, in which case an `Err` is returned and the
/// caller must try again with a fresh set of bytes.
/// This function <strong>partially</strong> implements <strong>Algorithm 6</strong> of the NIST FIPS 203 standard,
/// We say "partially" because this implementation only accepts a finite set of
/// bytes as input and returns an error if the set is not enough; Algorithm 6 of
/// the FIPS 203 standard on the other hand samples from an infinite stream of bytes
/// until the ring element is filled. Algorithm 6 is reproduced below:
/// ```plaintext
/// Input: byte stream B ∈ 𝔹*.
/// Output: array â ∈ ℤ₂₅₆.
/// i ← 0
/// j ← 0
/// while j < 256 do
///     d₁ ← B[i] + 256·(B[i+1] mod 16)
///     d₂ ← ⌊B[i+1]/16⌋ + 16·B[i+2]
///     if d₁ < q then
///         â[j] ← d₁
///         j ← j + 1
///     end if
///     if d₂ < q and j < 256 then
///         â[j] ← d₂
///         j ← j + 1
///     end if
///     i ← i + 3
/// end while
/// return â
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
/// One iteration of rejection sampling.
/// Decodes 24 bytes into 16 u16 values using 12-bit little-endian packing
/// (FIPS 203 Algorithm 6, inner loop body), keeps those `< q`, and returns
/// the accepted values packed at the start of a 16-element array together
/// with the count.  The unused tail of the array is filled with zeros.
/// This is the natural unit of rejection sampling at the trait boundary
/// (`Operations::rej_sample` consumes exactly 24 bytes); `sample_ntt`
/// below repeatedly applies this step until 256 coefficients have been
/// accepted or the byte stream is exhausted.
let rej_sample_step (bytes: t_Array u8 (mk_usize 24))
    : (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16) & usize) =
  let decoded:t_Array u16 (mk_usize 16) =
    Hacspec_ml_kem.Serialize.byte_decode_generic (mk_usize 2)
      (mk_usize 16)
      (mk_usize 24)
      (mk_usize 192)
      bytes
      (mk_usize 12)
  in
  let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16) =
    Rust_primitives.Hax.repeat (Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 0)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)
      (mk_usize 16)
  in
  let (count: usize):usize = mk_usize 0 in
  let (count: usize), (result: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16)) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 16)
      (fun temp_0_ i ->
          let
          (count: usize), (result: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16)) =
            temp_0_
          in
          let i:usize = i in
          count <=. i <: bool)
      (count, result <: (usize & t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16)))
      (fun temp_0_ i ->
          let
          (count: usize), (result: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16)) =
            temp_0_
          in
          let i:usize = i in
          if (decoded.[ i ] <: u16) <. Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: bool
          then
            let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16) =
              Rust_primitives.Hax.Monomorphized_update_at.update_at_usize result
                count
                (Hacspec_ml_kem.Parameters.impl_FieldElement__new (decoded.[ i ] <: u16)
                  <:
                  Hacspec_ml_kem.Parameters.t_FieldElement)
            in
            let count:usize = count +! mk_usize 1 in
            count, result
            <:
            (usize & t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16))
          else
            count, result
            <:
            (usize & t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16)))
  in
  result, count <: (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 16) & usize)

let sample_ntt (v_N v_N8 v_N12 v_N96: usize) (bytes: t_Array u8 v_N12)
    : Prims.Pure
      (Core_models.Result.t_Result (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
          t_BadRejectionSamplingRandomnessError)
      (requires
        v_N <=. (Hacspec_ml_kem.Serialize.v_MAX_BYTES /! mk_usize 96 <: usize) &&
        v_N8 =. (v_N *! mk_usize 8 <: usize) &&
        v_N12 =. (v_N *! mk_usize 12 <: usize) &&
        v_N96 =. (v_N12 *! mk_usize 8 <: usize))
      (fun _ -> Prims.l_True) =
  let decoded:t_Array u16 v_N8 =
    Hacspec_ml_kem.Serialize.byte_decode_generic v_N v_N8 v_N12 v_N96 bytes (mk_usize 12)
  in
  let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    Rust_primitives.Hax.repeat (Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 0)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)
      (mk_usize 256)
  in
  let (sampled_coefficients: usize):usize = mk_usize 0 in
  let
  (result: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)),
  (sampled_coefficients: usize) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      v_N8
      (fun temp_0_ temp_1_ ->
          let
          (result: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)),
          (sampled_coefficients: usize) =
            temp_0_
          in
          let _:usize = temp_1_ in
          true)
      (result, sampled_coefficients
        <:
        (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) & usize))
      (fun temp_0_ i ->
          let
          (result: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)),
          (sampled_coefficients: usize) =
            temp_0_
          in
          let i:usize = i in
          if
            ((decoded.[ i ] <: u16) <. Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: bool) &&
            (sampled_coefficients <. mk_usize 256 <: bool)
          then
            let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
              Rust_primitives.Hax.Monomorphized_update_at.update_at_usize result
                sampled_coefficients
                (Hacspec_ml_kem.Parameters.impl_FieldElement__new (decoded.[ i ] <: u16)
                  <:
                  Hacspec_ml_kem.Parameters.t_FieldElement)
            in
            result, sampled_coefficients +! mk_usize 1
            <:
            (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) & usize)
          else
            result, sampled_coefficients
            <:
            (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) & usize))
  in
  if sampled_coefficients =. mk_usize 256
  then
    Core_models.Result.Result_Ok result
    <:
    Core_models.Result.t_Result (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      t_BadRejectionSamplingRandomnessError
  else
    Core_models.Result.Result_Err
    (BadRejectionSamplingRandomnessError <: t_BadRejectionSamplingRandomnessError)
    <:
    Core_models.Result.t_Result (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      t_BadRejectionSamplingRandomnessError

let sum_coins (eta: usize) (coins: t_Slice bool)
    : Prims.Pure Hacspec_ml_kem.Parameters.t_FieldElement
      (requires eta <=. mk_usize 4 && (Core_models.Slice.impl__len #bool coins <: usize) =. eta)
      (ensures
        fun r ->
          let r:Hacspec_ml_kem.Parameters.t_FieldElement = r in
          r.Hacspec_ml_kem.Parameters.f_val <=. (cast (eta <: usize) <: u16)) =
  let _:Prims.unit = () <: Prims.unit in
  let (sum: u16):u16 = mk_u16 0 in
  let sum:u16 =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      eta
      (fun sum i ->
          let sum:u16 = sum in
          let i:usize = i in
          sum <=. (cast (i <: usize) <: u16) <: bool)
      sum
      (fun sum i ->
          let sum:u16 = sum in
          let i:usize = i in
          let sum:u16 = sum +! (cast (coins.[ i ] <: bool) <: u16) in
          sum)
  in
  Hacspec_ml_kem.Parameters.impl_FieldElement__new sum

#push-options "--z3rlimit 150"

/// Given a series of uniformly random bytes in `randomness`, sample
/// a ring element from a binomial distribution centered at 0 that uses two sets
/// of `eta` coin flips. If, for example,
/// `eta = ETA`, each ring coefficient is a value `v` such
/// such that `v ∈ {-ETA, -ETA + 1, ..., 0, ..., ETA + 1, ETA}` and:
/// ```plaintext
/// - If v < 0, Pr[v] = Pr[-v]
/// - If v >= 0, Pr[v] = BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) / 2 ^ (2 * ETA)
/// ```
/// The values `v < 0` are mapped to the appropriate `KyberFieldElement`.
/// The expected value is:
/// ```plaintext
/// E[X] = (-ETA)Pr[-ETA] + (-(ETA - 1))Pr[-(ETA - 1)] + ... + (ETA - 1)Pr[ETA - 1] + (ETA)Pr[ETA]
///      = 0 since Pr[-v] = Pr[v] when v < 0.
/// ```
/// And the variance is:
/// ```plaintext
/// Var(X) = E[(X - E[X])^2]
///        = E[X^2]
///        = sum_(v=-ETA to ETA)v^2 * (BINOMIAL_COEFFICIENT(2 * ETA; ETA - v) / 2^(2 * ETA))
///        = ETA / 2
/// ```
/// This function implements <strong>Algorithm 7</strong> of the NIST FIPS 203 standard, which is
/// reproduced below:
/// ```plaintext
/// Input: byte array B ∈ 𝔹^{64η}.
/// Output: array f ∈ ℤ₂₅₆.
/// b ← BytesToBits(B)
/// for (i ← 0; i < 256; i++)
///     x ← ∑(j=0 to η - 1) b[2iη + j]
///     y ← ∑(j=0 to η - 1) b[2iη + η + j]
///     f[i] ← x−y mod q
/// end for
/// return f
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let sample_poly_cbd (v_ETA64 v_ETA512 eta: usize) (bytes: t_Array u8 v_ETA64)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (requires
        eta <=. mk_usize 4 && v_ETA64 =. (eta *! mk_usize 64 <: usize) &&
        v_ETA512 =. (eta *! mk_usize 512 <: usize))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let (bits: t_Array bool v_ETA512):t_Array bool v_ETA512 =
    Hacspec_ml_kem.Serialize.bytes_to_bits v_ETA64 v_ETA512 bytes
  in
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun i ->
        let i:usize = i in
        let (x: Hacspec_ml_kem.Parameters.t_FieldElement):Hacspec_ml_kem.Parameters.t_FieldElement =
          sum_coins eta
            (bits.[ {
                  Core_models.Ops.Range.f_start = (mk_usize 2 *! i <: usize) *! eta <: usize;
                  Core_models.Ops.Range.f_end
                  =
                  ((mk_usize 2 *! i <: usize) *! eta <: usize) +! eta <: usize
                }
                <:
                Core_models.Ops.Range.t_Range usize ]
              <:
              t_Slice bool)
        in
        let (y: Hacspec_ml_kem.Parameters.t_FieldElement):Hacspec_ml_kem.Parameters.t_FieldElement =
          sum_coins eta
            (bits.[ {
                  Core_models.Ops.Range.f_start
                  =
                  ((mk_usize 2 *! i <: usize) *! eta <: usize) +! eta <: usize;
                  Core_models.Ops.Range.f_end
                  =
                  ((mk_usize 2 *! i <: usize) *! eta <: usize) +! (mk_usize 2 *! eta <: usize)
                  <:
                  usize
                }
                <:
                Core_models.Ops.Range.t_Range usize ]
              <:
              t_Slice bool)
        in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new (((x.Hacspec_ml_kem.Parameters.f_val +!
                Hacspec_ml_kem.Parameters.v_FIELD_MODULUS
                <:
                u16) -!
              y.Hacspec_ml_kem.Parameters.f_val
              <:
              u16) %!
            Hacspec_ml_kem.Parameters.v_FIELD_MODULUS
            <:
            u16))

#pop-options
