use crate::generic_keccak::simd128::keccak2;
use hax_lib;

#[cfg(hax)]
use hax_lib::prop::*;

/// A SHA3 224 implementation.
#[allow(unused_variables)]
#[inline(always)]
#[hax_lib::requires(digest.len() == 28)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 144) (mk_u8 6) $data <: t_Slice u8)"#)
})]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always")]
pub fn sha224(digest: &mut [u8], data: &[u8]) {
    let mut dummy = [0u8; 28];
    hax_lib::fstar!(
        r#"let inputs : t_Array (t_Slice u8) (mk_usize 2) =
               let l : list (t_Slice u8) = [ $data; $data ] in
               FStar.Pervasives.assert_norm (List.Tot.length l == 2);
               Rust_primitives.Hax.array_of_list 2 l in
           EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64
               (mk_usize 144) (mk_u8 6) inputs $digest ($dummy <: t_Slice u8)"#
    );
    keccak2::<144, 0x06u8>(&[data, data], digest, &mut dummy);
}

/// A SHA3 256 implementation.
#[allow(unused_variables)]
#[inline(always)]
#[hax_lib::requires(digest.len() == 32)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 136) (mk_u8 6) $data <: t_Slice u8)"#)
})]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always")]
pub fn sha256(digest: &mut [u8], data: &[u8]) {
    let mut dummy = [0u8; 32];
    hax_lib::fstar!(
        r#"let inputs : t_Array (t_Slice u8) (mk_usize 2) =
               let l : list (t_Slice u8) = [ $data; $data ] in
               FStar.Pervasives.assert_norm (List.Tot.length l == 2);
               Rust_primitives.Hax.array_of_list 2 l in
           EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64
               (mk_usize 136) (mk_u8 6) inputs $digest ($dummy <: t_Slice u8)"#
    );
    keccak2::<136, 0x06u8>(&[data, data], digest, &mut dummy);
}

/// A SHA3 384 implementation.
#[allow(unused_variables)]
#[inline(always)]
#[hax_lib::requires(digest.len() == 48)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 104) (mk_u8 6) $data <: t_Slice u8)"#)
})]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always")]
pub fn sha384(digest: &mut [u8], data: &[u8]) {
    let mut dummy = [0u8; 48];
    hax_lib::fstar!(
        r#"let inputs : t_Array (t_Slice u8) (mk_usize 2) =
               let l : list (t_Slice u8) = [ $data; $data ] in
               FStar.Pervasives.assert_norm (List.Tot.length l == 2);
               Rust_primitives.Hax.array_of_list 2 l in
           EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64
               (mk_usize 104) (mk_u8 6) inputs $digest ($dummy <: t_Slice u8)"#
    );
    keccak2::<104, 0x06u8>(&[data, data], digest, &mut dummy);
}

/// A SHA3 512 implementation.
#[allow(unused_variables)]
#[inline(always)]
#[hax_lib::requires(digest.len() == 64)]
#[hax_lib::ensures(|_| (future(digest).len() == digest.len()).to_prop() & {
    fstar!(r#"(digest_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $digest)
                 (mk_usize 72) (mk_u8 6) $data <: t_Slice u8)"#)
})]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always")]
pub fn sha512(digest: &mut [u8], data: &[u8]) {
    let mut dummy = [0u8; 64];
    hax_lib::fstar!(
        r#"let inputs : t_Array (t_Slice u8) (mk_usize 2) =
               let l : list (t_Slice u8) = [ $data; $data ] in
               FStar.Pervasives.assert_norm (List.Tot.length l == 2);
               Rust_primitives.Hax.array_of_list 2 l in
           EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64
               (mk_usize 72) (mk_u8 6) inputs $digest ($dummy <: t_Slice u8)"#
    );
    keccak2::<72, 0x06u8>(&[data, data], digest, &mut dummy);
}

/// A SHAKE128 implementation.
#[allow(unused_variables)]
#[inline(always)]
#[hax_lib::requires(LEN < usize::MAX - 200)]
#[hax_lib::ensures(|_| fstar!(r#"
    (digest_future <: t_Array u8 $LEN) ==
      (Hacspec_sha3.Sponge.keccak $LEN
         (mk_usize 168) (mk_u8 31) $data <: t_Array u8 $LEN)
"#))]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always")]
pub fn shake128<const LEN: usize>(digest: &mut [u8; LEN], data: &[u8]) {
    let mut dummy = [0u8; LEN];
    hax_lib::fstar!(
        r#"let inputs : t_Array (t_Slice u8) (mk_usize 2) =
               let l : list (t_Slice u8) = [ $data; $data ] in
               FStar.Pervasives.assert_norm (List.Tot.length l == 2);
               Rust_primitives.Hax.array_of_list 2 l in
           EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64
               (mk_usize 168) (mk_u8 31) inputs
               ($digest <: t_Slice u8) ($dummy <: t_Slice u8)"#
    );
    keccak2::<168, 0x1fu8>(&[data, data], digest, &mut dummy);
}

/// A SHAKE256 implementation.
#[allow(unused_variables)]
#[inline(always)]
#[hax_lib::requires(LEN < usize::MAX - 200)]
#[hax_lib::ensures(|_| fstar!(r#"
    (digest_future <: t_Array u8 $LEN) ==
      (Hacspec_sha3.Sponge.keccak $LEN
         (mk_usize 136) (mk_u8 31) $data <: t_Array u8 $LEN)
