pub mod int_vec {
    //! Provides a machine integer vectors intepretation for AVX2 intrinsics.

    use crate::abstractions::{
        bit::Bit,
        bitvec::{int_vec_interp::*, BitVec},
        funarr::FunArray,
    };
    #[allow(unused)]
    use crate::core_arch::x86;

    pub fn _mm256_set1_epi32(x: i32) -> i32x8 {
        i32x8::from_fn(|_| x)
    }

    pub fn i32_extended64_mul(x: i32, y: i32) -> i64 {
        (x as i64) * (y as i64)
    }

    pub fn _mm256_mul_epi32(x: i32x8, y: i32x8) -> i64x4 {
        i64x4::from_fn(|i| i32_extended64_mul(x[i * 2], y[i * 2]))
    }
    pub fn _mm256_sub_epi32(x: i32x8, y: i32x8) -> i32x8 {
        i32x8::from_fn(|i| x[i].wrapping_sub(y[i]))
    }

    pub fn _mm256_shuffle_epi32<const CONTROL: i32>(x: i32x8) -> i32x8 {
        let indexes: FunArray<4, u64> = FunArray::from_fn(|i| ((CONTROL >> i * 2) % 4) as u64);
        i32x8::from_fn(|i| {
            if i < 4 {
                x[indexes[i]]
            } else {
                x[4 + indexes[i - 4]]
            }
        })
    }

    pub fn _mm256_blend_epi32<const CONTROL: i32>(x: i32x8, y: i32x8) -> i32x8 {
        i32x8::from_fn(|i| if (CONTROL >> i) % 2 == 0 { x[i] } else { y[i] })
    }

    pub fn _mm256_setzero_si256() -> BitVec<256> {
        BitVec::from_fn(|_| Bit::Zero)
    }
    pub fn _mm256_set_m128i(hi: BitVec<128>, lo: BitVec<128>) -> BitVec<256> {
        BitVec::from_fn(|i| if i < 128 { lo[i] } else { hi[i - 128] })
    }

    pub fn _mm256_set1_epi16(a: i16) -> i16x16 {
        i16x16::from_fn(|_| a)
    }

    pub fn _mm_set1_epi16(a: i16) -> i16x8 {
        i16x8::from_fn(|_| a)
    }

    pub fn _mm_set_epi32(e3: i32, e2: i32, e1: i32, e0: i32) -> i32x4 {
        i32x4::from_fn(|i| match i {
            0 => e0,
            1 => e1,
            2 => e2,
            3 => e3,
            _ => unreachable!(),
        })
    }
    pub fn _mm_add_epi16(a: i16x8, b: i16x8) -> i16x8 {
        i16x8::from_fn(|i| a[i].wrapping_add(b[i]))
    }
    pub fn _mm256_add_epi16(a: i16x16, b: i16x16) -> i16x16 {
        i16x16::from_fn(|i| a[i].wrapping_add(b[i]))
    }

    pub fn _mm256_add_epi32(a: i32x8, b: i32x8) -> i32x8 {
        i32x8::from_fn(|i| a[i].wrapping_add(b[i]))
    }

    pub fn _mm256_add_epi64(a: i64x4, b: i64x4) -> i64x4 {
        i64x4::from_fn(|i| a[i].wrapping_add(b[i]))
    }

    pub fn _mm256_abs_epi32(a: i32x8) -> i32x8 {
        i32x8::from_fn(|i| {
            // See `_mm256_abs_epi32_min`: if the item is `i32::MIN`, the intrinsics returns `i32::MIN`, untouched.
            if a[i] == i32::MIN {
                a[i]
            } else {
                a[i].abs()
            }
        })
    }

    pub fn _mm256_sub_epi16(a: i16x16, b: i16x16) -> i16x16 {
        i16x16::from_fn(|i| a[i].wrapping_sub(b[i]))
    }

    pub fn _mm_sub_epi16(a: i16x8, b: i16x8) -> i16x8 {
        i16x8::from_fn(|i| a[i].wrapping_sub(b[i]))
    }

    pub fn _mm_mullo_epi16(a: i16x8, b: i16x8) -> i16x8 {
        i16x8::from_fn(|i| a[i].overflowing_mul(b[i]).0)
    }

    pub fn _mm256_cmpgt_epi16(a: i16x16, b: i16x16) -> i16x16 {
        i16x16::from_fn(|i| if a[i] > b[i] { -1 } else { 0 })
    }

    pub fn _mm256_cmpgt_epi32(a: i32x8, b: i32x8) -> i32x8 {
        i32x8::from_fn(|i| if a[i] > b[i] { -1 } else { 0 })
    }

    pub fn _mm256_cmpeq_epi32(a: i32x8, b: i32x8) -> i32x8 {
        i32x8::from_fn(|i| if a[i] == b[i] { -1 } else { 0 })
    }

    pub fn _mm256_sign_epi32(a: i32x8, b: i32x8) -> i32x8 {
        i32x8::from_fn(|i| {
            if b[i] < 0 {
                // See `_mm256_sign_epi32_min`: if the item is `i32::MIN`, the intrinsics returns `i32::MIN`, untouched.
                if a[i] == i32::MIN {
                    a[i]
                } else {
                    -a[i]
                }
            } else if b[i] > 0 {
                a[i]
            } else {
                0
            }
        })
    }

    pub fn _mm256_castsi256_ps(a: BitVec<256>) -> BitVec<256> {
        a
    }

    pub fn _mm256_castps_si256(a: BitVec<256>) -> BitVec<256> {
        a
    }

    pub fn _mm256_movemask_ps(a: i32x8) -> i32 {
        let a0: i32 = if a[0] < 0 { 1 } else { 0 };
        let a1 = if a[1] < 0 { 2 } else { 0 };
        let a2 = if a[2] < 0 { 4 } else { 0 };
        let a3 = if a[3] < 0 { 8 } else { 0 };
        let a4 = if a[4] < 0 { 16 } else { 0 };
        let a5 = if a[5] < 0 { 32 } else { 0 };
        let a6 = if a[6] < 0 { 64 } else { 0 };
        let a7 = if a[7] < 0 { 128 } else { 0 };
        a0 + a1 + a2 + a3 + a4 + a5 + a6 + a7
    }

    #[hax_lib::fstar::options("--z3rlimit 200")]
    pub fn _mm_mulhi_epi16(a: i16x8, b: i16x8) -> i16x8 {
        i16x8::from_fn(|i| ((a[i] as i32) * (b[i] as i32) >> 16) as i16)
    }

    pub fn _mm256_mullo_epi32(a: i32x8, b: i32x8) -> i32x8 {
        i32x8::from_fn(|i| a[i].overflowing_mul(b[i]).0)
    }

