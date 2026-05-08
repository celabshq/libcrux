pub const MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS: i16 = 1353;
pub const FIELD_MODULUS: i16 = 3329;
pub const FIELD_ELEMENTS_IN_VECTOR: usize = 16;
pub const INVERSE_OF_MODULUS_MOD_MONTGOMERY_R: u32 = 62209; // FIELD_MODULUS^{-1} mod MONTGOMERY_R
pub const BARRETT_SHIFT: i32 = 26;
pub const BARRETT_R: i32 = 1 << BARRETT_SHIFT;

#[cfg(hax)]
use hax_lib::prop::ToProp;

// We define a trait that allows us to talk about the contents of a vector.
// This is used extensively in pre- and post-conditions to reason about the code.
// The trait is duplicated for Eurydice to avoid the trait inheritance between Operations and Repr
// This is needed because of this issue: https://github.com/AeneasVerif/eurydice/issues/111
#[cfg(hax)]
#[hax_lib::attributes]
pub trait Repr: Copy + Clone {
    #[requires(true)]
    fn repr(&self) -> [i16; 16];
}

#[cfg(any(eurydice, not(hax)))]
pub trait Repr {}

#[cfg(hax)]
#[allow(dead_code, unused_variables)]
pub(crate) mod spec {

    // Spec-side infrastructure injected into the generated F* module
    // `Libcrux_ml_kem.Vector.Traits.Spec`:
    //
    //   * Generic integer-bound predicates (is_intb / is_i16b /
    //     is_i16b_array / is_i16b_array_opaque / map_array), copied
    //     verbatim from proofs/fstar/spec/Spec.Utils.fsti so that, with
    //     opaque_to_smt unfolded, the predicate content is identical.
    //
    //   * Two i16 -> t_FieldElement lifts, distinguished by whether the
    //     impl i16 is a *plain* representative mod q, or a *Montgomery*
    //     form representing abstract value `(v x * R^{-1}) mod q` with
    //     R = 2^16 and R^{-1} = 169 mod 3329:
    //       - i16_to_spec_fe x      := (v x) mod q
    //       - mont_i16_to_spec_fe x := (v x * 169) mod q
    //     NTT, inverse-NTT, and ntt_multiply live in Montgomery land;
    //     compress/decompress/serialize/deserialize/barrett live in
    //     plain land.  `add`/`sub` are linear and work in either.
    //
    //   * Array companions i16_to_spec_array / mont_i16_to_spec_array
    //     and the small helper builders zetas_{1,2,4} for the trait's
    //     explicit-zeta NTT steps.  Zetas in the impl are stored in
    //     Montgomery form (`zeta * R mod q`); the builders use
    //     `mont_i16_to_spec_fe` so the resulting slice holds plain
    //     abstract zetas that hacspec's ntt_layer_n expects.

    use hacspec_ml_kem::parameters::{createi, FieldElement};

    /// Plain-domain lift: x is a field representative mod q.
    /// Maps any i16 to its canonical FieldElement under Euclidean
    /// reduction.  Used by trait posts (`compress_post_N`,
    /// `decompress_post_N`, ...) and by the polynomial-level lift
    /// `poly_to_spec` in vector.rs.
    ///
    /// The `ensures` is the F*-side refinement bridges depend on
    /// (`v r.f_val == v x % 3329`).  Without it, Z3 cannot compute
    /// the field-element value from a `i16_to_spec_fe x` term, and
    /// the FE-arithmetic commute lemmas in
    /// `Hacspec_ml_kem.Commute.Chunk` (e.g.
    /// `lemma_base_case_mult_even_fe_commute`) time out.
    ///
    /// Pure-Rust form via `hax_lib::int::ToInt` + `Int::rem_euclid`
    /// was attempted but extracts to `Hax_lib.Int.t_Int` /
    /// `Hax_lib.Int.impl_Int__rem_euclid` — Z3 does not auto-bridge
    /// that to the F* native `v x % 3329` form the bridges' SMT
    /// queries are written against, so even simple bridges like
    /// `lemma_add_fe_commute_mont` (proof was `()`) failed.  Keeping
    /// the `fstar!` escape until the bridges are rewritten against
    /// the `Hax_lib.Int` form (out of scope for this cleanup).
    #[hax_lib::ensures(|r| fstar!(r#"v ${r}.Hacspec_ml_kem.Parameters.f_val == v ${x} % 3329"#))]
    pub fn i16_to_spec_fe(x: i16) -> FieldElement {
        FieldElement::from_i16(x)
    }

    /// Montgomery-domain lift: x stores `v_abs * R mod q` with R = 2^16,
    /// R^{-1} = 169 mod 3329.  Strips the R factor to recover the
    /// abstract value.  Used in NTT / inverse-NTT / ntt_multiply trait
    /// posts and in `Hacspec_ml_kem.Commute.{Bridges, Chunk}`.
    ///
    /// `ensures` matches the prior F*-injection refinement:
    /// `v r.f_val == (v x * 169) % 3329`.  Same `fstar!` rationale
    /// as `i16_to_spec_fe` above.
    #[hax_lib::ensures(|r| fstar!(r#"v ${r}.Hacspec_ml_kem.Parameters.f_val == (v ${x} * 169) % 3329"#))]
    pub fn mont_i16_to_spec_fe(x: i16) -> FieldElement {
        let q: i32 = 3329;
        let r = ((((x as i32) * 169) % q + q) % q) as u16;
        FieldElement::new(r)
    }

    /// Pointwise plain-domain lift over an i16 array of any length.
    pub fn i16_to_spec_array<const N: usize>(x: &[i16; N]) -> [FieldElement; N] {
        createi(|i| i16_to_spec_fe(x[i]))
    }

    /// Pointwise Montgomery-domain lift over an i16 array of any length.
    pub fn mont_i16_to_spec_array<const N: usize>(x: &[i16; N]) -> [FieldElement; N] {
        createi(|i| mont_i16_to_spec_fe(x[i]))
    }

    /// Build a 1-element zeta slice from a single Montgomery-form
    /// i16 zeta, holding the abstract plain zeta that hacspec expects.
    pub fn zetas_1(z0: i16) -> [FieldElement; 1] {
        createi(|_| mont_i16_to_spec_fe(z0))
    }

    /// Build a 2-element zeta slice from two Montgomery-form i16 zetas.
    pub fn zetas_2(z0: i16, z1: i16) -> [FieldElement; 2] {
        createi(|i| {
            if i == 0 {
                mont_i16_to_spec_fe(z0)
            } else {
                mont_i16_to_spec_fe(z1)
            }
        })
    }

    /// Build a 4-element zeta slice from four Montgomery-form i16 zetas.
    pub fn zetas_4(z0: i16, z1: i16, z2: i16, z3: i16) -> [FieldElement; 4] {
        createi(|i| {
            if i == 0 {
                mont_i16_to_spec_fe(z0)
            } else if i == 1 {
                mont_i16_to_spec_fe(z1)
            } else if i == 2 {
                mont_i16_to_spec_fe(z2)
            } else {
                mont_i16_to_spec_fe(z3)
            }
        })
    }

