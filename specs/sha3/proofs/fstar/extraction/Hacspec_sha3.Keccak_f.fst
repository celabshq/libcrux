module Hacspec_sha3.Keccak_f
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// Read lane `A[x, y]`.
let get (state: t_Array u64 (mk_usize 25)) (x y: usize)
    : Prims.Pure u64 (requires x <. mk_usize 5 && y <. mk_usize 5) (fun _ -> Prims.l_True) =
  state.[ (mk_usize 5 *! y <: usize) +! x <: usize ]

/// Round constants `RC[ir]` for `ir = 0..23` — FIPS 202, Algorithm 5.
let v_ROUND_CONSTANTS: t_Array u64 (mk_usize 24) =
  let list =
    [
      mk_u64 1; mk_u64 32898; mk_u64 9223372036854808714; mk_u64 9223372039002292224; mk_u64 32907;
      mk_u64 2147483649; mk_u64 9223372039002292353; mk_u64 9223372036854808585; mk_u64 138;
      mk_u64 136; mk_u64 2147516425; mk_u64 2147483658; mk_u64 2147516555;
      mk_u64 9223372036854775947; mk_u64 9223372036854808713; mk_u64 9223372036854808579;
      mk_u64 9223372036854808578; mk_u64 9223372036854775936; mk_u64 32778;
      mk_u64 9223372039002259466; mk_u64 9223372039002292353; mk_u64 9223372036854808704;
      mk_u64 2147483649; mk_u64 9223372039002292232
    ]
  in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 24);
  Rust_primitives.Hax.array_of_list 24 list

/// Rotation offsets for ρ step — FIPS 202, Algorithm 2 / Table 2.
/// Indexed as `RHO_OFFSETS[5*y + x]`.
let v_RHO_OFFSETS: t_Array u32 (mk_usize 25) =
  let list =
    [
      mk_u32 0; mk_u32 1; mk_u32 62; mk_u32 28; mk_u32 27; mk_u32 36; mk_u32 44; mk_u32 6; mk_u32 55;
      mk_u32 20; mk_u32 3; mk_u32 10; mk_u32 43; mk_u32 25; mk_u32 39; mk_u32 41; mk_u32 45;
      mk_u32 15; mk_u32 21; mk_u32 8; mk_u32 18; mk_u32 2; mk_u32 61; mk_u32 56; mk_u32 14
    ]
  in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 25);
  Rust_primitives.Hax.array_of_list 25 list

/// θ step — FIPS 202, Algorithm 1.
///   C[x]    = A[x,0] ⊕ A[x,1] ⊕ A[x,2] ⊕ A[x,3] ⊕ A[x,4]
///   D[x]    = C[x−1 mod 5] ⊕ rot(C[x+1 mod 5], 1)
///   A′[x,y] = A[x,y] ⊕ D[x]
let theta (state: t_Array u64 (mk_usize 25)) : t_Array u64 (mk_usize 25) =
  let (c: t_Array u64 (mk_usize 5)):t_Array u64 (mk_usize 5) =
    Hacspec_sha3.createi #u64
      (mk_usize 5)
      #(usize -> u64)
      (fun x ->
          let x:usize = x in
          ((((get state x (mk_usize 0) <: u64) ^. (get state x (mk_usize 1) <: u64) <: u64) ^.
              (get state x (mk_usize 2) <: u64)
              <:
              u64) ^.
            (get state x (mk_usize 3) <: u64)
            <:
            u64) ^.
          (get state x (mk_usize 4) <: u64)
          <:
          u64)
  in
  let (d: t_Array u64 (mk_usize 5)):t_Array u64 (mk_usize 5) =
    Hacspec_sha3.createi #u64
      (mk_usize 5)
      #(usize -> u64)
      (fun x ->
          let x:usize = x in
          (c.[ (x +! mk_usize 4 <: usize) %! mk_usize 5 <: usize ] <: u64) ^.
          (Core_models.Num.impl_u64__rotate_left (c.[ (x +! mk_usize 1 <: usize) %! mk_usize 5
                  <:
                  usize ]
                <:
                u64)
              (mk_u32 1)
            <:
            u64)
          <:
          u64)
  in
  Hacspec_sha3.createi #u64
    (mk_usize 25)
    #(usize -> u64)
    (fun idx ->
        let idx:usize = idx in
        (state.[ idx ] <: u64) ^. (d.[ idx %! mk_usize 5 <: usize ] <: u64) <: u64)

