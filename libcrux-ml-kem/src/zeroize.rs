use crate::{
    ind_cca::unpacked::{MlKemKeyPairUnpacked, MlKemPrivateKeyUnpacked},
    ind_cpa::unpacked::IndCpaPrivateKeyUnpacked,
    polynomial::PolynomialRingElement,
    vector::{portable::PortableVector, Operations},
    MlKemKeyPair, MlKemPrivateKey,
};

#[cfg(feature = "simd256")]
use crate::vector::SIMD256Vector;

#[cfg(feature = "simd128")]
use crate::vector::SIMD128Vector;

impl<const SIZE: usize> zeroize::Zeroize for MlKemPrivateKey<SIZE> {
    fn zeroize(&mut self) {
        self.value.zeroize();
    }
}
impl<const SIZE: usize> zeroize::ZeroizeOnDrop for MlKemPrivateKey<SIZE> {}

impl<const PRIVATE_KEY_SIZE: usize, const PUBLIC_KEY_SIZE: usize> zeroize::Zeroize
    for MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE>
{
    fn zeroize(&mut self) {
        self.sk.zeroize();
    }
}
impl<const PRIVATE_KEY_SIZE: usize, const PUBLIC_KEY_SIZE: usize> zeroize::ZeroizeOnDrop
    for MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE>
{
}

impl<Vector: Operations + zeroize::Zeroize> zeroize::Zeroize for PolynomialRingElement<Vector> {
    fn zeroize(&mut self) {
        self.coefficients.zeroize();
    }
}
impl<Vector: Operations + zeroize::Zeroize> zeroize::ZeroizeOnDrop for PolynomialRingElement<Vector> {}


impl<const SIZE: usize, Vector: Operations + zeroize::Zeroize> zeroize::Zeroize
    for MlKemPrivateKeyUnpacked<SIZE, Vector>
{
    fn zeroize(&mut self) {
        self.implicit_rejection_value.zeroize();
        self.ind_cpa_private_key.zeroize();
    }
}
impl<const SIZE: usize, Vector: Operations + zeroize::Zeroize> zeroize::ZeroizeOnDrop
    for MlKemPrivateKeyUnpacked<SIZE, Vector>
{
}

impl<const SIZE: usize, Vector: Operations + zeroize::Zeroize> zeroize::Zeroize
    for IndCpaPrivateKeyUnpacked<SIZE, Vector>
{
    fn zeroize(&mut self) {
        self.secret_as_ntt.zeroize();
    }
}
impl<const SIZE: usize, Vector: Operations + zeroize::Zeroize> zeroize::ZeroizeOnDrop
    for IndCpaPrivateKeyUnpacked<SIZE, Vector>
{
}

impl<const K: usize, Vector: Operations + zeroize::Zeroize> zeroize::Zeroize
    for MlKemKeyPairUnpacked<K, Vector>
{
    fn zeroize(&mut self) {
        self.private_key.zeroize();
    }
}
impl<const K: usize, Vector: Operations + zeroize::Zeroize> zeroize::ZeroizeOnDrop
    for MlKemKeyPairUnpacked<K, Vector>
{
}

// Don't implement ZeroizeOnDrop for vector types for performance reasons
impl zeroize::Zeroize for PortableVector {
    fn zeroize(&mut self) {
        self.elements.zeroize();
    }
}

#[cfg(feature = "simd128")]
impl zeroize::Zeroize for SIMD128Vector {
    fn zeroize(&mut self) {
        // Overwrite the SIMD registers (or memory backing them) with zeros.
        let zero = SIMD128Vector::ZERO();
        self.low = zero.low;
        self.high = zero.high;

        // Prevent Dead Store Elimination (DSE).
        // black_box forces the compiler to treat the memory as used, ensuring
        // the zeroing instructions aren't optimized away.
        //
        // Checking the assembly output on a Macbook M1 shows that this successfully
        // pushes the compiler to preserve the needed instructions in Debug and Release.
        core::hint::black_box(self);
    }
}