"#))]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always")]
pub fn shake256<const LEN: usize>(digest: &mut [u8; LEN], data: &[u8]) {
    let mut dummy = [0u8; LEN];
    hax_lib::fstar!(
        r#"let inputs : t_Array (t_Slice u8) (mk_usize 2) =
               let l : list (t_Slice u8) = [ $data; $data ] in
               FStar.Pervasives.assert_norm (List.Tot.length l == 2);
               Rust_primitives.Hax.array_of_list 2 l in
           EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64
               (mk_usize 136) (mk_u8 31) inputs
               ($digest <: t_Slice u8) ($dummy <: t_Slice u8)"#
    );
    keccak2::<136, 0x1fu8>(&[data, data], digest, &mut dummy);
}

/// Performing 2 operations in parallel
pub mod x2 {
    use super::*;

    /// Run SHAKE256 on both inputs in parallel.
    ///
    /// Writes the two results into `out0` and `out1`.
    ///
    /// The ensures below are F*-verified: each lane equals the scalar
    /// `keccak` of its respective input (via the two-lane Arm64 driver
    /// `lemma_keccak2_arm64`).  The ensures shape mirrors the parallel
    /// API analogue on the AVX2 (X4) side.
    #[allow(unused_variables)]
    #[inline(always)]
    #[hax_lib::requires(input0.len() == input1.len() && out0.len() == out1.len() && out0.len() < usize::MAX - 200)]
    #[hax_lib::ensures(|_| (future(out0).len() == out0.len() && future(out1).len() == out1.len()).to_prop() & {
        fstar!(r#"
            (out0_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $out0)
                 (mk_usize 136) (mk_u8 31) $input0 <: t_Slice u8) /\
            (out1_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $out1)
                 (mk_usize 136) (mk_u8 31) $input1 <: t_Slice u8)
        "#)
    })]
    #[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always")]
    pub fn shake256(input0: &[u8], input1: &[u8], out0: &mut [u8], out1: &mut [u8]) {
        hax_lib::fstar!(
            r#"let inputs : t_Array (t_Slice u8) (mk_usize 2) =
                   let l : list (t_Slice u8) = [ $input0; $input1 ] in
                   FStar.Pervasives.assert_norm (List.Tot.length l == 2);
                   Rust_primitives.Hax.array_of_list 2 l in
               EquivImplSpec.Sponge.Arm64.Driver.lemma_keccak2_arm64
                   (mk_usize 136) (mk_u8 31) inputs
                   ($out0 <: t_Slice u8) ($out1 <: t_Slice u8)"#
        );
        keccak2::<136, 0x1fu8>(&[input0, input1], out0, out1);
    }

    /// An incremental API to perform 2 operations in parallel
    pub mod incremental {
        #[cfg(hax)]
        use hax_lib::int::ToInt;

        use crate::generic_keccak::KeccakState as GenericState;

        /// The Keccak state for the incremental API.
        pub struct KeccakState {
            state: GenericState<2, crate::simd::arm64::uint64x2_t>,
        }

        type KeccakState2Internal = GenericState<2, crate::simd::arm64::uint64x2_t>;

        /// Initialise the `KeccakState2`.
        #[inline(always)]
        pub fn init() -> KeccakState {
            KeccakState {
                state: KeccakState2Internal::new(),
            }
        }

        /// Shake128 absorb `data0` and `data1` in the [`KeccakState`] `s`.
        #[inline(always)]
        #[hax_lib::requires(
            data0.len().to_int() < hax_lib::int!(168) &&
            data0.len() == data1.len()
        )]
        pub fn shake128_absorb_final(s: &mut KeccakState, data0: &[u8], data1: &[u8]) {
            s.state
                .absorb_final::<168, 0x1fu8>(&[data0, data1], 0, data0.len());
        }

        /// Shake256 absorb `data0` and `data1` in the [`KeccakState`] `s`.
        #[inline(always)]
        #[hax_lib::requires(
            data0.len().to_int() < hax_lib::int!(136) &&
            data0.len() == data1.len()
        )]
        pub fn shake256_absorb_final(s: &mut KeccakState, data0: &[u8], data1: &[u8]) {
            s.state
                .absorb_final::<136, 0x1fu8>(&[data0, data1], 0, data0.len());
        }

        /// Squeeze 2 times the first three blocks in parallel in the
        /// [`KeccakState`] and return the output in `out0` and `out1`.
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(504) && // 3 * 168 = 504
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake128_squeeze_first_three_blocks(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
        ) {
            s.state.squeeze_first_three_blocks::<168>(out0, out1)
        }

        /// Squeeze five blocks
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(840) && // 5 * 168 = 840
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake128_squeeze_first_five_blocks(
            s: &mut KeccakState,
            out0: &mut [u8],
            out1: &mut [u8],
        ) {
            s.state.squeeze_first_five_blocks::<168>(out0, out1);
        }

        /// Squeeze block
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(136) &&
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake256_squeeze_first_block(s: &mut KeccakState, out0: &mut [u8], out1: &mut [u8]) {
            s.state.squeeze_first_block::<136>(out0, out1);
        }

        /// Squeeze next block
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(136) &&
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake256_squeeze_next_block(s: &mut KeccakState, out0: &mut [u8], out1: &mut [u8]) {
            s.state.squeeze_next_block::<136>(out0, out1, 0);
        }

        /// Squeeze 2 times the next block in parallel in the
        /// [`KeccakState`] and return the output in `out0` and `out1`.
        #[inline(always)]
        #[hax_lib::requires(
            out0.len().to_int() >= hax_lib::int!(168) &&
            out0.len() == out1.len()
        )]
        #[hax_lib::ensures(|_|
            future(out0).len() == out0.len() &&
            future(out1).len() == out1.len()
        )]
        pub fn shake128_squeeze_next_block(s: &mut KeccakState, out0: &mut [u8], out1: &mut [u8]) {
            s.state.squeeze_next_block::<168>(out0, out1, 0)
        }
    }
}