    #[hax_lib::fstar::verification_status(lax)]
    pub fn _mm256_mulhi_epi16(a: i16x16, b: i16x16) -> i16x16 {
        i16x16::from_fn(|i| ((a[i] as i32) * (b[i] as i32) >> 16) as i16)
    }

    pub fn _mm256_mul_epu32(a: u32x8, b: u32x8) -> u64x4 {
        u64x4::from_fn(|i| (a[i * 2] as u64) * (b[i * 2] as u64))
    }

    pub fn _mm256_and_si256(a: BitVec<256>, b: BitVec<256>) -> BitVec<256> {
        BitVec::from_fn(|i| match (a[i], b[i]) {
            (Bit::One, Bit::One) => Bit::One,
            _ => Bit::Zero,
        })
    }

    pub fn _mm256_or_si256(a: BitVec<256>, b: BitVec<256>) -> BitVec<256> {
        BitVec::from_fn(|i| match (a[i], b[i]) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            _ => Bit::One,
        })
    }

    pub fn _mm256_testz_si256(a: BitVec<256>, b: BitVec<256>) -> i32 {
        let c = BitVec::<256>::from_fn(|i| match (a[i], b[i]) {
            (Bit::One, Bit::One) => Bit::One,
            _ => Bit::Zero,
        });
        let all_zero = c.fold(true, |acc, bit| acc && bit == Bit::Zero);
        if all_zero {
            1
        } else {
            0
        }
    }

    pub fn _mm256_xor_si256(a: BitVec<256>, b: BitVec<256>) -> BitVec<256> {
        BitVec::from_fn(|i| match (a[i], b[i]) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::One, Bit::One) => Bit::Zero,
            _ => Bit::One,
        })
    }

    pub fn _mm256_srai_epi16<const IMM8: i32>(a: i16x16) -> i16x16 {
        i16x16::from_fn(|i| {
            let imm8 = IMM8.rem_euclid(256);
            if imm8 > 15 {
                if a[i] < 0 {
                    -1
                } else {
                    0
                }
            } else {
                a[i] >> imm8
            }
        })
    }

    pub fn _mm256_srai_epi32<const IMM8: i32>(a: i32x8) -> i32x8 {
        i32x8::from_fn(|i| {
            let imm8 = IMM8.rem_euclid(256);
            if imm8 > 31 {
                if a[i] < 0 {
                    -1
                } else {
                    0
                }
            } else {
                a[i] >> imm8
            }
        })
    }

    pub fn _mm256_srli_epi16<const IMM8: i32>(a: i16x16) -> i16x16 {
        i16x16::from_fn(|i| {
            let imm8 = IMM8.rem_euclid(256);
            if imm8 > 15 {
                0
            } else {
                ((a[i] as u16) >> imm8) as i16
            }
        })
    }

    pub fn _mm256_srli_epi32<const IMM8: i32>(a: i32x8) -> i32x8 {
        i32x8::from_fn(|i| {
            let imm8 = IMM8.rem_euclid(256);
            if imm8 > 31 {
                0
            } else {
                ((a[i] as u32) >> imm8) as i32
            }
        })
    }

    pub fn _mm_srli_epi64<const IMM8: i32>(a: i64x2) -> i64x2 {
        i64x2::from_fn(|i| {
            let imm8 = IMM8.rem_euclid(256);
            if imm8 > 63 {
                0
            } else {
                ((a[i] as u64) >> imm8) as i64
            }
        })
    }

    pub fn _mm256_slli_epi32<const IMM8: i32>(a: i32x8) -> i32x8 {
        i32x8::from_fn(|i| {
            let imm8 = IMM8.rem_euclid(256);
            if imm8 > 31 {
                0
            } else {
                ((a[i] as u32) << imm8) as i32
            }
        })
    }

    pub fn _mm256_permute4x64_epi64<const IMM8: i32>(a: i64x4) -> i64x4 {
        let indexes: FunArray<4, u64> = FunArray::from_fn(|i| ((IMM8 >> i * 2) % 4) as u64);
        i64x4::from_fn(|i| a[indexes[i]])
    }

    pub fn _mm256_unpackhi_epi64(a: i64x4, b: i64x4) -> i64x4 {
        i64x4::from_fn(|i| match i {
            0 => a[1],
            1 => b[1],
            2 => a[3],
            3 => b[3],
            _ => unreachable!(),
        })
    }

    pub fn _mm256_unpacklo_epi32(a: i32x8, b: i32x8) -> i32x8 {
        i32x8::from_fn(|i| match i {
            0 => a[0],
            1 => b[0],
            2 => a[1],
            3 => b[1],
            4 => a[4],
            5 => b[4],
            6 => a[5],
            7 => b[5],
            _ => unreachable!(),
        })
    }

    pub fn _mm256_unpackhi_epi32(a: i32x8, b: i32x8) -> i32x8 {
        i32x8::from_fn(|i| match i {
            0 => a[2],
            1 => b[2],
            2 => a[3],
            3 => b[3],
            4 => a[6],
            5 => b[6],
            6 => a[7],
            7 => b[7],
            _ => unreachable!(),
        })
    }

    pub fn _mm256_castsi128_si256(a: BitVec<128>) -> BitVec<256> {
        BitVec::from_fn(|i| if i < 128 { a[i] } else { Bit::Zero })
    }

    pub fn _mm256_cvtepi16_epi32(a: i16x8) -> i32x8 {
        i32x8::from_fn(|i| a[i] as i32)
    }

    pub fn _mm_packs_epi16(a: i16x8, b: i16x8) -> i8x16 {
        i8x16::from_fn(|i| {
            if i < 8 {
                if a[i] > (i8::MAX as i16) {
                    i8::MAX
                } else if a[i] < (i8::MIN as i16) {
                    i8::MIN
                } else {
                    a[i] as i8
                }
            } else {
                if b[i - 8] > (i8::MAX as i16) {
                    i8::MAX
                } else if b[i - 8] < (i8::MIN as i16) {
                    i8::MIN
                } else {
                    b[i - 8] as i8
                }
            }
        })
    }

    pub fn _mm256_packs_epi32(a: i32x8, b: i32x8) -> i16x16 {
        i16x16::from_fn(|i| {
            if i < 4 {
                if a[i] > (i16::MAX as i32) {
                    i16::MAX
                } else if a[i] < (i16::MIN as i32) {
                    i16::MIN
                } else {
                    a[i] as i16
                }
            } else if i < 8 {
                if b[i - 4] > (i16::MAX as i32) {
                    i16::MAX
                } else if b[i - 4] < (i16::MIN as i32) {
                    i16::MIN
                } else {
                    b[i - 4] as i16
                }
            } else if i < 12 {
                if a[i - 4] > (i16::MAX as i32) {
                    i16::MAX
                } else if a[i - 4] < (i16::MIN as i32) {
                    i16::MIN
                } else {
                    a[i - 4] as i16
                }
            } else {
                if b[i - 8] > (i16::MAX as i32) {
                    i16::MAX
                } else if b[i - 8] < (i16::MIN as i32) {
                    i16::MIN
                } else {
                    b[i - 8] as i16
                }
            }
        })
    }

    pub fn _mm256_inserti128_si256<const IMM8: i32>(a: i128x2, b: i128x1) -> i128x2 {
        i128x2::from_fn(|i| {
            if IMM8 % 2 == 0 {
                match i {
                    0 => b[0],
                    1 => a[1],
                    _ => unreachable!(),
                }
            } else {
                match i {
                    0 => a[0],
                    1 => b[0],
                    _ => unreachable!(),
                }
            }
        })
    }

    pub fn _mm256_blend_epi16<const IMM8: i32>(a: i16x16, b: i16x16) -> i16x16 {
        i16x16::from_fn(|i| {
            if (IMM8 >> (i % 8)) % 2 == 0 {
                a[i]
            } else {
                b[i]
            }
        })
    }

    pub fn _mm256_blendv_ps(a: i32x8, b: i32x8, mask: i32x8) -> i32x8 {
        i32x8::from_fn(|i| if mask[i] < 0 { b[i] } else { a[i] })
    }

    #[hax_lib::fstar::verification_status(lax)]
    pub fn _mm_movemask_epi8(a: i8x16) -> i32 {
        let a0 = if a[0] < 0 { 1 } else { 0 };
        let a1 = if a[1] < 0 { 2 } else { 0 };
        let a2 = if a[2] < 0 { 4 } else { 0 };
        let a3 = if a[3] < 0 { 8 } else { 0 };
        let a4 = if a[4] < 0 { 16 } else { 0 };
        let a5 = if a[5] < 0 { 32 } else { 0 };
        let a6 = if a[6] < 0 { 64 } else { 0 };
        let a7 = if a[7] < 0 { 128 } else { 0 };
        let a8 = if a[8] < 0 { 256 } else { 0 };
        let a9 = if a[9] < 0 { 512 } else { 0 };
        let a10 = if a[10] < 0 { 1024 } else { 0 };
        let a11 = if a[11] < 0 { 2048 } else { 0 };
        let a12 = if a[12] < 0 { 4096 } else { 0 };
        let a13 = if a[13] < 0 { 8192 } else { 0 };
        let a14 = if a[14] < 0 { 16384 } else { 0 };
        let a15 = if a[15] < 0 { 32768 } else { 0 };

        a0 + a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 + a11 + a12 + a13 + a14 + a15
    }

    pub fn _mm256_srlv_epi64(a: i64x4, b: i64x4) -> i64x4 {
        i64x4::from_fn(|i| {
            if b[i] > 63 || b[i] < 0 {
                0
            } else {
                ((a[i] as u64) >> b[i]) as i64
            }
        })
    }

    pub fn _mm_sllv_epi32(a: i32x4, b: i32x4) -> i32x4 {
        i32x4::from_fn(|i| {
            if b[i] > 31 || b[i] < 0 {
                0
            } else {
                ((a[i] as u32) << b[i]) as i32
            }
        })
    }

    pub fn _mm256_slli_epi64<const IMM8: i32>(a: i64x4) -> i64x4 {
        i64x4::from_fn(|i| {
            let imm8 = IMM8 % 256;
            if imm8 > 63 {
                0
            } else {
                ((a[i] as u64) << imm8) as i64
            }
        })
    }

    pub fn _mm256_bsrli_epi128<const IMM8: i32>(a: i128x2) -> i128x2 {
        i128x2::from_fn(|i| {
            // Intel spec: a byte-shift amount > 15 clears the lane (the count is
            // clamped to 16), it is NOT taken modulo 16. The old `tmp % 16` here
            // mirrored a Rust core bug whose fix we upstreamed, so the model must
            // follow the spec rather than the (now-corrected) implementation.
            let tmp = IMM8 % 256;
            if tmp > 15 {
                0
            } else {
                ((a[i] as u128) >> (tmp * 8)) as i128
            }
        })
    }

    pub fn _mm256_andnot_si256(a: BitVec<256>, b: BitVec<256>) -> BitVec<256> {
        BitVec::from_fn(|i| match (a[i], b[i]) {
            (Bit::Zero, Bit::One) => Bit::One,
            _ => Bit::Zero,
        })
    }

    pub fn _mm256_set1_epi64x(a: i64) -> i64x4 {
        i64x4::from_fn(|_| a)
    }

    pub fn _mm256_set_epi64x(e3: i64, e2: i64, e1: i64, e0: i64) -> i64x4 {
        i64x4::from_fn(|i| match i {
            0 => e0,
            1 => e1,
            2 => e2,
            3 => e3,
            _ => unreachable!(),
        })
    }

    pub fn _mm256_unpacklo_epi64(a: i64x4, b: i64x4) -> i64x4 {
        i64x4::from_fn(|i| match i {
            0 => a[0],
            1 => b[0],
            2 => a[2],
            3 => b[2],
            _ => unreachable!(),
        })
    }

    pub fn _mm256_permute2x128_si256<const IMM8: i32>(a: i128x2, b: i128x2) -> i128x2 {
        i128x2::from_fn(|i| {
            let control = IMM8 >> (i * 4);
            if (control >> 3) % 2 == 1 {
                0
            } else {
                match control % 4 {
                    0 => a[0],
                    1 => a[1],
                    2 => b[0],
                    3 => b[1],
                    _ => unreachable!(),
                }
            }
        })
    }

    pub use lemmas::flatten_circuit;
    pub mod lemmas {
        //! This module provides lemmas allowing to lift the intrinsics modeled in `super` from their version operating on AVX2 vectors to functions operating on machine integer vectors (e.g. on `i32x8`).
        #[cfg(hax)]
        use super::*;

        #[cfg(hax)]
        use crate::core_arch::x86 as upstream;
        #[cfg(hax)]
        use crate::core_arch::x86::{__m128i, __m256, __m256i};

        /// An F* attribute that marks an item as being an lifting lemma.
        #[allow(dead_code)]
        #[hax_lib::fstar::before("irreducible")]
        pub const ETA_MATCH_EXPAND: () = ();

        #[hax_lib::fstar::before("[@@ $ETA_MATCH_EXPAND ]")]
        #[hax_lib::lemma]
        #[hax_lib::opaque]
        pub fn pointwise_i32x8(x: i32x8) -> Proof<{ hax_lib::eq(x, x.pointwise()) }> {}

        #[hax_lib::fstar::before("[@@ $ETA_MATCH_EXPAND ]")]
        #[hax_lib::lemma]
        #[hax_lib::opaque]
        pub fn pointwise_i64x4(x: i64x4) -> Proof<{ hax_lib::eq(x, x.pointwise()) }> {}

        /// An F* attribute that marks an item as being an lifting lemma.
        #[allow(dead_code)]
        #[hax_lib::fstar::before("irreducible")]
        pub const LIFT_LEMMA: () = ();

        #[hax_lib::fstar::replace(r#"
[@@ v_LIFT_LEMMA ]
assume val _mm256_set_epi32_interp: e7: i32 -> e6: i32 -> e5: i32 -> e4: i32 -> e3: i32 -> e2: i32 -> e1: i32 -> e0: i32 -> (i: u64 {v i < 8})
  -> Lemma
        (
            (
                Core_models.Abstractions.Bitvec.Int_vec_interp.e_ee_1__impl__to_i32x8
                    (Core_models.Core_arch.X86.Avx.e_mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0)
            ).[ i ]
         == ( match i with
            | MkInt 0 -> e0 | MkInt 1 -> e1 | MkInt 2 -> e2 | MkInt 3 -> e3
            | MkInt 4 -> e4 | MkInt 5 -> e5 | MkInt 6 -> e6 | MkInt 7 -> e7 )
        )"#)]
        const _: () = ();

        /// Derives automatically a lift lemma for a given function
        macro_rules! mk_lift_lemma {
            ($name:ident$(<$(const $c:ident : $cty:ty),*>)?($($x:ident : $ty:ty),*) == $lhs:expr) => {
                #[hax_lib::opaque]
                #[hax_lib::lemma]
                #[hax_lib::fstar::before("[@@ $LIFT_LEMMA ]")]
                fn $name$(<$(const $c : $cty,)*>)?($($x : $ty,)*) -> Proof<{
                    hax_lib::eq(
                        unsafe {upstream::$name$(::<$($c,)*>)?($($x,)*)},
                        $lhs
                    )
                }> {}
            }
        }
        mk_lift_lemma!(
            _mm256_set1_epi32(x: i32) ==
            __m256i::from_i32x8(super::_mm256_set1_epi32(x))
        );
        mk_lift_lemma!(
            _mm256_mul_epi32(x: __m256i, y: __m256i) ==
            __m256i::from_i64x4(super::_mm256_mul_epi32(BitVec::to_i32x8(x), BitVec::to_i32x8(y)))
        );
        mk_lift_lemma!(
            _mm256_sub_epi32(x: __m256i, y: __m256i) ==
            __m256i::from_i32x8(super::_mm256_sub_epi32(BitVec::to_i32x8(x), BitVec::to_i32x8(y)))
        );
        mk_lift_lemma!(
            _mm256_shuffle_epi32<const CONTROL: i32>(x: __m256i) ==
            __m256i::from_i32x8(super::_mm256_shuffle_epi32::<CONTROL>(BitVec::to_i32x8(x)))
        );
        mk_lift_lemma!(_mm256_blend_epi32<const CONTROL: i32>(x: __m256i, y: __m256i) ==
            __m256i::from_i32x8(super::_mm256_blend_epi32::<CONTROL>(BitVec::to_i32x8(x), BitVec::to_i32x8(y)))
        );
        mk_lift_lemma!(_mm256_set1_epi16(x: i16) ==
		       __m256i::from_i16x16(super::_mm256_set1_epi16(x)));
        mk_lift_lemma!(_mm_set1_epi16(x: i16) ==
		       __m128i::from_i16x8(super::_mm_set1_epi16(x)));
        mk_lift_lemma!(_mm_set_epi32(e3: i32, e2: i32, e1: i32, e0: i32) ==
               __m128i::from_i32x4(super::_mm_set_epi32(e3, e2, e1, e0)));
        mk_lift_lemma!(_mm_add_epi16(a: __m128i, b: __m128i) ==
               __m128i::from_i16x8(super::_mm_add_epi16(BitVec::to_i16x8(a), BitVec::to_i16x8(b))));
        mk_lift_lemma!(_mm256_add_epi16(a: __m256i, b: __m256i) ==
		       __m256i::from_i16x16(super::_mm256_add_epi16(BitVec::to_i16x16(a), BitVec::to_i16x16(b))));
        mk_lift_lemma!(_mm256_add_epi32(a: __m256i, b: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_add_epi32(BitVec::to_i32x8(a), BitVec::to_i32x8(b))));
        mk_lift_lemma!(_mm256_add_epi64(a: __m256i, b: __m256i) ==
		       __m256i::from_i64x4(super::_mm256_add_epi64(BitVec::to_i64x4(a), BitVec::to_i64x4(b))));
        mk_lift_lemma!(_mm256_abs_epi32(a: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_abs_epi32(BitVec::to_i32x8(a))));
        mk_lift_lemma!(_mm256_sub_epi16(a: __m256i, b: __m256i) ==
		       __m256i::from_i16x16(super::_mm256_sub_epi16(BitVec::to_i16x16(a), BitVec::to_i16x16(b))));
        mk_lift_lemma!(_mm_mullo_epi16(a: __m128i, b: __m128i) ==
		       __m128i::from_i16x8(super::_mm_mullo_epi16(BitVec::to_i16x8(a), BitVec::to_i16x8(b))));
        mk_lift_lemma!(_mm256_cmpgt_epi16(a: __m256i, b: __m256i) ==
		       __m256i::from_i16x16(super::_mm256_cmpgt_epi16(BitVec::to_i16x16(a), BitVec::to_i16x16(b))));
        mk_lift_lemma!(_mm256_cmpgt_epi32(a: __m256i, b: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_cmpgt_epi32(BitVec::to_i32x8(a), BitVec::to_i32x8(b))));
        mk_lift_lemma!(_mm256_sign_epi32(a: __m256i, b: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_sign_epi32(BitVec::to_i32x8(a), BitVec::to_i32x8(b))));
        mk_lift_lemma!(_mm256_movemask_ps(a: __m256) ==
               super::_mm256_movemask_ps(BitVec::to_i32x8(a)));
        mk_lift_lemma!(_mm_mulhi_epi16(a: __m128i, b: __m128i) ==
		       __m128i::from_i16x8(super::_mm_mulhi_epi16(BitVec::to_i16x8(a), BitVec::to_i16x8(b))));
        mk_lift_lemma!(_mm256_mullo_epi32(a: __m256i, b: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_mullo_epi32(BitVec::to_i32x8(a), BitVec::to_i32x8(b))));
        mk_lift_lemma!(_mm256_mulhi_epi16(a: __m256i, b: __m256i) ==
		       __m256i::from_i16x16(super::_mm256_mulhi_epi16(BitVec::to_i16x16(a), BitVec::to_i16x16(b))));
        mk_lift_lemma!(_mm256_mul_epu32(a: __m256i, b: __m256i) ==
		       __m256i::from_u64x4(super::_mm256_mul_epu32(BitVec::to_u32x8(a), BitVec::to_u32x8(b))));
        mk_lift_lemma!(_mm256_srai_epi16<const IMM8: i32>(a: __m256i) ==
		       __m256i::from_i16x16(super::_mm256_srai_epi16::<IMM8>(BitVec::to_i16x16(a))));
        mk_lift_lemma!(_mm256_srai_epi32<const IMM8: i32>(a: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_srai_epi32::<IMM8>(BitVec::to_i32x8(a))));
        mk_lift_lemma!(_mm256_srli_epi16<const IMM8: i32>(a: __m256i) ==
		       __m256i::from_i16x16(super::_mm256_srli_epi16::<IMM8>(BitVec::to_i16x16(a))));
        mk_lift_lemma!(_mm256_srli_epi32<const IMM8: i32>(a: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_srli_epi32::<IMM8>(BitVec::to_i32x8(a))));
        mk_lift_lemma!(_mm_srli_epi64<const IMM8: i32>(a: __m128i) ==
		       __m128i::from_i64x2(super::_mm_srli_epi64::<IMM8>(BitVec::to_i64x2(a))));
        mk_lift_lemma!(_mm256_slli_epi32<const IMM8: i32>(a: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_slli_epi32::<IMM8>(BitVec::to_i32x8(a))));
        mk_lift_lemma!(_mm256_permute4x64_epi64<const IMM8: i32>(a: __m256i) ==
		       __m256i::from_i64x4(super::_mm256_permute4x64_epi64::<IMM8>(BitVec::to_i64x4(a))));
        mk_lift_lemma!(_mm256_unpackhi_epi64(a: __m256i, b: __m256i) ==
		       __m256i::from_i64x4(super::_mm256_unpackhi_epi64(BitVec::to_i64x4(a), BitVec::to_i64x4(b))));
        mk_lift_lemma!(_mm256_unpacklo_epi32(a: __m256i, b: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_unpacklo_epi32(BitVec::to_i32x8(a), BitVec::to_i32x8(b))));
        mk_lift_lemma!(_mm256_unpackhi_epi32(a: __m256i, b: __m256i) ==
		       __m256i::from_i32x8(super::_mm256_unpackhi_epi32(BitVec::to_i32x8(a), BitVec::to_i32x8(b))));
        mk_lift_lemma!(_mm256_cvtepi16_epi32(a: __m128i) ==
		       __m256i::from_i32x8(super::_mm256_cvtepi16_epi32(BitVec::to_i16x8(a))));
        mk_lift_lemma!(_mm_packs_epi16(a: __m128i, b: __m128i) ==
		       __m128i::from_i8x16(super::_mm_packs_epi16(BitVec::to_i16x8(a), BitVec::to_i16x8(b))));
        mk_lift_lemma!(_mm256_packs_epi32(a: __m256i, b: __m256i) ==
               __m256i::from_i16x16(super::_mm256_packs_epi32(BitVec::to_i32x8(a), BitVec::to_i32x8(b))));
        mk_lift_lemma!(_mm256_inserti128_si256<const IMM8: i32>(a: __m256i, b: __m128i) ==
		       __m256i::from_i128x2(super::_mm256_inserti128_si256::<IMM8>(BitVec::to_i128x2(a),BitVec::to_i128x1(b))));
        mk_lift_lemma!(_mm256_blend_epi16<const IMM8: i32>(a: __m256i, b: __m256i) ==
		       __m256i::from_i16x16(super::_mm256_blend_epi16::<IMM8>(BitVec::to_i16x16(a),BitVec::to_i16x16(b))));
        mk_lift_lemma!(_mm256_blendv_ps(a: __m256, b: __m256, c: __m256) ==
		       __m256::from_i32x8(super::_mm256_blendv_ps(BitVec::to_i32x8(a),BitVec::to_i32x8(b),BitVec::to_i32x8(c))));
        mk_lift_lemma!(_mm_movemask_epi8(a: __m128i) ==
		       super::_mm_movemask_epi8(BitVec::to_i8x16(a)));
        mk_lift_lemma!(_mm256_srlv_epi64(a: __m256i, b: __m256i) ==
		       __m256i::from_i64x4(super::_mm256_srlv_epi64(BitVec::to_i64x4(a), BitVec::to_i64x4(b))));
        mk_lift_lemma!(_mm_sllv_epi32(a: __m128i, b: __m128i) ==
		       __m128i::from_i32x4(super::_mm_sllv_epi32(BitVec::to_i32x4(a), BitVec::to_i32x4(b))));
        mk_lift_lemma!(_mm256_slli_epi64<const IMM8: i32>(a: __m256i) ==
               __m256i::from_i64x4(super::_mm256_slli_epi64::<IMM8>(BitVec::to_i64x4(a))));
        mk_lift_lemma!(_mm256_bsrli_epi128<const IMM8: i32>(a: __m256i) ==
               __m256i::from_i128x2(super::_mm256_bsrli_epi128::<IMM8>(BitVec::to_i128x2(a))));
        mk_lift_lemma!(_mm256_set1_epi64x(a: i64) ==
		       __m256i::from_i64x4(super::_mm256_set1_epi64x(a)));
        mk_lift_lemma!(_mm256_set_epi64x(e3: i64, e2: i64, e1: i64, e0: i64) ==
		       __m256i::from_i64x4(super::_mm256_set_epi64x(e3, e2, e1, e0)));
        mk_lift_lemma!(_mm256_unpacklo_epi64(a: __m256i, b: __m256i) ==
		       __m256i::from_i64x4(super::_mm256_unpacklo_epi64(BitVec::to_i64x4(a), BitVec::to_i64x4(b))));
        mk_lift_lemma!(_mm256_permute2x128_si256<const IMM8: i32>(a: __m256i, b: __m256i) ==
		       __m256i::from_i128x2(super::_mm256_permute2x128_si256::<IMM8>(BitVec::to_i128x2(a), BitVec::to_i128x2(b))));

        #[hax_lib::fstar::replace(
            r#"
        let ${flatten_circuit} (): FStar.Tactics.Tac unit =
            let open Tactics.Circuits in
            flatten_circuit
                [
                    "Core_models";
                    "FStar.FunctionalExtensionality";
                    `%Rust_primitives.cast_tc; `%Rust_primitives.unsize_tc;
                    "Core.Ops"; `%(.[]);
                    `%${i64x4::into_i32x8};
                    `%${i32x8::into_i64x4};
                ]
                (top_levels_of_attr (` $LIFT_LEMMA ))
                (top_levels_of_attr (` $SIMPLIFICATION_LEMMA ))
                (top_levels_of_attr (` $ETA_MATCH_EXPAND ))
        "#
        )]
        /// F* tactic: specialization of `Tactics.Circuits.flatten_circuit`.
        pub fn flatten_circuit() {}
    }

    #[cfg(all(test, any(target_arch = "x86", target_arch = "x86_64")))]
    mod tests {
        use crate::abstractions::bitvec::BitVec;
        use crate::core_arch::x86::upstream;
        use crate::helpers::test::HasRandom;

        /// Derives tests for a given intrinsics. Test that a given intrisics and its model compute the same thing over random values (1000 by default).
        macro_rules! mk {
            ($([$N:literal])?$name:ident$({$(<$($c:literal),*>),*})?($($x:ident : $ty:ident),*)) => {
                #[test]
                fn $name() {
                    #[allow(unused)]
                    const N: usize = {
                        let n: usize = 1000;
                        $(let n: usize = $N;)?
                        n
                    };
                    mk!(@[N]$name$($(<$($c),*>)*)?($($x : $ty),*));
                }
            };
            (@[$N:ident]$name:ident$(<$($c:literal),*>)?($($x:ident : $ty:ident),*)) => {
                for _ in 0..$N {
                    $(let $x = $ty::random();)*
                    assert_eq!(super::$name$(::<$($c,)*>)?($($x.into(),)*), unsafe {
                        BitVec::from(upstream::$name$(::<$($c,)*>)?($($x.into(),)*)).into()
                    });
                }
            };
            (@[$N:ident]$name:ident<$($c1:literal),*>$(<$($c:literal),*>)*($($x:ident : $ty:ident),*)) => {
                let one = || {
                    mk!(@[$N]$name<$($c1),*>($($x : $ty),*));
                };
                one();
                mk!(@[$N]$name$(<$($c),*>)*($($x : $ty),*));
            }
        }

        mk!(_mm256_set1_epi32(x: i32));
        mk!(_mm256_sub_epi32(x: BitVec, y: BitVec));
        mk!(_mm256_mul_epi32(x: BitVec, y: BitVec));
        mk!(_mm256_shuffle_epi32{<0b01_00_10_11>, <0b01_11_01_10>}(x: BitVec));
        mk!([100]_mm256_blend_epi32{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(x: BitVec, y: BitVec));
        mk!(_mm256_setzero_si256());
        mk!(_mm256_set_m128i(x: BitVec, y: BitVec));
        mk!(_mm256_set1_epi16(x: i16 ));
        mk!(_mm_set1_epi16(x: i16));
        mk!(_mm_set_epi32(e3: i32, e2: i32, e1: i32, e0: i32));

        mk!(_mm_add_epi16(a: BitVec, b: BitVec));
        mk!(_mm256_add_epi16(a: BitVec, b: BitVec));
        mk!(_mm256_add_epi32(a: BitVec, b: BitVec));
        mk!(_mm256_add_epi64(a: BitVec, b: BitVec));
        mk!(_mm256_abs_epi32(a: BitVec));
        #[test]
        fn _mm256_abs_epi32_min() {
            let a: BitVec<256> = BitVec::from_slice(&[i32::MIN; 8], 32);
            assert_eq!(
                BitVec::from(super::_mm256_abs_epi32(a.into())),
                BitVec::from(unsafe { upstream::_mm256_abs_epi32(a.into()) })
            );
        }
        mk!(_mm256_sub_epi16(a: BitVec, b: BitVec));

        mk!(_mm_mullo_epi16(a: BitVec, b: BitVec));
        mk!(_mm256_cmpgt_epi16(a: BitVec, b: BitVec));
        mk!(_mm256_cmpgt_epi32(a: BitVec, b: BitVec));
        mk!(_mm256_sign_epi32(a: BitVec, b: BitVec));
        #[test]
        fn _mm256_sign_epi32_min() {
            let a: BitVec<256> = BitVec::from_slice(&[i32::MIN; 8], 32);
            let b: BitVec<256> = BitVec::from_slice(&[-1; 8], 32);
            assert_eq!(
                BitVec::from(super::_mm256_sign_epi32(a.into(), b.into())),
                BitVec::from(unsafe { upstream::_mm256_sign_epi32(a.into(), b.into()) })
            );
        }
        mk!(_mm256_castsi256_ps(a: BitVec));
        mk!(_mm256_castps_si256(a: BitVec));

        #[test]
        fn _mm256_movemask_ps() {
            let n = 1000;

            for _ in 0..n {
                let a: BitVec<256> = BitVec::random();
                assert_eq!(super::_mm256_movemask_ps(a.into()), unsafe {
                    upstream::_mm256_movemask_ps(a.into())
                });
            }
        }
        mk!(_mm_mulhi_epi16(a: BitVec, b: BitVec));
        mk!(_mm256_mullo_epi32(a: BitVec, b: BitVec));

        mk!(_mm256_and_si256(a: BitVec, b: BitVec));
        mk!(_mm256_or_si256(a: BitVec, b: BitVec));
        #[test]
        fn _mm256_testz_si256() {
            let n = 1000;

            for _ in 0..n {
                let a: BitVec<256> = BitVec::random();
                let b: BitVec<256> = BitVec::random();
                assert_eq!(super::_mm256_testz_si256(a.into(), b.into()), unsafe {
                    upstream::_mm256_testz_si256(a.into(), b.into())
                });
            }
        }
        mk!(_mm256_xor_si256(a: BitVec, b: BitVec));

        mk!([100]_mm256_srai_epi16{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec));

        mk!([100]_mm256_srai_epi32{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec));

        mk!([100]_mm256_srli_epi16{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec));

        mk!([100]_mm256_srli_epi32{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec));

        mk!([100]_mm_srli_epi64{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec));

        mk!([100]_mm256_slli_epi32{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec));

        mk!(_mm256_castsi128_si256(a: BitVec));

        mk!(_mm256_cvtepi16_epi32(a: BitVec));
        mk!(_mm_packs_epi16(a: BitVec, b: BitVec));
        mk!(_mm256_packs_epi32(a: BitVec, b: BitVec));
        mk!([100]_mm256_inserti128_si256{<0>,<1>}(a: BitVec, b: BitVec));
        mk!([100]_mm256_blend_epi16{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec, b: BitVec));
        mk!(_mm256_blendv_ps(a: BitVec, b: BitVec, mask: BitVec));

        #[test]
        fn _mm_movemask_epi8() {
            let n = 1000;

            for _ in 0..n {
                let a: BitVec<128> = BitVec::random();
                assert_eq!(super::_mm_movemask_epi8(a.into()), unsafe {
                    upstream::_mm_movemask_epi8(a.into())
                });
            }
        }
        mk!(_mm256_srlv_epi64(a: BitVec, b: BitVec));
        mk!(_mm_sllv_epi32(a: BitVec, b: BitVec));

        mk!([100]_mm256_slli_epi64{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec));

        mk!([100]_mm256_bsrli_epi128{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec));

        mk!(_mm256_andnot_si256(a: BitVec, b: BitVec));
        mk!(_mm256_set1_epi64x(a: i64));
        mk!(_mm256_set_epi64x(e3: i64, e2: i64, e1: i64, e0: i64));
        mk!(_mm256_unpacklo_epi64(a: BitVec, b: BitVec));

        mk!([100]_mm256_permute2x128_si256{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec, b: BitVec));
    }
}