    #[cfg_attr(
        hax,
        hax_lib::fstar::before(
            r#"
let is_intb (l:nat) (x:int) : bool = (x <= l) && (x >= -l)
let is_i16b (l:nat) (x:i16) : bool = is_intb l (v x)
let is_i16b_array (l:nat) (x:t_Slice i16) : prop =
    forall i. i < Seq.length x ==> is_i16b l (Seq.index x i)
[@@ "opaque_to_smt"]
let is_i16b_array_opaque (l:nat) (x:t_Slice i16) : prop =
    is_i16b_array l x

(* Generic interval bound on each lane of an i16 array.  Marked
   opaque so that typeclass instances (e.g. impl_1 : t_Operations
   PortableVector) do not drag the inner `forall` into per-method
   VCs.  Reveal at consumers that need the pointwise expansion.

   The two thin wrappers below (`bounded_abs_i16_array`,
   `bounded_pos_i16_array`) specialise to the two patterns that
   show up in trait posts:

     * symmetric absolute bound:  [-l, l]      (NTT, barrett etc.)
     * unsigned bit-width bound:  [0, 2^d)     (compress, serialize)

   They are kept transparent so the typeclass record sees them as
   `bounded_i16_array (mk_i16 ...) (mk_i16 ...) x` — still opaque
   to SMT, since the *base* predicate is opaque. *)
[@@ "opaque_to_smt"]
let bounded_i16_array (lo hi: i16) (x: t_Slice i16) : prop =
    forall (i: nat). i < Seq.length x ==>
        v lo <= v (Seq.index x i) /\ v (Seq.index x i) <= v hi

(* SMTPat-fired per-index unfolding (consume direction).  The dual trigger
   `(Seq.index x i, bounded_i16_array lo hi x)` only fires when Z3 actually
   needs the bound on a specific lane `x.[i]` AND has `bounded_i16_array`
   available — instead of dumping all 16 lane bounds whenever the predicate
   appears. *)
let lemma_bounded_i16_array_lookup (lo hi: i16) (x: t_Slice i16) (i: nat)
    : Lemma (requires bounded_i16_array lo hi x /\ i < Seq.length x)
            (ensures v lo <= v (Seq.index x i) /\ v (Seq.index x i) <= v hi)
            [SMTPat (Seq.index x i); SMTPat (bounded_i16_array lo hi x)] =
  reveal_opaque (`%bounded_i16_array) (bounded_i16_array lo hi x)

(* Reverse direction (intro): construct `bounded_i16_array lo hi x` from a
   per-lane bound.  No SMTPat — call explicitly at the call sites where
   needed.  A global SMTPat here would over-instantiate (every appearance
   of `bounded_i16_array` would force Z3 to try to derive it from a forall). *)
let lemma_bounded_i16_array_intro (lo hi: i16) (x: t_Slice i16)
    : Lemma (requires forall (i: nat). i < Seq.length x ==>
                       v lo <= v (Seq.index x i) /\ v (Seq.index x i) <= v hi)
            (ensures bounded_i16_array lo hi x) =
  reveal_opaque (`%bounded_i16_array) (bounded_i16_array lo hi x)

let bounded_abs_i16_array (l: nat {l < pow2 15}) (x: t_Slice i16) : prop =
    bounded_i16_array (mk_i16 (- l)) (mk_i16 l) x

let bounded_pos_i16_array (d: nat {d < 15}) (x: t_Slice i16) : prop =
    bounded_i16_array (mk_i16 0) (mk_i16 (pow2 d - 1)) x

(* `map_array` (used below by `bitwise_and_with_constant_constant_post`
   and `shift_right_post`) lives in
   `Hacspec_ml_kem.Commute.ProofUtils` as a generic helper marked for
   hax-lib upstream.  Brought into scope so the trait-post specs can
   cite it unqualified. *)
open Hacspec_ml_kem.Commute.ProofUtils

(* Hacspec equations stated elementwise over arrays of any length n.
   The trait instantiates them with n=16; the polynomial layer uses
   n=256.  Each predicate bakes in whatever pre-conditions hacspec's
   underlying primitive demands (e.g. compress_d requires d < 12).
   `opaque_to_smt` so that the trait class definition does not unfold
   the hacspec compress_d/decompress_d during typechecking of unrelated
   methods (otherwise impl_1's per-method VCs slow down — measured
   at ntt_layer_1_step's `assert (p_layer_1 3)` exceeding rlimit 400). *)

[@@ "opaque_to_smt"]
let compress_post_N (#n: usize) (d: usize{v d < 12})
    (input result: t_Array i16 n) : prop =
  forall (i: nat). i < v n ==>
    i16_to_spec_fe (Seq.index result i) ==
    Hacspec_ml_kem.Compress.compress_d (i16_to_spec_fe (Seq.index input i)) d

(* decompress_d (hacspec) has precondition
     to_bit_size < 12 /\ fe.f_val < 1 << to_bit_size.
   The first part is in our refinement on d; the second must be
   supplied by the caller's pre.  We use implication so the predicate
   is callable without an input refinement. *)
[@@ "opaque_to_smt"]
let decompress_post_N (#n: usize) (d: usize{v d < 12})
    (input result: t_Array i16 n) : prop =
  (forall (i: nat). i < v n ==>
     v (Seq.index input i) >= 0 /\ v (Seq.index input i) < pow2 (v d)) ==>
  (forall (i: nat). i < v n ==>
    i16_to_spec_fe (Seq.index result i) ==
    Hacspec_ml_kem.Compress.decompress_d (i16_to_spec_fe (Seq.index input i)) d)

(* Bit-packing pre/post.  An i16 array of n elements where each
   element's low d bits participate is packed into n8 bytes iff
   n * d == n8 * 8.  Stated via the BitVecEq helper — same path
   serialize_post_N used at n=16, now generic. *)
let serialize_pre_N (#n: usize)
    (d: nat{d > 0 /\ d <= 12})
    (input: t_Array i16 n) : prop =
  forall (i: nat). i < v n ==> Rust_primitives.BitVectors.bounded (Seq.index input i) d

let serialize_post_N (#n: usize)
    (d: nat{d > 0 /\ d <= 12})
    (input: t_Array i16 n {serialize_pre_N d input})
    (output: t_Slice u8 {Seq.length output * 8 == v n * d}) : prop =
  let n8 : usize = sz (Seq.length output) in
  let output_arr : t_Array u8 n8 = output in
  BitVecEq.int_t_array_bitwise_eq input d output_arr 8

let deserialize_post_N (#n: usize)
    (d: nat{d > 0 /\ d <= 12})
    (input: t_Slice u8 {Seq.length input * 8 == v n * d})
    (output: t_Array i16 n) : prop =
  let n8 : usize = sz (Seq.length input) in
  let input_arr : t_Array u8 n8 = input in
  BitVecEq.int_t_array_bitwise_eq input_arr 8 output d /\
  (forall (i: nat). i < v n ==> Rust_primitives.BitVectors.bounded (Seq.index output i) d)

(* Raw 16-bit LE byte load / store.  At n=16 lanes / 32 bytes, these
   are the trait's from_bytes / to_bytes.  Same BitVecEq path again,
   d=16 on the i16 side, d=8 on the u8 side.  Same slice-with-length
   coercion pattern as serialize_post_N. *)
let from_le_bytes_post_N (#n: usize)
    (input: t_Slice u8 {Seq.length input * 8 == v n * 16})
    (output: t_Array i16 n) : prop =
  let n2 : usize = sz (Seq.length input) in
  let input_arr : t_Array u8 n2 = input in
  BitVecEq.int_t_array_bitwise_eq input_arr 8 output 16

let to_le_bytes_post_N (#n: usize)
    (input: t_Array i16 n)
    (output: t_Slice u8 {Seq.length output * 8 == v n * 16}) : prop =
  let n2 : usize = sz (Seq.length output) in
  let output_arr : t_Array u8 n2 = output in
  BitVecEq.int_t_array_bitwise_eq input 16 output_arr 8

(* =====================================================================
   Per-lane / per-branch opaque predicates for the FE-form trait posts.

   The trait operates on 16-element vectors; modules above the trait
   (polynomial, ntt, invert_ntt, serialize, matrix, sampling) operate on
   256-element ring elements (16 vectors of 16 elements each) by *iterating*
   over vectors and chaining per-vector posts in loop invariants.

   To make those iterations efficient — and free of hacspec / FE-algebra
   internal details — we keep the iteration structure (`Spec.Utils.forall16`
   for compress/decompress, `Spec.Utils.forall4` for NTT-layer/butterfly)
   transparent at the trait surface, and wrap *only the per-lane (or
   per-branch) function* in an `[@@ "opaque_to_smt"]` predicate.

   Effect for callers above the trait:
     * They see `forall16 (fun i -> compress_1_lane_post v.[i] r.[i])` →
       a 16-conjunction over atomic facts.
     * Each conjunct is opaque — the hacspec `compress_d` / `decompress_d`
       body and the FE-algebra `add` / `sub` / `mul` body never unfold
       into the caller's SMT context.
     * Callers chain the atoms across loop iterations without paying any
       hacspec-internal SMT cost.

   Effect for impl methods (portable / AVX2):
     * Add one `reveal_opaque` per opaque predicate inside the impl proof
       body so the existing aux-lemma chain (which establishes the body
       facts pointwise) discharges the post.

   Bridge sites (Hacspec_ml_kem.Commute.Chunk lemmas) reveal when they
   genuinely need the underlying hacspec equation. *)

[@@ "opaque_to_smt"]
let compress_1_lane_post (input result: i16) : prop =
  i16_to_spec_fe result ==
  Hacspec_ml_kem.Compress.compress_d (i16_to_spec_fe input) (mk_usize 1)

[@@ "opaque_to_smt"]
let compress_d_lane_post
    (d: usize {v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11})
    (input result: i16) : prop =
  i16_to_spec_fe result ==
  Hacspec_ml_kem.Compress.compress_d (i16_to_spec_fe input) d

[@@ "opaque_to_smt"]
let decompress_1_lane_post (input result: i16) : prop =
  (v input >= 0 /\ v input < 2) ==>
  i16_to_spec_fe result ==
  Hacspec_ml_kem.Compress.decompress_d (i16_to_spec_fe input) (mk_usize 1)

[@@ "opaque_to_smt"]
let decompress_d_lane_post
    (d: usize {v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11})
    (input result: i16) : prop =
  (v input >= 0 /\ v input < pow2 (v d)) ==>
  i16_to_spec_fe result ==
  Hacspec_ml_kem.Compress.decompress_d (i16_to_spec_fe input) d

(* Per-lane atoms for L7 (`barrett_reduce_post`) and L8
   (`montgomery_multiply_by_constant_post`).  Both wrap the underlying
   `Hacspec_ml_kem.ModQ.mod_q_eq` invocation so that callers above the
   trait never see the raw `(v _) % 3329` form (which would expose
   non-linear arithmetic — `v vec[i] * v c * 169` for L8 — to Z3's NRA
   decision procedure at every consumer that mentions the post).  The
   `forall16` wrapper at each trait post stays transparent so callers
   iterate per-lane as opaque atoms.

   Reveal sites:
     * impl methods (`Vector.{Avx2,Portable}`) — discharge per-lane
       `mod_q_eq` first, then `reveal_opaque` to fold into the atom.
     * `Hacspec_ml_kem.Commute.Chunk` lemmas — reveal when bridging the
       trait post to per-lane FE equations. *)

[@@ "opaque_to_smt"]
let barrett_reduce_lane_post (vec_i result_i: i16) : prop =
  Hacspec_ml_kem.ModQ.mod_q_eq (v result_i) (v vec_i)

[@@ "opaque_to_smt"]
let montgomery_multiply_lane_post (vec_i c result_i: i16) : prop =
  Hacspec_ml_kem.ModQ.mod_q_eq (v result_i) (v vec_i * v c * 169)

(* Per-branch FE-butterfly predicates for NTT / inverse-NTT layers and
   ntt_multiply.  Each branch covers the 4 lane equations sharing a single
   zeta (or no zeta, for layer 3).  Marked opaque so the FE-algebra
   `impl_FieldElement__{add,sub,mul}` invocations do not unfold into
   caller SMT context.  The `forall4` wrapper at each trait post stays
   transparent — callers iterate per-branch as opaque atoms. *)

[@@ "opaque_to_smt"]
let ntt_layer_1_step_branch_post
    (b: nat{b < 4})
    (input: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
    (result: t_Array i16 (mk_usize 16)) : prop =
  let z = (if b = 0 then zeta0
           else if b = 1 then zeta1
           else if b = 2 then zeta2
           else zeta3) in
  let i1 : nat = 4 * b in
  let j1 : nat = 4 * b + 2 in
  let i2 : nat = 4 * b + 1 in
  let j2 : nat = 4 * b + 3 in
  mont_i16_to_spec_fe (Seq.index result i1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i1))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe z)
        (mont_i16_to_spec_fe (Seq.index input j1))) /\
  mont_i16_to_spec_fe (Seq.index result j1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__sub
      (mont_i16_to_spec_fe (Seq.index input i1))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe z)
        (mont_i16_to_spec_fe (Seq.index input j1))) /\
  mont_i16_to_spec_fe (Seq.index result i2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i2))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe z)
        (mont_i16_to_spec_fe (Seq.index input j2))) /\
  mont_i16_to_spec_fe (Seq.index result j2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__sub
      (mont_i16_to_spec_fe (Seq.index input i2))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe z)
        (mont_i16_to_spec_fe (Seq.index input j2)))

[@@ "opaque_to_smt"]
let ntt_layer_2_step_branch_post
    (b: nat{b < 4})
    (input: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16)
    (result: t_Array i16 (mk_usize 16)) : prop =
  let z = (if b < 2 then zeta0 else zeta1) in
  let base : nat = if b < 2 then 0 else 8 in
  let off  : nat = if b = 0 || b = 2 then 0 else 2 in
  let i1 : nat = base + off in
  let j1 : nat = i1 + 4 in
  let i2 : nat = i1 + 1 in
  let j2 : nat = j1 + 1 in
  mont_i16_to_spec_fe (Seq.index result i1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i1))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe z)
        (mont_i16_to_spec_fe (Seq.index input j1))) /\
  mont_i16_to_spec_fe (Seq.index result j1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__sub
      (mont_i16_to_spec_fe (Seq.index input i1))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe z)
        (mont_i16_to_spec_fe (Seq.index input j1))) /\
  mont_i16_to_spec_fe (Seq.index result i2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i2))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe z)
        (mont_i16_to_spec_fe (Seq.index input j2))) /\
  mont_i16_to_spec_fe (Seq.index result j2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__sub
      (mont_i16_to_spec_fe (Seq.index input i2))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe z)
        (mont_i16_to_spec_fe (Seq.index input j2)))

