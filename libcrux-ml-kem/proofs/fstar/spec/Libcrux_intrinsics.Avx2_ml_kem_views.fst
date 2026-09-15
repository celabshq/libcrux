module Libcrux_intrinsics.Avx2_ml_kem_views
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
open Libcrux_intrinsics.Avx2

(* Intrinsics lane view + PROVEN op-lemmas (thin adapter over core-models Avx2). *)
module Funarr = Libcrux_core_models.Abstractions.Funarr
module Canon  = Libcrux_core_models.Intrinsics_views
module IV     = Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec
module Avx2c  = Libcrux_core_models.Core_arch.X86.Avx2
module Sse2c  = Libcrux_core_models.Core_arch.X86.Sse2

(* ============================================================================
   ml-kem AVX2 lane-view + per-op fact companion.

   This module is the SINGLE ml-kem lane-view/fact companion over the REAL
   `Libcrux_intrinsics.Avx2` ops (which rest on the differentially-tested
   `libcrux-core-models` model).

   DESIGN (NO `Avx2_extract` shim).  The lane VIEWS (`vec256_as_i16x16` /
   `get_lane` / `lane32` / ...) and per-op FACT lemmas are phrased over the real
   ops (`open Libcrux_intrinsics.Avx2`).  The op BODIES come from the real `Avx2`
   (core-models `e_mm256_OP`); this module only re-exposes the lane-view fact
   surface ml-kem's proofs consume.

   TRUST.  This module is a THIN ADAPTER: the Seq lane view is a per-index read
   of the canonical core-models FunArray view (`Canon.to_i16x16` / `to_i16x8`),
   and every op-fact below is PROVEN from the canonical op-lemma set in
   `Libcrux_core_models.Intrinsics_views` (which itself rests only on the
   differentially-tested `Int_vec.Lemmas` lifts plus the PROVEN codec round-trip).
   NO fact in this module is assumed: `lemma_mm256_mul_epu32` (the only one crossing
   both a signedness and a width change) is proven from the canonical unsigned
   codec bridges `Canon.lemma_u32_of_i32` / `Canon.lemma_u64_concat32`.
   The views keep ml-kem's Seq/Array shape (`vec256_as_i16x16 : t_Array i16 (sz
   16)`, `get_lane`, `Spec.Utils.map2`) so the consuming proofs are untouched.

   ml-kem uses ONLY the i16x16 / lane32 / vec128 views.  The u64x4 view
   (`vec256_as_u64x4` / `get_lane_u64x4` / `lemma_mm256_*_u64x4`) is sha3-only and
   is NOT declared here.  The bit-level lane bridges (`bit_vec_of_int_t_array_*`)
   are PROVEN here — over core-models the lane view is the concrete codec, so each
   is one call to `Canon.lemma_readback`.  The bit-level shapes not yet needed
   (`mm256_{storeu,loadu}_si256_u8` bit_vec, `mm256_cmpgt_epi16` bit-form) are
   preserved verbatim in the DEFERRED block at the bottom of this file.

   This module lives in `proofs/fstar/spec/` (hand-maintained, NOT the
   hax-extraction dir), so `cargo hax into` never clobbers it; it is on ml-kem's
   include path but not sha3's.
   ========================================================================== *)

(* ── Lane-view types + abstract views ─────────────────────────────────────── *)

unfold type t_Vec256 = Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)
unfold type t_Vec128 = Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 128)

(* i16x16 lane view.  Its semantics are the core-models `to_i16x16`
   (FunArray -> t_Array via createi); the fact-lemmas below are PROVEN from the
   canonical op-lemma set. *)
(* Seq view = per-lane read of the canonical FunArray view.

   OPAQUE.  This definition is marked `opaque_to_smt` because exposing its body
   TRANSPARENTLY hurts consumers two ways:
     * the `t_Array i16 (sz 16)` -> `t_Slice i16` coercion VC
       (`Seq.length … <= max_usize`) does not follow from the declared result
       type and must be re-derived through `Seq.init`, which STARVES under
       `--ext context_pruning`; and
     * every `Seq.index (view x) i` goal acquires a second, dead-end path (unfold
       to `Seq.init`, then the OPAQUE `Canon.to_i16x16`) competing with the
       op-fact lemmas — 16 lanes of that saturates a split sub-query.
   `opaque_to_smt` keeps consumers seeing it as an ATOM while keeping the
   definition (so it is still PROVEN, not assumed).  The ONLY route from the Seq
   view to the canonical FunArray view is `vec256_index` below, which reveals
   internally. *)
[@@ "opaque_to_smt"]
let vec256_as_i16x16 (x: t_Vec256) : t_Array i16 (sz 16) =
  Seq.init 16 (fun i -> Funarr.impl_5__get (mk_u64 16) #i16 (Canon.to_i16x16 x) (mk_u64 i))
let get_lane (v: t_Vec256) (i:nat{i < 16}) = Seq.index (vec256_as_i16x16 v) i

(* One-line Seq<->FunArray index iso (Seq.init index). *)
let vec256_index (x: t_Vec256) (i: nat{i < 16})
  : Lemma (Seq.index (vec256_as_i16x16 x) i
           == Funarr.impl_5__get (mk_u64 16) #i16 (Canon.to_i16x16 x) (mk_u64 i))
          [SMTPat (Seq.index (vec256_as_i16x16 x) i)]
  = reveal_opaque (`%vec256_as_i16x16) vec256_as_i16x16

(* Length, from the declared result type — SMTPat so the `t_Array -> t_Slice`
   coercion VC of every consumer closes without touching the (now opaque) body. *)
let vec256_as_i16x16_len (x: t_Vec256)
  : Lemma (Seq.length (vec256_as_i16x16 x) == 16)
          [SMTPat (Seq.length (vec256_as_i16x16 x))]
  = ()

(* The `t_Array i16 (sz 16)` -> `t_Slice i16` coercion, as ONE GROUND fact.

   Every consumer that passes a lane view to a `t_Slice`-typed spec (e.g.
   `Spec.Utils.is_i16b_array`) pays the `t_Slice` refinement
   `Seq.length s <= max_usize`.  The `_len` lemma above is NOT enough: composing
   `Seq.length … == 16` with `16 <= max_usize` needs the numeral/`pow2` axioms for
   `max_usize`, and under `--ext context_pruning` those get crowded out of a heavy
   decl's pruned relevant-set — the VC then SATURATES (observed: Ntt's
   `inv_ntt_layer_1_step` sub-query 104, rlimit 300.000 canceled, on a coercion
   that is trivially true).  Handing over the FINISHED inequality survives pruning
   because the fact mentions all three of `Seq.length`, the view, and `max_usize`.

   The trigger is deliberately the view application itself rather than
   `Seq.length (view x)`: the narrower trigger does fire, yet the composition still
   starves, so the ground form has to be in scope wherever the view is. Safe
   despite the breadth — the payload is a single quantifier-free inequality. *)
let vec256_as_i16x16_slice_ok (x: t_Vec256)
  : Lemma (Seq.length (vec256_as_i16x16 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec256_as_i16x16 x)]
  = assert_norm (16 <= Rust_primitives.Integers.max_usize)

(* Reduction of an Int_vec FunArray produced by `from_fn 16` at a Seq index. *)
let index_from_fn16 (#t: Type0) (g: (i: u64{v i < 16}) -> t) (i: nat{i < 16})
  : Lemma (Funarr.impl_5__get (mk_u64 16) #t
             (Funarr.impl_5__from_fn (mk_u64 16) #t #(u64 -> t) g) (mk_u64 i)
           == g (mk_u64 i))
  = ()

let index_from_fn8 (#t: Type0) (g: (i: u64{v i < 8}) -> t) (i: nat{i < 8})
  : Lemma (Funarr.impl_5__get (mk_u64 8) #t
             (Funarr.impl_5__from_fn (mk_u64 8) #t #(u64 -> t) g) (mk_u64 i)
           == g (mk_u64 i))
  = ()

(* Signed value of the 32-bit lane `j` (low half = i16 lane 2j, high = 2j+1). *)
let lane32 (vec: t_Vec256) (j: nat{j < 8}) : int =
  (Rust_primitives.Integers.v (get_lane vec (2 * j)) % 65536) +
  65536 * Rust_primitives.Integers.v (get_lane vec (2 * j + 1))

(* Unsigned value of the 64-bit lane `i` (mm256_mul_epu32 output). *)
let lane64u (vec: t_Vec256) (i: nat{i < 4}) : int =
  (lane32 vec (2 * i) % 4294967296) + 4294967296 * (lane32 vec (2 * i + 1) % 4294967296)

(* Signed saturation into the i16 range (mm256_packs_epi32 per-lane clamp). *)
let sat_i16 (x: int) : i16 =
  if x > 32767 then mk_i16 32767
  else if x < (-32768) then mk_i16 (-32768)
  else mk_i16 x

(* Lane-permutation index helpers (opaque so they stay atomic under op-ensures
   foralls; consumers `reveal_opaque` inside small clean per-control lemmas). *)
[@@ "opaque_to_smt"]
let shuffle32_src (c: i32) (l: nat{l < 8}) : (s:nat{s < 8}) =
  let cb = (Rust_primitives.Integers.v c) % 256 in
  (l / 4) * 4 + ((match l % 4 with | 0 -> cb | 1 -> cb / 4 | 2 -> cb / 16 | _ -> cb / 64) % 4)

[@@ "opaque_to_smt"]
let permute64_src (c: i32) (q: nat{q < 4}) : (s:nat{s < 4}) =
  let cb = (Rust_primitives.Integers.v c) % 256 in
  (match q with | 0 -> cb | 1 -> cb / 4 | 2 -> cb / 16 | _ -> cb / 64) % 4

[@@ "opaque_to_smt"]
let blend_sel (c: i32) (k: nat{k < 16}) : bool =
  let cb = (Rust_primitives.Integers.v c) % 256 in
  ((match k % 8 with | 0 -> cb | 1 -> cb / 2 | 2 -> cb / 4 | 3 -> cb / 8
                     | 4 -> cb / 16 | 5 -> cb / 32 | 6 -> cb / 64 | _ -> cb / 128) % 2) = 1

(* i16x8 lane view of a 128-bit vector (A-on-B adapter over canonical to_i16x8).
   OPAQUE for the same reasons as `vec256_as_i16x16` above. *)
[@@ "opaque_to_smt"]
let vec128_as_i16x8 (x: t_Vec128) : t_Array i16 (sz 8) =
  Seq.init 8 (fun i -> Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 i))
let get_lane128 (v: t_Vec128) (i:nat{i < 8}) = Seq.index (vec128_as_i16x8 v) i

let vec128_index (x: t_Vec128) (i: nat{i < 8})
  : Lemma (Seq.index (vec128_as_i16x8 x) i
           == Funarr.impl_5__get (mk_u64 8) #i16 (Canon.to_i16x8 x) (mk_u64 i))
          [SMTPat (Seq.index (vec128_as_i16x8 x) i)]
  = reveal_opaque (`%vec128_as_i16x8) vec128_as_i16x8

let vec128_as_i16x8_len (x: t_Vec128)
  : Lemma (Seq.length (vec128_as_i16x8 x) == 8)
          [SMTPat (Seq.length (vec128_as_i16x8 x))]
  = ()

(* 128-bit twin of `vec256_as_i16x16_slice_ok` — same rationale. *)
let vec128_as_i16x8_slice_ok (x: t_Vec128)
  : Lemma (Seq.length (vec128_as_i16x8 x) <= Rust_primitives.Integers.max_usize)
          [SMTPat (vec128_as_i16x8 x)]
  = assert_norm (8 <= Rust_primitives.Integers.max_usize)

(* ── i16x16-view arithmetic/logical facts ─────────────────────────────────── *)

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_mm256_add_epi16 (lhs rhs: t_Vec256)
  : Lemma (vec256_as_i16x16 (mm256_add_epi16 lhs rhs)
           == Spec.Utils.map2 ( +. ) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs))
          [SMTPat (vec256_as_i16x16 (mm256_add_epi16 lhs rhs))] =
  reveal_opaque (`%mm256_add_epi16) mm256_add_epi16;
  Canon.lemma_mm256_add_epi16 lhs rhs;
  Seq.lemma_eq_intro (vec256_as_i16x16 (mm256_add_epi16 lhs rhs))
                     (Spec.Utils.map2 ( +. ) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_mm256_sub_epi16 (lhs rhs: t_Vec256)
  : Lemma (vec256_as_i16x16 (mm256_sub_epi16 lhs rhs)
           == Spec.Utils.map2 ( -. ) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs))
          [SMTPat (vec256_as_i16x16 (mm256_sub_epi16 lhs rhs))] =
  reveal_opaque (`%mm256_sub_epi16) mm256_sub_epi16;
  Canon.lemma_mm256_sub_epi16 lhs rhs;
  Seq.lemma_eq_intro (vec256_as_i16x16 (mm256_sub_epi16 lhs rhs))
                     (Spec.Utils.map2 ( -. ) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_mm256_mullo_epi16 (v1 v2: t_Vec256)
  : Lemma (vec256_as_i16x16 (mm256_mullo_epi16 v1 v2)
           == Spec.Utils.map2 mul_mod (vec256_as_i16x16 v1) (vec256_as_i16x16 v2))
          [SMTPat (vec256_as_i16x16 (mm256_mullo_epi16 v1 v2))] =
  reveal_opaque (`%mm256_mullo_epi16) mm256_mullo_epi16;
  Canon.lemma_mm256_mullo_epi16 v1 v2;
  Seq.lemma_eq_intro (vec256_as_i16x16 (mm256_mullo_epi16 v1 v2))
                     (Spec.Utils.map2 mul_mod (vec256_as_i16x16 v1) (vec256_as_i16x16 v2))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let lemma_mm256_mulhi_epi16 (lhs rhs: t_Vec256)
  : Lemma (vec256_as_i16x16 (mm256_mulhi_epi16 lhs rhs)
           == Spec.Utils.map2 (fun x y ->
                cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16)
              (vec256_as_i16x16 lhs)
              (vec256_as_i16x16 rhs))
          [SMTPat (vec256_as_i16x16 (mm256_mulhi_epi16 lhs rhs))] =
  reveal_opaque (`%mm256_mulhi_epi16) mm256_mulhi_epi16;
  Canon.lemma_mm256_mulhi_epi16 lhs rhs;
  Seq.lemma_eq_intro (vec256_as_i16x16 (mm256_mulhi_epi16 lhs rhs))
              (Spec.Utils.map2 (fun x y ->
                cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16)
              (vec256_as_i16x16 lhs)
              (vec256_as_i16x16 rhs))
#pop-options

(* PROVEN over core-models: reveal the intrinsic to the hardware op, then the
   canonical shared per-lane `and` commutation (`Canon.lemma_and_i16x16`, itself
   a proven i16 slice/decode commutation on top of the differentially-tested raw
   `and` lift). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_mm256_and_si256 (lhs rhs: t_Vec256)
  : Lemma (vec256_as_i16x16 (mm256_and_si256 lhs rhs)
           == Spec.Utils.map2 ( &. ) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs))
          [SMTPat (vec256_as_i16x16 (mm256_and_si256 lhs rhs))] =
  reveal_opaque (`%mm256_and_si256) mm256_and_si256;
  let r = mm256_and_si256 lhs rhs in
  let aux (i: nat{i < 16})
      : Lemma (Seq.index (vec256_as_i16x16 r) i ==
               Seq.index (Spec.Utils.map2 ( &. ) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs)) i) =
    Canon.lemma_and_i16x16 lhs rhs i
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec256_as_i16x16 r)
    (Spec.Utils.map2 ( &. ) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs))
#pop-options

(* ml-kem i16-view characterization of mm256_xor (called explicitly by Compress;
   also SMTPat).  Coexists with sha3's u64x4-view of the same op. *)
(* PROVEN over core-models: reveal the intrinsic to the hardware op, then the
   canonical shared per-lane `xor` commutation (`Canon.lemma_xor_i16x16`, a proven
   i16 slice/decode commutation on top of the differentially-tested raw `xor` lift). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_mm256_xor_si256 (lhs rhs: t_Vec256)
  : Lemma (vec256_as_i16x16 (mm256_xor_si256 lhs rhs)
           == Spec.Utils.map2 (^.) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs))
          [SMTPat (vec256_as_i16x16 (mm256_xor_si256 lhs rhs))] =
  reveal_opaque (`%mm256_xor_si256) mm256_xor_si256;
  let r = mm256_xor_si256 lhs rhs in
  let aux (i: nat{i < 16})
      : Lemma (Seq.index (vec256_as_i16x16 r) i ==
               Seq.index (Spec.Utils.map2 (^.) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs)) i) =
    Canon.lemma_xor_i16x16 lhs rhs i
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec256_as_i16x16 r)
    (Spec.Utils.map2 (^.) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_mm256_set1_epi16 (constant: i16)
  : Lemma (vec256_as_i16x16 (mm256_set1_epi16 constant) == Spec.Utils.create (sz 16) constant)
          [SMTPat (vec256_as_i16x16 (mm256_set1_epi16 constant))] =
  reveal_opaque (`%mm256_set1_epi16) mm256_set1_epi16;
  Canon.lemma_mm256_set1_epi16 constant;
  Seq.lemma_eq_intro (vec256_as_i16x16 (mm256_set1_epi16 constant))
                     (Spec.Utils.create (sz 16) constant)
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_mm256_set_epi16 (v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0: i16)
  : Lemma (vec256_as_i16x16 (mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0)
           == Spec.Utils.create16 v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15)
          [SMTPat (vec256_as_i16x16 (mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0))] =
  reveal_opaque (`%mm256_set_epi16) mm256_set_epi16;
  Canon.lemma_mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0;
  let r = mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0 in
  let expected = Spec.Utils.create16 v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15 in
  let aux (i: nat{i < 16}) : Lemma (Seq.index (vec256_as_i16x16 r) i == Seq.index expected i) =
    Canon.lemma_iv_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0 i
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec256_as_i16x16 r) expected
#pop-options

(* NB: NO SMTPat.  `mm256_setzero_si256 ()` is a fully GROUND core-models
   `t_BitVec` term, so an SMTPat on `vec256_as_i16x16 (mm256_setzero_si256 ())`
   is a variable-free trigger → Z3 emits "pattern does not contain any variable",
   which corrupts F*'s output parse into an Error 276.  Consumers call this
   explicitly. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_mm256_setzero_si256 (u: Prims.unit)
  : Lemma (vec256_as_i16x16 (mm256_setzero_si256 ()) == Seq.create 16 (mk_i16 0)) =
  reveal_opaque (`%mm256_setzero_si256) mm256_setzero_si256;
  let r = mm256_setzero_si256 () in
  let aux (i: nat{i < 16}) : Lemma (Seq.index (vec256_as_i16x16 r) i == mk_i16 0) =
    Canon.lemma_setzero_i16x16 i
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec256_as_i16x16 r) (Seq.create 16 (mk_i16 0))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let lemma_mm256_srai_epi16 (v_SHIFT_BY: i32) (vector: t_Vec256)
  : Lemma (requires v_SHIFT_BY >=. mk_i32 0 /\ v_SHIFT_BY <. mk_i32 16)
          (ensures vec256_as_i16x16 (mm256_srai_epi16 v_SHIFT_BY vector)
                   == Spec.Utils.map_array (fun x -> x >>! v_SHIFT_BY) (vec256_as_i16x16 vector))
          [SMTPat (vec256_as_i16x16 (mm256_srai_epi16 v_SHIFT_BY vector))] =
  reveal_opaque (`%mm256_srai_epi16) mm256_srai_epi16;
  Canon.lemma_mm256_srai_epi16 v_SHIFT_BY vector;
  Seq.lemma_eq_intro (vec256_as_i16x16 (mm256_srai_epi16 v_SHIFT_BY vector))
                   (Spec.Utils.map_array (fun x -> x >>! v_SHIFT_BY) (vec256_as_i16x16 vector))
#pop-options

(* ml-kem i16-view of the logical right shift (called explicitly by Compress,
   e.g. lemma_mm256_srli_epi16_15; also SMTPat). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_srli_epi16 (v_SHIFT_BY: i32 {v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 16}) (vector: t_Vec256)
  : Lemma (vec256_as_i16x16 (mm256_srli_epi16 v_SHIFT_BY vector)
           == Spec.Utils.map_array (fun (x:i16) -> cast ((cast x <: u16) >>! v_SHIFT_BY) <: i16)
                (vec256_as_i16x16 vector))
          [SMTPat (vec256_as_i16x16 (mm256_srli_epi16 v_SHIFT_BY vector))] =
  reveal_opaque (`%mm256_srli_epi16) mm256_srli_epi16;
  Canon.lemma_mm256_srli_epi16 v_SHIFT_BY vector;
  let r = mm256_srli_epi16 v_SHIFT_BY vector in
  let aux (i: nat{i < 16})
      : Lemma (Seq.index (vec256_as_i16x16 r) i ==
               (cast ((cast (Seq.index (vec256_as_i16x16 vector) i) <: u16) >>! v_SHIFT_BY <: u16)
                <: i16)) =
    Canon.lemma_iv_srli16 v_SHIFT_BY (Canon.to_i16x16 vector) i
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec256_as_i16x16 r)
    (Spec.Utils.map_array (fun (x:i16) -> cast ((cast x <: u16) >>! v_SHIFT_BY) <: i16)
       (vec256_as_i16x16 vector))