/// Track I (2026-06-10): differential validation of the F* TRUST AXIOMS added/fixed
/// for the ML-KEM AVX2 rejection-sampling proof, against the executable core-models
/// reference semantics in this file / `x86.rs` (which are themselves hardware-validated
/// by the `mk!` differential tests above, on x86 hosts).
///
/// Each test transcribes the F* axiom's formula literally to Rust and compares it with
/// the core-models model on randomized + edge inputs. Pure Rust: runs on any host
/// (no `upstream`, no x86 needed).
///
/// Axioms validated (see the cross-references at the axiom sites):
/// - `mm256_cmpgt_epi16` ensures
///   (`crates/utils/intrinsics/src/avx2_extract.rs`)
/// - `mm_storeu_si128` content+frame ensures
///   (`crates/utils/intrinsics/src/avx2_extract.rs`)
/// - `bit_vec_of_int_t_array_vec128_as_i16x8_lemma`
///   (`crates/utils/intrinsics/src/avx2_extract.rs`)
/// - `mm_shuffle_epi8_no_semantics_lemma`
///   (`libcrux-ml-kem/src/vector/avx2/sampling.rs`)
#[cfg(test)]
mod track_i_axiom_transcription_tests {
    use super::int_vec;
    use crate::abstractions::{bit::Bit, bitvec::BitVec, funarr::FunArray};
    use crate::core_arch::x86::{extra, ssse3};