/// ρ step — FIPS 202, Algorithm 2.
///   A′[x,y] = rot(A[x,y], offset(x,y))
let rho (state: t_Array u64 (mk_usize 25)) : t_Array u64 (mk_usize 25) =
  Hacspec_sha3.createi #u64
    (mk_usize 25)
    #(usize -> u64)
    (fun idx ->
        let idx:usize = idx in
        Core_models.Num.impl_u64__rotate_left (state.[ idx ] <: u64) (v_RHO_OFFSETS.[ idx ] <: u32)
        <:
        u64)

/// π step — FIPS 202, Algorithm 3.
///   A′[x,y] = A[(x + 3y) mod 5, x]
let pi (state: t_Array u64 (mk_usize 25)) : t_Array u64 (mk_usize 25) =
  Hacspec_sha3.createi #u64
    (mk_usize 25)
    #(usize -> u64)
    (fun idx ->
        let idx:usize = idx in
        let y:usize = idx /! mk_usize 5 in
        let x:usize = idx %! mk_usize 5 in
        get state ((x +! (mk_usize 3 *! y <: usize) <: usize) %! mk_usize 5 <: usize) x)

/// χ step — FIPS 202, Algorithm 4.
///   A′[x,y] = A[x,y] ⊕ (¬A[(x+1) mod 5, y] ∧ A[(x+2) mod 5, y])
let chi (state: t_Array u64 (mk_usize 25)) : t_Array u64 (mk_usize 25) =
  Hacspec_sha3.createi #u64
    (mk_usize 25)
    #(usize -> u64)
    (fun idx ->
        let idx:usize = idx in
        let y:usize = idx /! mk_usize 5 in
        let x:usize = idx %! mk_usize 5 in
        (get state x y <: u64) ^.
        ((~.(get state ((x +! mk_usize 1 <: usize) %! mk_usize 5 <: usize) y <: u64) <: u64) &.
          (get state ((x +! mk_usize 2 <: usize) %! mk_usize 5 <: usize) y <: u64)
          <:
          u64))

/// ι step — FIPS 202, Algorithm 6.
///   A′[0,0] = A[0,0] ⊕ RC[ir]
let iota (state: t_Array u64 (mk_usize 25)) (round: usize)
    : Prims.Pure (t_Array u64 (mk_usize 25)) (requires round <. mk_usize 24) (fun _ -> Prims.l_True) =
  let state:t_Array u64 (mk_usize 25) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize state
      (mk_usize 0)
      ((state.[ mk_usize 0 ] <: u64) ^. (v_ROUND_CONSTANTS.[ round ] <: u64) <: u64)
  in
  state

/// Keccak-f[1600] permutation — FIPS 202, Algorithm 7.
///   Rnd(A, ir) = ι(χ(π(ρ(θ(A)))), ir)
let keccak_f (state: t_Array u64 (mk_usize 25)) : t_Array u64 (mk_usize 25) =
  let state:t_Array u64 (mk_usize 25) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (mk_usize 24)
      (fun state temp_1_ ->
          let state:t_Array u64 (mk_usize 25) = state in
          let _:usize = temp_1_ in
          true)
      state
      (fun state round ->
          let state:t_Array u64 (mk_usize 25) = state in
          let round:usize = round in
          iota (chi (pi (rho (theta state <: t_Array u64 (mk_usize 25)) <: t_Array u64 (mk_usize 25)
                    )
                  <:
                  t_Array u64 (mk_usize 25))
              <:
              t_Array u64 (mk_usize 25))
            round
          <:
          t_Array u64 (mk_usize 25))
  in
  state
