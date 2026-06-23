module Hacspec_ml_kem.Matrix
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// N.B.: According to the NIST FIPS 203 standard (Page 9, Line 519), a matrix is
/// a set of column vectors.
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let add_polynomials (p1 p2: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun j ->
        let j:usize = j in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast (((cast ((p1.[ j ]
                          <:
                          Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32) +!
                  (cast ((p2.[ j ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32)
                  <:
                  u32) %!
                (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
                <:
                u32)
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

let sub_polynomials (p1 p2: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun j ->
        let j:usize = j in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast ((((cast ((p1.[ j ]
                            <:
                            Hacspec_ml_kem.Parameters.t_FieldElement)
                            .Hacspec_ml_kem.Parameters.f_val
                          <:
                          u16)
                      <:
                      u32) +!
                    (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
                    <:
                    u32) -!
                  (cast ((p2.[ j ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
                          .Hacspec_ml_kem.Parameters.f_val
                        <:
                        u16)
                    <:
                    u32)
                  <:
                  u32) %!
                (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
                <:
                u32)
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

let add_vectors
      (v_RANK: usize)
      (v1 v2: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    : t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
  Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
        (mk_usize 256))
    v_RANK
    #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (fun i ->
        let i:usize = i in
        add_polynomials (v1.[ i ] <: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
          )
          (v2.[ i ] <: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
        <:
        t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))

let multiply_matrix_by_column
      (v_RANK: usize)
      (matrix:
          t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK)
      (vector: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    : t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
  Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
        (mk_usize 256))
    v_RANK
    #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (fun i ->
        let i:usize = i in
        let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
          Rust_primitives.Hax.repeat (Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 0)
              <:
              Hacspec_ml_kem.Parameters.t_FieldElement)
            (mk_usize 256)
        in
        let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
          Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
            v_RANK
            (fun result temp_1_ ->
                let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
                  result
                in
                let _:usize = temp_1_ in
                true)
            result
            (fun result j ->
                let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
                  result
                in
                let j:usize = j in
                let product:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
                  Hacspec_ml_kem.Ntt.multiply_ntts ((matrix.[ j ]
                        <:
                        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                          v_RANK).[ i ]
                      <:
                      t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                    (vector.[ j ] <: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
                    )
                in
                let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
                  add_polynomials result product
                in
                result)
        in
        result)

let multiply_vectors
      (v_RANK: usize)
      (v1 v2: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    Rust_primitives.Hax.repeat (Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 0)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)
      (mk_usize 256)
  in
  let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      v_RANK
      (fun result temp_1_ ->
          let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) = result in
          let _:usize = temp_1_ in
          true)
      result
      (fun result j ->
          let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) = result in
          let j:usize = j in
          let product:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
            Hacspec_ml_kem.Ntt.multiply_ntts (v1.[ j ]
                <:
                t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
              (v2.[ j ] <: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
          in
          let result:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
            add_polynomials result product
          in
          result)
  in
  result

let transpose
      (v_RANK: usize)
      (matrix:
          t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK)
    : t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      v_RANK =
  Hacspec_ml_kem.Parameters.createi #(t_Array
        (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    v_RANK
    #(usize -> t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    (fun i ->
        let i:usize = i in
        Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
              (mk_usize 256))
          v_RANK
          #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
          (fun j ->
              let j:usize = j in
              (matrix.[ j ]
                <:
                t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK).[ i
              ]
              <:
              t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
        <:
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)

/// Sample the matrix A from a seed. Corresponds to `sample_matrix_A` in the implementation.
/// When `transpose` is true, A_transpose[j][i] = sampled(i, j).
/// When `transpose` is false, A_transpose[i][j] = sampled(i, j).
let sample_matrix_A (v_RANK: usize) (seed_for_A: t_Slice u8) (transpose: bool)
    : Prims.Pure
      (Core_models.Result.t_Result
          (t_Array
              (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
              v_RANK) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (requires
        (Core_models.Slice.impl__len #u8 seed_for_A <: usize) =. mk_usize 32 &&
        v_RANK <=. mk_usize 4)
      (fun _ -> Prims.l_True) =
  let
  (v_A_as_ntt:
    t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      v_RANK):t_Array
    (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK) v_RANK =
    Rust_primitives.Hax.repeat (Rust_primitives.Hax.repeat (Rust_primitives.Hax.repeat (Hacspec_ml_kem.Parameters.impl_FieldElement__new
                  (mk_u16 0)
                <:
                Hacspec_ml_kem.Parameters.t_FieldElement)
              (mk_usize 256)
            <:
            t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
          v_RANK
        <:
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      v_RANK
  in
  let xof_input:t_Array u8 (mk_usize 34) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 34) in
  let xof_input:t_Array u8 (mk_usize 34) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to xof_input
      ({ Core_models.Ops.Range.f_end = mk_usize 32 } <: Core_models.Ops.Range.t_RangeTo usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (xof_input.[ { Core_models.Ops.Range.f_end = mk_usize 32 }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          seed_for_A
        <:
        t_Slice u8)
  in
  match
    Rust_primitives.Hax.Folds.fold_range_return (mk_usize 0)
      v_RANK
      (fun temp_0_ temp_1_ ->
          let
          (v_A_as_ntt:
            t_Array
              (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
              v_RANK),
          (xof_input: t_Array u8 (mk_usize 34)) =
            temp_0_
          in
          let _:usize = temp_1_ in
          true)
      (v_A_as_ntt, xof_input
        <:
        (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK &
          t_Array u8 (mk_usize 34)))
      (fun temp_0_ i ->
          let
          (v_A_as_ntt:
            t_Array
              (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
              v_RANK),
          (xof_input: t_Array u8 (mk_usize 34)) =
            temp_0_
          in
          let i:usize = i in
          match
            Rust_primitives.Hax.Folds.fold_range_return (mk_usize 0)
              v_RANK
              (fun temp_0_ temp_1_ ->
                  let
                  (v_A_as_ntt:
                    t_Array
                      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                          v_RANK) v_RANK),
                  (xof_input: t_Array u8 (mk_usize 34)) =
                    temp_0_
                  in
                  let _:usize = temp_1_ in
                  true)
              (v_A_as_ntt, xof_input
                <:
                (t_Array
                    (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                        v_RANK) v_RANK &
                  t_Array u8 (mk_usize 34)))
              (fun temp_0_ j ->
                  let
                  (v_A_as_ntt:
                    t_Array
                      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                          v_RANK) v_RANK),
                  (xof_input: t_Array u8 (mk_usize 34)) =
                    temp_0_
                  in
                  let j:usize = j in
                  let xof_input:t_Array u8 (mk_usize 34) =
                    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof_input
                      (mk_usize 32)
                      (cast (i <: usize) <: u8)
                  in
                  let xof_input:t_Array u8 (mk_usize 34) =
                    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize xof_input
                      (mk_usize 33)
                      (cast (j <: usize) <: u8)
                  in
                  let (xof_bytes: t_Array u8 (mk_usize 840)):t_Array u8 (mk_usize 840) =
                    Hacspec_ml_kem.Parameters.Hash_functions.v_XOF (mk_usize 840)
                      (xof_input <: t_Slice u8)
                  in
                  match
                    Hacspec_ml_kem.Sampling.sample_ntt (mk_usize 70)
                      (mk_usize 560)
                      (mk_usize 840)
                      (mk_usize 6720)
                      xof_bytes
                    <:
                    Core_models.Result.t_Result
                      (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError
                  with
                  | Core_models.Result.Result_Ok sampled ->
                    if transpose
                    then
                      let v_A_as_ntt:t_Array
                        (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                            v_RANK) v_RANK =
                        Rust_primitives.Hax.Monomorphized_update_at.update_at_usize v_A_as_ntt
                          j
                          (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (v_A_as_ntt.[
                                  j ]
                                <:
                                t_Array
                                  (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                                  v_RANK)
                              i
                              sampled
                            <:
                            t_Array
                              (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                              v_RANK)
                      in
                      Core_models.Ops.Control_flow.ControlFlow_Continue
                      (v_A_as_ntt, xof_input
                        <:
                        (t_Array
                            (t_Array
                                (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                                v_RANK) v_RANK &
                          t_Array u8 (mk_usize 34)))
                      <:
                      Core_models.Ops.Control_flow.t_ControlFlow
                        (Core_models.Ops.Control_flow.t_ControlFlow
                            (Core_models.Result.t_Result
                                (t_Array
                                    (t_Array
                                        (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                                            (mk_usize 256)) v_RANK) v_RANK)
                                Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
                            (Prims.unit &
                              (t_Array
                                  (t_Array
                                      (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                                          (mk_usize 256)) v_RANK) v_RANK &
                                t_Array u8 (mk_usize 34))))
                        (t_Array
                            (t_Array
                                (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                                v_RANK) v_RANK &
                          t_Array u8 (mk_usize 34))
                    else
                      let v_A_as_ntt:t_Array
                        (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                            v_RANK) v_RANK =
                        Rust_primitives.Hax.Monomorphized_update_at.update_at_usize v_A_as_ntt
                          i
                          (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (v_A_as_ntt.[
                                  i ]
                                <:
                                t_Array
                                  (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                                  v_RANK)
                              j
                              sampled
                            <:
                            t_Array
                              (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                              v_RANK)
                      in
                      Core_models.Ops.Control_flow.ControlFlow_Continue
                      (v_A_as_ntt, xof_input
                        <:
                        (t_Array
                            (t_Array
                                (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                                v_RANK) v_RANK &
                          t_Array u8 (mk_usize 34)))
                      <:
                      Core_models.Ops.Control_flow.t_ControlFlow
                        (Core_models.Ops.Control_flow.t_ControlFlow
                            (Core_models.Result.t_Result
                                (t_Array
                                    (t_Array
                                        (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                                            (mk_usize 256)) v_RANK) v_RANK)
                                Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
                            (Prims.unit &
                              (t_Array
                                  (t_Array
                                      (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                                          (mk_usize 256)) v_RANK) v_RANK &
                                t_Array u8 (mk_usize 34))))
                        (t_Array
                            (t_Array
                                (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                                v_RANK) v_RANK &
                          t_Array u8 (mk_usize 34))
                  | Core_models.Result.Result_Err err ->
                    Core_models.Ops.Control_flow.ControlFlow_Break
                    (Core_models.Ops.Control_flow.ControlFlow_Break
                      (Core_models.Result.Result_Err err
                        <:
                        Core_models.Result.t_Result
                          (t_Array
                              (t_Array
                                  (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                                  v_RANK) v_RANK)
                          Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
                      <:
                      Core_models.Ops.Control_flow.t_ControlFlow
                        (Core_models.Result.t_Result
                            (t_Array
                                (t_Array
                                    (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
                                    ) v_RANK) v_RANK)
                            Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
                        (Prims.unit &
                          (t_Array
                              (t_Array
                                  (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                                  v_RANK) v_RANK &
                            t_Array u8 (mk_usize 34))))
                    <:
                    Core_models.Ops.Control_flow.t_ControlFlow
                      (Core_models.Ops.Control_flow.t_ControlFlow
                          (Core_models.Result.t_Result
                              (t_Array
                                  (t_Array
                                      (t_Array Hacspec_ml_kem.Parameters.t_FieldElement
                                          (mk_usize 256)) v_RANK) v_RANK)
                              Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
                          (Prims.unit &
                            (t_Array
                                (t_Array
                                    (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
                                    ) v_RANK) v_RANK &
                              t_Array u8 (mk_usize 34))))
                      (t_Array
                          (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                              v_RANK) v_RANK &
                        t_Array u8 (mk_usize 34)))
            <:
            Core_models.Ops.Control_flow.t_ControlFlow
              (Core_models.Result.t_Result
                  (t_Array
                      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                          v_RANK) v_RANK)
                  Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
              (t_Array
                  (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
                  v_RANK &
                t_Array u8 (mk_usize 34))
          with
          | Core_models.Ops.Control_flow.ControlFlow_Break ret ->
            Core_models.Ops.Control_flow.ControlFlow_Break
            (Core_models.Ops.Control_flow.ControlFlow_Break ret
              <:
              Core_models.Ops.Control_flow.t_ControlFlow
                (Core_models.Result.t_Result
                    (t_Array
                        (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                            v_RANK) v_RANK)
                    Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
                (Prims.unit &
                  (t_Array
                      (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                          v_RANK) v_RANK &
                    t_Array u8 (mk_usize 34))))
            <:
            Core_models.Ops.Control_flow.t_ControlFlow
              (Core_models.Ops.Control_flow.t_ControlFlow
                  (Core_models.Result.t_Result
                      (t_Array
                          (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                              v_RANK) v_RANK)
                      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
                  (Prims.unit &
                    (t_Array
                        (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                            v_RANK) v_RANK &
                      t_Array u8 (mk_usize 34))))
              (t_Array
                  (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
                  v_RANK &
                t_Array u8 (mk_usize 34))
          | Core_models.Ops.Control_flow.ControlFlow_Continue loop_res ->
            Core_models.Ops.Control_flow.ControlFlow_Continue loop_res
            <:
            Core_models.Ops.Control_flow.t_ControlFlow
              (Core_models.Ops.Control_flow.t_ControlFlow
                  (Core_models.Result.t_Result
                      (t_Array
                          (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                              v_RANK) v_RANK)
                      Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
                  (Prims.unit &
                    (t_Array
                        (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
                            v_RANK) v_RANK &
                      t_Array u8 (mk_usize 34))))
              (t_Array
                  (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
                  v_RANK &
                t_Array u8 (mk_usize 34)))
    <:
    Core_models.Ops.Control_flow.t_ControlFlow
      (Core_models.Result.t_Result
          (t_Array
              (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
              v_RANK) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError)
      (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK &
        t_Array u8 (mk_usize 34))
  with
  | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
  | Core_models.Ops.Control_flow.ControlFlow_Continue (v_A_as_ntt, xof_input) ->
    Core_models.Result.Result_Ok v_A_as_ntt
    <:
    Core_models.Result.t_Result
      (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK) Hacspec_ml_kem.Sampling.t_BadRejectionSamplingRandomnessError

/// Compute v − InverseNTT(sᵀ ◦ NTT(u)).
/// Corresponds to `compute_message` in the implementation's `matrix.rs`.
/// Used in K-PKE.Decrypt (Algorithm 15) to recover the message:
///   w ← v - NTT⁻¹(ŝᵀ ◦ NTT(u))
let compute_message
      (v_RANK: usize)
      (v: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (secret_as_ntt u_as_ntt:
          t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  let inner_product:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    multiply_vectors v_RANK secret_as_ntt u_as_ntt
  in
  let inner_product_inv:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    Hacspec_ml_kem.Invert_ntt.ntt_inverse inner_product
  in
  sub_polynomials v inner_product_inv

/// Compute InverseNTT(tᵀ ◦ r̂) + e₂ + message.
/// Corresponds to `compute_ring_element_v` in the implementation's `matrix.rs`.
/// Used in K-PKE.Encrypt (Algorithm 14):
///   v ← NTT⁻¹(t̂ᵀ ◦ r̂) + e₂ + μ
let compute_ring_element_v
      (v_RANK: usize)
      (tt_as_ntt r_as_ntt:
          t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      (error_2_ message: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  let inner_product:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    multiply_vectors v_RANK tt_as_ntt r_as_ntt
  in
  let inner_product_inv:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    Hacspec_ml_kem.Invert_ntt.ntt_inverse inner_product
  in
  add_polynomials (add_polynomials inner_product_inv error_2_
      <:
      t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    message

/// Compute u := InvertNTT(Aᵀ ◦ r̂) + e₁.
/// Corresponds to `compute_vector_u` in the implementation's `matrix.rs`.
/// Used in K-PKE.Encrypt (Algorithm 14):
///   u ← NTT⁻¹(Âᵀ ◦ r̂) + e₁
let compute_vector_u
      (v_RANK: usize)
      (a_as_ntt:
          t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK)
      (r_as_ntt error_1_:
          t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    : t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
  let a_transpose:t_Array
    (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK) v_RANK =
    transpose v_RANK a_as_ntt
  in
  let product:t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    multiply_matrix_by_column v_RANK a_transpose r_as_ntt
  in
  let
  (product_inv: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK):t_Array
    (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
          (mk_usize 256))
      v_RANK
      #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (fun i ->
          let i:usize = i in
          Hacspec_ml_kem.Invert_ntt.ntt_inverse (product.[ i ]
              <:
              t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
          <:
          t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
  in
  add_vectors v_RANK product_inv error_1_

/// Compute t̂ := Â ◦ ŝ + ê.
/// Corresponds to `compute_As_plus_e` in the implementation's `matrix.rs`.
/// Used in K-PKE.KeyGen (Algorithm 13):
///   t̂ ← Â◦ŝ + ê
let compute_As_plus_e
      (v_RANK: usize)
      (a_as_ntt:
          t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
            v_RANK)
      (s_as_ntt error_as_ntt:
          t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    : t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
  let product:t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
    multiply_matrix_by_column v_RANK a_as_ntt s_as_ntt
  in
  add_vectors v_RANK product error_as_ntt