#pop-options

(* ── cross-width bridge wrapper + lane32-half helper ──────────────────────── *)

(* thin ml-kem wrapper over the crate-independent canonical bridge: the i16-pair
   `lane32` value equals the native i32 decode of the same 32 bits. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let lemma_lane32_eq_to_i32x8 (vec: t_Vec256) (j: nat{j < 8})
  : Lemma (lane32 vec j ==
           Rust_primitives.Integers.v
             (Funarr.impl_5__get (mk_u64 8) #i32 (Canon.to_i32x8 vec) (mk_u64 j))) =
  assert_norm (pow2 16 == 65536);
  Canon.lemma_lane32_bridge vec j
#pop-options

(* lane32 decomposes into its two i16 sub-lanes (pure lane32-definition arithmetic;
   re-derived here as Arithmetic_theory.lemma_lane32_halves is a downstream consumer). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_halves (w: t_Vec256) (j: nat{j < 8})
  : Lemma ((lane32 w j) @% pow2 16 == Rust_primitives.Integers.v (get_lane w (2 * j)) /\
           (lane32 w j) / pow2 16 == Rust_primitives.Integers.v (get_lane w (2 * j + 1))) =
  let lo = Rust_primitives.Integers.v (get_lane w (2 * j)) in
  let hi = Rust_primitives.Integers.v (get_lane w (2 * j + 1)) in
  assert_norm (pow2 16 == 65536);
  FStar.Math.Lemmas.lemma_div_plus (lo % pow2 16) hi (pow2 16);
  FStar.Math.Lemmas.small_div (lo % pow2 16) (pow2 16);
  FStar.Math.Lemmas.modulo_addition_lemma (lo % pow2 16) (pow2 16) hi;
  FStar.Math.Lemmas.small_mod (lo % pow2 16) (pow2 16);
  Spec.Utils.lemma_range_at_percent lo (pow2 16)
#pop-options

(* left-shift by 16 of an i32 lane: the low i16 sub-lane becomes 0, the high i16
   sub-lane becomes the original low i16 (pure integer / modular arithmetic). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_shl16 (w: int)
  : Lemma (ensures (let vv = (w * pow2 16) @% pow2 32 in
                    vv @% pow2 16 == 0 /\ vv / pow2 16 == w @% pow2 16)) =
  assert_norm (pow2 32 == pow2 16 * pow2 16);
  assert_norm (pow2 32 / 2 == pow2 16 * pow2 15);
  assert_norm (pow2 16 == 2 * pow2 15);
  let n16 = pow2 16 in
  let wm = w % n16 in
  FStar.Math.Lemmas.lemma_mod_lt w n16;
  FStar.Math.Lemmas.modulo_scale_lemma w n16 n16;
  if wm >= pow2 15 then begin
    FStar.Math.Lemmas.cancel_mul_div (wm - n16) n16;
    FStar.Math.Lemmas.cancel_mul_mod (wm - n16) n16
  end
  else begin
    FStar.Math.Lemmas.cancel_mul_div wm n16;
    FStar.Math.Lemmas.cancel_mul_mod wm n16
  end
#pop-options

(* ── lane32-view (32-bit lane) facts ──────────────────────────────────────── *)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_add_epi32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (j: nat). j < 8 ==>
             lane32 (mm256_add_epi32 lhs rhs) j == (lane32 lhs j + lane32 rhs j) @% 4294967296)
          [SMTPat (mm256_add_epi32 lhs rhs)] =
  reveal_opaque (`%mm256_add_epi32) mm256_add_epi32;
  Canon.lemma_mm256_add_epi32 lhs rhs;
  let r = mm256_add_epi32 lhs rhs in
  assert_norm (pow2 32 == 4294967296);
  let aux (j: nat{j < 8})
      : Lemma (lane32 r j == (lane32 lhs j + lane32 rhs j) @% 4294967296) =
    lemma_lane32_eq_to_i32x8 r j;
    lemma_lane32_eq_to_i32x8 lhs j;
    lemma_lane32_eq_to_i32x8 rhs j;
    Canon.lemma_iv_add_epi32 (Canon.to_i32x8 lhs) (Canon.to_i32x8 rhs) j
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_mullo_epi32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (j: nat). j < 8 ==>
             lane32 (mm256_mullo_epi32 lhs rhs) j == (lane32 lhs j * lane32 rhs j) @% 4294967296)
          [SMTPat (mm256_mullo_epi32 lhs rhs)] =
  reveal_opaque (`%mm256_mullo_epi32) mm256_mullo_epi32;
  Canon.lemma_mm256_mullo_epi32 lhs rhs;
  let r = mm256_mullo_epi32 lhs rhs in
  assert_norm (pow2 32 == 4294967296);
  let aux (j: nat{j < 8})
      : Lemma (lane32 r j == (lane32 lhs j * lane32 rhs j) @% 4294967296) =
    lemma_lane32_eq_to_i32x8 r j;
    lemma_lane32_eq_to_i32x8 lhs j;
    lemma_lane32_eq_to_i32x8 rhs j;
    Canon.lemma_iv_mullo_epi32 (Canon.to_i32x8 lhs) (Canon.to_i32x8 rhs) j
  in
  Classical.forall_intro aux
#pop-options

(* The `lane64u` view is the only lane fact crossing BOTH a signedness change
   (`to_u32x8` vs `to_i32x8`) and a width change (32 -> 64 unsigned), so it needs
   two extra codec bridges in the canonical module that the rest of the set does
   not: `Canon.lemma_u32_of_i32` and `Canon.lemma_u64_concat32`.  Both are
   PROVEN there, so this fact is not assumed.  Consumer: Compress's
   `mul_epu32_lane_nn`. *)
(* PROVEN from the canonical unsigned codec bridges.  The chain, per 64-bit lane
   `i`:  `lane64u r i` is the base-2^32 concatenation of the two i32 sub-lanes
   `2i`/`2i+1` reduced mod 2^32 (= the u32 lane view, `lemma_u32_of_i32`), which
   IS the native u64 lane (`lemma_u64_concat32`); the canonical op-lemma pushes
   that onto `IV.e_mm256_mul_epu32`, whose per-lane value is the product of the
   two EVEN u32 operand lanes (`lemma_iv_mul_epu32`); each of those is the
   operand's `lane32 … % 2^32` by the same codec bridge. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_mm256_mul_epu32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (i: nat). i < 4 ==>
             lane64u (mm256_mul_epu32 lhs rhs) i ==
             (lane32 lhs (2 * i) % 4294967296) * (lane32 rhs (2 * i) % 4294967296))
          [SMTPat (mm256_mul_epu32 lhs rhs)] =
  reveal_opaque (`%mm256_mul_epu32) mm256_mul_epu32;
  Canon.lemma_mm256_mul_epu32 lhs rhs;
  let r = mm256_mul_epu32 lhs rhs in
  assert_norm (pow2 32 == 4294967296);
  let aux (i: nat{i < 4})
      : Lemma (lane64u r i ==
               (lane32 lhs (2 * i) % 4294967296) * (lane32 rhs (2 * i) % 4294967296)) =
    (* result side: lane64u r i == v (u64 lane i of r) *)
    Canon.lemma_u64_concat32 r i;
    Canon.lemma_u32_of_i32 r (2 * i);
    Canon.lemma_u32_of_i32 r (2 * i + 1);
    lemma_lane32_eq_to_i32x8 r (2 * i);
    lemma_lane32_eq_to_i32x8 r (2 * i + 1);
    (* the interpreted op's per-lane value *)
    Canon.lemma_iv_mul_epu32 (Canon.to_u32x8 lhs) (Canon.to_u32x8 rhs) i;
    (* operand side: v (u32 lane 2i of x) == lane32 x (2i) % 2^32 *)
    Canon.lemma_u32_of_i32 lhs (2 * i);
    Canon.lemma_u32_of_i32 rhs (2 * i);
    lemma_lane32_eq_to_i32x8 lhs (2 * i);
    lemma_lane32_eq_to_i32x8 rhs (2 * i)
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_madd_epi16_lane32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (j: nat). j < 8 ==>
             lane32 (mm256_madd_epi16 lhs rhs) j ==
               (Rust_primitives.Integers.v (get_lane lhs (2*j)) * Rust_primitives.Integers.v (get_lane rhs (2*j)) +
                Rust_primitives.Integers.v (get_lane lhs (2*j+1)) * Rust_primitives.Integers.v (get_lane rhs (2*j+1)))
               @% 4294967296)
          [SMTPat (mm256_madd_epi16 lhs rhs)] =
  reveal_opaque (`%mm256_madd_epi16) mm256_madd_epi16;
  Canon.lemma_mm256_madd_epi16 lhs rhs;
  let r = mm256_madd_epi16 lhs rhs in
  assert_norm (pow2 32 == 4294967296);
  let aux (j: nat{j < 8})
      : Lemma (lane32 r j ==
               (Rust_primitives.Integers.v (get_lane lhs (2*j)) * Rust_primitives.Integers.v (get_lane rhs (2*j)) +
                Rust_primitives.Integers.v (get_lane lhs (2*j+1)) * Rust_primitives.Integers.v (get_lane rhs (2*j+1)))
               @% 4294967296) =
    lemma_lane32_eq_to_i32x8 r j;
    Canon.lemma_iv_madd_epi16 (Canon.to_i16x16 lhs) (Canon.to_i16x16 rhs) j
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_set1_epi32 (constant: i32)
  : Lemma (ensures forall (j: nat). j < 8 ==>
             lane32 (mm256_set1_epi32 constant) j == v constant /\
             ((0 <= v constant /\ v constant < pow2 16) ==>
               (get_lane (mm256_set1_epi32 constant) (2 * j) == (cast constant <: i16) /\
                get_lane (mm256_set1_epi32 constant) (2 * j + 1) == mk_i16 0)))
          [SMTPat (mm256_set1_epi32 constant)] =
  reveal_opaque (`%mm256_set1_epi32) mm256_set1_epi32;
  Canon.lemma_mm256_set1_epi32 constant;
  let r = mm256_set1_epi32 constant in
  let aux (j: nat{j < 8})
      : Lemma (lane32 r j == v constant /\
               ((0 <= v constant /\ v constant < pow2 16) ==>
                 (get_lane r (2 * j) == (cast constant <: i16) /\
                  get_lane r (2 * j + 1) == mk_i16 0))) =
    lemma_lane32_eq_to_i32x8 r j;
    Canon.lemma_iv_set1_epi32 constant j;
    assert (lane32 r j == v constant);
    assert_norm (pow2 16 == 65536);
    introduce (0 <= v constant /\ v constant < pow2 16) ==>
              (get_lane r (2 * j) == (cast constant <: i16) /\ get_lane r (2 * j + 1) == mk_i16 0)
    with _pf. (
      lemma_halves r j;
      FStar.Math.Lemmas.small_div (v constant) (pow2 16);
      assert (v (get_lane r (2 * j + 1)) == 0);
      assert (get_lane r (2 * j + 1) == mk_i16 0);
      assert (v (get_lane r (2 * j)) == (v constant) @% pow2 16);
      assert (v (cast constant <: i16) == (v constant) @% pow2 16);
      assert (get_lane r (2 * j) == (cast constant <: i16))
    )
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_srai_epi32 (v_SHIFT_BY: i32) (vector: t_Vec256)
  : Lemma (ensures
             (v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 32) ==>
             (forall (j: nat). j < 8 ==>
                lane32 (mm256_srai_epi32 v_SHIFT_BY vector) j == (lane32 vector j) / pow2 (v v_SHIFT_BY)))
          [SMTPat (mm256_srai_epi32 v_SHIFT_BY vector)] =
  reveal_opaque (`%mm256_srai_epi32) mm256_srai_epi32;
  Canon.lemma_mm256_srai_epi32 v_SHIFT_BY vector;
  let r = mm256_srai_epi32 v_SHIFT_BY vector in
  introduce (v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 32) ==>
            (forall (j: nat). j < 8 ==>
               lane32 r j == (lane32 vector j) / pow2 (v v_SHIFT_BY))
  with _pf. (
    let aux (j: nat{j < 8}) : Lemma (lane32 r j == (lane32 vector j) / pow2 (v v_SHIFT_BY)) =
      lemma_lane32_eq_to_i32x8 r j;
      lemma_lane32_eq_to_i32x8 vector j;
      Canon.lemma_iv_srai32 v_SHIFT_BY (Canon.to_i32x8 vector) j
    in
    Classical.forall_intro aux
  )
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_mm256_srli_epi32 (v_SHIFT_BY: i32) (vector: t_Vec256)
  : Lemma (ensures
             (v v_SHIFT_BY > 0 /\ v v_SHIFT_BY < 32) ==>
             (forall (j: nat). j < 8 ==>
                lane32 (mm256_srli_epi32 v_SHIFT_BY vector) j ==
                (lane32 vector j % 4294967296) / pow2 (v v_SHIFT_BY)))
          [SMTPat (mm256_srli_epi32 v_SHIFT_BY vector)] =
  reveal_opaque (`%mm256_srli_epi32) mm256_srli_epi32;
  Canon.lemma_mm256_srli_epi32 v_SHIFT_BY vector;
  let r = mm256_srli_epi32 v_SHIFT_BY vector in
  introduce (v v_SHIFT_BY > 0 /\ v v_SHIFT_BY < 32) ==>
            (forall (j: nat). j < 8 ==>
               lane32 r j == (lane32 vector j % 4294967296) / pow2 (v v_SHIFT_BY))
  with _pf. (
    let aux (j: nat{j < 8})
        : Lemma (lane32 r j == (lane32 vector j % 4294967296) / pow2 (v v_SHIFT_BY)) =
      lemma_lane32_eq_to_i32x8 r j;
      lemma_lane32_eq_to_i32x8 vector j;
      Canon.lemma_iv_srli32 v_SHIFT_BY (Canon.to_i32x8 vector) j;
      let xj = Funarr.impl_5__get (mk_u64 8) #i32 (Canon.to_i32x8 vector) (mk_u64 j) in
      assert_norm (pow2 32 == 4294967296);
      (* cast i32->u32 is the unsigned rep; u32 >>! = floor div; the result is
         < 2^31 (shift>=1) so cast back to i32 is identity. *)
      assert (v (cast xj <: u32) == (v xj) % pow2 32);
      FStar.Math.Lemmas.lemma_div_lt_nat ((v xj) % pow2 32) 32 (v v_SHIFT_BY);
      Spec.Utils.lemma_range_at_percent (((v xj) % pow2 32) / pow2 (v v_SHIFT_BY)) (pow2 32)
    in
    Classical.forall_intro aux
  )
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_mm256_slli_epi32 (v_SHIFT_BY: i32) (vector: t_Vec256)
  : Lemma (ensures
             ((v v_SHIFT_BY == 16) ==>
               (forall (k: nat). {:pattern (get_lane (mm256_slli_epi32 v_SHIFT_BY vector) k)}
                  k < 16 ==>
                  get_lane (mm256_slli_epi32 v_SHIFT_BY vector) k ==
                    (if k % 2 = 0 then mk_i16 0 else get_lane vector (k - 1)))) /\
             ((v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 32) ==>
               (forall (j: nat). j < 8 ==>
                  lane32 (mm256_slli_epi32 v_SHIFT_BY vector) j ==
                    (lane32 vector j * pow2 (v v_SHIFT_BY)) @% 4294967296)))
          [SMTPat (mm256_slli_epi32 v_SHIFT_BY vector)] =
  reveal_opaque (`%mm256_slli_epi32) mm256_slli_epi32;
  Canon.lemma_mm256_slli_epi32 v_SHIFT_BY vector;
  let r = mm256_slli_epi32 v_SHIFT_BY vector in
  assert_norm (pow2 32 == 4294967296);
  let laneB (j: nat{j < 8})
      : Lemma (requires v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 32)
              (ensures lane32 r j == (lane32 vector j * pow2 (v v_SHIFT_BY)) @% 4294967296) =
    lemma_lane32_eq_to_i32x8 r j;
    lemma_lane32_eq_to_i32x8 vector j;
    Canon.lemma_iv_slli32 v_SHIFT_BY (Canon.to_i32x8 vector) j;
    let xj = Funarr.impl_5__get (mk_u64 8) #i32 (Canon.to_i32x8 vector) (mk_u64 j) in
    assert (v (cast xj <: u32) == (v xj) % pow2 32);
    FStar.Math.Lemmas.lemma_mod_mul_distr_l (v xj) (pow2 (v v_SHIFT_BY)) (pow2 32);
    FStar.Math.Lemmas.lemma_mod_mod ((v xj * pow2 (v v_SHIFT_BY)) % pow2 32)
      (v xj * pow2 (v v_SHIFT_BY)) (pow2 32)
  in
  let laneA (j: nat{j < 8})
      : Lemma (requires v v_SHIFT_BY == 16)
              (ensures get_lane r (2 * j) == mk_i16 0 /\
                       get_lane r (2 * j + 1) == get_lane vector (2 * j)) =
    laneB j;
    lemma_halves r j;
    lemma_halves vector j;
    assert_norm (pow2 32 == 4294967296);
    lemma_shl16 (lane32 vector j)
  in
  introduce (v v_SHIFT_BY == 16) ==>
            (forall (k: nat). {:pattern (get_lane r k)} k < 16 ==>
               get_lane r k == (if k % 2 = 0 then mk_i16 0 else get_lane vector (k - 1)))
  with _pf. (
    let auxA (k: nat{k < 16})
        : Lemma (get_lane r k == (if k % 2 = 0 then mk_i16 0 else get_lane vector (k - 1))) =
      laneA (k / 2)
    in
    Classical.forall_intro auxA
  );
  introduce (v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 32) ==>
            (forall (j: nat). j < 8 ==>
               lane32 r j == (lane32 vector j * pow2 (v v_SHIFT_BY)) @% 4294967296)
  with _pf. (
    let auxB (j: nat{j < 8})
        : Lemma (lane32 r j == (lane32 vector j * pow2 (v v_SHIFT_BY)) @% 4294967296) = laneB j
    in
    Classical.forall_intro auxB
  )
#pop-options

(* the 8 lane32 <-> to_i32x8 equations for one vector, in one call (the
   permutation facts below need them at every source index at once). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_lane32_all (w: t_Vec256)
  : Lemma (forall (j: nat). j < 8 ==>
             lane32 w j == Rust_primitives.Integers.v
               (Funarr.impl_5__get (mk_u64 8) #i32 (Canon.to_i32x8 w) (mk_u64 j))) =
  Classical.forall_intro (lemma_lane32_eq_to_i32x8 w)
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_mm256_unpacklo_epi32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (j: nat). j < 8 ==>
             lane32 (mm256_unpacklo_epi32 lhs rhs) j ==
             (match j with
               | 0 -> lane32 lhs 0 | 1 -> lane32 rhs 0
               | 2 -> lane32 lhs 1 | 3 -> lane32 rhs 1
               | 4 -> lane32 lhs 4 | 5 -> lane32 rhs 4
               | 6 -> lane32 lhs 5 | _ -> lane32 rhs 5))
          [SMTPat (mm256_unpacklo_epi32 lhs rhs)] =
  reveal_opaque (`%mm256_unpacklo_epi32) mm256_unpacklo_epi32;
  Canon.lemma_mm256_unpacklo_epi32 lhs rhs;
  let r = mm256_unpacklo_epi32 lhs rhs in
  lemma_lane32_all lhs; lemma_lane32_all rhs; lemma_lane32_all r;
  let aux (j: nat{j < 8})
      : Lemma (lane32 r j ==
               (match j with
                 | 0 -> lane32 lhs 0 | 1 -> lane32 rhs 0
                 | 2 -> lane32 lhs 1 | 3 -> lane32 rhs 1
                 | 4 -> lane32 lhs 4 | 5 -> lane32 rhs 4
                 | 6 -> lane32 lhs 5 | _ -> lane32 rhs 5)) =
    Canon.lemma_iv_unpacklo_epi32 (Canon.to_i32x8 lhs) (Canon.to_i32x8 rhs) j
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_mm256_unpackhi_epi32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (j: nat). j < 8 ==>
             lane32 (mm256_unpackhi_epi32 lhs rhs) j ==
             (match j with
               | 0 -> lane32 lhs 2 | 1 -> lane32 rhs 2
               | 2 -> lane32 lhs 3 | 3 -> lane32 rhs 3
               | 4 -> lane32 lhs 6 | 5 -> lane32 rhs 6
               | 6 -> lane32 lhs 7 | _ -> lane32 rhs 7))
          [SMTPat (mm256_unpackhi_epi32 lhs rhs)] =
  reveal_opaque (`%mm256_unpackhi_epi32) mm256_unpackhi_epi32;
  Canon.lemma_mm256_unpackhi_epi32 lhs rhs;
  let r = mm256_unpackhi_epi32 lhs rhs in
  lemma_lane32_all lhs; lemma_lane32_all rhs; lemma_lane32_all r;
  let aux (j: nat{j < 8})
      : Lemma (lane32 r j ==
               (match j with
                 | 0 -> lane32 lhs 2 | 1 -> lane32 rhs 2
                 | 2 -> lane32 lhs 3 | 3 -> lane32 rhs 3
                 | 4 -> lane32 lhs 6 | 5 -> lane32 rhs 6
                 | 6 -> lane32 lhs 7 | _ -> lane32 rhs 7)) =
    Canon.lemma_iv_unpackhi_epi32 (Canon.to_i32x8 lhs) (Canon.to_i32x8 rhs) j
  in
  Classical.forall_intro aux
#pop-options

(* lane32-view of the qword permutation (mm256_unpackhi_epi64); sha3's u64x4-view
   of the same op stays in the intrinsics tree.  Called by Compress (mulhi
   composite); also SMTPat. *)
(* qword-granular permutation: equal i64 lanes give equal i16 sub-lanes (the
   canonical `lemma_sub_i64_i16`), and each i64 lane is exactly two lane32s. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 250"
let lemma_qword_lane32 (r src: t_Vec256) (q: nat{q < 4}) (s: nat{s < 4})
  : Lemma (requires Funarr.impl_5__get (mk_u64 4) #i64 (Canon.to_i64x4 r) (mk_u64 q) ==
                    Funarr.impl_5__get (mk_u64 4) #i64 (Canon.to_i64x4 src) (mk_u64 s))
          (ensures lane32 r (2 * q) == lane32 src (2 * s) /\
                   lane32 r (2 * q + 1) == lane32 src (2 * s + 1)) =
  Canon.lemma_sub_i64_i16 r src q s 0;
  Canon.lemma_sub_i64_i16 r src q s 1;
  Canon.lemma_sub_i64_i16 r src q s 2;
  Canon.lemma_sub_i64_i16 r src q s 3
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_mm256_unpackhi_epi64_lane32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (j: nat). j < 8 ==>
            lane32 (mm256_unpackhi_epi64 lhs rhs) j ==
            (match j with
              | 0 -> lane32 lhs 2 | 1 -> lane32 lhs 3
              | 2 -> lane32 rhs 2 | 3 -> lane32 rhs 3
              | 4 -> lane32 lhs 6 | 5 -> lane32 lhs 7
              | 6 -> lane32 rhs 6 | _ -> lane32 rhs 7))
          [SMTPat (mm256_unpackhi_epi64 lhs rhs)] =
  reveal_opaque (`%mm256_unpackhi_epi64) mm256_unpackhi_epi64;
  Canon.lemma_mm256_unpackhi_epi64 lhs rhs;
  let r = mm256_unpackhi_epi64 lhs rhs in
  Canon.lemma_iv_unpackhi_epi64 (Canon.to_i64x4 lhs) (Canon.to_i64x4 rhs) 0;
  Canon.lemma_iv_unpackhi_epi64 (Canon.to_i64x4 lhs) (Canon.to_i64x4 rhs) 1;
  Canon.lemma_iv_unpackhi_epi64 (Canon.to_i64x4 lhs) (Canon.to_i64x4 rhs) 2;
  Canon.lemma_iv_unpackhi_epi64 (Canon.to_i64x4 lhs) (Canon.to_i64x4 rhs) 3;
  lemma_qword_lane32 r lhs 0 1;
  lemma_qword_lane32 r rhs 1 1;
  lemma_qword_lane32 r lhs 2 3;
  lemma_qword_lane32 r rhs 3 3
#pop-options

(* ── get_lane-permutation facts ───────────────────────────────────────────── *)

(* the two ml-kem selector helpers agree with the canonical `ctl2` digit that the
   core-models interpretations use, for a valid (8-bit, non-negative) immediate. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 150"
let lemma_shuffle32_src_ctl2 (c: i32) (l: nat{l < 8})
  : Lemma (requires v c >= 0 /\ v c < 256)
          (ensures shuffle32_src c l == 4 * (l / 4) + Canon.ctl2 c (l % 4)) =
  reveal_opaque (`%shuffle32_src) shuffle32_src;
  assert_norm (pow2 0 == 1); assert_norm (pow2 2 == 4);
  assert_norm (pow2 4 == 16); assert_norm (pow2 6 == 64);
  FStar.Math.Lemmas.small_mod (v c) 256

let lemma_permute64_src_ctl2 (c: i32) (q: nat{q < 4})
  : Lemma (requires v c >= 0 /\ v c < 256)
          (ensures permute64_src c q == Canon.ctl2 c q) =
  reveal_opaque (`%permute64_src) permute64_src;
  assert_norm (pow2 0 == 1); assert_norm (pow2 2 == 4);
  assert_norm (pow2 4 == 16); assert_norm (pow2 6 == 64);
  FStar.Math.Lemmas.small_mod (v c) 256
#pop-options

(* NOTE (immediate range).  The core-models interpretation reads the immediate as
   `(IMM8 >> 2m) % 4`, which is the intended selector only for an 8-bit
   non-negative immediate — outside that range the model's own index leaves the
   lane domain.  The const-generic controls ml-kem passes are literals in
   [0,256) (68/160/238/245 for shuffle, 160/216/245 for permute, 170/204/240 for
   blend, 1 for inserti128), so taking the range as a `requires` is honest and
   discharges at every call site by normalisation.  An unconditional form would
   claim something unprovable (and in general false) for out-of-range immediates. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_mm256_shuffle_epi32 (v_CONTROL: i32) (vector: t_Vec256)
  : Lemma (requires v v_CONTROL >= 0 /\ v v_CONTROL < 256)
          (ensures forall (k: nat). {:pattern (get_lane (mm256_shuffle_epi32 v_CONTROL vector) k)}
             k < 16 ==>
             get_lane (mm256_shuffle_epi32 v_CONTROL vector) k ==
               get_lane vector (2 * shuffle32_src v_CONTROL (k / 2) + k % 2))
          [SMTPat (mm256_shuffle_epi32 v_CONTROL vector)] =
  reveal_opaque (`%mm256_shuffle_epi32) mm256_shuffle_epi32;
  Canon.lemma_mm256_shuffle_epi32 v_CONTROL vector;
  let r = mm256_shuffle_epi32 v_CONTROL vector in
  let aux (k: nat{k < 16})
      : Lemma (get_lane r k == get_lane vector (2 * shuffle32_src v_CONTROL (k / 2) + k % 2)) =
    let j = k / 2 in
    let i = k % 2 in
    Canon.lemma_iv_shuffle_epi32 v_CONTROL (Canon.to_i32x8 vector) j;
    lemma_shuffle32_src_ctl2 v_CONTROL j;
    Canon.lemma_sub_i32_i16 r vector j (shuffle32_src v_CONTROL j) i
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_mm256_permute4x64_epi64 (v_CONTROL: i32) (vector: t_Vec256)
  : Lemma (requires v v_CONTROL >= 0 /\ v v_CONTROL < 256)
          (ensures forall (k: nat). {:pattern (get_lane (mm256_permute4x64_epi64 v_CONTROL vector) k)}
             k < 16 ==>
             get_lane (mm256_permute4x64_epi64 v_CONTROL vector) k ==
               get_lane vector (4 * permute64_src v_CONTROL (k / 4) + k % 4))
          [SMTPat (mm256_permute4x64_epi64 v_CONTROL vector)] =
  reveal_opaque (`%mm256_permute4x64_epi64) mm256_permute4x64_epi64;
  Canon.lemma_mm256_permute4x64_epi64 v_CONTROL vector;
  let r = mm256_permute4x64_epi64 v_CONTROL vector in
  let aux (k: nat{k < 16})
      : Lemma (get_lane r k == get_lane vector (4 * permute64_src v_CONTROL (k / 4) + k % 4)) =
    let q = k / 4 in
    let i = k % 4 in
    Canon.lemma_iv_permute4x64_epi64 v_CONTROL (Canon.to_i64x4 vector) q;
    lemma_permute64_src_ctl2 v_CONTROL q;
    Canon.lemma_sub_i64_i16 r vector q (permute64_src v_CONTROL q) i
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_castsi128_si256 (vector: t_Vec128)
  : Lemma (ensures forall (k: nat). {:pattern (get_lane (mm256_castsi128_si256 vector) k)}
             k < 8 ==> get_lane (mm256_castsi128_si256 vector) k == get_lane128 vector k)
          [SMTPat (mm256_castsi128_si256 vector)] =
  reveal_opaque (`%mm256_castsi128_si256) mm256_castsi128_si256;
  let r = mm256_castsi128_si256 vector in
  let aux (k: nat{k < 8}) : Lemma (get_lane r k == get_lane128 vector k) =
    Canon.lemma_castsi128_i16x16 vector k
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_mm256_cvtepi16_epi32 (vector: t_Vec128)
  : Lemma (ensures forall (j: nat). j < 8 ==>
             get_lane (mm256_cvtepi16_epi32 vector) (2 * j) == get_lane128 vector j /\
             get_lane (mm256_cvtepi16_epi32 vector) (2 * j + 1) ==
               (if v (get_lane128 vector j) < 0 then mk_i16 (- 1) else mk_i16 0))
          [SMTPat (mm256_cvtepi16_epi32 vector)] =
  reveal_opaque (`%mm256_cvtepi16_epi32) mm256_cvtepi16_epi32;
  Canon.lemma_mm256_cvtepi16_epi32 vector;
  let r = mm256_cvtepi16_epi32 vector in
  assert_norm (pow2 16 == 65536);
  let aux (j: nat{j < 8})
      : Lemma (get_lane r (2 * j) == get_lane128 vector j /\
               get_lane r (2 * j + 1) ==
                 (if v (get_lane128 vector j) < 0 then mk_i16 (- 1) else mk_i16 0)) =
    lemma_lane32_eq_to_i32x8 r j;
    Canon.lemma_iv_cvtepi16_epi32 (Canon.to_i16x8 vector) j;
    lemma_halves r j;
    Spec.Utils.lemma_range_at_percent (v (get_lane128 vector j)) (pow2 16)
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_mm256_packs_epi32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (k: nat). k < 16 ==>
             get_lane (mm256_packs_epi32 lhs rhs) k ==
             (if k < 4
               then sat_i16 (lane32 lhs k)
               else
                 if k < 8
                 then sat_i16 (lane32 rhs (k - 4))
                 else if k < 12 then sat_i16 (lane32 lhs (k - 4)) else sat_i16 (lane32 rhs (k - 8))))
          [SMTPat (mm256_packs_epi32 lhs rhs)] =
  reveal_opaque (`%mm256_packs_epi32) mm256_packs_epi32;
  Canon.lemma_mm256_packs_epi32 lhs rhs;
  let r = mm256_packs_epi32 lhs rhs in
  lemma_lane32_all lhs; lemma_lane32_all rhs;
  let aux (k: nat{k < 16})
      : Lemma (get_lane r k ==
               (if k < 4 then sat_i16 (lane32 lhs k)
                else if k < 8 then sat_i16 (lane32 rhs (k - 4))
                else if k < 12 then sat_i16 (lane32 lhs (k - 4)) else sat_i16 (lane32 rhs (k - 8)))) =
    Canon.lemma_iv_packs_epi32 (Canon.to_i32x8 lhs) (Canon.to_i32x8 rhs) k
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_mm256_inserti128_si256 (v_CONTROL: i32) (vector: t_Vec256) (vector_i128: t_Vec128)
  : Lemma (requires v v_CONTROL >= 0 /\ v v_CONTROL < 256)
          (ensures forall (k: nat). {:pattern (get_lane (mm256_inserti128_si256 v_CONTROL vector vector_i128) k)}
             k < 16 ==>
             get_lane (mm256_inserti128_si256 v_CONTROL vector vector_i128) k ==
             (if (v v_CONTROL) % 2 = 1
               then (if k < 8 then get_lane vector k else get_lane128 vector_i128 (k - 8))
               else (if k < 8 then get_lane128 vector_i128 k else get_lane vector k)))
          [SMTPat (mm256_inserti128_si256 v_CONTROL vector vector_i128)] =
  reveal_opaque (`%mm256_inserti128_si256) mm256_inserti128_si256;
  Canon.lemma_mm256_inserti128_si256 v_CONTROL vector vector_i128;
  let r = mm256_inserti128_si256 v_CONTROL vector vector_i128 in
  Canon.lemma_iv_inserti128_si256 v_CONTROL (Canon.to_i128x2 vector) (Canon.to_i128x1 vector_i128) 0;
  Canon.lemma_iv_inserti128_si256 v_CONTROL (Canon.to_i128x2 vector) (Canon.to_i128x1 vector_i128) 1;
  let aux (k: nat{k < 16})
      : Lemma (get_lane r k ==
               (if (v v_CONTROL) % 2 = 1
                 then (if k < 8 then get_lane vector k else get_lane128 vector_i128 (k - 8))
                 else (if k < 8 then get_lane128 vector_i128 k else get_lane vector k))) =
    let q = k / 8 in
    let i = k % 8 in
    if (v v_CONTROL) % 2 = 0
    then (if q = 0 then Canon.lemma_sub_i128_i16_128 r vector_i128 0 i
                   else Canon.lemma_sub_i128_i16 r vector 1 1 i)
    else (if q = 0 then Canon.lemma_sub_i128_i16 r vector 0 0 i
                   else Canon.lemma_sub_i128_i16_128 r vector_i128 1 i)
  in
  Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 250"
let lemma_mm256_blend_epi16 (v_CONTROL: i32) (lhs rhs: t_Vec256)
  : Lemma (requires v v_CONTROL >= 0 /\ v v_CONTROL < 256)
          (ensures forall (k: nat). {:pattern (get_lane (mm256_blend_epi16 v_CONTROL lhs rhs) k)}
             k < 16 ==>
             get_lane (mm256_blend_epi16 v_CONTROL lhs rhs) k ==
               (if blend_sel v_CONTROL k then get_lane rhs k else get_lane lhs k))
          [SMTPat (mm256_blend_epi16 v_CONTROL lhs rhs)] =
  reveal_opaque (`%mm256_blend_epi16) mm256_blend_epi16;
  reveal_opaque (`%blend_sel) blend_sel;
  Canon.lemma_mm256_blend_epi16 v_CONTROL lhs rhs;
  let r = mm256_blend_epi16 v_CONTROL lhs rhs in
  assert_norm (pow2 0 == 1); assert_norm (pow2 1 == 2); assert_norm (pow2 2 == 4);
  assert_norm (pow2 3 == 8); assert_norm (pow2 4 == 16); assert_norm (pow2 5 == 32);
  assert_norm (pow2 6 == 64); assert_norm (pow2 7 == 128);
  FStar.Math.Lemmas.small_mod (v v_CONTROL) 256;
  let aux (k: nat{k < 16})
      : Lemma (get_lane r k == (if blend_sel v_CONTROL k then get_lane rhs k else get_lane lhs k)) =
    Canon.lemma_iv_blend_epi16 v_CONTROL (Canon.to_i16x16 lhs) (Canon.to_i16x16 rhs) k
  in
  Classical.forall_intro aux
#pop-options

(* ── i16x8-view (128-bit vector) facts ────────────────────────────────────── *)

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_mm_add_epi16 (lhs rhs: t_Vec128)
  : Lemma (vec128_as_i16x8 (mm_add_epi16 lhs rhs)
           == Spec.Utils.map2 ( +. ) (vec128_as_i16x8 lhs) (vec128_as_i16x8 rhs))
          [SMTPat (vec128_as_i16x8 (mm_add_epi16 lhs rhs))] =
  reveal_opaque (`%mm_add_epi16) mm_add_epi16;
  Canon.lemma_mm_add_epi16 lhs rhs;
  Seq.lemma_eq_intro (vec128_as_i16x8 (mm_add_epi16 lhs rhs))
                     (Spec.Utils.map2 ( +. ) (vec128_as_i16x8 lhs) (vec128_as_i16x8 rhs))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_mm_sub_epi16 (lhs rhs: t_Vec128)
  : Lemma (vec128_as_i16x8 (mm_sub_epi16 lhs rhs)
           == Spec.Utils.map2 ( -. ) (vec128_as_i16x8 lhs) (vec128_as_i16x8 rhs))
          [SMTPat (vec128_as_i16x8 (mm_sub_epi16 lhs rhs))] =
  reveal_opaque (`%mm_sub_epi16) mm_sub_epi16;
  Canon.lemma_mm_sub_epi16 lhs rhs;
  Seq.lemma_eq_intro (vec128_as_i16x8 (mm_sub_epi16 lhs rhs))
                     (Spec.Utils.map2 ( -. ) (vec128_as_i16x8 lhs) (vec128_as_i16x8 rhs))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_mm_mullo_epi16 (lhs rhs: t_Vec128)
  : Lemma (vec128_as_i16x8 (mm_mullo_epi16 lhs rhs)
           == Spec.Utils.map2 mul_mod (vec128_as_i16x8 lhs) (vec128_as_i16x8 rhs))
          [SMTPat (vec128_as_i16x8 (mm_mullo_epi16 lhs rhs))] =
  reveal_opaque (`%mm_mullo_epi16) mm_mullo_epi16;
  Canon.lemma_mm_mullo_epi16 lhs rhs;
  Seq.lemma_eq_intro (vec128_as_i16x8 (mm_mullo_epi16 lhs rhs))
                     (Spec.Utils.map2 mul_mod (vec128_as_i16x8 lhs) (vec128_as_i16x8 rhs))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 200"
let lemma_mm_mulhi_epi16 (lhs rhs: t_Vec128)
  : Lemma (vec128_as_i16x8 (mm_mulhi_epi16 lhs rhs)
           == Spec.Utils.map2 (fun x y ->
                cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16)
              (vec128_as_i16x8 lhs)
              (vec128_as_i16x8 rhs))
          [SMTPat (vec128_as_i16x8 (mm_mulhi_epi16 lhs rhs))] =
  reveal_opaque (`%mm_mulhi_epi16) mm_mulhi_epi16;
  Canon.lemma_mm_mulhi_epi16 lhs rhs;
  Seq.lemma_eq_intro (vec128_as_i16x8 (mm_mulhi_epi16 lhs rhs))
              (Spec.Utils.map2 (fun x y ->
                cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16)
              (vec128_as_i16x8 lhs)
              (vec128_as_i16x8 rhs))
#pop-options

#push-options "--fuel 2 --ifuel 1 --z3rlimit 150"
let lemma_mm_set1_epi16 (constant: i16)
  : Lemma (vec128_as_i16x8 (mm_set1_epi16 constant) == Spec.Utils.create (sz 8) constant)
          [SMTPat (vec128_as_i16x8 (mm_set1_epi16 constant))] =
  reveal_opaque (`%mm_set1_epi16) mm_set1_epi16;
  Canon.lemma_mm_set1_epi16 constant;
  Seq.lemma_eq_intro (vec128_as_i16x8 (mm_set1_epi16 constant))
                     (Spec.Utils.create (sz 8) constant)
#pop-options

(* ── Bit-function view of a core-models t_BitVec ──────────────────────────────
   core-models `t_BitVec N` is structurally a bit-array (a `t_FunArray` of
   `t_Bit`).  `bv_bit` is its DEFINITIONAL view as a bit function
   (`nat -> bit`): it reads bit `i` through the `t_Index` instance and maps
   `t_Bit` to `{0,1}`.  This is a DEFINITION, NOT a trusted axiom.  The
   Serialize/Compress/Sampling bit proofs apply the vector via `bv_bit`. *)
let bv_bit (#n: u64) (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec n)
           (i: nat{i < v n}) : Rust_primitives.Integers.bit =
  match bv.[ mk_u64 i ] <: Libcrux_core_models.Abstractions.Bit.t_Bit with
  | Libcrux_core_models.Abstractions.Bit.Bit_One  -> 1
  | Libcrux_core_models.Abstractions.Bit.Bit_Zero -> 0

(* Cast / extract preserve the underlying bits — PROVEN from the transparent
   core-models ops (`e_mm256_castsi256_si128 v = from_fn (fun i -> v.[i])`,
   `e_mm256_extracti128_si256 1 v = from_fn (fun i -> v.[i + 128])`).  NOT
   trusted axioms; they discharge the `cast vc k == vc k` step the
   Compress/Ntt bit proofs rely on. *)
let lemma_bv_bit_castsi256_si128 (vc: t_Vec256) (k: nat{k < 128})
  : Lemma (bv_bit (mm256_castsi256_si128 vc) k == bv_bit vc k)
  = reveal_opaque (`%mm256_castsi256_si128) mm256_castsi256_si128

let lemma_bv_bit_extracti128_si256_1 (vc: t_Vec256) (k: nat{k < 128})
  : Lemma (bv_bit (mm256_extracti128_si256 (mk_i32 1) vc) k == bv_bit vc (k + 128))
  = reveal_opaque (`%mm256_extracti128_si256) mm256_extracti128_si256

(* ── Bit-level lane-view bridges (over core-models `t_BitVec`) ─────────────────
   The i16x16 / i16x8 lane view's `d`-bit-per-element serialization at bit `i`
   equals raw bit `(i/d)*16 + i%d` of the underlying vector.

   PROVEN: over core-models the view is the CONCRETE codec, so this is exactly
   the canonical read-back lemma `Canon.lemma_readback` at I16 — bit `b` of lane
   `l` of the `to_iv` view IS raw bit `16*l + b`.  This is the bridge the
   bit-level modules (Serialize / Sampling / top-Avx2) consume; proving it
   (rather than assuming it) removes the last representational assumption between
   the lane view and the raw bit vector. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 250"
let bit_vec_of_int_t_array_vec256_as_i16x16_lemma
      (vec: t_Vec256) (d: nat{d > 0 /\ d <= 16}) (i: nat{i < 16 * d})
    : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array (vec256_as_i16x16 vec) d i
             == bv_bit vec ((i / d) * 16 + i % d)) =
  FStar.Math.Lemmas.euclidean_division_definition i d;
  FStar.Math.Lemmas.cancel_mul_div 16 d;
  FStar.Math.Lemmas.lemma_div_le i (16 * d) d;
  assert (i / d <= 16);
  assert (i / d < 16);
  assert (i % d < 16);
  Canon.lemma_readback Rust_primitives.Integers.I16 (mk_u64 256) (mk_u64 16) vec
    (mk_u64 (i / d)) (i % d)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 250"
let bit_vec_of_int_t_array_vec128_as_i16x8_lemma
      (vec: t_Vec128) (d: nat{d > 0 /\ d <= 16}) (i: nat{i < 8 * d})
    : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array (vec128_as_i16x8 vec) d i
             == bv_bit vec ((i / d) * 16 + i % d)) =
  FStar.Math.Lemmas.euclidean_division_definition i d;
  FStar.Math.Lemmas.cancel_mul_div 8 d;
  FStar.Math.Lemmas.lemma_div_le i (8 * d) d;
  assert (i / d <= 8);
  assert (i / d < 8);
  assert (i % d < 16);
  Canon.lemma_readback Rust_primitives.Integers.I16 (mk_u64 128) (mk_u64 8) vec
    (mk_u64 (i / d)) (i % d)
#pop-options

(* ============================================================================
   DEFERRED — bit-level bridges + byte store/load whose reference shapes are
   parked here.  They need a `t_BitVec`-struct representation adaptation: the
   reference form below applies the underlying vector as a `bit_vec` FUNCTION
   (`v (idx)`), invalid over core-models `t_BitVec`; the replacement indexes via
   `t_Index` / `to_vec`.  The reference `val` shapes are kept below so the exact
   fact shapes are recoverable.

   val bit_vec_of_int_t_array_vec256_as_i16x16_lemma
         (v: bit_vec 256) (d: nat{d > 0 /\ d <= 16}) (i: nat{i < 16 * d})
       : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array
                 (vec256_as_i16x16 v) d i == v ((i / d) * 16 + i % d))

   val bit_vec_of_int_t_array_vec128_as_i16x8_lemma
         (v: bit_vec 128) (d: nat{d > 0 /\ d <= 16}) (i: nat{i < 8 * d})
       : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array
                 (vec128_as_i16x8 v) d i == v ((i / d) * 16 + i % d))

   lemma_mm256_storeu_si256_u8_bit_vec (output: t_Slice u8) (vector: t_Vec256)
     : Lemma (requires Core_models.Slice.impl__len #u8 output == mk_usize 32)
             (ensures (let output_future = mm256_storeu_si256_u8 output vector in
                Core_models.Slice.impl__len #u8 output_future ==
                  Core_models.Slice.impl__len #u8 output /\
                (let output_arr: t_Array u8 (sz 32) = output_future in
                 BitVecEq.bit_vec_equal
                   (Rust_primitives.BitVectors.bit_vec_of_int_t_array output_arr 8) vector)))
       [SMTPat (mm256_storeu_si256_u8 output vector)]

   lemma_mm256_loadu_si256_u8_bit_vec (input: t_Slice u8)
       : Lemma (requires Core_models.Slice.impl__len #u8 input == mk_usize 32)
               (ensures (let input_arr: t_Array u8 (sz 32) = input in
                 BitVecEq.bit_vec_equal (mm256_loadu_si256_u8 input)
                   (Rust_primitives.BitVectors.bit_vec_of_int_t_array input_arr 8)))
       [SMTPat (mm256_loadu_si256_u8 input)]

   mm256_cmpgt_epi16 bit-form: forall (i: nat{i < 256}). result i ==
     (if (vec256_as_i16x16 lhs).[i/16] > (vec256_as_i16x16 rhs).[i/16] then 1 else 0)
   ========================================================================== *)

(* ============================================================================
   Per-lane GROUND corollary of `lemma_mm256_set_epi16`.

   `lemma_mm256_set_epi16` hands consumers the `Spec.Utils.create16` FORM of the
   lane view.  Every `Ntt` layer step then needs the 16 lane VALUES, and deriving
   them from that form forces Z3 to chase, per lane, the chain
     get_lane r k -> Seq.index (vec256_as_i16x16 r) k -> Seq.index (create16 ...) k
   with `vec256_index`'s SMTPat offering a competing (dead-end, opaque-codec)
   rewrite of the same term -- 16 times, inside the layer step's heavy
   `--split_queries always` context.  That saturates.

   This corollary pays the `create16` index chain ONCE, here, in a clean context,
   and hands consumers 16 GROUND equalities instead.  It reuses the trigger term
   of the `create16` lemma above, so no new trigger SHAPE enters any consumer.
   ============================================================================ *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm256_set_epi16_lanes (v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0: i16)
  : Lemma
    (ensures
      (let r = mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0 in
       get_lane r 0 == v0 /\ get_lane r 1 == v1 /\ get_lane r 2 == v2 /\ get_lane r 3 == v3 /\
       get_lane r 4 == v4 /\ get_lane r 5 == v5 /\ get_lane r 6 == v6 /\ get_lane r 7 == v7 /\
       get_lane r 8 == v8 /\ get_lane r 9 == v9 /\ get_lane r 10 == v10 /\ get_lane r 11 == v11 /\
       get_lane r 12 == v12 /\ get_lane r 13 == v13 /\ get_lane r 14 == v14 /\
       get_lane r 15 == v15))
    [SMTPat (vec256_as_i16x16 (mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0))]
  = lemma_mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0
#pop-options

(* Bound corollary of `lemma_mm256_set_epi16`, in IMPLICATION form.

   `Spec.Utils.is_i16b_array b arr` is a `forall i` over a SYMBOLIC index, so the
   16 ground lane equalities above cannot discharge it — that needs the
   `create16` if-ladder at a symbolic `i`, which is exactly the expensive step we
   are keeping out of consumer contexts.  So prove it here, once, universally in
   the bound `b`: consumers get the bound by instantiating `b` and discharging 16
   GROUND `is_i16b` hypotheses (their own `requires` plus literal lanes).

   Same trigger term as the two lemmas above; the inner `forall b` carries its own
   goal-directed pattern. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_mm256_set_epi16_bound (v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0: i16)
  : Lemma
    (ensures
      (let r = mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0 in
       forall (b: nat).
         {:pattern (Spec.Utils.is_i16b_array b (vec256_as_i16x16 r))}
         (Spec.Utils.is_i16b b v0 /\ Spec.Utils.is_i16b b v1 /\ Spec.Utils.is_i16b b v2 /\
          Spec.Utils.is_i16b b v3 /\ Spec.Utils.is_i16b b v4 /\ Spec.Utils.is_i16b b v5 /\
          Spec.Utils.is_i16b b v6 /\ Spec.Utils.is_i16b b v7 /\ Spec.Utils.is_i16b b v8 /\
          Spec.Utils.is_i16b b v9 /\ Spec.Utils.is_i16b b v10 /\ Spec.Utils.is_i16b b v11 /\
          Spec.Utils.is_i16b b v12 /\ Spec.Utils.is_i16b b v13 /\ Spec.Utils.is_i16b b v14 /\
          Spec.Utils.is_i16b b v15) ==>
         Spec.Utils.is_i16b_array b (vec256_as_i16x16 r)))
    [SMTPat (vec256_as_i16x16 (mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0))] =
  lemma_mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0
#pop-options

(* ── `mm256_cmpgt_epi16`: lane form, and the bit-0 form its consumers need ─────
   Both facts below are PROVEN: over core-models the op IS modeled,
   `IV.e_mm256_cmpgt_epi16` yielding `mk_i16 (-1)` or `mk_i16 0` per lane.

   Stated at TWO granularities on purpose.  The lane form is the primitive; the
   bit form is what the consumers actually ask for, and note their bit index is
   `16 * l` — a symbolic LANE at a CONCRETE bit offset (`Serialize.serialize_1_`'s
   post reads `vector (i * 16)`).  Keeping the offset ground is what makes the
   bit form cheap: a symbolic-offset version needs "every bit of `mk_i16 (-1)` is
   set", which is a 16-way `pow2` argument Z3 will not find (measured: it gives up
   at rlimit 5.9 of 300, i.e. a trigger gap, not a budget one). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_lane_mm256_cmpgt_epi16 (lhs rhs: t_Vec256) (l: nat{l < 16})
  : Lemma (Seq.index (vec256_as_i16x16 (mm256_cmpgt_epi16 lhs rhs)) l
           == (if Seq.index (vec256_as_i16x16 lhs) l >. Seq.index (vec256_as_i16x16 rhs) l
               then mk_i16 (-1) else mk_i16 0)) =
  reveal_opaque (`%mm256_cmpgt_epi16) mm256_cmpgt_epi16;
  Canon.lemma_mm256_cmpgt_epi16 lhs rhs;
  assert (Funarr.impl_5__get (mk_u64 16) #i16
            (IV.e_mm256_cmpgt_epi16 (Canon.to_i16x16 lhs) (Canon.to_i16x16 rhs)) (mk_u64 l)
          == (if Funarr.impl_5__get (mk_u64 16) #i16 (Canon.to_i16x16 lhs) (mk_u64 l) >.
                 Funarr.impl_5__get (mk_u64 16) #i16 (Canon.to_i16x16 rhs) (mk_u64 l)
              then mk_i16 (-1) else mk_i16 0))
    by (FStar.Tactics.norm [delta_only [`%IV.e_mm256_cmpgt_epi16]; iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit0_mm256_cmpgt_epi16 (lhs rhs: t_Vec256) (l: nat{l < 16})
  : Lemma (bv_bit (mm256_cmpgt_epi16 lhs rhs) (16 * l)
           == (if Seq.index (vec256_as_i16x16 lhs) l >. Seq.index (vec256_as_i16x16 rhs) l
               then 1 else 0)) =
  reveal_opaque (`%mm256_cmpgt_epi16) mm256_cmpgt_epi16;
  let r = mm256_cmpgt_epi16 lhs rhs in
  lemma_lane_mm256_cmpgt_epi16 lhs rhs l;
  (* bit 0 of i16 lane `l` IS raw bit `16*l` *)
  Canon.lemma_readback Rust_primitives.Integers.I16 (mk_u64 256) (mk_u64 16) r (mk_u64 l) 0;
  (* ground: bit 0 of -1 is set, bit 0 of 0 is clear *)
  reveal_opaque (`%Rust_primitives.Integers.get_bit)
                (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.I16);
  assert_norm (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.I16 (mk_i16 (-1))
                 (sz 0) == 1);
  assert_norm (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.I16 (mk_i16 0)
                 (sz 0) == 0)
#pop-options

(* ============================================================================
   SERIALIZE / SAMPLING bit proofs
   ============================================================================ *)

module IVi = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp

(* ── the bv_bit <-> lane_reader collapse (definitional; both sides read
   `bv._0` at index `w*l + b`, and `bval` is exactly `bv_bit`'s Bit match). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_bv_bit_reader (#n: u64) (w: pos)
    (bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec n)
    (l: nat) (b: nat{b < w /\ w * l + b < v n})
  : Lemma (IVi.bval (IVi.lane_reader n w bv (mk_u64 l) b) == bv_bit bv (w * l + b)) =
  FStar.Math.Lemmas.lemma_mult_le_right l 1 w;
  assert (l <= w * l)
#pop-options

(* ── PROVEN slice-I/O semantics ─────────────────────────────────────────────
   The memory-op WRAPPERS in `crates/utils/intrinsics/src/avx2.rs` carry
   extractable model bodies under the hax cfg: they delegate to the slice-I/O
   models in core-models' `Extra` module (loads: `from_iv` over a guarded
   byte/lane read; stores: a ground spine of guarded per-lane writes), so the
   facts below are PROVEN from the concrete definitions — reveals of the
   (opaque) wrapper + model, the canonical `to_iv`/`from_iv` round-trip
   (`IVi.lemma_conv_rt`), `Canon.lemma_readback`, and `lemma_bv_bit_reader`.
   The models are differentially tested against the real intrinsics
   (`*_model_diff` tests in core-models interpretations.rs, including the
   i16-store test, plus host-independent round-trips).
   No trusted-extern axioms remain for slice-I/O.  The u8 256-bit pair (consumed
   by `Vector.Avx2` from_bytes/to_bytes) is proven here too, so no new axiom is
   ever needed for it. *)

module Extra = Libcrux_core_models.Core_arch.X86.Extra

(* from_fn-at-index reduction at width 32 (16/8 twins earlier in the file). *)
let index_from_fn32 (#t: Type0) (g: (i: u64{v i < 32}) -> t) (i: nat{i < 32})
  : Lemma (Funarr.impl_5__get (mk_u64 32) #t
             (Funarr.impl_5__from_fn (mk_u64 32) #t #(u64 -> t) g) (mk_u64 i)
           == g (mk_u64 i))
  = ()

(* Load models: lane `l` of the canonical `to_iv` view is the input element.
   `to_iv (from_iv arr)` collapses by `IVi.lemma_conv_rt` (SMTPat); the guarded
   from_fn read reduces at the in-range index. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_loadu_si128_lane (input: t_Slice u8) (l: nat{l < 16})
  : Lemma (requires Seq.length input == 16)
          (ensures Funarr.impl_5__get (mk_u64 16) #u8
                     (IVi.to_iv Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16)
                        (Extra.mm_loadu_si128_model input)) (mk_u64 l)
                   == Seq.index input l) =
  reveal_opaque (`%Extra.mm_loadu_si128_model) Extra.mm_loadu_si128_model
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_loadu_si256_u8_lane (input: t_Slice u8) (l: nat{l < 32})
  : Lemma (requires Seq.length input == 32)
          (ensures Funarr.impl_5__get (mk_u64 32) #u8
                     (IVi.to_iv Rust_primitives.Integers.U8 (mk_u64 256) (mk_u64 32)
                        (Extra.mm256_loadu_si256_u8_model input)) (mk_u64 l)
                   == Seq.index input l) =
  reveal_opaque (`%Extra.mm256_loadu_si256_u8_model) Extra.mm256_loadu_si256_u8_model
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_loadu_si256_i16_lane (input: t_Slice i16) (l: nat{l < 16})
  : Lemma (requires Seq.length input == 16)
          (ensures Funarr.impl_5__get (mk_u64 16) #i16
                     (Canon.to_i16x16 (Extra.mm256_loadu_si256_i16_model input)) (mk_u64 l)
                   == Seq.index input l) =
  reveal_opaque (`%Extra.mm256_loadu_si256_i16_model) Extra.mm256_loadu_si256_i16_model
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_mm_loadu_si128 (input: t_Slice u8) (i: nat{i < 128})
  : Lemma (requires Seq.length input == 16)
          (ensures bv_bit (mm_loadu_si128 input) i ==
                   Rust_primitives.Integers.get_bit (Seq.index input (i / 8)) (sz (i % 8))) =
  reveal_opaque (`%mm_loadu_si128) mm_loadu_si128;
  let bv = mm_loadu_si128 input in
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  lemma_bv_bit_reader #(mk_u64 128) 8 bv (i / 8) (i % 8);
  Canon.lemma_readback Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) bv
    (mk_u64 (i / 8)) (i % 8);
  lemma_loadu_si128_lane input (i / 8)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_mm256_loadu_si256_u8 (input: t_Slice u8) (i: nat{i < 256})
  : Lemma (requires Seq.length input == 32)
          (ensures bv_bit (mm256_loadu_si256_u8 input) i ==
                   Rust_primitives.Integers.get_bit (Seq.index input (i / 8)) (sz (i % 8))) =
  reveal_opaque (`%mm256_loadu_si256_u8) mm256_loadu_si256_u8;
  let bv = mm256_loadu_si256_u8 input in
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  lemma_bv_bit_reader #(mk_u64 256) 8 bv (i / 8) (i % 8);
  Canon.lemma_readback Rust_primitives.Integers.U8 (mk_u64 256) (mk_u64 32) bv
    (mk_u64 (i / 8)) (i % 8);
  lemma_loadu_si256_u8_lane input (i / 8)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_mm256_loadu_si256_i16 (input: t_Slice i16)
  : Lemma (requires Seq.length input == 16)
          (ensures vec256_as_i16x16 (mm256_loadu_si256_i16 input) == input) =
  reveal_opaque (`%mm256_loadu_si256_i16) mm256_loadu_si256_i16;
  let bv = mm256_loadu_si256_i16 input in
  let aux (l: nat{l < 16}) : Lemma (Seq.index (vec256_as_i16x16 bv) l == Seq.index input l) =
    lemma_loadu_si256_i16_lane input l
  in
  FStar.Classical.forall_intro aux;
  Seq.lemma_eq_intro (vec256_as_i16x16 bv) input
#pop-options

(* Store models: the extracted body is a ground spine of guarded per-lane
   `update_at_usize` writes of the canonical `to_iv` lanes; under the length
   hypothesis every guard is true, so lane `j` of the result is `to_iv` lane
   `j` and the tail is framed. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_mm_storeu_si128 (output: t_Slice i16) (vector: t_Vec128)
  : Lemma (requires Seq.length output >= 8)
          (ensures (let output' = mm_storeu_si128 output vector in
                    Seq.length output' == Seq.length output /\
                    (forall (j: nat{j < 8}).
                       Seq.index output' j == Seq.index (vec128_as_i16x8 vector) j) /\
                    (forall (j: nat{j < Seq.length output}).
                       j >= 8 ==> Seq.index output' j == Seq.index output j))) =
  reveal_opaque (`%mm_storeu_si128) mm_storeu_si128;
  reveal_opaque (`%Extra.mm_storeu_si128_i16_model) Extra.mm_storeu_si128_i16_model
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_mm_storeu_bytes_si128 (output: t_Slice u8) (vector: t_Vec128)
  : Lemma (requires Seq.length output == 16)
          (ensures (let output' = mm_storeu_bytes_si128 output vector in
                    Seq.length output' == 16 /\
                    (forall (i: nat{i < 128}).
                       Rust_primitives.BitVectors.bit_vec_of_int_t_array
                         (output' <: t_Array u8 (sz 16)) 8 i ==
                       bv_bit vector i))) =
  reveal_opaque (`%mm_storeu_bytes_si128) mm_storeu_bytes_si128;
  reveal_opaque (`%Extra.mm_storeu_bytes_si128_model) Extra.mm_storeu_bytes_si128_model;
  let output' = mm_storeu_bytes_si128 output vector in
  let lanes = IVi.e_ee_17__impl__to_u8x16 vector in
  let s0 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize output (mk_usize 0) (lanes.[ mk_u64 0 ] <: u8) in
  let s1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s0 (mk_usize 1) (lanes.[ mk_u64 1 ] <: u8) in
  let s2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s1 (mk_usize 2) (lanes.[ mk_u64 2 ] <: u8) in
  let s3 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s2 (mk_usize 3) (lanes.[ mk_u64 3 ] <: u8) in
  let s4 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s3 (mk_usize 4) (lanes.[ mk_u64 4 ] <: u8) in
  let s5 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s4 (mk_usize 5) (lanes.[ mk_u64 5 ] <: u8) in
  let s6 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s5 (mk_usize 6) (lanes.[ mk_u64 6 ] <: u8) in
  let s7 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s6 (mk_usize 7) (lanes.[ mk_u64 7 ] <: u8) in
  let s8 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s7 (mk_usize 8) (lanes.[ mk_u64 8 ] <: u8) in
  let s9 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s8 (mk_usize 9) (lanes.[ mk_u64 9 ] <: u8) in
  let s10 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s9 (mk_usize 10) (lanes.[ mk_u64 10 ] <: u8) in
  let s11 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s10 (mk_usize 11) (lanes.[ mk_u64 11 ] <: u8) in
  let s12 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s11 (mk_usize 12) (lanes.[ mk_u64 12 ] <: u8) in
  let s13 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s12 (mk_usize 13) (lanes.[ mk_u64 13 ] <: u8) in
  let s14 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s13 (mk_usize 14) (lanes.[ mk_u64 14 ] <: u8) in
  let s15 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s14 (mk_usize 15) (lanes.[ mk_u64 15 ] <: u8) in
  assert (output' == s15);
  assert (lanes == IVi.to_iv Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) vector);
  let aux (i: nat{i < 128})
    : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array
               (output' <: t_Array u8 (sz 16)) 8 i == bv_bit vector i) =
    FStar.Math.Lemmas.euclidean_division_definition i 8;
    Canon.lemma_readback Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) vector
      (mk_u64 (i / 8)) (i % 8);
    lemma_bv_bit_reader #(mk_u64 128) 8 vector (i / 8) (i % 8);
    assert (Seq.index output' (i / 8) ==
            Funarr.impl_5__get (mk_u64 16) #u8
              (IVi.to_iv Rust_primitives.Integers.U8 (mk_u64 128) (mk_u64 16) vector)
              (mk_u64 (i / 8)))
  in
  FStar.Classical.forall_intro aux
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_mm256_storeu_si256_i16 (output: t_Slice i16) (vector: t_Vec256)
  : Lemma (requires Seq.length output == 16)
          (ensures mm256_storeu_si256_i16 output vector == vec256_as_i16x16 vector) =
  reveal_opaque (`%mm256_storeu_si256_i16) mm256_storeu_si256_i16;
  reveal_opaque (`%Extra.mm256_storeu_si256_i16_model) Extra.mm256_storeu_si256_i16_model;
  let output' = mm256_storeu_si256_i16 output vector in
  let lanes = IVi.e_ee_2__impl__to_i16x16 vector in
  let s0 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize output (mk_usize 0) (lanes.[ mk_u64 0 ] <: i16) in
  let s1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s0 (mk_usize 1) (lanes.[ mk_u64 1 ] <: i16) in
  let s2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s1 (mk_usize 2) (lanes.[ mk_u64 2 ] <: i16) in
  let s3 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s2 (mk_usize 3) (lanes.[ mk_u64 3 ] <: i16) in
  let s4 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s3 (mk_usize 4) (lanes.[ mk_u64 4 ] <: i16) in
  let s5 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s4 (mk_usize 5) (lanes.[ mk_u64 5 ] <: i16) in
  let s6 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s5 (mk_usize 6) (lanes.[ mk_u64 6 ] <: i16) in
  let s7 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s6 (mk_usize 7) (lanes.[ mk_u64 7 ] <: i16) in
  let s8 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s7 (mk_usize 8) (lanes.[ mk_u64 8 ] <: i16) in
  let s9 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s8 (mk_usize 9) (lanes.[ mk_u64 9 ] <: i16) in
  let s10 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s9 (mk_usize 10) (lanes.[ mk_u64 10 ] <: i16) in
  let s11 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s10 (mk_usize 11) (lanes.[ mk_u64 11 ] <: i16) in
  let s12 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s11 (mk_usize 12) (lanes.[ mk_u64 12 ] <: i16) in
  let s13 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s12 (mk_usize 13) (lanes.[ mk_u64 13 ] <: i16) in
  let s14 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s13 (mk_usize 14) (lanes.[ mk_u64 14 ] <: i16) in
  let s15 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s14 (mk_usize 15) (lanes.[ mk_u64 15 ] <: i16) in
  assert (output' == s15);
  let aux (j: nat{j < 16})
    : Lemma (Seq.index output' j == Seq.index (vec256_as_i16x16 vector) j) = ()
  in
  FStar.Classical.forall_intro aux;
  Seq.lemma_eq_intro output' (vec256_as_i16x16 vector)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_mm256_storeu_si256_u8 (output: t_Slice u8) (vector: t_Vec256)
  : Lemma (requires Seq.length output == 32)
          (ensures (let output' = mm256_storeu_si256_u8 output vector in
                    Seq.length output' == 32 /\
                    (forall (i: nat{i < 256}).
                       Rust_primitives.BitVectors.bit_vec_of_int_t_array
                         (output' <: t_Array u8 (sz 32)) 8 i ==
                       bv_bit vector i))) =
  reveal_opaque (`%mm256_storeu_si256_u8) mm256_storeu_si256_u8;
  reveal_opaque (`%Extra.mm256_storeu_si256_u8_model) Extra.mm256_storeu_si256_u8_model;
  let output' = mm256_storeu_si256_u8 output vector in
  let lanes = IVi.e_ee_8__impl__to_u8x32 vector in
  let s0 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize output (mk_usize 0) (lanes.[ mk_u64 0 ] <: u8) in
  let s1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s0 (mk_usize 1) (lanes.[ mk_u64 1 ] <: u8) in
  let s2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s1 (mk_usize 2) (lanes.[ mk_u64 2 ] <: u8) in
  let s3 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s2 (mk_usize 3) (lanes.[ mk_u64 3 ] <: u8) in
  let s4 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s3 (mk_usize 4) (lanes.[ mk_u64 4 ] <: u8) in
  let s5 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s4 (mk_usize 5) (lanes.[ mk_u64 5 ] <: u8) in
  let s6 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s5 (mk_usize 6) (lanes.[ mk_u64 6 ] <: u8) in
  let s7 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s6 (mk_usize 7) (lanes.[ mk_u64 7 ] <: u8) in
  let s8 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s7 (mk_usize 8) (lanes.[ mk_u64 8 ] <: u8) in
  let s9 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s8 (mk_usize 9) (lanes.[ mk_u64 9 ] <: u8) in
  let s10 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s9 (mk_usize 10) (lanes.[ mk_u64 10 ] <: u8) in
  let s11 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s10 (mk_usize 11) (lanes.[ mk_u64 11 ] <: u8) in
  let s12 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s11 (mk_usize 12) (lanes.[ mk_u64 12 ] <: u8) in
  let s13 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s12 (mk_usize 13) (lanes.[ mk_u64 13 ] <: u8) in
  let s14 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s13 (mk_usize 14) (lanes.[ mk_u64 14 ] <: u8) in
  let s15 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s14 (mk_usize 15) (lanes.[ mk_u64 15 ] <: u8) in
  let s16 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s15 (mk_usize 16) (lanes.[ mk_u64 16 ] <: u8) in
  let s17 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s16 (mk_usize 17) (lanes.[ mk_u64 17 ] <: u8) in
  let s18 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s17 (mk_usize 18) (lanes.[ mk_u64 18 ] <: u8) in
  let s19 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s18 (mk_usize 19) (lanes.[ mk_u64 19 ] <: u8) in
  let s20 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s19 (mk_usize 20) (lanes.[ mk_u64 20 ] <: u8) in
  let s21 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s20 (mk_usize 21) (lanes.[ mk_u64 21 ] <: u8) in
  let s22 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s21 (mk_usize 22) (lanes.[ mk_u64 22 ] <: u8) in
  let s23 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s22 (mk_usize 23) (lanes.[ mk_u64 23 ] <: u8) in
  let s24 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s23 (mk_usize 24) (lanes.[ mk_u64 24 ] <: u8) in
  let s25 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s24 (mk_usize 25) (lanes.[ mk_u64 25 ] <: u8) in
  let s26 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s25 (mk_usize 26) (lanes.[ mk_u64 26 ] <: u8) in
  let s27 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s26 (mk_usize 27) (lanes.[ mk_u64 27 ] <: u8) in
  let s28 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s27 (mk_usize 28) (lanes.[ mk_u64 28 ] <: u8) in
  let s29 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s28 (mk_usize 29) (lanes.[ mk_u64 29 ] <: u8) in
  let s30 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s29 (mk_usize 30) (lanes.[ mk_u64 30 ] <: u8) in
  let s31 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s30 (mk_usize 31) (lanes.[ mk_u64 31 ] <: u8) in
  assert (output' == s31);
  assert (lanes == IVi.to_iv Rust_primitives.Integers.U8 (mk_u64 256) (mk_u64 32) vector);
  (* split the 32-deep index walk in half: each forall is a <=16-step chain *)
  assert (forall (l: nat{l < 32}). l >= 16 ==> Seq.index output' l == (lanes.[ mk_u64 l ] <: u8));
  assert (forall (l: nat{l < 16}). Seq.index s15 l == (lanes.[ mk_u64 l ] <: u8));
  assert (forall (l: nat{l < 16}). Seq.index s23 l == Seq.index s15 l);
  assert (forall (l: nat{l < 16}). Seq.index output' l == Seq.index s23 l);
  assert (forall (l: nat{l < 16}). Seq.index output' l == Seq.index s15 l);
  assert (forall (l: nat{l < 32}). Seq.index output' l == (lanes.[ mk_u64 l ] <: u8));
  let aux (i: nat{i < 256})
    : Lemma (Rust_primitives.BitVectors.bit_vec_of_int_t_array
               (output' <: t_Array u8 (sz 32)) 8 i == bv_bit vector i) =
    FStar.Math.Lemmas.euclidean_division_definition i 8;
    Canon.lemma_readback Rust_primitives.Integers.U8 (mk_u64 256) (mk_u64 32) vector
      (mk_u64 (i / 8)) (i % 8);
    lemma_bv_bit_reader #(mk_u64 256) 8 vector (i / 8) (i % 8);
    assert (Seq.index output' (i / 8) ==
            Funarr.impl_5__get (mk_u64 32) #u8
              (IVi.to_iv Rust_primitives.Integers.U8 (mk_u64 256) (mk_u64 32) vector)
              (mk_u64 (i / 8)))
  in
  FStar.Classical.forall_intro aux
#pop-options

(* ── PROVEN serialize_1 machinery (the A1 sign analog) ──────────────────────── *)

(* value of an i16 lane shifted left by 15 (as u16, cast back): only the parity
   of the input lane survives, as the sign bit. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_shl15_value (x: i16)
  : Lemma (v (cast ((cast x <: u16) <<! mk_i32 15 <: u16) <: i16) ==
           (if v x % 2 = 1 then -32768 else 0)) =
  let xu : u16 = cast x <: u16 in
  let sh : u16 = xu <<! mk_i32 15 in
  assert_norm (pow2 15 == 32768); assert_norm (pow2 16 == 65536);
  FStar.Math.Lemmas.pow2_multiplication_modulo_lemma_2 (v xu) 16 15;
  FStar.Math.Lemmas.modulo_modulo_lemma (v x) 2 32768
#pop-options

(* (A1) sign bit of byte i of `packs(cast(slli15 v), extract1(slli15 v))` ==
   raw bit 16*i of v — PROVEN from the canonical per-lane facts (slli16
   value, 128-half transfers, packs saturation). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_slli15_packs_sign (vector: t_Vec256) (i: nat{i < 16})
  : Lemma
      (let s = mm256_slli_epi16 (mk_i32 15) vector in
       let msbs = mm_packs_epi16 (mm256_castsi256_si128 s) (mm256_extracti128_si256 (mk_i32 1) s) in
       Canon.sign_bit8 (Canon.to_i8x16 msbs) i == bv_bit vector (16 * i)) =
  reveal_opaque (`%mm256_slli_epi16) mm256_slli_epi16;
  reveal_opaque (`%mm256_castsi256_si128) mm256_castsi256_si128;
  reveal_opaque (`%mm256_extracti128_si256) mm256_extracti128_si256;
  reveal_opaque (`%mm_packs_epi16) mm_packs_epi16;
  let s = mm256_slli_epi16 (mk_i32 15) vector in
  let lo = mm256_castsi256_si128 s in
  let hi = mm256_extracti128_si256 (mk_i32 1) s in
  let msbs = mm_packs_epi16 lo hi in
  Canon.lemma_mm256_slli_epi16 (mk_i32 15) vector;
  Canon.lemma_iv_slli16 (mk_i32 15) (Canon.to_i16x16 vector) i;
  Canon.lemma_mm256_castsi256_si128 s;
  Canon.lemma_mm256_extracti128_si256 (mk_i32 1) s;
  (if i < 8
   then Canon.lemma_cast256_si128_lane_i16 s i
   else Canon.lemma_extracti128_1_lane_i16 s (i - 8));
  Canon.lemma_mm_packs_epi16 lo hi;
  Canon.lemma_iv_mm_packs_epi16 (Canon.to_i16x8 lo) (Canon.to_i16x8 hi) i;
  let x = Funarr.impl_5__get (mk_u64 16) #i16 (Canon.to_i16x16 vector) (mk_u64 i) in
  lemma_shl15_value x;
  Canon.lemma_readback Rust_primitives.Integers.I16 (mk_u64 256) (mk_u64 16) vector (mk_u64 i) 0;
  lemma_bv_bit_reader 16 vector i 0;
  reveal_opaque (`%Rust_primitives.Integers.get_bit)
                (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.I16);
  assert_norm (pow2 0 == 1); assert_norm (pow2 16 == 65536);
  FStar.Math.Lemmas.lemma_mod_plus (v x) 32768 2
#pop-options

(* movemask wrappers over the canonical PROVEN movemask companions *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mm_movemask_bound (a: t_Vec128)
  : Lemma (0 <= v (mm_movemask_epi8 a) /\ v (mm_movemask_epi8 a) < pow2 16) =
  reveal_opaque (`%mm_movemask_epi8) mm_movemask_epi8;
  Canon.lemma_mm_movemask_epi8 a;
  IV.e_movemask_bit_sum_i8_bound (Canon.to_i8x16 a) 0 16;
  assert_norm (pow2 16 == 65536)

let lemma_bv_bit_mm_movemask_epi8 (a: t_Vec128) (i: nat{i < 16})
  : Lemma ((v (mm_movemask_epi8 a) / pow2 i) % 2 == Canon.sign_bit8 (Canon.to_i8x16 a) i) =
  reveal_opaque (`%mm_movemask_epi8) mm_movemask_epi8;
  Canon.movemask_epi8_bit a i
#pop-options

(* byte packaging: bit i of the 2-byte array
   [x as u8; (x >> 8) as u8] == bit i of the movemask scalar x. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_bit_mod (x: nat) (n: nat{n >= 1}) (i: nat{i < n})
  : Lemma (((x % pow2 n) / pow2 i) % 2 == (x / pow2 i) % 2) =
  FStar.Math.Lemmas.pow2_modulo_division_lemma_1 x i n;
  FStar.Math.Lemmas.pow2_plus 1 (n - i - 1);
  FStar.Math.Lemmas.modulo_modulo_lemma (x / pow2 i) 2 (pow2 (n - i - 1))
#pop-options

#push-options "--fuel 0 --ifuel 0 --z3rlimit 200"
let lemma_cast_u8 (x: i32)
  : Lemma (requires 0 <= v x) (ensures v (cast (x <: i32) <: u8) == (v x) % pow2 8) =
  assert_norm (pow2 8 == 256)

let lemma_shift8 (x: i32)
  : Lemma (requires 0 <= v x /\ v x < pow2 16)
          (ensures v (x >>! mk_i32 8 <: i32) == (v x) / pow2 8) =
  assert_norm (pow2 8 == 256); assert_norm (pow2 16 == 65536)
#pop-options

(* THE serialize_1 bit obligation, assembled: `result` is the 2-byte packaging of
   the movemask of the slli15/packs chain, and its d=8 serialization at bit i is
   raw bit 16*i of the input vector.  Called from serialize.rs's proof! block
   with the extracted `bits_packed`/`result` terms. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_serialize_1_bits (vector: t_Vec256) (result: t_Array u8 (mk_usize 2)) (i: nat{i < 16})
  : Lemma
      (requires
        (let s = mm256_slli_epi16 (mk_i32 15) vector in
         let msbs = mm_packs_epi16 (mm256_castsi256_si128 s) (mm256_extracti128_si256 (mk_i32 1) s) in
         let bits_packed = mm_movemask_epi8 msbs in
         Seq.index result 0 == (cast (bits_packed <: i32) <: u8) /\
         Seq.index result 1 == (cast (bits_packed >>! mk_i32 8 <: i32) <: u8)))
      (ensures Rust_primitives.BitVectors.bit_vec_of_int_t_array result 8 i == bv_bit vector (16 * i)) =
  let s = mm256_slli_epi16 (mk_i32 15) vector in
  let msbs = mm_packs_epi16 (mm256_castsi256_si128 s) (mm256_extracti128_si256 (mk_i32 1) s) in
  let bits_packed = mm_movemask_epi8 msbs in
  lemma_mm_movemask_bound msbs;
  lemma_bv_bit_mm_movemask_epi8 msbs i;
  lemma_slli15_packs_sign vector i;
  (* res_bit: bit (i%8) of byte (i/8) == bit i of the scalar *)
  (if i < 8
   then begin
     lemma_cast_u8 bits_packed;
     lemma_bit_mod (v bits_packed) 8 i
   end
   else begin
     let j = i - 8 in
     lemma_shift8 bits_packed;
     lemma_cast_u8 (bits_packed >>! mk_i32 8 <: i32);
     lemma_bit_mod (v bits_packed / pow2 8) 8 j;
     FStar.Math.Lemmas.pow2_plus 8 j;
     FStar.Math.Lemmas.division_multiplication_lemma (v bits_packed) (pow2 8) (pow2 j)
   end);
  reveal_opaque (`%Rust_primitives.Integers.get_bit)
                (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.U8)
#pop-options

(* ── PROVEN 128-bit PSHUFB bit semantics (retires the Sampling_theory axiom) ── *)

(* weighted LSB-first bit sum of byte `nth` == the dsum2 lane reader fold *)
#push-options "--fuel 9 --ifuel 1 --z3rlimit 300"
let lemma_byte_bits_dsum2 (b: t_Vec128) (nth: nat{nth < 16})
  : Lemma (IVi.dsum2 (IVi.lane_reader (mk_u64 128) 8 b (mk_u64 nth)) 0 8 ==
           bv_bit b (8 * nth) + 2 * bv_bit b (8 * nth + 1) + 4 * bv_bit b (8 * nth + 2) +
           8 * bv_bit b (8 * nth + 3) + 16 * bv_bit b (8 * nth + 4) + 32 * bv_bit b (8 * nth + 5) +
           64 * bv_bit b (8 * nth + 6) + 128 * bv_bit b (8 * nth + 7)) =
  let f = IVi.lane_reader (mk_u64 128) 8 b (mk_u64 nth) in
  lemma_bv_bit_reader 8 b nth 0;
  lemma_bv_bit_reader 8 b nth 1;
  lemma_bv_bit_reader 8 b nth 2;
  lemma_bv_bit_reader 8 b nth 3;
  lemma_bv_bit_reader 8 b nth 4;
  lemma_bv_bit_reader 8 b nth 5;
  lemma_bv_bit_reader 8 b nth 6;
  lemma_bv_bit_reader 8 b nth 7;
  assert (IVi.dsum2 f 8 0 == 0);
  assert (IVi.dsum2 f 7 1 == IVi.bval (f 7) + 2 * IVi.dsum2 f 8 0);
  assert (IVi.dsum2 f 6 2 == IVi.bval (f 6) + 2 * IVi.dsum2 f 7 1);
  assert (IVi.dsum2 f 5 3 == IVi.bval (f 5) + 2 * IVi.dsum2 f 6 2);
  assert (IVi.dsum2 f 4 4 == IVi.bval (f 4) + 2 * IVi.dsum2 f 5 3);
  assert (IVi.dsum2 f 3 5 == IVi.bval (f 3) + 2 * IVi.dsum2 f 4 4);
  assert (IVi.dsum2 f 2 6 == IVi.bval (f 2) + 2 * IVi.dsum2 f 3 5);
  assert (IVi.dsum2 f 1 7 == IVi.bval (f 1) + 2 * IVi.dsum2 f 2 6);
  assert (IVi.dsum2 f 0 8 == IVi.bval (f 0) + 2 * IVi.dsum2 f 1 7)
#pop-options

(* PSHUFB (128-bit), full bit form: exactly the shape of
   Hacspec_ml_kem.Commute.Rej_table.shuffle_semantics — PROVEN, replacing the
   trusted `mm_shuffle_epi8_no_semantics_lemma` axiom (Sampling_theory). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_bv_bit_mm_shuffle_epi8 (a b: t_Vec128) (i: nat{i < 128})
  : Lemma (bv_bit (mm_shuffle_epi8 a b) i ==
           (let nth = i / 8 in
            let idx: nat =
              bv_bit b (8 * nth) + 2 * bv_bit b (8 * nth + 1) + 4 * bv_bit b (8 * nth + 2) +
              8 * bv_bit b (8 * nth + 3) + 16 * bv_bit b (8 * nth + 4) + 32 * bv_bit b (8 * nth + 5) +
              64 * bv_bit b (8 * nth + 6) + 128 * bv_bit b (8 * nth + 7) in
            if idx > 127 then 0 else bv_bit a ((idx % 16) * 8 + i % 8))) =
  reveal_opaque (`%mm_shuffle_epi8) mm_shuffle_epi8;
  Canon.lemma_mm_shuffle_epi8 a b;
  let nth = i / 8 in
  let sb = i % 8 in
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  lemma_byte_bits_dsum2 b nth;
  Canon.lemma_to_i8_val_128 b nth;
  IVi.dsum2_bound (IVi.lane_reader (mk_u64 128) 8 b (mk_u64 nth)) 0 8;
  let u : nat = IVi.dsum2 (IVi.lane_reader (mk_u64 128) 8 b (mk_u64 nth)) 0 8 in
  let r = mm_shuffle_epi8 a b in
  Canon.lemma_readback Rust_primitives.Integers.I8 (mk_u64 128) (mk_u64 16) r (mk_u64 nth) sb;
  lemma_bv_bit_reader 8 r nth sb;
  assert_norm (pow2 8 == 256);
  if u > 127
  then begin
    Canon.lemma_iv_mm_shuffle_epi8_neg (Canon.to_i8x16 a) (Canon.to_i8x16 b) nth;
    reveal_opaque (`%Rust_primitives.Integers.get_bit)
                  (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.I8)
  end
  else begin
    Canon.lemma_iv_mm_shuffle_epi8_sel (Canon.to_i8x16 a) (Canon.to_i8x16 b) nth;
    Canon.lemma_readback Rust_primitives.Integers.I8 (mk_u64 128) (mk_u64 16) a (mk_u64 (u % 16)) sb;
    lemma_bv_bit_reader 8 a (u % 16) sb
  end
#pop-options

(* ── deserialize_1 machinery: ground mullo-by-2^k bit move, srli15 lane bits,
   and THE per-index deserialize_1 bit obligation (mirror of
   `lemma_serialize_1_bits` for the decode direction).  serialize.rs's
   deserialize_1_i16s proof! block calls `lemma_deserialize_1_bits` per index. *)

(* bit 15 of a wrapping i16 multiply by (2^k mod 2^16) is bit (15-k) of the
   input — the "multiply moves the wanted bit to the MSB" step. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400"
let lemma_mul_pow2_bit15 (x m: i16) (k: nat{k <= 15})
  : Lemma (requires (v m) % pow2 16 == pow2 k)
          (ensures Rust_primitives.Integers.get_bit (Rust_primitives.Integers.mul_mod x m) (sz 15) ==
                   Rust_primitives.Integers.get_bit x (sz (15 - k))) =
  let y : i16 = Rust_primitives.Integers.mul_mod x m in
  let n16 = pow2 16 in
  assert_norm (pow2 16 == 65536); assert_norm (pow2 15 == 32768);
  reveal_opaque (`%Rust_primitives.Integers.get_bit)
                (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.I16);
  (* unsigned 16-bit images: get_bit z nth == ((v z % 2^16) / 2^nth) % 2 *)
  let x16 = (v x) % n16 in
  let y16 = (v y) % n16 in
  (* wrap: v y == (v x * v m) @% 2^16, whose plain mod-2^16 image is the same *)
  assert ((v y) % n16 == (v x * v m) % n16);
  FStar.Math.Lemmas.lemma_mod_mul_distr_r (v x) (v m) n16;
  FStar.Math.Lemmas.lemma_mod_mul_distr_l (v x) (pow2 k) n16;
  assert (y16 == (x16 * pow2 k) % n16);
  (* bit 15 of (x16 * 2^k) mod 2^16 == bit (15-k) of x16 *)
  FStar.Math.Lemmas.pow2_modulo_division_lemma_1 (x16 * pow2 k) 15 16;
  FStar.Math.Lemmas.pow2_plus k (15 - k);
  FStar.Math.Lemmas.division_multiplication_lemma (x16 * pow2 k) (pow2 k) (pow2 (15 - k));
  FStar.Math.Lemmas.cancel_mul_div x16 (pow2 k);
  FStar.Math.Lemmas.modulo_modulo_lemma ((x16 * pow2 k) / pow2 15) 2 (pow2 0);
  assert ((y16 / pow2 15) % 2 == (x16 / pow2 (15 - k)) % 2)
#pop-options

(* srli-15 lane: bit 0 of the shifted lane is bit 15 of the input lane; all
   higher bits are 0. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 200 --split_queries always"
let lemma_srli15_lane_bits (y: i16) (bb: nat{bb < 16})
  : Lemma (Rust_primitives.Integers.get_bit
             (cast ((cast y <: u16) >>! mk_i32 15 <: u16) <: i16) (sz bb) ==
           (if bb = 0 then Rust_primitives.Integers.get_bit y (sz 15) else 0)) =
  assert_norm (pow2 15 == 32768); assert_norm (pow2 16 == 65536);
  reveal_opaque (`%Rust_primitives.Integers.get_bit)
                (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.I16);
  reveal_opaque (`%Rust_primitives.Integers.get_bit)
                (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.U16);
  let yu : u16 = cast y <: u16 in
  let sh : u16 = yu >>! mk_i32 15 in
  assert (v yu == (v y) % pow2 16);
  assert (v sh == (v yu) / pow2 15);
  assert (v sh == 0 \/ v sh == 1);
  let r : i16 = cast sh <: i16 in
  assert (v r == v sh);
  if bb = 0 then ()
  else FStar.Math.Lemmas.small_division_lemma_1 (v r) (pow2 bb)
#pop-options

(* THE deserialize_1 bit obligation: bit i of the srli15(mullo(set_epi16 lanes,
   set_epi16 mults)) spine.  Statement recomputes the extracted spine verbatim
   (same set/mullo/srli applications), so the proof! call site links by
   congruence. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deserialize_1_bits (a b: i16) (i: nat{i < 256})
  : Lemma
      (let coeff = mm256_set_epi16 b b b b b b b b a a a a a a a a in
       let mults = mm256_set_epi16 (mk_i16 1 <<! mk_i32 8 <: i16)
           (mk_i16 1 <<! mk_i32 9 <: i16) (mk_i16 1 <<! mk_i32 10 <: i16)
           (mk_i16 1 <<! mk_i32 11 <: i16) (mk_i16 1 <<! mk_i32 12 <: i16)
           (mk_i16 1 <<! mk_i32 13 <: i16) (mk_i16 1 <<! mk_i32 14 <: i16) (mk_i16 (-32768))
           (mk_i16 1 <<! mk_i32 8 <: i16) (mk_i16 1 <<! mk_i32 9 <: i16)
           (mk_i16 1 <<! mk_i32 10 <: i16) (mk_i16 1 <<! mk_i32 11 <: i16)
           (mk_i16 1 <<! mk_i32 12 <: i16) (mk_i16 1 <<! mk_i32 13 <: i16)
           (mk_i16 1 <<! mk_i32 14 <: i16) (mk_i16 (-32768)) in
       let r = mm256_srli_epi16 (mk_i32 15) (mm256_mullo_epi16 coeff mults) in
       bv_bit r i = (if i % 16 >= 1 then 0
                     else let j = (i / 16) * 1 + i % 16 in
                          if i < 128 then Rust_primitives.Integers.get_bit a (sz j)
                          else Rust_primitives.Integers.get_bit b (sz (j - 8)))) =
  let coeff = mm256_set_epi16 b b b b b b b b a a a a a a a a in
  let mults = mm256_set_epi16 (mk_i16 1 <<! mk_i32 8 <: i16)
      (mk_i16 1 <<! mk_i32 9 <: i16) (mk_i16 1 <<! mk_i32 10 <: i16)
      (mk_i16 1 <<! mk_i32 11 <: i16) (mk_i16 1 <<! mk_i32 12 <: i16)
      (mk_i16 1 <<! mk_i32 13 <: i16) (mk_i16 1 <<! mk_i32 14 <: i16) (mk_i16 (-32768))
      (mk_i16 1 <<! mk_i32 8 <: i16) (mk_i16 1 <<! mk_i32 9 <: i16)
      (mk_i16 1 <<! mk_i32 10 <: i16) (mk_i16 1 <<! mk_i32 11 <: i16)
      (mk_i16 1 <<! mk_i32 12 <: i16) (mk_i16 1 <<! mk_i32 13 <: i16)
      (mk_i16 1 <<! mk_i32 14 <: i16) (mk_i16 (-32768)) in
  let msb = mm256_mullo_epi16 coeff mults in
  let r = mm256_srli_epi16 (mk_i32 15) msb in
  let l = i / 16 in
  let bb = i % 16 in
  bit_vec_of_int_t_array_vec256_as_i16x16_lemma r 16 i;
  assert (Seq.index (vec256_as_i16x16 msb) l ==
          Rust_primitives.Integers.mul_mod (Seq.index (vec256_as_i16x16 coeff) l)
                             (Seq.index (vec256_as_i16x16 mults) l));
  assert (Seq.index (vec256_as_i16x16 r) l ==
          (cast ((cast (Seq.index (vec256_as_i16x16 msb) l) <: u16) >>! mk_i32 15 <: u16) <: i16));
  (if false then ()
   else if l = 0 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 0 == a);
     assert (Seq.index (vec256_as_i16x16 mults) 0 == (mk_i16 (-32768)));
     assert_norm ((v ((mk_i16 (-32768)) <: i16)) % pow2 16 == pow2 15);
     lemma_mul_pow2_bit15 a ((mk_i16 (-32768)) <: i16) 15;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod a ((mk_i16 (-32768)) <: i16)) bb
   end
   else if l = 1 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 1 == a);
     assert (Seq.index (vec256_as_i16x16 mults) 1 == (mk_i16 1 <<! mk_i32 14 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 14 <: i16) <: i16)) % pow2 16 == pow2 14);
     lemma_mul_pow2_bit15 a ((mk_i16 1 <<! mk_i32 14 <: i16) <: i16) 14;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod a ((mk_i16 1 <<! mk_i32 14 <: i16) <: i16)) bb
   end
   else if l = 2 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 2 == a);
     assert (Seq.index (vec256_as_i16x16 mults) 2 == (mk_i16 1 <<! mk_i32 13 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 13 <: i16) <: i16)) % pow2 16 == pow2 13);
     lemma_mul_pow2_bit15 a ((mk_i16 1 <<! mk_i32 13 <: i16) <: i16) 13;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod a ((mk_i16 1 <<! mk_i32 13 <: i16) <: i16)) bb
   end
   else if l = 3 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 3 == a);
     assert (Seq.index (vec256_as_i16x16 mults) 3 == (mk_i16 1 <<! mk_i32 12 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 12 <: i16) <: i16)) % pow2 16 == pow2 12);
     lemma_mul_pow2_bit15 a ((mk_i16 1 <<! mk_i32 12 <: i16) <: i16) 12;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod a ((mk_i16 1 <<! mk_i32 12 <: i16) <: i16)) bb
   end
   else if l = 4 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 4 == a);
     assert (Seq.index (vec256_as_i16x16 mults) 4 == (mk_i16 1 <<! mk_i32 11 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 11 <: i16) <: i16)) % pow2 16 == pow2 11);
     lemma_mul_pow2_bit15 a ((mk_i16 1 <<! mk_i32 11 <: i16) <: i16) 11;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod a ((mk_i16 1 <<! mk_i32 11 <: i16) <: i16)) bb
   end
   else if l = 5 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 5 == a);
     assert (Seq.index (vec256_as_i16x16 mults) 5 == (mk_i16 1 <<! mk_i32 10 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 10 <: i16) <: i16)) % pow2 16 == pow2 10);
     lemma_mul_pow2_bit15 a ((mk_i16 1 <<! mk_i32 10 <: i16) <: i16) 10;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod a ((mk_i16 1 <<! mk_i32 10 <: i16) <: i16)) bb
   end
   else if l = 6 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 6 == a);
     assert (Seq.index (vec256_as_i16x16 mults) 6 == (mk_i16 1 <<! mk_i32 9 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 9 <: i16) <: i16)) % pow2 16 == pow2 9);
     lemma_mul_pow2_bit15 a ((mk_i16 1 <<! mk_i32 9 <: i16) <: i16) 9;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod a ((mk_i16 1 <<! mk_i32 9 <: i16) <: i16)) bb
   end
   else if l = 7 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 7 == a);
     assert (Seq.index (vec256_as_i16x16 mults) 7 == (mk_i16 1 <<! mk_i32 8 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 8 <: i16) <: i16)) % pow2 16 == pow2 8);
     lemma_mul_pow2_bit15 a ((mk_i16 1 <<! mk_i32 8 <: i16) <: i16) 8;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod a ((mk_i16 1 <<! mk_i32 8 <: i16) <: i16)) bb
   end
   else if l = 8 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 8 == b);
     assert (Seq.index (vec256_as_i16x16 mults) 8 == (mk_i16 (-32768)));
     assert_norm ((v ((mk_i16 (-32768)) <: i16)) % pow2 16 == pow2 15);
     lemma_mul_pow2_bit15 b ((mk_i16 (-32768)) <: i16) 15;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod b ((mk_i16 (-32768)) <: i16)) bb
   end
   else if l = 9 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 9 == b);
     assert (Seq.index (vec256_as_i16x16 mults) 9 == (mk_i16 1 <<! mk_i32 14 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 14 <: i16) <: i16)) % pow2 16 == pow2 14);
     lemma_mul_pow2_bit15 b ((mk_i16 1 <<! mk_i32 14 <: i16) <: i16) 14;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod b ((mk_i16 1 <<! mk_i32 14 <: i16) <: i16)) bb
   end
   else if l = 10 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 10 == b);
     assert (Seq.index (vec256_as_i16x16 mults) 10 == (mk_i16 1 <<! mk_i32 13 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 13 <: i16) <: i16)) % pow2 16 == pow2 13);
     lemma_mul_pow2_bit15 b ((mk_i16 1 <<! mk_i32 13 <: i16) <: i16) 13;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod b ((mk_i16 1 <<! mk_i32 13 <: i16) <: i16)) bb
   end
   else if l = 11 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 11 == b);
     assert (Seq.index (vec256_as_i16x16 mults) 11 == (mk_i16 1 <<! mk_i32 12 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 12 <: i16) <: i16)) % pow2 16 == pow2 12);
     lemma_mul_pow2_bit15 b ((mk_i16 1 <<! mk_i32 12 <: i16) <: i16) 12;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod b ((mk_i16 1 <<! mk_i32 12 <: i16) <: i16)) bb
   end
   else if l = 12 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 12 == b);
     assert (Seq.index (vec256_as_i16x16 mults) 12 == (mk_i16 1 <<! mk_i32 11 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 11 <: i16) <: i16)) % pow2 16 == pow2 11);
     lemma_mul_pow2_bit15 b ((mk_i16 1 <<! mk_i32 11 <: i16) <: i16) 11;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod b ((mk_i16 1 <<! mk_i32 11 <: i16) <: i16)) bb
   end
   else if l = 13 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 13 == b);
     assert (Seq.index (vec256_as_i16x16 mults) 13 == (mk_i16 1 <<! mk_i32 10 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 10 <: i16) <: i16)) % pow2 16 == pow2 10);
     lemma_mul_pow2_bit15 b ((mk_i16 1 <<! mk_i32 10 <: i16) <: i16) 10;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod b ((mk_i16 1 <<! mk_i32 10 <: i16) <: i16)) bb
   end
   else if l = 14 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 14 == b);
     assert (Seq.index (vec256_as_i16x16 mults) 14 == (mk_i16 1 <<! mk_i32 9 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 9 <: i16) <: i16)) % pow2 16 == pow2 9);
     lemma_mul_pow2_bit15 b ((mk_i16 1 <<! mk_i32 9 <: i16) <: i16) 9;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod b ((mk_i16 1 <<! mk_i32 9 <: i16) <: i16)) bb
   end
   else if l = 15 then begin
     assert (Seq.index (vec256_as_i16x16 coeff) 15 == b);
     assert (Seq.index (vec256_as_i16x16 mults) 15 == (mk_i16 1 <<! mk_i32 8 <: i16));
     assert_norm ((v ((mk_i16 1 <<! mk_i32 8 <: i16) <: i16)) % pow2 16 == pow2 8);
     lemma_mul_pow2_bit15 b ((mk_i16 1 <<! mk_i32 8 <: i16) <: i16) 8;
     lemma_srli15_lane_bits (Rust_primitives.Integers.mul_mod b ((mk_i16 1 <<! mk_i32 8 <: i16) <: i16)) bb
   end
   else ());
  ()
#pop-options

(* ── concat-pairs (madd-by-[2^n;1]) machinery — the P2 keystone shared by
   serialize_4/5/10/12.  `mm256_concat_pairs_n n x` = madd(x, set_epi16(2^n,1,…));
   with per-lane bits >= n of x ZERO the 32-lane value is the exact bit
   concatenation x_{2q} + 2^n * x_{2q+1}, so bit b of the lane is bit b of the
   even lane (b < n), bit b-n of the odd lane (n <= b < 2n), else 0. *)

(* dsum2 of an all-zero window is 0. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let rec lemma_dsum2_zero (f: nat -> Libcrux_core_models.Abstractions.Bit.t_Bit) (off n: nat)
    (h: (k: nat{off <= k /\ k < off + n}) -> Lemma (IVi.bval (f k) == 0))
  : Lemma (ensures IVi.dsum2 f off n == 0) (decreases n) =
  if n = 0 then ()
  else (h off; lemma_dsum2_zero f (off + 1) (n - 1) (fun k -> h k))
#pop-options

(* bits >= n of i16 lane l all zero ==> the lane VALUE is in [0, 2^n). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_lane_high_zero_bound (x: t_Vec256) (l: nat{l < 16}) (n: nat{1 <= n /\ n <= 15})
  : Lemma (requires forall (c: nat{c < 16}). c >= n ==> bv_bit x (16 * l + c) = 0)
          (ensures 0 <= v (get_lane x l) /\ v (get_lane x l) < pow2 n) =
  let reader = IVi.lane_reader (mk_u64 256) 16 x (mk_u64 l) in
  Canon.lemma_to_i16_val x l;
  (* dsum2 0 16 = dsum2 0 n + 2^n * dsum2 n (16-n); the tail is an all-zero window *)
  Canon.dsum2_split reader 0 n (16 - n);
  let htail (k: nat{n <= k /\ k < 16}) : Lemma (IVi.bval (reader k) == 0) =
    lemma_bv_bit_reader #(mk_u64 256) 16 x l k
  in
  lemma_dsum2_zero reader n (16 - n) htail;
  IVi.dsum2_bound reader 0 n;
  (* value = tc_of_u I16 u with u = dsum2 0 n < 2^n <= 2^15 -> tc is the identity *)
  IVi.lemma_tc_range Rust_primitives.Integers.I16 (IVi.dsum2 reader 0 16);
  assert (IVi.dsum2 reader 0 16 == IVi.dsum2 reader 0 n);
  FStar.Math.Lemmas.pow2_le_compat 15 n
#pop-options

(* bit c of a lane with a NON-NEGATIVE value is a plain digit of that value. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 200"
let lemma_get_bit_nonneg (y: i16) (c: nat{c < 16})
  : Lemma (requires v y >= 0)
          (ensures Rust_primitives.Integers.get_bit y (sz c) == ((v y) / pow2 c) % 2) =
  reveal_opaque (`%Rust_primitives.Integers.get_bit)
                (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.I16)
#pop-options

(* digit b of the concatenation S = x0 + 2^n * x1 (both halves in [0, 2^n)). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400 --split_queries always"
let lemma_concat_digit (x0 x1: nat) (n: nat{1 <= n /\ n <= 12}) (b: nat{b < 32})
  : Lemma (requires x0 < pow2 n /\ x1 < pow2 n)
          (ensures ((x0 + pow2 n * x1) / pow2 b) % 2 ==
                   (if b < n then (x0 / pow2 b) % 2
                    else if b < 2 * n then (x1 / pow2 (b - n)) % 2
                    else 0)) =
  let s = x0 + pow2 n * x1 in
  if b < n then begin
    (* s / 2^b = x0/2^b + 2^(n-b)*x1, and the second summand is even *)
    FStar.Math.Lemmas.pow2_plus (n - b) b;
    FStar.Math.Lemmas.lemma_div_plus x0 (pow2 (n - b) * x1) (pow2 b);
    FStar.Math.Lemmas.pow2_plus 1 (n - b - 1);
    FStar.Math.Lemmas.lemma_mod_plus (x0 / pow2 b) (pow2 (n - b - 1) * x1) 2
  end
  else if b < 2 * n then begin
    (* s / 2^n = x1 + x0/2^n = x1; then divide by 2^(b-n) *)
    FStar.Math.Lemmas.lemma_div_plus x0 x1 (pow2 n);
    FStar.Math.Lemmas.small_division_lemma_1 x0 (pow2 n);
    FStar.Math.Lemmas.pow2_plus n (b - n);
    FStar.Math.Lemmas.division_multiplication_lemma s (pow2 n) (pow2 (b - n))
  end
  else begin
    (* s < 2^(2n) <= 2^b *)
    FStar.Math.Lemmas.pow2_plus n n;
    FStar.Math.Lemmas.lemma_mult_lt_left (pow2 n) x1 (pow2 n);
    FStar.Math.Lemmas.pow2_le_compat b (2 * n);
    FStar.Math.Lemmas.small_division_lemma_1 s (pow2 b)
  end
#pop-options

(* digit bridges: bit (32q+b) / (16l+c) of the vector == a plain digit of the
   corresponding lane VALUE (clean-context, reusable for every width). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_lane32_digit (r: t_Vec256) (q: nat{q < 8}) (b: nat{b < 32})
  : Lemma (requires 0 <= lane32 r q /\ lane32 r q < pow2 31)
          (ensures bv_bit r (32 * q + b) == ((lane32 r q) / pow2 b) % 2) =
  assert_norm (256 == 8 * 32);
  lemma_bv_bit_reader #(mk_u64 256) 32 r q b;
  Canon.lemma_readback Rust_primitives.Integers.I32 (mk_u64 256) (mk_u64 8) r (mk_u64 q) b;
  Canon.lemma_to_i32_val r q;
  lemma_lane32_eq_to_i32x8 r q;
  let reader32 = IVi.lane_reader (mk_u64 256) 32 r (mk_u64 q) in
  IVi.dsum2_bound reader32 0 32;
  IVi.lemma_tc_range Rust_primitives.Integers.I32 (IVi.dsum2 reader32 0 32);
  assert (IVi.dsum2 reader32 0 32 == lane32 r q);
  Canon.lemma_get_bit_tc Rust_primitives.Integers.I32 (IVi.dsum2 reader32 0 32) b
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_lane16_digit (x: t_Vec256) (l: nat{l < 16}) (c: nat{c < 16})
  : Lemma (requires v (get_lane x l) >= 0)
          (ensures bv_bit x (16 * l + c) == ((v (get_lane x l)) / pow2 c) % 2) =
  assert_norm (256 == 16 * 16);
  lemma_get_bit_nonneg (get_lane x l) c;
  lemma_bv_bit_reader #(mk_u64 256) 16 x l c;
  Canon.lemma_readback Rust_primitives.Integers.I16 (mk_u64 256) (mk_u64 16) x (mk_u64 l) c
#pop-options

(* ── concat-pairs keystone ─────────────────────────────────────────────────
   `lemma_concat_pairs_lane32` / `lemma_concat_pairs_bits` live in
   `Libcrux_ml_kem.Vector.Avx2.Concat_pairs_theory` (kept out of this module
   because in-module they suffer solver-state pollution from the preceding ~2000
   lines of queries and need a per-decl `#restart-solver`).  They reach the facts
   below by EXPLICIT CALL (per feedback_smtpat_only_for_user_consumed_lemmas).
   The digit bridges above
   (`lemma_bv_bit_lane32_digit` / `lemma_bv_bit_lane16_digit` /
   `lemma_lane_high_zero_bound` / `lemma_concat_digit` / `lemma_dsum2_zero` /
   `lemma_get_bit_nonneg`) STAY here: they are width-generic and every
   deserialize-width proof reuses them. *)