[@@ "opaque_to_smt"]
let ntt_layer_3_step_branch_post
    (b: nat{b < 4})
    (input: t_Array i16 (mk_usize 16))
    (zeta0: i16)
    (result: t_Array i16 (mk_usize 16)) : prop =
  let i1 : nat = 2 * b in
  let j1 : nat = 2 * b + 8 in
  let i2 : nat = 2 * b + 1 in
  let j2 : nat = 2 * b + 9 in
  mont_i16_to_spec_fe (Seq.index result i1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i1))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe zeta0)
        (mont_i16_to_spec_fe (Seq.index input j1))) /\
  mont_i16_to_spec_fe (Seq.index result j1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__sub
      (mont_i16_to_spec_fe (Seq.index input i1))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe zeta0)
        (mont_i16_to_spec_fe (Seq.index input j1))) /\
  mont_i16_to_spec_fe (Seq.index result i2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i2))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe zeta0)
        (mont_i16_to_spec_fe (Seq.index input j2))) /\
  mont_i16_to_spec_fe (Seq.index result j2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__sub
      (mont_i16_to_spec_fe (Seq.index input i2))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe zeta0)
        (mont_i16_to_spec_fe (Seq.index input j2)))

[@@ "opaque_to_smt"]
let inv_ntt_layer_1_step_branch_post
    (b: nat{b < 4})
    (input: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
    (result: t_Array i16 (mk_usize 16)) : prop =
  let z = (if b = 0 then zeta0
           else if b = 1 then zeta1
           else if b = 2 then zeta2
           else zeta3) in
  let i1 : nat = 4 * b in
  let j1 : nat = 4 * b + 2 in
  let i2 : nat = 4 * b + 1 in
  let j2 : nat = 4 * b + 3 in
  mont_i16_to_spec_fe (Seq.index result i1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i1))
      (mont_i16_to_spec_fe (Seq.index input j1)) /\
  mont_i16_to_spec_fe (Seq.index result j1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__mul
      (mont_i16_to_spec_fe z)
      (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
        (mont_i16_to_spec_fe (Seq.index input j1))
        (mont_i16_to_spec_fe (Seq.index input i1))) /\
  mont_i16_to_spec_fe (Seq.index result i2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i2))
      (mont_i16_to_spec_fe (Seq.index input j2)) /\
  mont_i16_to_spec_fe (Seq.index result j2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__mul
      (mont_i16_to_spec_fe z)
      (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
        (mont_i16_to_spec_fe (Seq.index input j2))
        (mont_i16_to_spec_fe (Seq.index input i2)))

[@@ "opaque_to_smt"]
let inv_ntt_layer_2_step_branch_post
    (b: nat{b < 4})
    (input: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16)
    (result: t_Array i16 (mk_usize 16)) : prop =
  let z = (if b < 2 then zeta0 else zeta1) in
  let base : nat = if b < 2 then 0 else 8 in
  let off  : nat = if b = 0 || b = 2 then 0 else 2 in
  let i1 : nat = base + off in
  let j1 : nat = i1 + 4 in
  let i2 : nat = i1 + 1 in
  let j2 : nat = j1 + 1 in
  mont_i16_to_spec_fe (Seq.index result i1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i1))
      (mont_i16_to_spec_fe (Seq.index input j1)) /\
  mont_i16_to_spec_fe (Seq.index result j1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__mul
      (mont_i16_to_spec_fe z)
      (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
        (mont_i16_to_spec_fe (Seq.index input j1))
        (mont_i16_to_spec_fe (Seq.index input i1))) /\
  mont_i16_to_spec_fe (Seq.index result i2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i2))
      (mont_i16_to_spec_fe (Seq.index input j2)) /\
  mont_i16_to_spec_fe (Seq.index result j2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__mul
      (mont_i16_to_spec_fe z)
      (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
        (mont_i16_to_spec_fe (Seq.index input j2))
        (mont_i16_to_spec_fe (Seq.index input i2)))

[@@ "opaque_to_smt"]
let inv_ntt_layer_3_step_branch_post
    (b: nat{b < 4})
    (input: t_Array i16 (mk_usize 16))
    (zeta0: i16)
    (result: t_Array i16 (mk_usize 16)) : prop =
  let i1 : nat = 2 * b in
  let j1 : nat = 2 * b + 8 in
  let i2 : nat = 2 * b + 1 in
  let j2 : nat = 2 * b + 9 in
  mont_i16_to_spec_fe (Seq.index result i1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i1))
      (mont_i16_to_spec_fe (Seq.index input j1)) /\
  mont_i16_to_spec_fe (Seq.index result j1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__mul
      (mont_i16_to_spec_fe zeta0)
      (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
        (mont_i16_to_spec_fe (Seq.index input j1))
        (mont_i16_to_spec_fe (Seq.index input i1))) /\
  mont_i16_to_spec_fe (Seq.index result i2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (mont_i16_to_spec_fe (Seq.index input i2))
      (mont_i16_to_spec_fe (Seq.index input j2)) /\
  mont_i16_to_spec_fe (Seq.index result j2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__mul
      (mont_i16_to_spec_fe zeta0)
      (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
        (mont_i16_to_spec_fe (Seq.index input j2))
        (mont_i16_to_spec_fe (Seq.index input i2)))

[@@ "opaque_to_smt"]
let ntt_multiply_branch_post
    (b: nat{b < 4})
    (lhs rhs: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
    (result: t_Array i16 (mk_usize 16)) : prop =
  let zp = (if b = 0 then zeta0
            else if b = 1 then zeta1
            else if b = 2 then zeta2
            else zeta3) in
  let k_even : nat = 2 * b in
  let lane0 : nat = 2 * k_even in
  let lane1 : nat = 2 * k_even + 1 in
  let k_odd  : nat = 2 * b + 1 in
  let lane2 : nat = 2 * k_odd in
  let lane3 : nat = 2 * k_odd + 1 in
  mont_i16_to_spec_fe (Seq.index result lane0) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe (Seq.index lhs lane0))
        (mont_i16_to_spec_fe (Seq.index rhs lane0)))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
          (mont_i16_to_spec_fe (Seq.index lhs lane1))
          (mont_i16_to_spec_fe (Seq.index rhs lane1)))
        (mont_i16_to_spec_fe zp)) /\
  mont_i16_to_spec_fe (Seq.index result lane1) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe (Seq.index lhs lane0))
        (mont_i16_to_spec_fe (Seq.index rhs lane1)))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe (Seq.index lhs lane1))
        (mont_i16_to_spec_fe (Seq.index rhs lane0))) /\
  mont_i16_to_spec_fe (Seq.index result lane2) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe (Seq.index lhs lane2))
        (mont_i16_to_spec_fe (Seq.index rhs lane2)))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
          (mont_i16_to_spec_fe (Seq.index lhs lane3))
          (mont_i16_to_spec_fe (Seq.index rhs lane3)))
        (mont_i16_to_spec_fe (Spec.Utils.neg_i16 zp))) /\
  mont_i16_to_spec_fe (Seq.index result lane3) ==
    Hacspec_ml_kem.Parameters.impl_FieldElement__add
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe (Seq.index lhs lane2))
        (mont_i16_to_spec_fe (Seq.index rhs lane3)))
      (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (mont_i16_to_spec_fe (Seq.index lhs lane3))
        (mont_i16_to_spec_fe (Seq.index rhs lane2)))