    /// F* axiom: `forall (i: nat{i < 256}). result i ==
    ///   (if Seq.index (vec256_as_i16x16 lhs) (i/16) >. Seq.index (vec256_as_i16x16 rhs) (i/16)
    ///    then 1 else 0)`
    /// vs the model `int_vec::_mm256_cmpgt_epi16` (interpretations.rs:
    /// `i16x16::from_fn(|i| if a[i] > b[i] { -1 } else { 0 })`).
    fn check_cmpgt(a: BitVec<256>, b: BitVec<256>) {
        let model: BitVec<256> = BitVec::from_i16x16(int_vec::_mm256_cmpgt_epi16(
            BitVec::to_i16x16(a),
            BitVec::to_i16x16(b),
        ));
        let la: Vec<i16> = a.to_vec();
        let lb: Vec<i16> = b.to_vec();
        let formula = BitVec::<256>::from_fn(|i| {
            if la[(i / 16) as usize] > lb[(i / 16) as usize] {
                Bit::One
            } else {
                Bit::Zero
            }
        });
        assert_eq!(model, formula);
    }

    #[test]
    fn cmpgt_epi16_bit_level_formula() {
        for _ in 0..1000 {
            check_cmpgt(BitVec::rand(), BitVec::rand());
        }
        // Edge lanes: equal / greater / less, including INT16_MIN/MAX and the
        // FIELD_MODULUS values the rejection sampler compares against.
        let specials: [i16; 9] = [i16::MIN, i16::MIN + 1, -1, 0, 1, 3328, 3329, i16::MAX - 1, i16::MAX];
        for &x in specials.iter() {
            for &y in specials.iter() {
                let a = BitVec::<256>::from_slice(&[x; 16], 16);
                let b = BitVec::<256>::from_slice(&[y; 16], 16);
                check_cmpgt(a, b);
            }
        }
        // Mixed lanes (per-lane independence).
        for _ in 0..100 {
            let mut xs = [0i16; 16];
            let mut ys = [0i16; 16];
            for k in 0..16 {
                xs[k] = specials[(k * 7 + 3) % specials.len()];
                ys[k] = specials[(k * 5 + 1) % specials.len()];
            }
            check_cmpgt(
                BitVec::<256>::from_slice(&xs, 16),
                BitVec::<256>::from_slice(&ys, 16),
            );
        }
    }

