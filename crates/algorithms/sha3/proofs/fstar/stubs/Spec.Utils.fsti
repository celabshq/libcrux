module Spec.Utils
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"

/// Minimal stub re-exporting the small surface of Spec.Utils that
/// `Libcrux_intrinsics.Avx2_extract.fsti` references.  The real
/// definitions live in `libcrux-ml-kem/proofs/fstar/spec/Spec.Utils.fsti`,
/// but the SHA-3 proofs do not exercise those intrinsics, so a stub
/// keeps the typecheck self-contained.
///
/// All functions are total wrappers around `Seq.init` so that signatures
/// in `Avx2_extract.fsti` resolve.

open Rust_primitives

let create #a (len: usize) (c: a) : t_Array a len =
  Seq.create (v len) c

let create16 #a (v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15: a)
  : t_Array a (sz 16) =
  let l = [v0; v1; v2; v3; v4; v5; v6; v7; v8; v9; v10; v11; v12; v13; v14; v15] in
  assert_norm (List.Tot.length l == 16);
  Seq.seq_of_list l

let map2 #a #b #c #len (f: a -> b -> c) (x: t_Array a len) (y: t_Array b len)
  : t_Array c len =
  Seq.init (v len) (fun i -> f (Seq.index x i) (Seq.index y i))

let map_array #a #b #len (f: a -> b) (s: t_Array a len) : t_Array b len =
  Seq.init (v len) (fun i -> f (Seq.index s i))

val mul_mod (x y: i16) : i16