"#
        )
    )]
    // Pre is stated on the *sum* `v lhs[i] + v rhs[i]`, not on lhs / rhs
    // separately.  Callers pass typed-bounded vectors (e.g.
    // `is_i16b_array_opaque L _`) and the sum-bound holds whenever
    // `2*L <= pow2 15 - 1`.  Stating the pre on the sum keeps the
    // trait-level pre symmetric with `add_post`'s sum-form equation
    // and avoids forcing every caller to re-prove the elementwise sum
    // bound (it is what they already establish to invoke `add`).
    pub(crate) fn add_pre(lhs: &[i16; 16], rhs: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i.
            is_intb (pow2 15 - 1)
                (v (Seq.index ${lhs} i) + v (Seq.index ${rhs} i))"#
        )
    }

    // Equation and output-bound bundled under a single `forall` so Z3
    // fires both facts on the same instantiation — splitting them into
    // two foralls triggers "incomplete quantifiers" in the typeclass-
    // implication check at the impl-side wrapper site.  The bound
    // `is_intb (pow2 15 - 1) (v result[i])` is derivable from the
    // equation + `add_pre` (which states the sum fits in i16); making
    // it explicit lets callers cite the i16-bound on result without
    // re-substituting through the equation.
    pub(crate) fn add_post(lhs: &[i16; 16], rhs: &[i16; 16], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i.
                v (Seq.index ${result} i) ==
                v (Seq.index ${lhs} i) + v (Seq.index ${rhs} i) /\
                is_intb (pow2 15 - 1) (v (Seq.index ${result} i))"#
        )
    }

    // Sum-form pre (cf. `add_pre`): every caller establishes the
    // elementwise difference bound to invoke `sub`.
    pub(crate) fn sub_pre(lhs: &[i16; 16], rhs: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i.
                is_intb (pow2 15 - 1)
                (v (Seq.index ${lhs} i) - v (Seq.index ${rhs} i))"#
        )
    }

    pub(crate) fn sub_post(lhs: &[i16; 16], rhs: &[i16; 16], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i.
                v (Seq.index ${result} i) ==
                v (Seq.index ${lhs} i) - v (Seq.index ${rhs} i) /\
                is_intb (pow2 15 - 1) (v (Seq.index ${result} i))"#
        )
    }

    pub(crate) fn negate_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i.
                is_intb (pow2 15 - 1) (v (Seq.index ${vec} i))"#
        )
    }

    pub(crate) fn negate_post(vec: &[i16; 16], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i.
                v (Seq.index ${result} i) ==
                - (v (Seq.index ${vec} i)) /\
                is_intb (pow2 15 - 1) (v (Seq.index ${result} i))"#
        )
    }

    pub(crate) fn multiply_by_constant_pre(vec: &[i16; 16], c: i16) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i.
                is_intb (pow2 15 - 1) (v (Seq.index ${vec} i) * v $c)"#
        )
    }

    pub(crate) fn multiply_by_constant_post(
        vec: &[i16; 16],
        c: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i.
                v (Seq.index ${result} i) ==
                v (Seq.index ${vec} i) * v $c /\
                is_intb (pow2 15 - 1) (v (Seq.index ${result} i))"#
        )
    }

    pub(crate) fn bitwise_and_with_constant_constant_post(
        vec: &[i16; 16],
        c: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"$result == 
               map_array (fun x -> x &. $c) $vec"#
        )
    }

    pub(crate) fn shift_right_post(
        vec: &[i16; 16],
        shift_by: i32,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"(v $shift_by >= 0 /\ v $shift_by < 16) ==>
                $result == 
                map_array (fun x -> x >>! ${shift_by}) $vec"#
        )
    }

    pub(crate) fn cond_subtract_3329_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(r#"is_i16b_array_opaque (pow2 12 - 1) $vec"#)
    }

    pub(crate) fn cond_subtract_3329_post(vec: &[i16; 16], result: &[i16; 16]) -> hax_lib::Prop {
        // Stated pointwise rather than as a `map_array` equation: every
        // lane either stays put or drops by q, preserving the residue
        // class mod q.  This lets Layer-1 commutativity proofs cite the
        // residue equation directly (the same shape barrett_reduce uses).
        // Mod-q residue is wrapped in the opaque `mod_q_eq` to keep raw
        // `% 3329` from leaking above the trait — see Hacspec_ml_kem.ModQ.
        hax_lib::fstar_prop_expr!(
            r#"forall i.
                let x = Seq.index $vec i in
                let y = Seq.index $result i in
                ((v y == v x - 3329 \/ v y == v x) /\
                 Hacspec_ml_kem.ModQ.mod_q_eq (v y) (v x))"#
        )
    }

    pub(crate) fn barrett_reduce_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(r#"is_i16b_array_opaque 28296 $vec"#)
    }

    pub(crate) fn barrett_reduce_post(vec: &[i16; 16], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"is_i16b_array_opaque 3328 ${result} /\
               Spec.Utils.forall16 (fun (i: nat{i < 16}) ->
                 barrett_reduce_lane_post (Seq.index ${vec} i) (Seq.index ${result} i))"#
        )
    }

    // Pre is asymmetric — only `c` is bounded (`is_i16b 1664`), `vec` is
    // unconstrained.  This matches the implementation: the inner product
    // `vec[i] * c` always fits in i32 because `|c| < 2^11` and `|vec[i]|
    // <= pow2 15 - 1`, so we don't need a vec-side bound to rule out
    // overflow.  Don't tighten the pre — it would force callers to
    // establish a redundant `is_i16b_array` they currently don't need.
    pub(crate) fn montgomery_multiply_by_constant_pre(vec: &[i16; 16], c: i16) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(r#"is_i16b 1664 c"#)
    }

    pub(crate) fn montgomery_multiply_by_constant_post(
        vec: &[i16; 16],
        c: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"is_i16b_array_opaque 3328 ${result} /\
               Spec.Utils.forall16 (fun (i: nat{i < 16}) ->
                 montgomery_multiply_lane_post (Seq.index ${vec} i) ${c} (Seq.index ${result} i))"#
        )
    }

    pub(crate) fn to_unsigned_representative_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(r#"is_i16b_array_opaque 3328 ${vec}"#)
    }

    // The post is intentionally stated in *algebraic-int* form
    // (`v y >= 0 /\ v y <= 3328` plus `mod_q_eq (v y) (v x)`), not as a
    // direct hacspec `to_standard_domain` citation.  Reason: callers
    // (compress chain, the message decode path) consume the result via
    // residue equations, not via hacspec's t_FieldElement.  Lifting this
    // post to a hacspec equation would force every caller to unfold the
    // mod-q reasoning that `mod_q_eq` exists specifically to hide.
    // Bridge to hacspec where genuinely needed via Hacspec_ml_kem.ModQ
    // unfold lemmas inside Commute.* lemmas only (per SD1).
    pub(crate) fn to_unsigned_representative_post(
        vec: &[i16; 16],
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i.
                let x = Seq.index ${vec} i in
                let y = Seq.index ${result} i in
                (v y >= 0 /\ v y <= 3328 /\ Hacspec_ml_kem.ModQ.mod_q_eq (v y) (v x))"#
        )
    }

    pub(crate) fn compress_1_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"forall i. 
                v (Seq.index ${vec} i) >= 0 /\
                v (Seq.index ${vec} i) < 3329"#
        )
    }

    pub(crate) fn compress_1_post(vec: &[i16; 16], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"bounded_pos_i16_array 1 ${result} /\
               Spec.Utils.forall16 (fun (i: nat{i < 16}) ->
                 compress_1_lane_post (Seq.index ${vec} i) (Seq.index ${result} i))"#
        )
    }

    pub(crate) fn compress_pre(vec: &[i16; 16], coefficient_bits: i32) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"(v $coefficient_bits == 4 \/
                v $coefficient_bits == 5 \/
                v $coefficient_bits == 10 \/
                v $coefficient_bits == 11) /\
                bounded_i16_array (mk_i16 0) (mk_i16 3328) ${vec}"#
        )
    }

    pub(crate) fn compress_post(
        vec: &[i16; 16],
        coefficient_bits: i32,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"(v $coefficient_bits == 4 \/
                v $coefficient_bits == 5 \/
                v $coefficient_bits == 10 \/
                v $coefficient_bits == 11) ==>
                (bounded_pos_i16_array (v $coefficient_bits) ${result} /\
                 Spec.Utils.forall16 (fun (i: nat{i < 16}) ->
                   compress_d_lane_post (mk_usize (v $coefficient_bits))
                     (Seq.index ${vec} i) (Seq.index ${result} i)))"#
        )
    }

    pub(crate) fn decompress_1_post(vec: &[i16; 16], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"Spec.Utils.forall16 (fun (i: nat{i < 16}) ->
                 decompress_1_lane_post (Seq.index ${vec} i) (Seq.index ${result} i))"#
        )
    }

    pub(crate) fn decompress_ciphertext_coefficient_post(
        vec: &[i16; 16],
        coefficient_bits: i32,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        // Strengthened 2026-05-02: in addition to the FE-level spec equality,
        // expose the i16 result bound `[0, FIELD_MODULUS - 1] = [0, 3328]`
        // that all impls naturally maintain (Vector.Portable.Compress maintains
        // it via loop invariant; Avx2/Neon wrappers admit panic-freedom hence
        // also satisfy this).  Needed by callers that pass the result to
        // `subtract_reduce` / `compute_message` (which require
        // `is_bounded_poly 4095`) and to `ntt_vector_u` (which requires
        // `is_bounded_poly 3328`).  Without this clause the trait post is
        // strictly the mod-3329 FE equality, which cannot pin the i16 result
        // range.
        hax_lib::fstar_prop_expr!(
            r#"(v $coefficient_bits == 4 \/
                v $coefficient_bits == 5 \/
                v $coefficient_bits == 10 \/
                v $coefficient_bits == 11) ==>
               (bounded_i16_array (mk_i16 0) (mk_i16 3328) ${result} /\
                Spec.Utils.forall16 (fun (i: nat{i < 16}) ->
                  decompress_d_lane_post (mk_usize (v $coefficient_bits))
                    (Seq.index ${vec} i) (Seq.index ${result} i)))"#
        )
    }

    pub(crate) fn decompress_1_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"bounded_pos_i16_array 1 ${vec}"#
        )
    }

    pub(crate) fn decompress_ciphertext_coefficient_pre(
        vec: &[i16; 16],
        coefficient_bits: i32,
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"(v $coefficient_bits == 4 \/
                v $coefficient_bits == 5 \/
                v $coefficient_bits == 10 \/
                v $coefficient_bits == 11) /\
                bounded_pos_i16_array (v $coefficient_bits) ${vec}"#
        )
    }

    pub(crate) fn ntt_layer_1_step_pre(
        vec: &[i16; 16],
        zeta0: i16,
        zeta1: i16,
        zeta2: i16,
        zeta3: i16,
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#" is_i16b 1664 $zeta0 /\ is_i16b 1664 $zeta1 /\ 
                is_i16b 1664 $zeta2 /\ is_i16b 1664 $zeta3 /\
                is_i16b_array_opaque (7*3328) ${vec}"#
        )
    }

    // Cooley–Tukey butterfly post: bound + per-branch FE-equation atom.
    // `forall4` stays transparent so callers iterate per-branch; the
    // per-branch FE equations are wrapped in `ntt_layer_1_step_branch_post`
    // (opaque), so callers see the iteration structure but never the
    // FE-algebra body.
    pub(crate) fn ntt_layer_1_step_post(
        vec: &[i16; 16],
        zeta0: i16,
        zeta1: i16,
        zeta2: i16,
        zeta3: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"is_i16b_array_opaque (8 * 3328) ${result} /\
               Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
                 ntt_layer_1_step_branch_post b ${vec} $zeta0 $zeta1 $zeta2 $zeta3 ${result})"#
        )
    }

    pub(crate) fn ntt_layer_2_step_pre(vec: &[i16; 16], zeta0: i16, zeta1: i16) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#" is_i16b 1664 $zeta0 /\ is_i16b 1664 $zeta1 /\ 
                is_i16b_array_opaque (6*3328) ${vec}"#
        )
    }

    pub(crate) fn ntt_layer_2_step_post(
        vec: &[i16; 16],
        zeta0: i16,
        zeta1: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"is_i16b_array_opaque (7 * 3328) ${result} /\
               Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
                 ntt_layer_2_step_branch_post b ${vec} $zeta0 $zeta1 ${result})"#
        )
    }

    pub(crate) fn ntt_layer_3_step_pre(vec: &[i16; 16], zeta0: i16) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#" is_i16b 1664 $zeta0 /\
                is_i16b_array_opaque (5*3328) ${vec}"#
        )
    }

    pub(crate) fn ntt_layer_3_step_post(
        vec: &[i16; 16],
        zeta0: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"is_i16b_array_opaque (6 * 3328) ${result} /\
               Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
                 ntt_layer_3_step_branch_post b ${vec} $zeta0 ${result})"#
        )
    }

    pub(crate) fn inv_ntt_layer_1_step_pre(
        vec: &[i16; 16],
        zeta0: i16,
        zeta1: i16,
        zeta2: i16,
        zeta3: i16,
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#" is_i16b 1664 zeta0 /\ is_i16b 1664 zeta1 /\ 
                is_i16b 1664 zeta2 /\ is_i16b 1664 zeta3 /\
                is_i16b_array_opaque (4*3328) ${vec}"#
        )
    }

    pub(crate) fn inv_ntt_layer_1_step_post(
        vec: &[i16; 16],
        zeta0: i16,
        zeta1: i16,
        zeta2: i16,
        zeta3: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"is_i16b_array_opaque 3328 ${result} /\
               Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
                 inv_ntt_layer_1_step_branch_post b ${vec} $zeta0 $zeta1 $zeta2 $zeta3 ${result})"#
        )
    }

    pub(crate) fn inv_ntt_layer_2_step_pre(
        vec: &[i16; 16],
        zeta0: i16,
        zeta1: i16,
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#" is_i16b 1664 zeta0 /\ is_i16b 1664 zeta1 /\ 
                is_i16b_array_opaque 3328 ${vec}"#
        )
    }

    // Output bound is `2*3328` (not `3328`) because the body computes a raw
    // `a + b` for half the output lanes without an intervening Barrett
    // reduction.  See `src/invert_ntt.rs` for the bound trace.
    pub(crate) fn inv_ntt_layer_2_step_post(
        vec: &[i16; 16],
        zeta0: i16,
        zeta1: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"is_i16b_array_opaque (2*3328) ${result} /\
               Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
                 inv_ntt_layer_2_step_branch_post b ${vec} $zeta0 $zeta1 ${result})"#
        )
    }

    // Input bound is `2*3328` (not `3328`) because the previous layer
    // (`inv_ntt_layer_2_step`) skips Barrett reduction on its sum lanes.
    pub(crate) fn inv_ntt_layer_3_step_pre(vec: &[i16; 16], zeta0: i16) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#" is_i16b 1664 $zeta0 /\
                is_i16b_array_opaque (2*3328) ${vec}"#
        )
    }

    // Output bound is `4*3328` because the body, like layer 2, skips
    // Barrett reduction on the raw sum lanes.  Reduction to `3328` happens
    // in `invert_ntt_at_layer_4_plus`'s `inv_ntt_layer_int_vec_step_reduce`.
    pub(crate) fn inv_ntt_layer_3_step_post(
        vec: &[i16; 16],
        zeta0: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"is_i16b_array_opaque (4*3328) ${result} /\
               Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
                 inv_ntt_layer_3_step_branch_post b ${vec} $zeta0 ${result})"#
        )
    }

    pub(crate) fn ntt_multiply_pre(
        lhs: &[i16; 16],
        rhs: &[i16; 16],
        zeta0: i16,
        zeta1: i16,
        zeta2: i16,
        zeta3: i16,
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#" is_i16b 1664 zeta0 /\ is_i16b 1664 zeta1 /\
                is_i16b 1664 zeta2 /\ is_i16b 1664 zeta3 /\
                is_i16b_array_opaque 3328 ${lhs} /\
                is_i16b_array_opaque 3328 ${rhs} "#
        )
    }

    // Bound + already-opaque ntt_multiply_butterfly_post + per-branch FE
    // equation atom.  `forall4` stays transparent so the loop in
    // Polynomial.ntt_multiply iterates per-branch as opaque atoms.
    pub(crate) fn ntt_multiply_post(
        lhs: &[i16; 16],
        rhs: &[i16; 16],
        zeta0: i16,
        zeta1: i16,
        zeta2: i16,
        zeta3: i16,
        result: &[i16; 16],
    ) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"is_i16b_array_opaque 3328 ${result} /\
               Spec.Utils.ntt_multiply_butterfly_post ${lhs} ${rhs} ${result}
                 $zeta0 $zeta1 $zeta2 $zeta3 /\
               Spec.Utils.forall4 (fun (b: nat{b < 4}) ->
                 ntt_multiply_branch_post b ${lhs} ${rhs}
                   $zeta0 $zeta1 $zeta2 $zeta3 ${result})"#
        )
    }

    pub(crate) fn serialize_1_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            serialize_pre_N 1 $vec"#
        )
    }

    pub(crate) fn serialize_1_post(vec: &[i16; 16], result: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"   
            Seq.length ${result} == 2 /\
            (serialize_pre_N 1 $vec ==> 
               serialize_post_N 1 ${vec} ${result})"#
        )
    }

    pub(crate) fn deserialize_1_pre(input: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(r#"Seq.length ${input} == 2"#)
    }

    pub(crate) fn deserialize_1_post(input: &[u8], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${input} == 2 ==>
            deserialize_post_N 1 ${input} ${result}"#
        )
    }

    pub(crate) fn serialize_4_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            serialize_pre_N 4 $vec"#
        )
    }

    pub(crate) fn serialize_4_post(vec: &[i16; 16], result: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"   
            Seq.length ${result} == 8 /\
            (serialize_pre_N 4 $vec ==> 
               serialize_post_N 4 ${vec} ${result})"#
        )
    }

    pub(crate) fn deserialize_4_pre(input: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(r#"Seq.length ${input} == 8"#)
    }

    pub(crate) fn deserialize_4_post(input: &[u8], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${input} == 8 ==>
            deserialize_post_N 4 ${input} ${result}"#
        )
    }

    pub(crate) fn serialize_10_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#" 
            serialize_pre_N 10 $vec"#
        )
    }

    pub(crate) fn serialize_10_post(vec: &[i16; 16], result: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"   
            Seq.length ${result} == 20 /\
            (serialize_pre_N 10 $vec ==> 
               serialize_post_N 10 ${vec} ${result})"#
        )
    }

    pub(crate) fn deserialize_10_pre(input: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(r#"Seq.length ${input} == 20"#)
    }

    pub(crate) fn deserialize_10_post(input: &[u8], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${input} == 20 ==>
            deserialize_post_N 10 ${input} ${result}"#
        )
    }

    pub(crate) fn serialize_12_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#" 
            serialize_pre_N 12 $vec"#
        )
    }

    pub(crate) fn serialize_12_post(vec: &[i16; 16], result: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"   
            Seq.length ${result} == 24 /\
            (serialize_pre_N 12 $vec ==> 
               serialize_post_N 12 ${vec} ${result})"#
        )
    }

    pub(crate) fn deserialize_12_pre(input: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(r#"Seq.length ${input} == 24"#)
    }

    pub(crate) fn deserialize_12_post(input: &[u8], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${input} == 24 ==>
            deserialize_post_N 12 ${input} ${result}"#
        )
    }

    pub(crate) fn serialize_5_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            serialize_pre_N 5 $vec"#
        )
    }

    pub(crate) fn serialize_5_post(vec: &[i16; 16], result: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${result} == 10 /\
            (serialize_pre_N 5 $vec ==>
               serialize_post_N 5 ${vec} ${result})"#
        )
    }

    pub(crate) fn deserialize_5_pre(input: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${input} == 10"#
        )
    }

    pub(crate) fn deserialize_5_post(input: &[u8], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${input} == 10 ==>
              deserialize_post_N 5 ${input} ${result}"#
        )
    }

    pub(crate) fn serialize_11_pre(vec: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            serialize_pre_N 11 $vec"#
        )
    }

    pub(crate) fn serialize_11_post(vec: &[i16; 16], result: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${result} == 22 /\
            (serialize_pre_N 11 $vec ==>
               serialize_post_N 11 ${vec} ${result})"#
        )
    }

    pub(crate) fn deserialize_11_pre(input: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${input} == 22"#
        )
    }

    pub(crate) fn deserialize_11_post(input: &[u8], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"
            Seq.length ${input} == 22 ==>
              deserialize_post_N 11 ${input} ${result}"#
        )
    }

    pub(crate) fn from_bytes_post(input: &[u8], result: &[i16; 16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"Seq.length ${input} >= 32 ==>
               (let head : t_Slice u8 = Seq.slice ${input} 0 32 in
                from_le_bytes_post_N #(mk_usize 16) head ${result})"#
        )
    }

    pub(crate) fn to_bytes_post(input: &[i16; 16], result: &[u8]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"Seq.length ${result} >= 32 ==>
               (let head : t_Slice u8 = Seq.slice ${result} 0 32 in
                to_le_bytes_post_N #(mk_usize 16) ${input} head)"#
        )
    }

    pub(crate) fn rej_sample_post(input: &[u8], count: usize, out: &[i16]) -> hax_lib::Prop {
        hax_lib::fstar_prop_expr!(
            r#"Seq.length ${input} == 24 ==>
               (Seq.length ${out} == 16 /\ v ${count} <= 16 /\
                (let bytes : t_Array u8 (mk_usize 24) = ${input} in
                 let spec_result, spec_count =
                   Hacspec_ml_kem.Sampling.rej_sample_step bytes in
                 v ${count} == v spec_count /\
                 (forall (j: nat). j < v ${count} ==>
                    v (Seq.index ${out} j) >= 0 /\
                    v (Seq.index ${out} j) <= 3328 /\
                    v (Seq.index ${out} j) ==
                      v (Seq.index spec_result j).Hacspec_ml_kem.Parameters.f_val)))"#
        )
    }
}