#[cfg(feature = "simd256")]
impl zeroize::Zeroize for SIMD256Vector {
    fn zeroize(&mut self) {
        // AVX2 registers implemented natively by zeroize crate
        self.elements.zeroize();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        hash_functions::portable::PortableHash,
        mlkem768::{MlKem768KeyPair, MlKem768PrivateKey, MlKem768PublicKey},
        variant::MlKem,
    };
    use zeroize::Zeroize;

    // Using black_box forces the CPU to perform an actual memory
    // fetch and comparison, ensuring the test validates the physical state
    // of the memory rather than a compiler-level assumption.
    trait CheckZero {
        fn is_zero(slice: &[Self]) -> bool
        where
            Self: Sized;
    }

    impl CheckZero for u8 {
        #[inline(never)]
        fn is_zero(slice: &[u8]) -> bool {
            slice.iter().all(|&byte| core::hint::black_box(byte) == 0)
        }
    }

    impl CheckZero for i16 {
        #[inline(never)]
        fn is_zero(slice: &[i16]) -> bool {
            // We still check the i16 value, but black_box the whole 16-bit word
            slice.iter().all(|&val| core::hint::black_box(val) == 0)
        }
    }

    #[test]
    fn test_is_zero_sanity_check() {
        {
            let mut canary = [0xFFu8; 32];

            // Wrap in black_box so the compiler doesn't "pre-calculate" the result.
            let is_it_zero = u8::is_zero(core::hint::black_box(&canary));

            assert!(!is_it_zero, "is_zero should have detected the 0xFF bytes");
            canary.zeroize();
            let is_it_zero_now = u8::is_zero(core::hint::black_box(&canary));

            assert!(
                is_it_zero_now,
                "is_zero should return true after manual zeroing"
            );
        }

        {
            let mut canary = [0x4444i16; 16];

            // Wrap in black_box so the compiler doesn't "pre-calculate" the result.
            let is_it_zero = i16::is_zero(core::hint::black_box(&canary));

            assert!(!is_it_zero, "is_zero should have detected the 0xFF bytes");
            canary.zeroize();
            let is_it_zero_now = i16::is_zero(core::hint::black_box(&canary));

            assert!(
                is_it_zero_now,
                "is_zero should return true after manual zeroing"
            );
        }
    }

    #[test]
    fn mlkem_private_key() {
        // Create a dummy private key with non-zero data
        let sk_bytes = [0xFFu8; 2400]; // Size for Kyber768 roughly
        let mut sk = MlKem768PrivateKey::from(sk_bytes);

        // Verify it's not zero initially
        assert!(!u8::is_zero(&sk.as_ref()), "Key should start non-zero");
        sk.zeroize();

        // Verify it is now zero
        assert!(
            u8::is_zero(&sk.as_ref()),
            "Key should be zeroed after zeroize()"
        );
    }

    #[test]
    fn mlkem_keypair() {
        // Setup dummy keys
        let sk_bytes = [0xAAu8; 2400];
        let pk_bytes = [0xBBu8; 1184];
        let sk = MlKem768PrivateKey::from(sk_bytes);
        let pk = MlKem768PublicKey::from(pk_bytes);

        let mut keypair = MlKem768KeyPair::from(sk, pk);

        // Check internal private key is not zero
        assert!(!u8::is_zero(&keypair.sk().as_ref()));

        // Zeroize the pair
        keypair.zeroize();

        // Check private key is zeroed
        assert!(
            u8::is_zero(&keypair.sk().as_ref()),
            "Private key inside pair should be zeroed"
        );
    }

    #[test]
    fn portable_vector() {
        let mut vec = PortableVector::from_bytes(&[0xFF; 32]);

        vec.zeroize();

        assert!(
            i16::is_zero(vec.elements.as_ref()),
            "Element was not zeroed"
        );
    }

    #[test]
    #[cfg(feature = "simd128")]
    fn simd128_vector() {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }

