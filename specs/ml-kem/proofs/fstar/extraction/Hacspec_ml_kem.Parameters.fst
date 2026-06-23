module Hacspec_ml_kem.Parameters
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// Field modulus: 3329
let v_FIELD_MODULUS: u16 = mk_u16 3329

/// Each field element needs floor(log_2(FIELD_MODULUS)) + 1 = 12 bits to represent
let v_BITS_PER_COEFFICIENT: usize = mk_usize 12

/// Coefficients per ring element
let v_COEFFICIENTS_IN_RING_ELEMENT: usize = mk_usize 256

/// Bits required per (uncompressed) ring element
let v_BITS_PER_RING_ELEMENT: usize = v_COEFFICIENTS_IN_RING_ELEMENT *! mk_usize 12

/// Bytes required per (uncompressed) ring element
let v_BYTES_PER_RING_ELEMENT: usize = v_BITS_PER_RING_ELEMENT /! mk_usize 8

/// Seed size for rejection sampling.
/// See <https://eprint.iacr.org/2023/708> for some background regarding
/// this choice.
let v_REJECTION_SAMPLING_SEED_SIZE: usize = mk_usize 168 *! mk_usize 5

/// ML-KEM parameter set
type t_MlKemParams = {
  f_rank:f_rank: usize{b2t (f_rank <=. mk_usize 4 <: bool)};
  f_eta1:f_eta1: usize{b2t ((f_eta1 <=. mk_usize 2 <: bool) || (f_eta1 =. mk_usize 3 <: bool))};
  f_eta2:f_eta2: usize{b2t (f_eta2 =. mk_usize 2 <: bool)};
  f_du:f_du: usize{b2t ((f_du =. mk_usize 10 <: bool) || (f_du =. mk_usize 11 <: bool))};
  f_dv:f_dv: usize{b2t ((f_dv =. mk_usize 4 <: bool) || (f_dv =. mk_usize 5 <: bool))}
}

let impl_MlKemParams__tt_as_ntt_encoded_size (self: t_MlKemParams) : usize =
  self.f_rank *! v_BYTES_PER_RING_ELEMENT

let impl_MlKemParams__ek_size (self: t_MlKemParams) : usize =
  (impl_MlKemParams__tt_as_ntt_encoded_size self <: usize) +! mk_usize 32

let impl_MlKemParams__dk_pke_size (self: t_MlKemParams) : usize =
  self.f_rank *! v_BYTES_PER_RING_ELEMENT

let impl_MlKemParams__dk_size (self: t_MlKemParams) : usize =
  (((impl_MlKemParams__dk_pke_size self <: usize) +! (impl_MlKemParams__ek_size self <: usize)
      <:
      usize) +!
    Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
    <:
    usize) +!
  mk_usize 32

let impl_MlKemParams__u_encoded_size (self: t_MlKemParams) : usize =
  ((self.f_rank *! v_COEFFICIENTS_IN_RING_ELEMENT <: usize) *! self.f_du <: usize) /! mk_usize 8

let impl_MlKemParams__vv_encoded_size (self: t_MlKemParams) : usize =
  (v_COEFFICIENTS_IN_RING_ELEMENT *! self.f_dv <: usize) /! mk_usize 8

let impl_MlKemParams__ciphertext_size (self: t_MlKemParams) : usize =
  (impl_MlKemParams__u_encoded_size self <: usize) +!
  (impl_MlKemParams__vv_encoded_size self <: usize)

let v_ML_KEM_512_: t_MlKemParams =
  {
    f_rank = mk_usize 2;
    f_eta1 = mk_usize 3;
    f_eta2 = mk_usize 2;
    f_du = mk_usize 10;
    f_dv = mk_usize 4
  }
  <:
  t_MlKemParams

let v_ML_KEM_768_: t_MlKemParams =
  {
    f_rank = mk_usize 3;
    f_eta1 = mk_usize 2;
    f_eta2 = mk_usize 2;
    f_du = mk_usize 10;
    f_dv = mk_usize 4
  }
  <:
  t_MlKemParams