    /// F* axiom (`mm256_cvtepi16_epi32` ensures, avx2_extract.rs): sign-extends
    /// each of the 8 i16 lanes of `vector` to an i32 lane of the result.  On the
    /// i16x16 view of the result: `get_lane result (2j) == get_lane128 vector j`
    /// and `get_lane result (2j+1) == (if v (get_lane128 vector j) < 0 then
    /// mk_i16 (-1) else mk_i16 0)` (the sign fill, 0xffff/0x0000).
    /// Model anchor: `int_vec::_mm256_cvtepi16_epi32`
    /// (`i32x8::from_fn(|i| a[i] as i32)` — Rust `as i32` sign-extends).
    fn check_cvt(a: BitVec<128>) {
        let model: Vec<i16> =
            BitVec::<256>::from_i32x8(int_vec::_mm256_cvtepi16_epi32(BitVec::to_i16x8(a))).to_vec();
        let input: Vec<i16> = a.to_vec();
        let formula: Vec<i16> = (0..16usize)
            .map(|i| {
                let j = i / 2;
                if i % 2 == 0 {
                    input[j]
                } else if input[j] < 0 {
                    -1i16
                } else {
                    0i16
                }
            })
            .collect();
        assert_eq!(model, formula);
    }

    #[test]
    fn cvtepi16_epi32_lane_formula() {
        for _ in 0..1000 {
            check_cvt(BitVec::rand());
        }
        // Edge lanes incl. INT16_MIN/MAX and the FIELD_MODULUS neighbourhood.
        let specials: [i16; 9] =
            [i16::MIN, i16::MIN + 1, -1, 0, 1, 3328, 3329, i16::MAX - 1, i16::MAX];
        for &x in specials.iter() {
            check_cvt(BitVec::<128>::from_slice(&[x; 8], 16));
        }
        // Mixed lanes (per-lane independence).
        for _ in 0..100 {
            let mut xs = [0i16; 8];
            for k in 0..8 {
                xs[k] = specials[(k * 7 + 3) % specials.len()];
            }
            check_cvt(BitVec::<128>::from_slice(&xs, 16));
        }
    }

