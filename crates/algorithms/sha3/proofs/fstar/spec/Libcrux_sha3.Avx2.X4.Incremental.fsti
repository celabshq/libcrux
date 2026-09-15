module Libcrux_sha3.Avx2.X4.Incremental
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// The Keccak state for the incremental API.
///
/// ABSTRACT (hand-written companion, ca6ea1dac technique): kept opaque so
/// external consumers (ml-kem / ml-dsa / kmac) depend only on this LIGHT
/// interface, never sha3's heavy `Simd.Avx2` lane cone or the AVX2 keccak4
/// equivalence proof. `Type0` (not a record) => noeq for free, and the wrapper
/// state is never read field-wise by any sha3-internal module. sha3's own build
/// verifies the concrete `.fst` implements this abstract `val`.
val t_KeccakState : Type0

/// Initialise the [`KeccakState`].
val init: Prims.unit -> Prims.Pure t_KeccakState Prims.l_True (fun _ -> Prims.l_True)

/// Absorb
val shake128_absorb_final (s: t_KeccakState) (data0 data1 data2 data3: t_Slice u8)
    : Prims.Pure t_KeccakState
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data0 <: usize)
          <:
          Hax_lib.Int.t_Int) <
        (168 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data1 <: usize) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data2 <: usize) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data3 <: usize))
      (fun _ -> Prims.l_True)

/// Absorb
val shake256_absorb_final (s: t_KeccakState) (data0 data1 data2 data3: t_Slice u8)
    : Prims.Pure t_KeccakState
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 data0 <: usize)
          <:
          Hax_lib.Int.t_Int) <
        (136 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data1 <: usize) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data2 <: usize) &&
        (Core_models.Slice.impl__len #u8 data0 <: usize) =.
        (Core_models.Slice.impl__len #u8 data3 <: usize))
      (fun _ -> Prims.l_True)

/// Squeeze block
val shake256_squeeze_first_block (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (136 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize))

/// Squeeze next block
val shake256_squeeze_next_block (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (136 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize))

/// Squeeze three blocks
val shake128_squeeze_first_three_blocks (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (504 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize))

/// Squeeze five blocks
val shake128_squeeze_first_five_blocks (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (840 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize))

/// Squeeze another block
val shake128_squeeze_next_block (s: t_KeccakState) (out0 out1 out2 out3: t_Slice u8)
    : Prims.Pure (t_KeccakState & t_Slice u8 & t_Slice u8 & t_Slice u8 & t_Slice u8)
      (requires
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 out0 <: usize)
          <:
          Hax_lib.Int.t_Int) >=
        (168 <: Hax_lib.Int.t_Int) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out1 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out2 <: usize) &&
        (Core_models.Slice.impl__len #u8 out0 <: usize) =.
        (Core_models.Slice.impl__len #u8 out3 <: usize))
      (ensures
        fun temp_0_ ->
          let
          (s_future: t_KeccakState),
          (out0_future: t_Slice u8),
          (out1_future: t_Slice u8),
          (out2_future: t_Slice u8),
          (out3_future: t_Slice u8) =
            temp_0_
          in
          (Core_models.Slice.impl__len #u8 out0_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out0 <: usize) &&
          (Core_models.Slice.impl__len #u8 out1_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out1 <: usize) &&
          (Core_models.Slice.impl__len #u8 out2_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out2 <: usize) &&
          (Core_models.Slice.impl__len #u8 out3_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out3 <: usize))