let v_ML_KEM_1024_: t_MlKemParams =
  {
    f_rank = mk_usize 4;
    f_eta1 = mk_usize 2;
    f_eta2 = mk_usize 2;
    f_du = mk_usize 11;
    f_dv = mk_usize 5
  }
  <:
  t_MlKemParams

let v_ML_KEM_512_EK_SIZE: usize = mk_usize 800

let v_ML_KEM_512_DK_PKE_SIZE: usize = mk_usize 768

let v_ML_KEM_512_DK_SIZE: usize = mk_usize 1632

let v_ML_KEM_512_U_SIZE: usize = mk_usize 640

let v_ML_KEM_512_V_SIZE: usize = mk_usize 128

let v_ML_KEM_512_CT_SIZE: usize = mk_usize 768

let v_ML_KEM_512_J_INPUT_SIZE: usize = mk_usize 800

let v_ML_KEM_768_EK_SIZE: usize = mk_usize 1184

let v_ML_KEM_768_DK_PKE_SIZE: usize = mk_usize 1152

let v_ML_KEM_768_DK_SIZE: usize = mk_usize 2400

let v_ML_KEM_768_U_SIZE: usize = mk_usize 960

let v_ML_KEM_768_V_SIZE: usize = mk_usize 128

let v_ML_KEM_768_CT_SIZE: usize = mk_usize 1088

let v_ML_KEM_768_J_INPUT_SIZE: usize = mk_usize 1120

let v_ML_KEM_1024_EK_SIZE: usize = mk_usize 1568

let v_ML_KEM_1024_DK_PKE_SIZE: usize = mk_usize 1536

let v_ML_KEM_1024_DK_SIZE: usize = mk_usize 3168

let v_ML_KEM_1024_U_SIZE: usize = mk_usize 1408

let v_ML_KEM_1024_V_SIZE: usize = mk_usize 160

let v_ML_KEM_1024_CT_SIZE: usize = mk_usize 1568

let v_ML_KEM_1024_J_INPUT_SIZE: usize = mk_usize 1600

/// Rank-generic CPA ciphertext size, matching the Spec.MLKEM
/// `v_CPA_CIPHERTEXT_SIZE` shape.  Use this in rank-generic
/// `hax_lib::requires`/`ensures` annotations where threading a
/// `MlKemParams` value would be an architectural refactor.  For
/// fixed-rank consumers, prefer the `ML_KEM_{512,768,1024}_CT_SIZE`
/// constants directly; for `MlKemParams`-aware callers, prefer
/// `MlKemParams::ciphertext_size()`.
let cpa_ciphertext_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) =
  if rank =. mk_usize 2
  then v_ML_KEM_512_CT_SIZE
  else if rank =. mk_usize 3 then v_ML_KEM_768_CT_SIZE else v_ML_KEM_1024_CT_SIZE

/// Rank-to-`MlKemParams` lookup for rank-generic consumers that need to
/// invoke a `MlKemParams`-shape Hacspec function (e.g.
/// `Hacspec_ml_kem.Ind_cca.generate_keypair`) from an
/// `hax_lib::requires`/`ensures` annotation.  This is the canonical
/// adapter from the libcrux-side `const K: usize` shape to the
/// Hacspec-side `params: MlKemParams` shape.
let rank_to_params (rank: usize)
    : Prims.Pure t_MlKemParams
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) =
  if rank =. mk_usize 2
  then v_ML_KEM_512_
  else if rank =. mk_usize 3 then v_ML_KEM_768_ else v_ML_KEM_1024_

/// Rank predicate: ML-KEM is parameterised over rank ∈ {2, 3, 4}.
/// Use in `hax_lib::requires` to express rank refinement in pure Rust.
let is_rank (rank: usize) : bool = rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4

/// Shared-secret length in bytes (FIPS 203 §7).
let v_SHARED_SECRET_SIZE: usize = mk_usize 32