    /// F* axiom: `mm_storeu_si128` stores exactly the 8 LSB-first i16 lanes
    /// (`vec128_as_i16x8 vector`) to `output[0..8]`, framing the rest.
    /// Model anchor: `other::_mm_storeu_si128` / `extra::mm_storeu_bytes_si128`
    /// (x86.rs): the store writes exactly the 16 bytes of the vector (`*output = a`),
    /// i.e. 8 little-endian i16 lanes — and nothing else (the frame clause).
    #[test]
    fn storeu_si128_lane_formula() {
        for _ in 0..1000 {
            let vec: BitVec<128> = BitVec::rand();
            let mut bytes = [0u8; 16];
            extra::mm_storeu_bytes_si128(&mut bytes, vec);
            // model: the 16 stored bytes, read back as 8 LE i16 lanes
            let model: Vec<i16> = bytes
                .chunks(2)
                .map(|c| i16::from_le_bytes([c[0], c[1]]))
                .collect();
            // F* formula: output_future[0..8] == vec128_as_i16x8 vector,
            // where vec128_as_i16x8 is the canonical LSB-first 16-bit lane view
            // (= BitVec::to_vec::<i16>, as pinned by vec128_lane_bit_decomposition below)
            let formula: Vec<i16> = vec.to_vec();
            assert_eq!(model, formula);
        }
    }