#[hax_lib::attributes]
pub trait Operations: Copy + Clone + Repr {
    #[allow(non_snake_case)]
    #[requires(true)]
    #[ensures(|result| result.repr() == [0i16; 16])]
    fn ZERO() -> Self;

    #[requires(array.len() == 16)]
    #[ensures(|result| result.repr() == *array)]
    fn from_i16_array(array: &[i16]) -> Self;

    #[requires(true)]
    #[ensures(|result| result == x.repr())]
    fn to_i16_array(x: Self) -> [i16; 16];

    #[requires(array.len() >= 32)]
    #[ensures(|result| spec::from_bytes_post(&array, &result.repr()))]
    fn from_bytes(array: &[u8]) -> Self;

    #[requires(bytes.len() >= 32)]
    #[ensures(|_| (future(bytes).len() == bytes.len()).to_prop() &
                  spec::to_bytes_post(&x.repr(), &future(bytes)))]
    fn to_bytes(x: Self, bytes: &mut [u8]);

    // Basic arithmetic
    #[requires(spec::add_pre(&lhs.repr(), &rhs.repr()))]
    #[ensures(|result| spec::add_post(&lhs.repr(), &rhs.repr(), &result.repr()))]
    fn add(lhs: Self, rhs: &Self) -> Self;

    #[requires(spec::sub_pre(&lhs.repr(), &rhs.repr()))]
    #[ensures(|result| spec::sub_post(&lhs.repr(), &rhs.repr(), &result.repr()))]
    fn sub(lhs: Self, rhs: &Self) -> Self;

    #[requires(spec::multiply_by_constant_pre(&vec.repr(), c))]
    #[ensures(|result| spec::multiply_by_constant_post(&vec.repr(), c, &result.repr()))]
    fn multiply_by_constant(vec: Self, c: i16) -> Self;

    // Modular operations
    #[requires(spec::cond_subtract_3329_pre(&v.repr()))]
    #[ensures(|result| spec::cond_subtract_3329_post(&v.repr(), &result.repr()))]
    fn cond_subtract_3329(v: Self) -> Self;

    #[requires(spec::barrett_reduce_pre(&vector.repr()))]
    #[ensures(|result| spec::barrett_reduce_post(&vector.repr(), &result.repr()))]
    fn barrett_reduce(vector: Self) -> Self;

    #[requires(spec::montgomery_multiply_by_constant_pre(&vector.repr(), constant))]
    #[ensures(|result| spec::montgomery_multiply_by_constant_post(&vector.repr(), constant, &result.repr()))]
    fn montgomery_multiply_by_constant(vector: Self, constant: i16) -> Self;

    #[requires(spec::to_unsigned_representative_pre(&a.repr()))]
    #[ensures(|result| spec::to_unsigned_representative_post(&a.repr(), &result.repr()))]
    fn to_unsigned_representative(a: Self) -> Self;

    // Compression
    #[requires(spec::compress_1_pre(&a.repr()))]
    #[ensures(|result| spec::compress_1_post(&a.repr(), &result.repr()))]
    fn compress_1(a: Self) -> Self;

    #[requires(spec::compress_pre(&a.repr(), COEFFICIENT_BITS))]
    #[ensures(|result| spec::compress_post(&a.repr(), COEFFICIENT_BITS, &result.repr()))]
    fn compress<const COEFFICIENT_BITS: i32>(a: Self) -> Self;

    #[requires(spec::decompress_1_pre(&a.repr()))]
    #[ensures(|result| spec::decompress_1_post(&a.repr(), &result.repr()))]
    fn decompress_1(a: Self) -> Self;

    #[requires(spec::decompress_ciphertext_coefficient_pre(&a.repr(), COEFFICIENT_BITS))]
    #[ensures(|result| spec::decompress_ciphertext_coefficient_post(&a.repr(), COEFFICIENT_BITS, &result.repr()))]
    fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(a: Self) -> Self;

    // NTT
    #[requires(spec::ntt_layer_1_step_pre(&a.repr(), zeta0, zeta1, zeta2, zeta3))]
    #[ensures(|result| spec::ntt_layer_1_step_post(&a.repr(), zeta0, zeta1, zeta2, zeta3, &result.repr()))]
    fn ntt_layer_1_step(a: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self;

    #[requires(spec::ntt_layer_2_step_pre(&a.repr(), zeta0, zeta1))]
    #[ensures(|result| spec::ntt_layer_2_step_post(&a.repr(), zeta0, zeta1, &result.repr()))]
    fn ntt_layer_2_step(a: Self, zeta0: i16, zeta1: i16) -> Self;

    #[requires(spec::ntt_layer_3_step_pre(&a.repr(), zeta))]
    #[ensures(|result| spec::ntt_layer_3_step_post(&a.repr(), zeta, &result.repr()))]
    fn ntt_layer_3_step(a: Self, zeta: i16) -> Self;

    #[requires(spec::inv_ntt_layer_1_step_pre(&a.repr(), zeta0, zeta1, zeta2, zeta3))]
    #[ensures(|result| spec::inv_ntt_layer_1_step_post(&a.repr(), zeta0, zeta1, zeta2, zeta3, &result.repr()))]
    fn inv_ntt_layer_1_step(a: Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> Self;

    #[requires(spec::inv_ntt_layer_2_step_pre(&a.repr(), zeta0, zeta1))]
    #[ensures(|result| spec::inv_ntt_layer_2_step_post(&a.repr(), zeta0, zeta1, &result.repr()))]
    fn inv_ntt_layer_2_step(a: Self, zeta0: i16, zeta1: i16) -> Self;

    #[requires(spec::inv_ntt_layer_3_step_pre(&a.repr(), zeta))]
    #[ensures(|result| spec::inv_ntt_layer_3_step_post(&a.repr(), zeta, &result.repr()))]
    fn inv_ntt_layer_3_step(a: Self, zeta: i16) -> Self;

    #[requires(spec::ntt_multiply_pre(&lhs.repr(), &rhs.repr(), zeta0, zeta1, zeta2, zeta3))]
    #[ensures(|result| spec::ntt_multiply_post(&lhs.repr(), &rhs.repr(), zeta0, zeta1, zeta2, zeta3, &result.repr()))]
    fn ntt_multiply(lhs: &Self, rhs: &Self, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16)
        -> Self;

    // Serialization and deserialization
    #[requires(spec::serialize_1_pre(&a.repr()))]
    #[ensures(|result| spec::serialize_1_post(&a.repr(), &result))]
    fn serialize_1(a: Self) -> [u8; 2];

    #[requires(spec::deserialize_1_pre(&a))]
    #[ensures(|result| spec::deserialize_1_post(&a, &result.repr()))]
    fn deserialize_1(a: &[u8]) -> Self;

    #[requires(spec::serialize_4_pre(&a.repr()))]
    #[ensures(|result| spec::serialize_4_post(&a.repr(), &result))]
    fn serialize_4(a: Self) -> [u8; 8];

    #[requires(spec::deserialize_4_pre(&a))]
    #[ensures(|result| spec::deserialize_4_post(&a, &result.repr()))]
    fn deserialize_4(a: &[u8]) -> Self;

    #[requires(spec::serialize_5_pre(&a.repr()))]
    #[ensures(|result| spec::serialize_5_post(&a.repr(), &result))]
    fn serialize_5(a: Self) -> [u8; 10];

    #[requires(spec::deserialize_5_pre(&a))]
    #[ensures(|result| spec::deserialize_5_post(&a, &result.repr()))]
    fn deserialize_5(a: &[u8]) -> Self;

    #[requires(spec::serialize_10_pre(&a.repr()))]
    #[ensures(|result| spec::serialize_10_post(&a.repr(), &result))]
    fn serialize_10(a: Self) -> [u8; 20];

    #[requires(spec::deserialize_10_pre(&a))]
    #[ensures(|result| spec::deserialize_10_post(&a, &result.repr()))]
    fn deserialize_10(a: &[u8]) -> Self;

    #[requires(spec::serialize_11_pre(&a.repr()))]
    #[ensures(|result| spec::serialize_11_post(&a.repr(), &result))]
    fn serialize_11(a: Self) -> [u8; 22];

    #[requires(spec::deserialize_11_pre(&a))]
    #[ensures(|result| spec::deserialize_11_post(&a, &result.repr()))]
    fn deserialize_11(a: &[u8]) -> Self;

    #[requires(spec::serialize_12_pre(&a.repr()))]
    #[ensures(|result| spec::serialize_12_post(&a.repr(), &result))]
    fn serialize_12(a: Self) -> [u8; 24];

    #[requires(spec::deserialize_12_pre(&a))]
    #[ensures(|result| spec::deserialize_12_post(&a, &result.repr()))]
    fn deserialize_12(a: &[u8]) -> Self;

    // Rejection sampling
    #[requires(a.len() == 24 && out.len() == 16)]
    #[ensures(|result| (future(out).len() == 16 && result <= 16).to_prop() & (
            hax_lib::forall(|j: usize|
                hax_lib::implies(j < result,
                    future(out)[j] >= 0 && future(out)[j] <= 3328))))]
    // TODO(C4): strengthen to `spec::rej_sample_post(&a, result, &future(out))`
    // once impl bodies can discharge the rej_sample_step equation.
    fn rej_sample(a: &[u8], out: &mut [i16]) -> usize;
}