/// CPA key-generation seed length in bytes (FIPS 203 §7).
let v_CPA_KEY_GENERATION_SEED_SIZE: usize = mk_usize 32

/// Rank-generic encoded-NTT-vector size: `rank * 384`.  Mirrors
/// `Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE` and
/// `Spec.MLKEM.v_RANKED_BYTES_PER_RING_ELEMENT`.
let tt_as_ntt_encoded_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = rank *! v_BYTES_PER_RING_ELEMENT

/// Synonym of `t_as_ntt_encoded_size` matching `Spec.MLKEM`\'s
/// `v_RANKED_BYTES_PER_RING_ELEMENT` naming.
let ranked_bytes_per_ring_element (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = rank *! v_BYTES_PER_RING_ELEMENT

/// CPA encryption-key size: `rank * 384 + 32`.
let cpa_public_key_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = (rank *! v_BYTES_PER_RING_ELEMENT <: usize) +! mk_usize 32

/// CPA decryption-key size: `rank * 384`.
let cpa_private_key_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = rank *! v_BYTES_PER_RING_ELEMENT

/// CCA decapsulation-key size: `cpa_private_key + cpa_public_key + H_DIGEST + z`.
let cca_private_key_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) =
  (((cpa_private_key_size rank <: usize) +! (cpa_public_key_size rank <: usize) <: usize) +!
    Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE
    <:
    usize) +!
  mk_usize 32

/// `du` compression factor for vector u: 10 for ranks 2,3 and 11 for rank 4.
let vector_u_compression_factor (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = if rank =. mk_usize 4 then mk_usize 11 else mk_usize 10

/// `dv` compression factor for ring element v: 4 for ranks 2,3 and 5 for rank 4.
let vector_v_compression_factor (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = if rank =. mk_usize 4 then mk_usize 5 else mk_usize 4

/// Per-block (per-ring-element) size of c1 in bytes: `(256 * du)/8`.
let c1_block_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) =
  (v_COEFFICIENTS_IN_RING_ELEMENT *! (vector_u_compression_factor rank <: usize) <: usize) /!
  mk_usize 8

/// Total c1 (encoded vector u) size in bytes: `rank * c1_block_size`.
let c1_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = rank *! (c1_block_size rank <: usize)

/// Total c2 (encoded ring element v) size in bytes: `(256 * dv)/8`.
let c2_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) =
  (v_COEFFICIENTS_IN_RING_ELEMENT *! (vector_v_compression_factor rank <: usize) <: usize) /!
  mk_usize 8

