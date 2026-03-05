module Hacspec_kyber.Ind_cpa
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open Core
open FStar.Mul

let encode_and_compress_u
      (input:
          Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)
        )
    : Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
  let out:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = Alloc.Vec.impl__new in
  let out:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter (Hacspec_lib.Vector.impl__into_iter
              (sz 3)
              (sz 256)
              input
            <:
            Core.Array.Iter.t_IntoIter
              (Hacspec_lib.Ring.t_PolynomialRingElement
                  (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256)) (sz 3))
        <:
        Core.Array.Iter.t_IntoIter
          (Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256)) (sz 3))
      out
      (fun out re ->
          let out:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = out in
          let re:Hacspec_lib.Ring.t_PolynomialRingElement
            (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) =
            re
          in
          Alloc.Vec.impl_2__extend_from_slice out
            (Core.Ops.Deref.f_deref (Hacspec_kyber.Serialize.byte_encode Hacspec_kyber.Parameters.v_VECTOR_U_COMPRESSION_FACTOR
                    (Hacspec_kyber.Compress.compress re
                        Hacspec_kyber.Parameters.v_VECTOR_U_COMPRESSION_FACTOR
                      <:
                      Hacspec_lib.Ring.t_PolynomialRingElement
                        (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
                  <:
                  Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
              <:
              t_Slice u8)
          <:
          Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)
  in
  out

let decrypt (secret_key: t_Array u8 (sz 1152)) (ciphertext: t_Array u8 (sz 1088))
    : t_Array u8 (sz 32) =
  let u:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) =
    Hacspec_lib.Vector.impl__ZERO
  in
  let u:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter (Core.Iter.Traits.Iterator.f_enumerate
              (Core.Slice.impl__chunks (ciphertext.[ {
                        Core.Ops.Range.f_end = Hacspec_kyber.Parameters.v_VECTOR_U_ENCODED_SIZE
                      }
                      <:
                      Core.Ops.Range.t_RangeTo usize ]
                    <:
                    t_Slice u8)
                  ((Hacspec_kyber.Parameters.v_COEFFICIENTS_IN_RING_ELEMENT *!
                      Hacspec_kyber.Parameters.v_VECTOR_U_COMPRESSION_FACTOR
                      <:
                      usize) /!
                    sz 8
                    <:
                    usize)
                <:
                Core.Slice.Iter.t_Chunks u8)
            <:
            Core.Iter.Adapters.Enumerate.t_Enumerate (Core.Slice.Iter.t_Chunks u8))
        <:
        Core.Iter.Adapters.Enumerate.t_Enumerate (Core.Slice.Iter.t_Chunks u8))
      u
      (fun u temp_1_ ->
          let u:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
            (sz 256)
            (sz 3) =
            u
          in
          let i, u_bytes:(usize & t_Slice u8) = temp_1_ in
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize u
            i
            (Hacspec_kyber.Compress.decompress (Hacspec_kyber.Serialize.byte_decode Hacspec_kyber.Parameters.v_VECTOR_U_COMPRESSION_FACTOR
                    u_bytes
                  <:
                  Hacspec_lib.Ring.t_PolynomialRingElement
                    (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
                Hacspec_kyber.Parameters.v_VECTOR_U_COMPRESSION_FACTOR
              <:
              Hacspec_lib.Ring.t_PolynomialRingElement
                (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
          <:
          Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)
      )
  in
  let v:Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256) =
    Hacspec_kyber.Compress.decompress (Hacspec_kyber.Serialize.byte_decode Hacspec_kyber.Parameters.v_VECTOR_V_COMPRESSION_FACTOR
          (ciphertext.[ {
                Core.Ops.Range.f_start = Hacspec_kyber.Parameters.v_VECTOR_U_ENCODED_SIZE
              }
              <:
              Core.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
        <:
        Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
          (sz 256))
      Hacspec_kyber.Parameters.v_VECTOR_V_COMPRESSION_FACTOR
  in
  let secret_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Hacspec_lib.Vector.impl__ZERO
  in
  let secret_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter (Core.Iter.Traits.Iterator.f_enumerate
              (Core.Slice.impl__chunks_exact (Rust_primitives.unsize secret_key <: t_Slice u8)
                  Hacspec_kyber.Parameters.v_BYTES_PER_RING_ELEMENT
                <:
                Core.Slice.Iter.t_ChunksExact u8)
            <:
            Core.Iter.Adapters.Enumerate.t_Enumerate (Core.Slice.Iter.t_ChunksExact u8))
        <:
        Core.Iter.Adapters.Enumerate.t_Enumerate (Core.Slice.Iter.t_ChunksExact u8))
      secret_as_ntt
      (fun secret_as_ntt temp_1_ ->
          let secret_as_ntt:Hacspec_lib.Vector.t_Vector
            (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) =
            secret_as_ntt
          in
          let i, secret_bytes:(usize & t_Slice u8) = temp_1_ in
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize secret_as_ntt
            i
            (Hacspec_kyber.Serialize.byte_decode (sz 12) secret_bytes
              <:
              Hacspec_lib.Ring.t_PolynomialRingElement
                (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
          <:
          Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)
      )
  in
  let u_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Hacspec_kyber.Ntt.vector_ntt u
  in
  let message:Hacspec_lib.Ring.t_PolynomialRingElement
    (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) =
    v -!
    (Hacspec_kyber.Ntt.ntt_inverse (Hacspec_kyber.Matrix.multiply_column_by_row secret_as_ntt
            u_as_ntt
          <:
          Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
            (sz 256))
      <:
      Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
        (sz 256))
  in
  Hacspec_lib.f_as_len_array (sz 32)
    (Hacspec_kyber.Serialize.byte_encode (sz 1)
        (Hacspec_kyber.Compress.compress message (sz 1)
          <:
          Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
            (sz 256))
      <:
      Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global)

type t_KeyPair = {
  f_sk:t_Array u8 (sz 1152);
  f_pk:t_Array u8 (sz 1184)
}

let impl__KeyPair__new (sk: t_Array u8 (sz 1152)) (pk: t_Array u8 (sz 1184)) : t_KeyPair =
  { f_sk = sk; f_pk = pk } <: t_KeyPair

let impl__KeyPair__pk (self: t_KeyPair) : t_Array u8 (sz 1184) = self.f_pk

let impl__KeyPair__serialize_secret_key
      (self: t_KeyPair)
      (implicit_rejection_value: t_Array u8 (sz 32))
    : t_Array u8 (sz 2400) =
  Core.Convert.f_into (Hacspec_lib.f_push (Hacspec_lib.f_push (Hacspec_lib.f_push (Hacspec_lib.f_push
                    (Hacspec_lib.impl_6__new (sz 2400)
                        (Rust_primitives.Hax.repeat 0uy (sz 2400) <: t_Array u8 (sz 2400))
                      <:
                      Hacspec_lib.t_UpdatableArray (sz 2400))
                    (Rust_primitives.unsize self.f_sk <: t_Slice u8)
                  <:
                  Hacspec_lib.t_UpdatableArray (sz 2400))
                (Rust_primitives.unsize self.f_pk <: t_Slice u8)
              <:
              Hacspec_lib.t_UpdatableArray (sz 2400))
            (Rust_primitives.unsize (Hacspec_kyber.Parameters.Hash_functions.v_H (Rust_primitives.unsize
                        self.f_pk
                      <:
                      t_Slice u8)
                  <:
                  t_Array u8 (sz 32))
              <:
              t_Slice u8)
          <:
          Hacspec_lib.t_UpdatableArray (sz 2400))
        (Rust_primitives.unsize implicit_rejection_value <: t_Slice u8)
      <:
      Hacspec_lib.t_UpdatableArray (sz 2400))

let encrypt (public_key: t_Array u8 (sz 1184)) (message randomness: t_Array u8 (sz 32))
    : Core.Result.t_Result (t_Array u8 (sz 1088))
      Hacspec_kyber.t_BadRejectionSamplingRandomnessError =
  let (domain_separator: u8):u8 = 0uy in
  let tt_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Hacspec_lib.Vector.impl__ZERO
  in
  let tt_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter (Core.Iter.Traits.Iterator.f_enumerate
              (Core.Slice.impl__chunks (public_key.[ {
                        Core.Ops.Range.f_end = Hacspec_kyber.Parameters.v_T_AS_NTT_ENCODED_SIZE
                      }
                      <:
                      Core.Ops.Range.t_RangeTo usize ]
                    <:
                    t_Slice u8)
                  Hacspec_kyber.Parameters.v_BYTES_PER_RING_ELEMENT
                <:
                Core.Slice.Iter.t_Chunks u8)
            <:
            Core.Iter.Adapters.Enumerate.t_Enumerate (Core.Slice.Iter.t_Chunks u8))
        <:
        Core.Iter.Adapters.Enumerate.t_Enumerate (Core.Slice.Iter.t_Chunks u8))
      tt_as_ntt
      (fun tt_as_ntt temp_1_ ->
          let tt_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
            (sz 256)
            (sz 3) =
            tt_as_ntt
          in
          let i, tt_as_ntt_bytes:(usize & t_Slice u8) = temp_1_ in
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize tt_as_ntt
            i
            (Hacspec_kyber.Serialize.byte_decode (sz 12) tt_as_ntt_bytes
              <:
              Hacspec_lib.Ring.t_PolynomialRingElement
                (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
          <:
          Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)
      )
  in
  let seed_for_A:t_Slice u8 =
    public_key.[ { Core.Ops.Range.f_start = Hacspec_kyber.Parameters.v_T_AS_NTT_ENCODED_SIZE }
      <:
      Core.Ops.Range.t_RangeFrom usize ]
  in
  let v_A_as_ntt:t_Array
    (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3))
    (sz 3) =
    Rust_primitives.Hax.repeat Hacspec_lib.Vector.impl__ZERO (sz 3)
  in
  let (xof_input: t_Array u8 (sz 34)):t_Array u8 (sz 34) =
    Hacspec_lib.f_into_padded_array (sz 34) seed_for_A
  in
  let v_A_as_ntt, xof_input:(t_Array
      (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3))
      (sz 3) &
    t_Array u8 (sz 34)) =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter ({
              Core.Ops.Range.f_start = sz 0;
              Core.Ops.Range.f_end = Hacspec_kyber.Parameters.v_RANK
            }
            <:
            Core.Ops.Range.t_Range usize)
        <:
        Core.Ops.Range.t_Range usize)
      (v_A_as_ntt, xof_input
        <:
        (t_Array
            (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                (sz 256)
                (sz 3)) (sz 3) &
          t_Array u8 (sz 34)))
      (fun temp_0_ i ->
          let v_A_as_ntt, xof_input:(t_Array
              (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                  (sz 256)
                  (sz 3)) (sz 3) &
            t_Array u8 (sz 34)) =
            temp_0_
          in
          let i:usize = i in
          Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter ({
                    Core.Ops.Range.f_start = sz 0;
                    Core.Ops.Range.f_end = Hacspec_kyber.Parameters.v_RANK
                  }
                  <:
                  Core.Ops.Range.t_Range usize)
              <:
              Core.Ops.Range.t_Range usize)
            (v_A_as_ntt, xof_input
              <:
              (t_Array
                  (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                      (sz 256)
                      (sz 3)) (sz 3) &
                t_Array u8 (sz 34)))
            (fun temp_0_ j ->
                let v_A_as_ntt, xof_input:(t_Array
                    (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                        (sz 256)
                        (sz 3)) (sz 3) &
                  t_Array u8 (sz 34)) =
                  temp_0_
                in
                let j:usize = j in
                let xof_input:t_Array u8 (sz 34) =
                  Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof_input
                    (sz 32)
                    (Hacspec_lib.f_as_u8 i <: u8)
                in
                let xof_input:t_Array u8 (sz 34) =
                  Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof_input
                    (sz 33)
                    (Hacspec_lib.f_as_u8 j <: u8)
                in
                let (xof_bytes: t_Array u8 (sz 840)):t_Array u8 (sz 840) =
                  Hacspec_kyber.Parameters.Hash_functions.v_XOF (sz 840)
                    (Rust_primitives.unsize xof_input <: t_Slice u8)
                in
                let* hoist10:Hacspec_lib.Ring.t_PolynomialRingElement
                  (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) =
                  match
                    Core.Ops.Try_trait.f_branch (Hacspec_kyber.Sampling.sample_ntt xof_bytes
                        <:
                        Core.Result.t_Result
                          (Hacspec_lib.Ring.t_PolynomialRingElement
                              (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
                          Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                  with
                  | Core.Ops.Control_flow.ControlFlow_Break residual ->
                    let* hoist9:Rust_primitives.Hax.t_Never =
                      Core.Ops.Control_flow.ControlFlow.v_Break (Core.Ops.Try_trait.f_from_residual residual

                          <:
                          Core.Result.t_Result (t_Array u8 (sz 1088))
                            Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                    in
                    Core.Ops.Control_flow.ControlFlow_Continue
                    (Rust_primitives.Hax.never_to_any hoist9)
                    <:
                    Core.Ops.Control_flow.t_ControlFlow
                      (Core.Result.t_Result (t_Array u8 (sz 1088))
                          Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                      (Hacspec_lib.Ring.t_PolynomialRingElement
                          (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
                  | Core.Ops.Control_flow.ControlFlow_Continue v_val ->
                    Core.Ops.Control_flow.ControlFlow_Continue v_val
                    <:
                    Core.Ops.Control_flow.t_ControlFlow
                      (Core.Result.t_Result (t_Array u8 (sz 1088))
                          Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                      (Hacspec_lib.Ring.t_PolynomialRingElement
                          (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
                in
                Core.Ops.Control_flow.ControlFlow_Continue
                (let hoist11:Hacspec_lib.Vector.t_Vector
                    (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) =
                    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (v_A_as_ntt.[ i ]
                        <:
                        Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                          (sz 256)
                          (sz 3))
                      j
                      hoist10
                  in
                  let hoist12:t_Array
                    (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                        (sz 256)
                        (sz 3)) (sz 3) =
                    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize v_A_as_ntt i hoist11
                  in
                  let v_A_as_ntt:t_Array
                    (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                        (sz 256)
                        (sz 3)) (sz 3) =
                    hoist12
                  in
                  v_A_as_ntt, xof_input
                  <:
                  (t_Array
                      (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                          (sz 256)
                          (sz 3)) (sz 3) &
                    t_Array u8 (sz 34)))
                <:
                Core.Ops.Control_flow.t_ControlFlow
                  (Core.Result.t_Result (t_Array u8 (sz 1088))
                      Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                  (t_Array
                      (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                          (sz 256)
                          (sz 3)) (sz 3) &
                    t_Array u8 (sz 34)))
          <:
          (t_Array
              (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                  (sz 256)
                  (sz 3)) (sz 3) &
            t_Array u8 (sz 34)))
  in
  let r:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) =
    Hacspec_lib.Vector.impl__ZERO
  in
  let (prf_input: t_Array u8 (sz 33)):t_Array u8 (sz 33) =
    Hacspec_lib.f_into_padded_array (sz 32) (sz 33) randomness
  in
  let domain_separator, prf_input, r:(u8 & t_Array u8 (sz 33) &
    Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)) =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter ({
              Core.Ops.Range.f_start = sz 0;
              Core.Ops.Range.f_end = Hacspec_lib.Vector.impl__len (sz 3) (sz 256) r <: usize
            }
            <:
            Core.Ops.Range.t_Range usize)
        <:
        Core.Ops.Range.t_Range usize)
      (domain_separator, prf_input, r
        <:
        (u8 & t_Array u8 (sz 33) &
          Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)
        ))
      (fun temp_0_ i ->
          let domain_separator, prf_input, r:(u8 & t_Array u8 (sz 33) &
            Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256)
              (sz 3)) =
            temp_0_
          in
          let i:usize = i in
          let prf_input:t_Array u8 (sz 33) =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize prf_input
              (sz 32)
              domain_separator
          in
          let domain_separator:u8 = domain_separator +! 1uy in
          let (prf_output: t_Array u8 (sz 128)):t_Array u8 (sz 128) =
            Hacspec_kyber.Parameters.Hash_functions.v_PRF (sz 128)
              (Rust_primitives.unsize prf_input <: t_Slice u8)
          in
          let r:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
            (sz 256)
            (sz 3) =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize r
              i
              (Hacspec_kyber.Sampling.sample_poly_cbd (sz 2)
                  (Rust_primitives.unsize prf_output <: t_Slice u8)
                <:
                Hacspec_lib.Ring.t_PolynomialRingElement
                  (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
          in
          domain_separator, prf_input, r
          <:
          (u8 & t_Array u8 (sz 33) &
            Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256)
              (sz 3)))
  in
  let error_1_:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Hacspec_lib.Vector.impl__ZERO
  in
  let domain_separator, error_1_, prf_input:(u8 &
    Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) &
    t_Array u8 (sz 33)) =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter ({
              Core.Ops.Range.f_start = sz 0;
              Core.Ops.Range.f_end = Hacspec_lib.Vector.impl__len (sz 3) (sz 256) error_1_ <: usize
            }
            <:
            Core.Ops.Range.t_Range usize)
        <:
        Core.Ops.Range.t_Range usize)
      (domain_separator, error_1_, prf_input
        <:
        (u8 &
          Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) &
          t_Array u8 (sz 33)))
      (fun temp_0_ i ->
          let domain_separator, error_1_, prf_input:(u8 &
            Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256)
              (sz 3) &
            t_Array u8 (sz 33)) =
            temp_0_
          in
          let i:usize = i in
          let prf_input:t_Array u8 (sz 33) =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize prf_input
              (sz 32)
              domain_separator
          in
          let domain_separator:u8 = domain_separator +! 1uy in
          let (prf_output: t_Array u8 (sz 128)):t_Array u8 (sz 128) =
            Hacspec_kyber.Parameters.Hash_functions.v_PRF (sz 128)
              (Rust_primitives.unsize prf_input <: t_Slice u8)
          in
          let error_1_:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
            (sz 256)
            (sz 3) =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize error_1_
              i
              (Hacspec_kyber.Sampling.sample_poly_cbd (sz 2)
                  (Rust_primitives.unsize prf_output <: t_Slice u8)
                <:
                Hacspec_lib.Ring.t_PolynomialRingElement
                  (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
          in
          domain_separator, error_1_, prf_input
          <:
          (u8 &
            Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256)
              (sz 3) &
            t_Array u8 (sz 33)))
  in
  let prf_input:t_Array u8 (sz 33) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize prf_input (sz 32) domain_separator
  in
  let (prf_output: t_Array u8 (sz 128)):t_Array u8 (sz 128) =
    Hacspec_kyber.Parameters.Hash_functions.v_PRF (sz 128)
      (Rust_primitives.unsize prf_input <: t_Slice u8)
  in
  let error_2_:Hacspec_lib.Ring.t_PolynomialRingElement
    (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) =
    Hacspec_kyber.Sampling.sample_poly_cbd (sz 2) (Rust_primitives.unsize prf_output <: t_Slice u8)
  in
  let r_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Hacspec_kyber.Ntt.vector_ntt r
  in
  let v_A_as_ntt_transpose:t_Array
    (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3))
    (sz 3) =
    Hacspec_kyber.Matrix.transpose v_A_as_ntt
  in
  let u:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) =
    (Hacspec_kyber.Ntt.vector_inverse_ntt (Hacspec_kyber.Matrix.multiply_matrix_by_column v_A_as_ntt_transpose
            r_as_ntt
          <:
          Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)
        )
      <:
      Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)) +!
    error_1_
  in
  let message_as_ring_element:Hacspec_lib.Ring.t_PolynomialRingElement
    (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) =
    Hacspec_kyber.Compress.decompress (Hacspec_kyber.Serialize.byte_decode (sz 1)
          (Rust_primitives.unsize message <: t_Slice u8)
        <:
        Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
          (sz 256))
      (sz 1)
  in
  let v:Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256) =
    ((Hacspec_kyber.Ntt.ntt_inverse (Hacspec_kyber.Matrix.multiply_column_by_row tt_as_ntt r_as_ntt
            <:
            Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256))
        <:
        Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
          (sz 256)) +!
      error_2_
      <:
      Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
        (sz 256)) +!
    message_as_ring_element
  in
  let c1:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = encode_and_compress_u u in
  let c2:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global =
    Hacspec_kyber.Serialize.byte_encode Hacspec_kyber.Parameters.v_VECTOR_V_COMPRESSION_FACTOR
      (Hacspec_kyber.Compress.compress v Hacspec_kyber.Parameters.v_VECTOR_V_COMPRESSION_FACTOR
        <:
        Hacspec_lib.Ring.t_PolynomialRingElement (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
          (sz 256))
  in
  let (ciphertext: t_Array u8 (sz 1088)):t_Array u8 (sz 1088) =
    Hacspec_lib.f_into_padded_array (sz 1088) c1
  in
  let ciphertext:t_Array u8 (sz 1088) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from ciphertext
      ({ Core.Ops.Range.f_start = Hacspec_kyber.Parameters.v_VECTOR_U_ENCODED_SIZE }
        <:
        Core.Ops.Range.t_RangeFrom usize)
      (Core.Slice.impl__copy_from_slice (ciphertext.[ {
                Core.Ops.Range.f_start = Hacspec_kyber.Parameters.v_VECTOR_U_ENCODED_SIZE
              }
              <:
              Core.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          (Alloc.Vec.impl_1__as_slice c2 <: t_Slice u8)
        <:
        t_Slice u8)
  in
  Core.Result.Result_Ok ciphertext
  <:
  Core.Result.t_Result (t_Array u8 (sz 1088)) Hacspec_kyber.t_BadRejectionSamplingRandomnessError

let generate_keypair (key_generation_seed: t_Array u8 (sz 32))
    : Core.Result.t_Result t_KeyPair Hacspec_kyber.t_BadRejectionSamplingRandomnessError =
  let hashed:t_Array u8 (sz 64) =
    Hacspec_kyber.Parameters.Hash_functions.v_G (Rust_primitives.unsize key_generation_seed
        <:
        t_Slice u8)
  in
  let seed_for_A, seed_for_secret_and_error:(t_Slice u8 & t_Slice u8) =
    Core.Slice.impl__split_at (Rust_primitives.unsize hashed <: t_Slice u8) (sz 32)
  in
  let (domain_separator: u8):u8 = 0uy in
  let v_A_as_ntt:t_Array
    (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3))
    (sz 3) =
    Rust_primitives.Hax.repeat Hacspec_lib.Vector.impl__ZERO (sz 3)
  in
  let (xof_input: t_Array u8 (sz 34)):t_Array u8 (sz 34) =
    Hacspec_lib.f_into_padded_array (sz 34) seed_for_A
  in
  let v_A_as_ntt, xof_input:(t_Array
      (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3))
      (sz 3) &
    t_Array u8 (sz 34)) =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter ({
              Core.Ops.Range.f_start = sz 0;
              Core.Ops.Range.f_end = Hacspec_kyber.Parameters.v_RANK
            }
            <:
            Core.Ops.Range.t_Range usize)
        <:
        Core.Ops.Range.t_Range usize)
      (v_A_as_ntt, xof_input
        <:
        (t_Array
            (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                (sz 256)
                (sz 3)) (sz 3) &
          t_Array u8 (sz 34)))
      (fun temp_0_ i ->
          let v_A_as_ntt, xof_input:(t_Array
              (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                  (sz 256)
                  (sz 3)) (sz 3) &
            t_Array u8 (sz 34)) =
            temp_0_
          in
          let i:usize = i in
          Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter ({
                    Core.Ops.Range.f_start = sz 0;
                    Core.Ops.Range.f_end = Hacspec_kyber.Parameters.v_RANK
                  }
                  <:
                  Core.Ops.Range.t_Range usize)
              <:
              Core.Ops.Range.t_Range usize)
            (v_A_as_ntt, xof_input
              <:
              (t_Array
                  (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                      (sz 256)
                      (sz 3)) (sz 3) &
                t_Array u8 (sz 34)))
            (fun temp_0_ j ->
                let v_A_as_ntt, xof_input:(t_Array
                    (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                        (sz 256)
                        (sz 3)) (sz 3) &
                  t_Array u8 (sz 34)) =
                  temp_0_
                in
                let j:usize = j in
                let xof_input:t_Array u8 (sz 34) =
                  Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof_input
                    (sz 32)
                    (Hacspec_lib.f_as_u8 i <: u8)
                in
                let xof_input:t_Array u8 (sz 34) =
                  Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof_input
                    (sz 33)
                    (Hacspec_lib.f_as_u8 j <: u8)
                in
                let (xof_bytes: t_Array u8 (sz 840)):t_Array u8 (sz 840) =
                  Hacspec_kyber.Parameters.Hash_functions.v_XOF (sz 840)
                    (Rust_primitives.unsize xof_input <: t_Slice u8)
                in
                let* hoist14:Hacspec_lib.Ring.t_PolynomialRingElement
                  (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) =
                  match
                    Core.Ops.Try_trait.f_branch (Hacspec_kyber.Sampling.sample_ntt xof_bytes
                        <:
                        Core.Result.t_Result
                          (Hacspec_lib.Ring.t_PolynomialRingElement
                              (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
                          Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                  with
                  | Core.Ops.Control_flow.ControlFlow_Break residual ->
                    let* hoist13:Rust_primitives.Hax.t_Never =
                      Core.Ops.Control_flow.ControlFlow.v_Break (Core.Ops.Try_trait.f_from_residual residual

                          <:
                          Core.Result.t_Result t_KeyPair
                            Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                    in
                    Core.Ops.Control_flow.ControlFlow_Continue
                    (Rust_primitives.Hax.never_to_any hoist13)
                    <:
                    Core.Ops.Control_flow.t_ControlFlow
                      (Core.Result.t_Result t_KeyPair
                          Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                      (Hacspec_lib.Ring.t_PolynomialRingElement
                          (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
                  | Core.Ops.Control_flow.ControlFlow_Continue v_val ->
                    Core.Ops.Control_flow.ControlFlow_Continue v_val
                    <:
                    Core.Ops.Control_flow.t_ControlFlow
                      (Core.Result.t_Result t_KeyPair
                          Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                      (Hacspec_lib.Ring.t_PolynomialRingElement
                          (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
                in
                Core.Ops.Control_flow.ControlFlow_Continue
                (let hoist15:Hacspec_lib.Vector.t_Vector
                    (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) =
                    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (v_A_as_ntt.[ i ]
                        <:
                        Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                          (sz 256)
                          (sz 3))
                      j
                      hoist14
                  in
                  let hoist16:t_Array
                    (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                        (sz 256)
                        (sz 3)) (sz 3) =
                    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize v_A_as_ntt i hoist15
                  in
                  let v_A_as_ntt:t_Array
                    (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                        (sz 256)
                        (sz 3)) (sz 3) =
                    hoist16
                  in
                  v_A_as_ntt, xof_input
                  <:
                  (t_Array
                      (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                          (sz 256)
                          (sz 3)) (sz 3) &
                    t_Array u8 (sz 34)))
                <:
                Core.Ops.Control_flow.t_ControlFlow
                  (Core.Result.t_Result t_KeyPair
                      Hacspec_kyber.t_BadRejectionSamplingRandomnessError)
                  (t_Array
                      (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                          (sz 256)
                          (sz 3)) (sz 3) &
                    t_Array u8 (sz 34)))
          <:
          (t_Array
              (Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
                  (sz 256)
                  (sz 3)) (sz 3) &
            t_Array u8 (sz 34)))
  in
  let secret:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Hacspec_lib.Vector.impl__ZERO
  in
  let (prf_input: t_Array u8 (sz 33)):t_Array u8 (sz 33) =
    Hacspec_lib.f_into_padded_array (sz 33) seed_for_secret_and_error
  in
  let domain_separator, prf_input, secret:(u8 & t_Array u8 (sz 33) &
    Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)) =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter ({
              Core.Ops.Range.f_start = sz 0;
              Core.Ops.Range.f_end = Hacspec_lib.Vector.impl__len (sz 3) (sz 256) secret <: usize
            }
            <:
            Core.Ops.Range.t_Range usize)
        <:
        Core.Ops.Range.t_Range usize)
      (domain_separator, prf_input, secret
        <:
        (u8 & t_Array u8 (sz 33) &
          Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)
        ))
      (fun temp_0_ i ->
          let domain_separator, prf_input, secret:(u8 & t_Array u8 (sz 33) &
            Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256)
              (sz 3)) =
            temp_0_
          in
          let i:usize = i in
          let prf_input:t_Array u8 (sz 33) =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize prf_input
              (sz 32)
              domain_separator
          in
          let domain_separator:u8 = domain_separator +! 1uy in
          let (prf_output: t_Array u8 (sz 128)):t_Array u8 (sz 128) =
            Hacspec_kyber.Parameters.Hash_functions.v_PRF (sz 128)
              (Rust_primitives.unsize prf_input <: t_Slice u8)
          in
          let secret:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
            (sz 256)
            (sz 3) =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize secret
              i
              (Hacspec_kyber.Sampling.sample_poly_cbd (sz 2)
                  (prf_output.[ Core.Ops.Range.RangeFull <: Core.Ops.Range.t_RangeFull ]
                    <:
                    t_Slice u8)
                <:
                Hacspec_lib.Ring.t_PolynomialRingElement
                  (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
          in
          domain_separator, prf_input, secret
          <:
          (u8 & t_Array u8 (sz 33) &
            Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256)
              (sz 3)))
  in
  let error:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Hacspec_lib.Vector.impl__ZERO
  in
  let domain_separator, error, prf_input:(u8 &
    Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) &
    t_Array u8 (sz 33)) =
    Core.Iter.Traits.Iterator.f_fold (Core.Iter.Traits.Collect.f_into_iter ({
              Core.Ops.Range.f_start = sz 0;
              Core.Ops.Range.f_end = Hacspec_lib.Vector.impl__len (sz 3) (sz 256) error <: usize
            }
            <:
            Core.Ops.Range.t_Range usize)
        <:
        Core.Ops.Range.t_Range usize)
      (domain_separator, error, prf_input
        <:
        (u8 &
          Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3) &
          t_Array u8 (sz 33)))
      (fun temp_0_ i ->
          let domain_separator, error, prf_input:(u8 &
            Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256)
              (sz 3) &
            t_Array u8 (sz 33)) =
            temp_0_
          in
          let i:usize = i in
          let prf_input:t_Array u8 (sz 33) =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize prf_input
              (sz 32)
              domain_separator
          in
          let domain_separator:u8 = domain_separator +! 1uy in
          let (prf_output: t_Array u8 (sz 128)):t_Array u8 (sz 128) =
            Hacspec_kyber.Parameters.Hash_functions.v_PRF (sz 128)
              (Rust_primitives.unsize prf_input <: t_Slice u8)
          in
          let error:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
            (sz 256)
            (sz 3) =
            Rust_primitives.Hax.Monomorphized_update_at.update_at_usize error
              i
              (Hacspec_kyber.Sampling.sample_poly_cbd (sz 2)
                  (prf_output.[ Core.Ops.Range.RangeFull <: Core.Ops.Range.t_RangeFull ]
                    <:
                    t_Slice u8)
                <:
                Hacspec_lib.Ring.t_PolynomialRingElement
                  (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256))
          in
          domain_separator, error, prf_input
          <:
          (u8 &
            Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
              (sz 256)
              (sz 3) &
            t_Array u8 (sz 33)))
  in
  let secret_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Hacspec_kyber.Ntt.vector_ntt secret
  in
  let error_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    Hacspec_kyber.Ntt.vector_ntt error
  in
  let tt_as_ntt:Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us)
    (sz 256)
    (sz 3) =
    (Hacspec_kyber.Matrix.multiply_matrix_by_column v_A_as_ntt secret_as_ntt
      <:
      Hacspec_lib.Vector.t_Vector (Hacspec_lib.Field.t_PrimeFieldElement 3329us) (sz 256) (sz 3)) +!
    error_as_ntt
  in
  let public_key_serialized:t_Array u8 (sz 1184) =
    Hacspec_lib.impl_6__array (sz 1184)
      (Hacspec_lib.f_push (Hacspec_lib.f_push (Hacspec_lib.impl_6__new (sz 1184)
                  (Rust_primitives.Hax.repeat 0uy (sz 1184) <: t_Array u8 (sz 1184))
                <:
                Hacspec_lib.t_UpdatableArray (sz 1184))
              (Rust_primitives.unsize (Hacspec_kyber.Serialize.vector_encode_12_ tt_as_ntt
                    <:
                    t_Array u8 (sz 1152))
                <:
                t_Slice u8)
            <:
            Hacspec_lib.t_UpdatableArray (sz 1184))
          seed_for_A
        <:
        Hacspec_lib.t_UpdatableArray (sz 1184))
  in
  let secret_key_serialized:t_Array u8 (sz 1152) =
    Hacspec_kyber.Serialize.vector_encode_12_ secret_as_ntt
  in
  Core.Result.Result_Ok
  (impl__KeyPair__new (Hacspec_lib.f_into_len_array (sz 1152)
          (Rust_primitives.unsize secret_key_serialized <: t_Slice u8)
        <:
        t_Array u8 (sz 1152))
      (Hacspec_lib.f_into_len_array (sz 1184)
          (Rust_primitives.unsize public_key_serialized <: t_Slice u8)
        <:
        t_Array u8 (sz 1184)))
  <:
  Core.Result.t_Result t_KeyPair Hacspec_kyber.t_BadRejectionSamplingRandomnessError