        let mut vec = SIMD128Vector::from_bytes(&[0xFF; 32]);

        vec.zeroize();

        // Security Verification:
        // Manual inspection of the assembly for this test on AArch64 (Apple M1)
        // confirms that the compiler does NOT optimize this check away.
        //
        // The resulting binary uses `stp` (Store Pair) to write zeros to the
        // stack/memory and `umaxv.16b` (Unsigned Maximum Across Vector) to
        // perform the horizontal reduction for the zero check. The use of
        // `black_box` prevents Dead Store Elimination, forcing the CPU to
        // physically verify the state of the registers/memory.
        let opaque_vec = core::hint::black_box(vec);

        let low_bytes = libcrux_intrinsics::arm64::_vreinterpretq_u8_s16(opaque_vec.low);
        let high_bytes = libcrux_intrinsics::arm64::_vreinterpretq_u8_s16(opaque_vec.high);

        let low_max = libcrux_intrinsics::arm64::_vmaxvq_u8(low_bytes);
        let high_max = libcrux_intrinsics::arm64::_vmaxvq_u8(high_bytes);

        assert_eq!(low_max, 0, "Low 128 bits were not zeroed");
        assert_eq!(high_max, 0, "High 128 bits were not zeroed");
    }

    #[test]
    #[cfg(feature = "simd256")]
    fn simd256_vector() {
        if !std::is_x86_feature_detected!("avx2") {
            return;
        }

        let mut vec = SIMD256Vector::from_bytes(&[0xFF; 32]);

        vec.zeroize();

        // Security Verification:
        // Manual inspection of the assembly for this test (via RUSTFLAGS="--emit asm")
        // confirms that the compiler does NOT optimize this check away.
        //
        // The resulting binary uses `vmovdqa` to load the memory and `vptest`
        // to verify the bits. The presence of the #APP/#NO_APP (black_box)
        // markers ensures that Dead Store Elimination is bypassed, forcing
        // the CPU to physically confirm the RAM is zeroed.
        let opaque_vec = core::hint::black_box(vec);
        assert!(
            libcrux_intrinsics::avx2::mm256_testz_si256(opaque_vec.elements, opaque_vec.elements)
                != 0,
            "Element was not zeroed"
        );
    }

    #[test]
    fn unpacked_keypair_cascade() {
        let randomness = [0x42u8; 64]; // Deterministic seed
        let mut unpacked = MlKemKeyPairUnpacked::<3, PortableVector>::new(); // Rank 3 = Kyber768

        crate::ind_cca::unpacked::generate_keypair::<
            3,               // Rank 3 = Kyber768
            1152,            // CPA_PRIVATE_KEY_SIZE (3 * 384)
            2400,            // PRIVATE_KEY_SIZE
            1184,            // PUBLIC_KEY_SIZE
            2,               // ETA1
            128,             // ETA1_RANDOMNESS_SIZE (64 * ETA1)
            PortableVector,  // Vector Type
            PortableHash<3>, // Hasher Type
            MlKem,           // Variant Scheme
        >(randomness, &mut unpacked);

        // Sanity check: ensure Implicit Rejection Value is set
        let irv_before = unpacked.private_key.implicit_rejection_value;
        assert!(
            !u8::is_zero(&irv_before),
            "Implicit rejection value should be set"
        );

        // Zeroize the top-level Unpacked KeyPair
        unpacked.zeroize();

        // Verify Implicit Rejection Value is gone
        let irv_after = unpacked.private_key.implicit_rejection_value;
        assert!(
            u8::is_zero(&irv_after),
            "Implicit rejection value should be zeroed"
        );

        // Verify the IndCpaPrivateKey (Polynomials) are zeroed
        // We have to iterate over the polynomials in the secret vector
        for poly in &unpacked.private_key.ind_cpa_private_key.secret_as_ntt {
            for coeff in &poly.coefficients {
                assert!(
                    i16::is_zero(coeff.elements.as_ref()),
                    "Coefficient was not zeroed"
                );
            }
        }
    }
}