/// `eta1` CBD parameter: 3 for rank 2, 2 for ranks 3,4.
let eta1 (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = if rank =. mk_usize 2 then mk_usize 3 else mk_usize 2

/// `eta2` CBD parameter: always 2.
let eta2 (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) =
  let _:usize = rank in
  mk_usize 2

/// PRF output size for eta1 CBD sampling: `64 * eta1`.
let eta1_randomness_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = mk_usize 64 *! (eta1 rank <: usize)

/// PRF output size for eta2 CBD sampling: `64 * eta2`.
let eta2_randomness_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = mk_usize 64 *! (eta2 rank <: usize)

/// Implicit-rejection hash input length: `32 + cpa_ciphertext_size`.
let implicit_rejection_hash_input_size (rank: usize)
    : Prims.Pure usize
      (requires rank =. mk_usize 2 || rank =. mk_usize 3 || rank =. mk_usize 4)
      (fun _ -> Prims.l_True) = mk_usize 32 +! (cpa_ciphertext_size rank <: usize)

/// An ML-KEM field element:
/// - after reduction modulo FIELD_MODULUS, it is an integer in the range [0, FIELD_MODULUS - 1]
/// - it is represented as a u16
type t_FieldElement = { f_val:f_val: u16{b2t (f_val <. v_FIELD_MODULUS <: bool)} }

let impl_1: Core_models.Clone.t_Clone t_FieldElement =
  { f_clone = (fun x -> x); f_clone_pre = (fun _ -> True); f_clone_post = (fun _ _ -> True) }

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_2': Core_models.Marker.t_Copy t_FieldElement

unfold
let impl_2 = impl_2'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_3': Core_models.Marker.t_StructuralPartialEq t_FieldElement

unfold
let impl_3 = impl_3'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_4': Core_models.Cmp.t_PartialEq t_FieldElement t_FieldElement

unfold
let impl_4 = impl_4'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_5': Core_models.Cmp.t_Eq t_FieldElement

unfold
let impl_5 = impl_5'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_6': Core_models.Cmp.t_PartialOrd t_FieldElement t_FieldElement

unfold
let impl_6 = impl_6'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_7': Core_models.Cmp.t_Ord t_FieldElement

unfold
let impl_7 = impl_7'

[@@ FStar.Tactics.Typeclasses.tcinstance]
assume
val impl_8': Core_models.Fmt.t_Debug t_FieldElement

unfold
let impl_8 = impl_8'

let impl_FieldElement__new (v_val: u16)
    : Prims.Pure t_FieldElement (requires v_val <. v_FIELD_MODULUS) (fun _ -> Prims.l_True) =
  { f_val = v_val } <: t_FieldElement

/// Reduce an arbitrary `i16` (e.g. an impl-side coefficient or
/// Montgomery-domain value) into a canonical `FieldElement` in
/// [0, FIELD_MODULUS).  Used by the impl→spec lift functions
/// (`Libcrux_ml_kem.Vector.to_spec_*_t`) to bridge the trait-layer
/// `i16` representation to the spec-layer `FieldElement` form.
let impl_FieldElement__from_i16 (v: i16) : t_FieldElement =
  let q:i32 = cast (v_FIELD_MODULUS <: u16) <: i32 in
  let r:u16 = cast ((((cast (v <: i16) <: i32) %! q <: i32) +! q <: i32) %! q <: i32) <: u16 in
  impl_FieldElement__new r

/// Addition in ℤ/q.
let impl_FieldElement__add (self other: t_FieldElement) : t_FieldElement =
  impl_FieldElement__new (cast (((cast (self.f_val <: u16) <: u32) +!
            (cast (other.f_val <: u16) <: u32)
            <:
            u32) %!
          (cast (v_FIELD_MODULUS <: u16) <: u32)
          <:
          u32)
      <:
      u16)

/// Subtraction in ℤ/q.  Adding q avoids unsigned underflow before reducing.
let impl_FieldElement__sub (self other: t_FieldElement) : t_FieldElement =
  impl_FieldElement__new (cast ((((cast (self.f_val <: u16) <: u32) +!
              (cast (v_FIELD_MODULUS <: u16) <: u32)
              <:
              u32) -!
            (cast (other.f_val <: u16) <: u32)
            <:
            u32) %!
          (cast (v_FIELD_MODULUS <: u16) <: u32)
          <:
          u32)
      <:
      u16)

/// Multiplication in ℤ/q.
let impl_FieldElement__mul (self other: t_FieldElement) : t_FieldElement =
  impl_FieldElement__new (cast (((cast (self.f_val <: u16) <: u32) *!
            (cast (other.f_val <: u16) <: u32)
            <:
            u32) %!
          (cast (v_FIELD_MODULUS <: u16) <: u32)
          <:
          u32)
      <:
      u16)

/// Additive inverse in ℤ/q.  `a.neg()` = q − a   (0 when a = 0).
let impl_FieldElement__neg (self: t_FieldElement) : t_FieldElement =
  impl_FieldElement__new ((v_FIELD_MODULUS -! self.f_val <: u16) %! v_FIELD_MODULUS <: u16)

assume val createi
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
    : t_Array v_T v_N

assume val createi_lemma 
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
      (i: usize{i <. v_N})
     : Lemma (Seq.index (createi #v_T v_N #v_F f) (v i) == f i)
       [SMTPat (Seq.index (createi #v_T v_N #v_F f) (v i))]