    /// F* axiom: `bit_vec_of_int_t_array (vec128_as_i16x8 v) d i == v ((i/d)*16 + i%d)`
    /// for `0 < d <= 16`, `i < 8*d` — i.e. lane k of the canonical i16x8 view occupies
    /// bits `16k..16k+15` of the bit-vector, LSB-first (two's complement bits).
    /// Model anchor: `BitVec::to_vec::<i16>` chunking (abstractions/bitvec.rs), the
    /// same lane view used by `BitVec::to_i16x8` and `mm_storeu_bytes_si128`.
    #[test]
    fn vec128_lane_bit_decomposition() {
        for _ in 0..1000 {
            let v: BitVec<128> = BitVec::rand();
            let lanes: Vec<i16> = v.to_vec();
            for d in 1..=16u64 {
                for i in 0..(8 * d) {
                    let lane = lanes[(i / d) as usize] as u16;
                    let lane_bit = (lane >> (i % d)) & 1;
                    let v_bit: u16 = u16::from(v[(i / d) * 16 + i % d]);
                    assert_eq!(lane_bit, v_bit, "d={d} i={i}");
                }
            }
        }
    }

    /// F* axiom `count_ones_u8_popcount8` (Track I M2, landed in the
    /// `fstar::before` block of `libcrux-ml-kem/src/vector/avx2/sampling.rs`):
    /// `v (count_ones_u8 x) == popcount8 (v x)` with
    /// `popcount8 g = if g = 0 then 0 else g % 2 + popcount8 (g / 2)`
    /// (defined in `Hacspec_ml_kem.Commute.Rej_table`).
    /// Model anchor: Rust core's `u8::count_ones` — the operation that
    /// `Rust_primitives.Arithmetic.count_ones_u8` (an uninterpreted F* val) models.
    /// Exhaustive over all 256 inputs.
    #[test]
    fn count_ones_popcount8_formula() {
        fn popcount8(g: u32) -> u32 {
            if g == 0 {
                0
            } else {
                g % 2 + popcount8(g / 2)
            }
        }
        for x in 0..=255u8 {
            assert_eq!(x.count_ones(), popcount8(x as u32));
        }
    }

