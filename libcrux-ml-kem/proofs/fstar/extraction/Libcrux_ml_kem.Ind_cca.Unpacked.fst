module Libcrux_ml_kem.Ind_cca.Unpacked
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_ml_kem.Hash_functions in
  let open Libcrux_ml_kem.Hash_functions.Portable in
  let open Libcrux_ml_kem.Ind_cpa.Unpacked in
  let open Libcrux_ml_kem.Types in
  let open Libcrux_ml_kem.Variant in
  let open Libcrux_ml_kem.Vector in
  let open Libcrux_ml_kem.Vector.Traits in
  ()

#push-options "--admit_smt_queries true"

#push-options "--z3rlimit 300 --split_queries always"

let unpack_public_key
      (v_K v_T_AS_NTT_ENCODED_SIZE v_PUBLIC_KEY_SIZE: usize)
      (#v_Hasher #v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Hash_functions.t_Hash v_Hasher v_K)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (public_key: Libcrux_ml_kem.Types.t_MlKemPublicKey v_PUBLIC_KEY_SIZE)
      (unpacked_public_key: t_MlKemPublicKeyUnpacked v_K v_Vector)
     =
  let unpacked_public_key:t_MlKemPublicKeyUnpacked v_K v_Vector =
    {
      unpacked_public_key with
      f_ind_cpa_public_key
      =
      {
        unpacked_public_key.f_ind_cpa_public_key with
        Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt
        =
        Libcrux_ml_kem.Serialize.deserialize_ring_elements_reduced v_K
          #v_Vector
          (public_key.Libcrux_ml_kem.Types.f_value.[ {
                Core_models.Ops.Range.f_end = v_T_AS_NTT_ENCODED_SIZE
              }
              <:
              Core_models.Ops.Range.t_RangeTo usize ]
            <:
            t_Slice u8)
          unpacked_public_key.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt
      }
      <:
      Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPublicKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemPublicKeyUnpacked v_K v_Vector
  in
  let _:Prims.unit =
    let _, seed = split public_key.f_value (Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size v_K) in
    eq_intro (Libcrux_ml_kem.Utils.into_padded_array (sz 32) seed) seed;
    eq_intro (Seq.slice (Libcrux_ml_kem.Utils.into_padded_array (sz 34) seed) 0 32) seed
  in
  let unpacked_public_key:t_MlKemPublicKeyUnpacked v_K v_Vector =
    {
      unpacked_public_key with
      f_ind_cpa_public_key
      =
      {
        unpacked_public_key.f_ind_cpa_public_key with
        Libcrux_ml_kem.Ind_cpa.Unpacked.f_seed_for_A
        =
        Libcrux_ml_kem.Utils.into_padded_array (mk_usize 32)
          (public_key.Libcrux_ml_kem.Types.f_value.[ {
                Core_models.Ops.Range.f_start = v_T_AS_NTT_ENCODED_SIZE
              }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
      }
      <:
      Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPublicKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemPublicKeyUnpacked v_K v_Vector
  in
  let unpacked_public_key:t_MlKemPublicKeyUnpacked v_K v_Vector =
    {
      unpacked_public_key with
      f_ind_cpa_public_key
      =
      {
        unpacked_public_key.f_ind_cpa_public_key with
        Libcrux_ml_kem.Ind_cpa.Unpacked.f_A
        =
        Libcrux_ml_kem.Matrix.sample_matrix_A v_K
          #v_Vector
          #v_Hasher
          unpacked_public_key.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_A
          (Libcrux_ml_kem.Utils.into_padded_array (mk_usize 34)
              (public_key.Libcrux_ml_kem.Types.f_value.[ {
                    Core_models.Ops.Range.f_start = v_T_AS_NTT_ENCODED_SIZE
                  }
                  <:
                  Core_models.Ops.Range.t_RangeFrom usize ]
                <:
                t_Slice u8)
            <:
            t_Array u8 (mk_usize 34))
          false
      }
      <:
      Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPublicKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemPublicKeyUnpacked v_K v_Vector
  in
  let unpacked_public_key:t_MlKemPublicKeyUnpacked v_K v_Vector =
    {
      unpacked_public_key with
      f_public_key_hash
      =
      Libcrux_ml_kem.Hash_functions.f_H #v_Hasher
        #v_K
        #FStar.Tactics.Typeclasses.solve
        (Libcrux_ml_kem.Types.impl_23__as_slice v_PUBLIC_KEY_SIZE public_key <: t_Slice u8)
    }
    <:
    t_MlKemPublicKeyUnpacked v_K v_Vector
  in
  unpacked_public_key

#pop-options

#pop-options

let impl_3__serialized_mut
      (v_K: usize)
      (#v_Vector: Type0)
      (v_PUBLIC_KEY_SIZE: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (self: t_MlKemPublicKeyUnpacked v_K v_Vector)
      (serialized: Libcrux_ml_kem.Types.t_MlKemPublicKey v_PUBLIC_KEY_SIZE)
     =
  let serialized:Libcrux_ml_kem.Types.t_MlKemPublicKey v_PUBLIC_KEY_SIZE =
    {
      serialized with
      Libcrux_ml_kem.Types.f_value
      =
      Libcrux_ml_kem.Ind_cpa.serialize_public_key_mut v_K
        v_PUBLIC_KEY_SIZE
        #v_Vector
        self.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt
        (self.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_seed_for_A <: t_Slice u8)
        serialized.Libcrux_ml_kem.Types.f_value
    }
    <:
    Libcrux_ml_kem.Types.t_MlKemPublicKey v_PUBLIC_KEY_SIZE
  in
  serialized

let impl_3__serialized
      (v_K: usize)
      (#v_Vector: Type0)
      (v_PUBLIC_KEY_SIZE: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (self: t_MlKemPublicKeyUnpacked v_K v_Vector)
     =
  Core_models.Convert.f_from #(Libcrux_ml_kem.Types.t_MlKemPublicKey v_PUBLIC_KEY_SIZE)
    #(t_Array u8 v_PUBLIC_KEY_SIZE)
    #FStar.Tactics.Typeclasses.solve
    (Libcrux_ml_kem.Ind_cpa.serialize_public_key v_K
        v_PUBLIC_KEY_SIZE
        #v_Vector
        self.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt
        (self.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_seed_for_A <: t_Slice u8)
      <:
      t_Array u8 v_PUBLIC_KEY_SIZE)

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl
      (v_K: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    : Core_models.Default.t_Default (t_MlKemPublicKeyUnpacked v_K v_Vector) =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: t_MlKemPublicKeyUnpacked v_K v_Vector) -> true);
    f_default
    =
    fun (_: Prims.unit) ->
      {
        f_ind_cpa_public_key
        =
        Core_models.Default.f_default #(Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPublicKeyUnpacked
              v_K v_Vector)
          #FStar.Tactics.Typeclasses.solve
          ();
        f_public_key_hash = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32)
      }
      <:
      t_MlKemPublicKeyUnpacked v_K v_Vector
  }

let keys_from_private_key
      (v_K v_SECRET_KEY_SIZE v_CPA_SECRET_KEY_SIZE v_PUBLIC_KEY_SIZE v_T_AS_NTT_ENCODED_SIZE: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (private_key: Libcrux_ml_kem.Types.t_MlKemPrivateKey v_SECRET_KEY_SIZE)
      (key_pair: t_MlKemKeyPairUnpacked v_K v_Vector)
     =
  let
  (ind_cpa_secret_key: t_Slice u8),
  (ind_cpa_public_key: t_Slice u8),
  (ind_cpa_public_key_hash: t_Slice u8),
  (implicit_rejection_value: t_Slice u8) =
    Libcrux_ml_kem.Types.unpack_private_key v_CPA_SECRET_KEY_SIZE
      v_PUBLIC_KEY_SIZE
      (private_key.Libcrux_ml_kem.Types.f_value <: t_Slice u8)
  in
  let key_pair:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      key_pair with
      f_private_key
      =
      {
        key_pair.f_private_key with
        f_ind_cpa_private_key
        =
        {
          key_pair.f_private_key.f_ind_cpa_private_key with
          Libcrux_ml_kem.Ind_cpa.Unpacked.f_secret_as_ntt
          =
          Libcrux_ml_kem.Ind_cpa.deserialize_vector v_K
            #v_Vector
            ind_cpa_secret_key
            key_pair.f_private_key.f_ind_cpa_private_key
              .Libcrux_ml_kem.Ind_cpa.Unpacked.f_secret_as_ntt
        }
        <:
        Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPrivateKeyUnpacked v_K v_Vector
      }
      <:
      t_MlKemPrivateKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  let key_pair:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      key_pair with
      f_public_key
      =
      {
        key_pair.f_public_key with
        f_ind_cpa_public_key
        =
        Libcrux_ml_kem.Ind_cpa.build_unpacked_public_key_mut v_K
          v_T_AS_NTT_ENCODED_SIZE
          #v_Vector
          #(Libcrux_ml_kem.Hash_functions.Portable.t_PortableHash v_K)
          ind_cpa_public_key
          key_pair.f_public_key.f_ind_cpa_public_key
      }
      <:
      t_MlKemPublicKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  let key_pair:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      key_pair with
      f_public_key
      =
      {
        key_pair.f_public_key with
        f_public_key_hash
        =
        Core_models.Slice.impl__copy_from_slice #u8
          key_pair.f_public_key.f_public_key_hash
          ind_cpa_public_key_hash
      }
      <:
      t_MlKemPublicKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  let key_pair:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      key_pair with
      f_private_key
      =
      {
        key_pair.f_private_key with
        f_implicit_rejection_value
        =
        Core_models.Slice.impl__copy_from_slice #u8
          key_pair.f_private_key.f_implicit_rejection_value
          implicit_rejection_value
      }
      <:
      t_MlKemPrivateKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  let key_pair:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      key_pair with
      f_public_key
      =
      {
        key_pair.f_public_key with
        f_ind_cpa_public_key
        =
        {
          key_pair.f_public_key.f_ind_cpa_public_key with
          Libcrux_ml_kem.Ind_cpa.Unpacked.f_seed_for_A
          =
          Core_models.Slice.impl__copy_from_slice #u8
            key_pair.f_public_key.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_seed_for_A
            (ind_cpa_public_key.[ { Core_models.Ops.Range.f_start = v_T_AS_NTT_ENCODED_SIZE }
                <:
                Core_models.Ops.Range.t_RangeFrom usize ]
              <:
              t_Slice u8)
        }
        <:
        Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPublicKeyUnpacked v_K v_Vector
      }
      <:
      t_MlKemPublicKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  key_pair

let impl_4__public_key
      (v_K: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (self: t_MlKemKeyPairUnpacked v_K v_Vector)
     = self.f_public_key

let impl_4__private_key
      (v_K: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (self: t_MlKemKeyPairUnpacked v_K v_Vector)
     = self.f_private_key

let impl_4__serialized_public_key_mut
      (v_K: usize)
      (#v_Vector: Type0)
      (v_PUBLIC_KEY_SIZE: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (self: t_MlKemKeyPairUnpacked v_K v_Vector)
      (serialized: Libcrux_ml_kem.Types.t_MlKemPublicKey v_PUBLIC_KEY_SIZE)
     =
  let serialized:Libcrux_ml_kem.Types.t_MlKemPublicKey v_PUBLIC_KEY_SIZE =
    impl_3__serialized_mut v_K #v_Vector v_PUBLIC_KEY_SIZE self.f_public_key serialized
  in
  serialized

let impl_4__serialized_public_key
      (v_K: usize)
      (#v_Vector: Type0)
      (v_PUBLIC_KEY_SIZE: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (self: t_MlKemKeyPairUnpacked v_K v_Vector)
     = impl_3__serialized v_K #v_Vector v_PUBLIC_KEY_SIZE self.f_public_key

let impl_4__serialized_private_key_mut
      (v_K: usize)
      (#v_Vector: Type0)
      (v_CPA_PRIVATE_KEY_SIZE v_PRIVATE_KEY_SIZE v_PUBLIC_KEY_SIZE: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (self: t_MlKemKeyPairUnpacked v_K v_Vector)
      (serialized: Libcrux_ml_kem.Types.t_MlKemPrivateKey v_PRIVATE_KEY_SIZE)
     =
  let
  (ind_cpa_private_key: t_Array u8 v_CPA_PRIVATE_KEY_SIZE),
  (ind_cpa_public_key: t_Array u8 v_PUBLIC_KEY_SIZE) =
    Libcrux_ml_kem.Ind_cpa.serialize_unpacked_secret_key v_K
      v_CPA_PRIVATE_KEY_SIZE
      v_PUBLIC_KEY_SIZE
      #v_Vector
      self.f_public_key.f_ind_cpa_public_key
      self.f_private_key.f_ind_cpa_private_key
  in
  let serialized:Libcrux_ml_kem.Types.t_MlKemPrivateKey v_PRIVATE_KEY_SIZE =
    {
      serialized with
      Libcrux_ml_kem.Types.f_value
      =
      Libcrux_ml_kem.Ind_cca.serialize_kem_secret_key_mut v_K
        v_PRIVATE_KEY_SIZE
        #(Libcrux_ml_kem.Hash_functions.Portable.t_PortableHash v_K)
        (ind_cpa_private_key <: t_Slice u8)
        (ind_cpa_public_key <: t_Slice u8)
        (self.f_private_key.f_implicit_rejection_value <: t_Slice u8)
        serialized.Libcrux_ml_kem.Types.f_value
    }
    <:
    Libcrux_ml_kem.Types.t_MlKemPrivateKey v_PRIVATE_KEY_SIZE
  in
  serialized

let impl_4__serialized_private_key
      (v_K: usize)
      (#v_Vector: Type0)
      (v_CPA_PRIVATE_KEY_SIZE v_PRIVATE_KEY_SIZE v_PUBLIC_KEY_SIZE: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (self: t_MlKemKeyPairUnpacked v_K v_Vector)
     =
  let sk:Libcrux_ml_kem.Types.t_MlKemPrivateKey v_PRIVATE_KEY_SIZE =
    Core_models.Default.f_default #(Libcrux_ml_kem.Types.t_MlKemPrivateKey v_PRIVATE_KEY_SIZE)
      #FStar.Tactics.Typeclasses.solve
      ()
  in
  let sk:Libcrux_ml_kem.Types.t_MlKemPrivateKey v_PRIVATE_KEY_SIZE =
    impl_4__serialized_private_key_mut v_K
      #v_Vector
      v_CPA_PRIVATE_KEY_SIZE
      v_PRIVATE_KEY_SIZE
      v_PUBLIC_KEY_SIZE
      self
      sk
  in
  sk

[@@ FStar.Tactics.Typeclasses.tcinstance]
let impl_1
      (v_K: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
    : Core_models.Default.t_Default (t_MlKemKeyPairUnpacked v_K v_Vector) =
  {
    f_default_pre = (fun (_: Prims.unit) -> true);
    f_default_post = (fun (_: Prims.unit) (out: t_MlKemKeyPairUnpacked v_K v_Vector) -> true);
    f_default
    =
    fun (_: Prims.unit) ->
      {
        f_private_key
        =
        {
          f_ind_cpa_private_key
          =
          Core_models.Default.f_default #(Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPrivateKeyUnpacked
                v_K v_Vector)
            #FStar.Tactics.Typeclasses.solve
            ();
          f_implicit_rejection_value = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32)
        }
        <:
        t_MlKemPrivateKeyUnpacked v_K v_Vector;
        f_public_key
        =
        Core_models.Default.f_default #(t_MlKemPublicKeyUnpacked v_K v_Vector)
          #FStar.Tactics.Typeclasses.solve
          ()
      }
      <:
      t_MlKemKeyPairUnpacked v_K v_Vector
  }

let impl_4__new
      (v_K: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (_: Prims.unit)
     =
  Core_models.Default.f_default #(t_MlKemKeyPairUnpacked v_K v_Vector)
    #FStar.Tactics.Typeclasses.solve
    ()

let impl_4__from_private_key
      (v_K: usize)
      (#v_Vector: Type0)
      (v_SECRET_KEY_SIZE v_CPA_SECRET_KEY_SIZE v_PUBLIC_KEY_SIZE v_T_AS_NTT_ENCODED_SIZE: usize)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (private_key: Libcrux_ml_kem.Types.t_MlKemPrivateKey v_SECRET_KEY_SIZE)
     =
  let out:t_MlKemKeyPairUnpacked v_K v_Vector =
    Core_models.Default.f_default #(t_MlKemKeyPairUnpacked v_K v_Vector)
      #FStar.Tactics.Typeclasses.solve
      ()
  in
  let out:t_MlKemKeyPairUnpacked v_K v_Vector =
    keys_from_private_key v_K
      v_SECRET_KEY_SIZE
      v_CPA_SECRET_KEY_SIZE
      v_PUBLIC_KEY_SIZE
      v_T_AS_NTT_ENCODED_SIZE
      #v_Vector
      private_key
      out
  in
  out

#push-options "--z3rlimit 200"

[@ "opaque_to_smt"]

let transpose_a
      (v_K: usize)
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (ind_cpa_a:
          t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K) v_K)
     =
  let v_A:t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K) v_K =
    Core_models.Array.from_fn #(t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K
      )
      v_K
      #(usize -> t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K)
      (fun e_i ->
          let e_i:usize = e_i in
          Core_models.Array.from_fn #(Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
            v_K
            #(usize -> Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
            (fun e_j ->
                let e_j:usize = e_j in
                Libcrux_ml_kem.Polynomial.impl__ZERO #v_Vector ()
                <:
                Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
          <:
          t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K)
  in
  let v_A:t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K) v_K =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      v_K
      (fun v_A i ->
          let v_A:t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K) v_K
          =
            v_A
          in
          let i:usize = i in
          forall (j: nat).
            j < v i ==>
            (forall (k: nat).
                k < v v_K ==> Seq.index (Seq.index v_A j) k == Seq.index (Seq.index ind_cpa_a k) j))
      v_A
      (fun v_A i ->
          let v_A:t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K) v_K
          =
            v_A
          in
          let i:usize = i in
          let e_a_i:t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K)
            v_K =
            v_A
          in
          Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
            v_K
            (fun v_A j ->
                let v_A:t_Array
                  (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K) v_K =
                  v_A
                in
                let j:usize = j in
                (forall (k: nat). k < v i ==> Seq.index v_A k == Seq.index e_a_i k) /\
                (forall (k: nat).
                    k < v j ==>
                    Seq.index (Seq.index v_A (v i)) k == Seq.index (Seq.index ind_cpa_a k) (v i)))
            v_A
            (fun v_A j ->
                let v_A:t_Array
                  (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K) v_K =
                  v_A
                in
                let j:usize = j in
                let v_A:t_Array
                  (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K) v_K =
                  Rust_primitives.Hax.Monomorphized_update_at.update_at_usize v_A
                    i
                    (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize (v_A.[ i ]
                          <:
                          t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K)
                        j
                        (Core_models.Clone.f_clone #(Libcrux_ml_kem.Vector.t_PolynomialRingElement
                              v_Vector)
                            #FStar.Tactics.Typeclasses.solve
                            ((ind_cpa_a.[ j ]
                                <:
                                t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K
                              ).[ i ]
                              <:
                              Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
                          <:
                          Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
                      <:
                      t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K)
                in
                v_A))
  in
  v_A

#pop-options

#push-options "--z3rlimit 300 --ext context_pruning --split_queries always"

let generate_keypair
      (v_K v_CPA_PRIVATE_KEY_SIZE v_PRIVATE_KEY_SIZE v_PUBLIC_KEY_SIZE v_ETA1 v_ETA1_RANDOMNESS_SIZE:
          usize)
      (#v_Vector #v_Hasher #v_Scheme: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_ml_kem.Hash_functions.t_Hash v_Hasher v_K)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i2: Libcrux_ml_kem.Variant.t_Variant v_Scheme)
      (randomness: t_Array u8 (mk_usize 64))
      (out: t_MlKemKeyPairUnpacked v_K v_Vector)
     =
  let ind_cpa_keypair_randomness:t_Slice u8 =
    randomness.[ {
        Core_models.Ops.Range.f_start = mk_usize 0;
        Core_models.Ops.Range.f_end = Libcrux_ml_kem.Constants.v_CPA_PKE_KEY_GENERATION_SEED_SIZE
      }
      <:
      Core_models.Ops.Range.t_Range usize ]
  in
  let implicit_rejection_value:t_Slice u8 =
    randomness.[ {
        Core_models.Ops.Range.f_start = Libcrux_ml_kem.Constants.v_CPA_PKE_KEY_GENERATION_SEED_SIZE
      }
      <:
      Core_models.Ops.Range.t_RangeFrom usize ]
  in
  let
  (tmp0: Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPrivateKeyUnpacked v_K v_Vector),
  (tmp1: Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPublicKeyUnpacked v_K v_Vector) =
    Libcrux_ml_kem.Ind_cpa.generate_keypair_unpacked v_K
      v_ETA1
      v_ETA1_RANDOMNESS_SIZE
      #v_Vector
      #v_Hasher
      #v_Scheme
      ind_cpa_keypair_randomness
      out.f_private_key.f_ind_cpa_private_key
      out.f_public_key.f_ind_cpa_public_key
  in
  let out:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      out with
      f_private_key
      =
      { out.f_private_key with f_ind_cpa_private_key = tmp0 }
      <:
      t_MlKemPrivateKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  let out:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      out with
      f_public_key
      =
      { out.f_public_key with f_ind_cpa_public_key = tmp1 } <: t_MlKemPublicKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  let _:Prims.unit = () in
  let v_A:t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) v_K) v_K =
    transpose_a v_K
      #v_Vector
      out.f_public_key.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_A
  in
  let out:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      out with
      f_public_key
      =
      {
        out.f_public_key with
        f_ind_cpa_public_key
        =
        { out.f_public_key.f_ind_cpa_public_key with Libcrux_ml_kem.Ind_cpa.Unpacked.f_A = v_A }
        <:
        Libcrux_ml_kem.Ind_cpa.Unpacked.t_IndCpaPublicKeyUnpacked v_K v_Vector
      }
      <:
      t_MlKemPublicKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  let pk_serialized:t_Array u8 v_PUBLIC_KEY_SIZE =
    Libcrux_ml_kem.Ind_cpa.serialize_public_key v_K
      v_PUBLIC_KEY_SIZE
      #v_Vector
      out.f_public_key.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_tt_as_ntt
      (out.f_public_key.f_ind_cpa_public_key.Libcrux_ml_kem.Ind_cpa.Unpacked.f_seed_for_A
        <:
        t_Slice u8)
  in
  let out:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      out with
      f_public_key
      =
      {
        out.f_public_key with
        f_public_key_hash
        =
        Libcrux_ml_kem.Hash_functions.f_H #v_Hasher
          #v_K
          #FStar.Tactics.Typeclasses.solve
          (pk_serialized <: t_Slice u8)
      }
      <:
      t_MlKemPublicKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  let out:t_MlKemKeyPairUnpacked v_K v_Vector =
    {
      out with
      f_private_key
      =
      {
        out.f_private_key with
        f_implicit_rejection_value
        =
        Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 32))
          #Core_models.Array.t_TryFromSliceError
          (Core_models.Convert.f_try_into #(t_Slice u8)
              #(t_Array u8 (mk_usize 32))
              #FStar.Tactics.Typeclasses.solve
              implicit_rejection_value
            <:
            Core_models.Result.t_Result (t_Array u8 (mk_usize 32))
              Core_models.Array.t_TryFromSliceError)
      }
      <:
      t_MlKemPrivateKeyUnpacked v_K v_Vector
    }
    <:
    t_MlKemKeyPairUnpacked v_K v_Vector
  in
  let _:Prims.unit = admit () (* Panic freedom *) in
  out

#pop-options

let encaps_prepare
      (v_K: usize)
      (#v_Hasher: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Hash_functions.t_Hash v_Hasher v_K)
      (randomness pk_hash: t_Slice u8)
     =
  let _:Prims.unit =
    eq_intro (Seq.slice (Libcrux_ml_kem.Utils.into_padded_array (sz 64) randomness) 0 32) randomness
  in
  let (to_hash: t_Array u8 (mk_usize 64)):t_Array u8 (mk_usize 64) =
    Libcrux_ml_kem.Utils.into_padded_array (mk_usize 64) randomness
  in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from to_hash
      ({ Core_models.Ops.Range.f_start = Libcrux_ml_kem.Constants.v_H_DIGEST_SIZE }
        <:
        Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ { Core_models.Ops.Range.f_start = Libcrux_ml_kem.Constants.v_H_DIGEST_SIZE }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          pk_hash
        <:
        t_Slice u8)
  in
  let _:Prims.unit = eq_intro to_hash (concat randomness pk_hash) in
  Libcrux_ml_kem.Hash_functions.f_G #v_Hasher
    #v_K
    #FStar.Tactics.Typeclasses.solve
    (to_hash <: t_Slice u8)

let encapsulate
      (v_K v_CIPHERTEXT_SIZE v_PUBLIC_KEY_SIZE v_T_AS_NTT_ENCODED_SIZE v_C1_SIZE v_C2_SIZE v_VECTOR_U_COMPRESSION_FACTOR v_VECTOR_V_COMPRESSION_FACTOR v_VECTOR_U_BLOCK_LEN v_ETA1 v_ETA1_RANDOMNESS_SIZE v_ETA2 v_ETA2_RANDOMNESS_SIZE:
          usize)
      (#v_Vector #v_Hasher: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_ml_kem.Hash_functions.t_Hash v_Hasher v_K)
      (public_key: t_MlKemPublicKeyUnpacked v_K v_Vector)
      (randomness: t_Array u8 (mk_usize 32))
     =
  let hashed:t_Array u8 (mk_usize 64) =
    encaps_prepare v_K
      #v_Hasher
      (randomness <: t_Slice u8)
      (public_key.f_public_key_hash <: t_Slice u8)
  in
  let (shared_secret: t_Slice u8), (pseudorandomness: t_Slice u8) =
    Core_models.Slice.impl__split_at #u8
      (hashed <: t_Slice u8)
      Libcrux_ml_kem.Constants.v_SHARED_SECRET_SIZE
  in
  let ciphertext:t_Array u8 v_CIPHERTEXT_SIZE =
    Libcrux_ml_kem.Ind_cpa.encrypt_unpacked v_K v_CIPHERTEXT_SIZE v_T_AS_NTT_ENCODED_SIZE v_C1_SIZE
      v_C2_SIZE v_VECTOR_U_COMPRESSION_FACTOR v_VECTOR_V_COMPRESSION_FACTOR v_VECTOR_U_BLOCK_LEN
      v_ETA1 v_ETA1_RANDOMNESS_SIZE v_ETA2 v_ETA2_RANDOMNESS_SIZE #v_Vector #v_Hasher
      public_key.f_ind_cpa_public_key randomness pseudorandomness
  in
  let shared_secret_array:t_Array u8 (mk_usize 32) =
    Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32)
  in
  let shared_secret_array:t_Array u8 (mk_usize 32) =
    Core_models.Slice.impl__copy_from_slice #u8 shared_secret_array shared_secret
  in
  let result:(Libcrux_ml_kem.Types.t_MlKemCiphertext v_CIPHERTEXT_SIZE & t_Array u8 (mk_usize 32)) =
    Core_models.Convert.f_from #(Libcrux_ml_kem.Types.t_MlKemCiphertext v_CIPHERTEXT_SIZE)
      #(t_Array u8 v_CIPHERTEXT_SIZE)
      #FStar.Tactics.Typeclasses.solve
      ciphertext,
    shared_secret_array
    <:
    (Libcrux_ml_kem.Types.t_MlKemCiphertext v_CIPHERTEXT_SIZE & t_Array u8 (mk_usize 32))
  in
  let _:Prims.unit = admit () (* Panic freedom *) in
  result

#push-options "--z3rlimit 400 --ext context_pruning --split_queries always"

let decapsulate
      (v_K v_SECRET_KEY_SIZE v_CPA_SECRET_KEY_SIZE v_PUBLIC_KEY_SIZE v_CIPHERTEXT_SIZE v_T_AS_NTT_ENCODED_SIZE v_C1_SIZE v_C2_SIZE v_VECTOR_U_COMPRESSION_FACTOR v_VECTOR_V_COMPRESSION_FACTOR v_C1_BLOCK_SIZE v_ETA1 v_ETA1_RANDOMNESS_SIZE v_ETA2 v_ETA2_RANDOMNESS_SIZE v_IMPLICIT_REJECTION_HASH_INPUT_SIZE:
          usize)
      (#v_Vector #v_Hasher: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i1:
          Libcrux_ml_kem.Hash_functions.t_Hash v_Hasher v_K)
      (key_pair: t_MlKemKeyPairUnpacked v_K v_Vector)
      (ciphertext: Libcrux_ml_kem.Types.t_MlKemCiphertext v_CIPHERTEXT_SIZE)
     =
  let _:Prims.unit =
    assert (v v_IMPLICIT_REJECTION_HASH_INPUT_SIZE ==
        32 + v (Hacspec_ml_kem.Parameters.cpa_ciphertext_size v_K));
    assert (v (Hacspec_ml_kem.Parameters.c1_size v_K +! Hacspec_ml_kem.Parameters.c2_size v_K) ==
        v (Hacspec_ml_kem.Parameters.c1_size v_K) + v (Hacspec_ml_kem.Parameters.c2_size v_K));
    assert (v (Hacspec_ml_kem.Parameters.c1_size v_K) ==
        v (Hacspec_ml_kem.Parameters.c1_block_size v_K) * v v_K);
    assert (v (Hacspec_ml_kem.Parameters.c1_block_size v_K) ==
        32 * v (Hacspec_ml_kem.Parameters.vector_u_compression_factor v_K));
    assert (v (Hacspec_ml_kem.Parameters.c2_size v_K) ==
        32 * v (Hacspec_ml_kem.Parameters.vector_v_compression_factor v_K))
  in
  let decrypted:t_Array u8 (mk_usize 32) =
    Libcrux_ml_kem.Ind_cpa.decrypt_unpacked v_K
      v_CIPHERTEXT_SIZE
      v_C1_SIZE
      v_VECTOR_U_COMPRESSION_FACTOR
      v_VECTOR_V_COMPRESSION_FACTOR
      #v_Vector
      key_pair.f_private_key.f_ind_cpa_private_key
      ciphertext.Libcrux_ml_kem.Types.f_value
  in
  let (to_hash: t_Array u8 (mk_usize 64)):t_Array u8 (mk_usize 64) =
    Libcrux_ml_kem.Utils.into_padded_array (mk_usize 64) (decrypted <: t_Slice u8)
  in
  let _:Prims.unit = eq_intro (Seq.slice to_hash 0 32) decrypted in
  let to_hash:t_Array u8 (mk_usize 64) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from to_hash
      ({ Core_models.Ops.Range.f_start = Libcrux_ml_kem.Constants.v_SHARED_SECRET_SIZE }
        <:
        Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ {
                Core_models.Ops.Range.f_start = Libcrux_ml_kem.Constants.v_SHARED_SECRET_SIZE
              }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          (key_pair.f_public_key.f_public_key_hash <: t_Slice u8)
        <:
        t_Slice u8)
  in
  let _:Prims.unit = lemma_slice_append to_hash decrypted key_pair.f_public_key.f_public_key_hash in
  let hashed:t_Array u8 (mk_usize 64) =
    Libcrux_ml_kem.Hash_functions.f_G #v_Hasher
      #v_K
      #FStar.Tactics.Typeclasses.solve
      (to_hash <: t_Slice u8)
  in
  let (shared_secret: t_Slice u8), (pseudorandomness: t_Slice u8) =
    Core_models.Slice.impl__split_at #u8
      (hashed <: t_Slice u8)
      Libcrux_ml_kem.Constants.v_SHARED_SECRET_SIZE
  in
  let (to_hash: t_Array u8 v_IMPLICIT_REJECTION_HASH_INPUT_SIZE):t_Array u8
    v_IMPLICIT_REJECTION_HASH_INPUT_SIZE =
    Libcrux_ml_kem.Utils.into_padded_array v_IMPLICIT_REJECTION_HASH_INPUT_SIZE
      (key_pair.f_private_key.f_implicit_rejection_value <: t_Slice u8)
  in
  let _:Prims.unit =
    eq_intro (Seq.slice to_hash 0 32) key_pair.f_private_key.f_implicit_rejection_value
  in
  let to_hash:t_Array u8 v_IMPLICIT_REJECTION_HASH_INPUT_SIZE =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range_from to_hash
      ({ Core_models.Ops.Range.f_start = Libcrux_ml_kem.Constants.v_SHARED_SECRET_SIZE }
        <:
        Core_models.Ops.Range.t_RangeFrom usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (to_hash.[ {
                Core_models.Ops.Range.f_start = Libcrux_ml_kem.Constants.v_SHARED_SECRET_SIZE
              }
              <:
              Core_models.Ops.Range.t_RangeFrom usize ]
            <:
            t_Slice u8)
          (Core_models.Convert.f_as_ref #(Libcrux_ml_kem.Types.t_MlKemCiphertext v_CIPHERTEXT_SIZE)
              #(t_Slice u8)
              #FStar.Tactics.Typeclasses.solve
              ciphertext
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let _:Prims.unit =
    lemma_slice_append to_hash key_pair.f_private_key.f_implicit_rejection_value ciphertext.f_value
  in
  let (implicit_rejection_shared_secret: t_Array u8 (mk_usize 32)):t_Array u8 (mk_usize 32) =
    Libcrux_ml_kem.Hash_functions.f_PRF #v_Hasher
      #v_K
      #FStar.Tactics.Typeclasses.solve
      (mk_usize 32)
      (to_hash <: t_Slice u8)
  in
  let expected_ciphertext:t_Array u8 v_CIPHERTEXT_SIZE =
    Libcrux_ml_kem.Ind_cpa.encrypt_unpacked v_K v_CIPHERTEXT_SIZE v_T_AS_NTT_ENCODED_SIZE v_C1_SIZE
      v_C2_SIZE v_VECTOR_U_COMPRESSION_FACTOR v_VECTOR_V_COMPRESSION_FACTOR v_C1_BLOCK_SIZE v_ETA1
      v_ETA1_RANDOMNESS_SIZE v_ETA2 v_ETA2_RANDOMNESS_SIZE #v_Vector #v_Hasher
      key_pair.f_public_key.f_ind_cpa_public_key decrypted pseudorandomness
  in
  let selector:u8 =
    Libcrux_ml_kem.Constant_time_ops.compare_ciphertexts_in_constant_time (Core_models.Convert.f_as_ref
          #(Libcrux_ml_kem.Types.t_MlKemCiphertext v_CIPHERTEXT_SIZE)
          #(t_Slice u8)
          #FStar.Tactics.Typeclasses.solve
          ciphertext
        <:
        t_Slice u8)
      (expected_ciphertext <: t_Slice u8)
  in
  let result:t_Array u8 (mk_usize 32) =
    Libcrux_ml_kem.Constant_time_ops.select_shared_secret_in_constant_time shared_secret
      (implicit_rejection_shared_secret <: t_Slice u8)
      selector
  in
  let _:Prims.unit = admit () (* Panic freedom *) in
  result

#pop-options
