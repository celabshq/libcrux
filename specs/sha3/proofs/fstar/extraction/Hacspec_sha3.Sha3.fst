module Hacspec_sha3.Sha3
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let v_SHA3_224_RATE: usize = mk_usize 144

let v_SHA3_256_RATE: usize = mk_usize 136

let v_SHA3_384_RATE: usize = mk_usize 104

let v_SHA3_512_RATE: usize = mk_usize 72

let v_SHAKE128_RATE: usize = mk_usize 168

let v_SHAKE256_RATE: usize = mk_usize 136

/// SHA-3 domain separation byte (0x06 = 0b0110: two-bit suffix "01" + first bit of pad10*1).
let v_SHA3_DELIM: u8 = mk_u8 6

/// SHAKE domain separation byte (0x1F = 0b11111: four-bit suffix "1111" + first bit of pad10*1).
let v_SHAKE_DELIM: u8 = mk_u8 31

/// SHA3-224 — FIPS 202, Section 6.1.
let sha3_224_ (message: t_Slice u8) : t_Array u8 (mk_usize 28) =
  Hacspec_sha3.Sponge.keccak (mk_usize 28) v_SHA3_224_RATE v_SHA3_DELIM message

/// SHA3-256 — FIPS 202, Section 6.1.
let sha3_256_ (message: t_Slice u8) : t_Array u8 (mk_usize 32) =
  Hacspec_sha3.Sponge.keccak (mk_usize 32) v_SHA3_256_RATE v_SHA3_DELIM message

/// SHA3-384 — FIPS 202, Section 6.1.
let sha3_384_ (message: t_Slice u8) : t_Array u8 (mk_usize 48) =
  Hacspec_sha3.Sponge.keccak (mk_usize 48) v_SHA3_384_RATE v_SHA3_DELIM message

/// SHA3-512 — FIPS 202, Section 6.1.
let sha3_512_ (message: t_Slice u8) : t_Array u8 (mk_usize 64) =
  Hacspec_sha3.Sponge.keccak (mk_usize 64) v_SHA3_512_RATE v_SHA3_DELIM message

/// SHAKE128 — FIPS 202, Section 6.2.
/// FIPS 202 places no upper bound on the output length `N`.
/// The `N < usize::MAX - 200` precondition is a Rust implementation artifact
/// to prevent arithmetic overflow during squeeze-loop bound computation.
let shake128 (v_N: usize) (message: t_Slice u8)
    : Prims.Pure (t_Array u8 v_N)
      (requires v_N <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (fun _ -> Prims.l_True) = Hacspec_sha3.Sponge.keccak v_N v_SHAKE128_RATE v_SHAKE_DELIM message

/// SHAKE256 — FIPS 202, Section 6.2.
/// FIPS 202 places no upper bound on the output length `N`.
/// The `N < usize::MAX - 200` precondition is a Rust implementation artifact
/// to prevent arithmetic overflow during squeeze-loop bound computation.
let shake256 (v_N: usize) (message: t_Slice u8)
    : Prims.Pure (t_Array u8 v_N)
      (requires v_N <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (fun _ -> Prims.l_True) = Hacspec_sha3.Sponge.keccak v_N v_SHAKE256_RATE v_SHAKE_DELIM message