    /// F* axiom (`mm_shuffle_epi8_no_semantics_lemma`, ml-kem sampling.rs):
    ///   `result i == (let nth = i / 8 in
    ///                 let idx = sum_k b (8*nth+k) * 2^k in
    ///                 if idx > 127 then 0 else a ((idx % 16) * 8 + i % 8))`
    /// vs the model `ssse3::_mm_shuffle_epi8` / `extra::mm_shuffle_epi8_u8_array`
    /// (x86.rs:1107: `if index > 127 { Zero } else { vector[(index % 16)*8 + i%8] }`).
    ///
    /// The mask bit-vector `b` is built with the same byte->bit mapping as
    /// `BitVec.Intrinsics.mm_loadu_si128` (`get_bit bytes[i/8] (i%8)`), which is
    /// `BitVec::from_slice(bytes, 8)` — so this also validates the byte-decode
    /// (`idx = sum_k b(8*nth+k)*2^k`) used in the F* axiom.
    fn check_shuffle(a: BitVec<128>, mask_bytes: [u8; 16]) {
        let b = BitVec::<128>::from_slice(&mask_bytes, 8);
        // model via the ssse3 entry point (FunArray<16, u8> indexes)
        let model_ssse3 = ssse3::_mm_shuffle_epi8(a, b);
        // model via the array primitive directly
        let indexes = FunArray::<16, u8>::from_fn(|i| mask_bytes[i as usize]);
        let model_extra = extra::mm_shuffle_epi8_u8_array(a, indexes);
        assert_eq!(model_ssse3, model_extra);
        // F* formula transcription
        let formula = BitVec::<128>::from_fn(|i| {
            let nth = i / 8;
            let idx: u64 = (0..8u64).map(|k| u64::from(b[8 * nth + k]) << k).sum();
            if idx > 127 {
                Bit::Zero
            } else {
                a[(idx % 16) * 8 + i % 8]
            }
        });
        assert_eq!(model_extra, formula);
    }

    #[test]
    fn shuffle_epi8_dynamic_mask_formula() {
        // randomized masks (uniform over all byte values incl. MSB-set)
        for _ in 0..1000 {
            let a: BitVec<128> = BitVec::rand();
            let mask_bv: BitVec<128> = BitVec::rand();
            let mask_bytes: [u8; 16] = mask_bv.to_vec::<u8>().try_into().unwrap();
            check_shuffle(a, mask_bytes);
        }
        // exhaustive index coverage: every mask byte value 0..=255
        // (covers idx <= 127 with %16 wrap, and idx > 127 => zero)
        for v in 0..=255u8 {
            let a: BitVec<128> = BitVec::rand();
            check_shuffle(a, [v; 16]);
        }
        // REJECTION_SAMPLE_SHUFFLE_TABLE-shaped masks: pairs (2k, 2k+1) + 0xff fill
        for _ in 0..100 {
            let a: BitVec<128> = BitVec::rand();
            let mut mask = [0xffu8; 16];
            for j in 0..8 {
                let k = (j * 3) % 8;
                mask[2 * j] = (2 * k) as u8;
                mask[2 * j + 1] = (2 * k + 1) as u8;
            }
            check_shuffle(a, mask);
        }
    }
}
