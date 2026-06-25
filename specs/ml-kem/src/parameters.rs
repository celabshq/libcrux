/// Field modulus: 3329
pub const FIELD_MODULUS: u16 = 3329;

/// Each field element needs floor(log_2(FIELD_MODULUS)) + 1 = 12 bits to represent
pub const BITS_PER_COEFFICIENT: usize = 12;

/// Coefficients per ring element
pub const COEFFICIENTS_IN_RING_ELEMENT: usize = 256;

/// Bits required per (uncompressed) ring element
pub const BITS_PER_RING_ELEMENT: usize = COEFFICIENTS_IN_RING_ELEMENT * 12;

/// Bytes required per (uncompressed) ring element
pub const BYTES_PER_RING_ELEMENT: usize = BITS_PER_RING_ELEMENT / 8;

/// Seed size for rejection sampling.
///
/// See <https://eprint.iacr.org/2023/708> for some background regarding
/// this choice.
pub const REJECTION_SAMPLING_SEED_SIZE: usize = 168 * 5;

pub use hash_functions::H_DIGEST_SIZE;

/// ML-KEM parameter set
#[hax_lib::attributes]
pub struct MlKemParams {
    #[hax_lib::refine(rank <= 4)]
    pub rank: usize,
    #[hax_lib::refine(eta1 <= 2 || eta1 == 3)]
    pub eta1: usize,
    #[hax_lib::refine(eta2 == 2)]
    pub eta2: usize,
    #[hax_lib::refine(du == 10 || du == 11)]
    pub du: usize,
    #[hax_lib::refine(dv == 4 || dv == 5)]
    pub dv: usize,
}

impl MlKemParams {
    pub const fn t_as_ntt_encoded_size(&self) -> usize {
        self.rank * BYTES_PER_RING_ELEMENT
    }
    pub const fn ek_size(&self) -> usize {
        self.t_as_ntt_encoded_size() + 32
    }
    pub const fn dk_pke_size(&self) -> usize {
        self.rank * BYTES_PER_RING_ELEMENT
    }
    pub const fn dk_size(&self) -> usize {
        self.dk_pke_size() + self.ek_size() + H_DIGEST_SIZE + 32
    }
    pub const fn u_encoded_size(&self) -> usize {
        (self.rank * COEFFICIENTS_IN_RING_ELEMENT * self.du) / 8
    }
    pub const fn v_encoded_size(&self) -> usize {
        (COEFFICIENTS_IN_RING_ELEMENT * self.dv) / 8
    }
    pub const fn ciphertext_size(&self) -> usize {
        self.u_encoded_size() + self.v_encoded_size()
    }
}

pub const ML_KEM_512: MlKemParams = MlKemParams {
    rank: 2,
    eta1: 3,
    eta2: 2,
    du: 10,
    dv: 4,
};
pub const ML_KEM_768: MlKemParams = MlKemParams {
    rank: 3,
    eta1: 2,
    eta2: 2,
    du: 10,
    dv: 4,
};
pub const ML_KEM_1024: MlKemParams = MlKemParams {
    rank: 4,
    eta1: 2,
    eta2: 2,
    du: 11,
    dv: 5,
};

// Derived sizes for ML-KEM-512 (k=2, du=10, dv=4)
pub const ML_KEM_512_EK_SIZE: usize = 800; // 2*384 + 32
pub const ML_KEM_512_DK_PKE_SIZE: usize = 768; // 2*384
pub const ML_KEM_512_DK_SIZE: usize = 1632; // 768 + 800 + 32 + 32
pub const ML_KEM_512_U_SIZE: usize = 640; // 2*256*10/8
pub const ML_KEM_512_V_SIZE: usize = 128; // 256*4/8
pub const ML_KEM_512_CT_SIZE: usize = 768; // U_SIZE + V_SIZE
pub const ML_KEM_512_J_INPUT_SIZE: usize = 800; // 32 + 768

// Derived sizes for ML-KEM-768 (k=3, du=10, dv=4)
pub const ML_KEM_768_EK_SIZE: usize = 1184; // 3*384 + 32
pub const ML_KEM_768_DK_PKE_SIZE: usize = 1152; // 3*384
pub const ML_KEM_768_DK_SIZE: usize = 2400; // 1152 + 1184 + 32 + 32
pub const ML_KEM_768_U_SIZE: usize = 960; // 3*256*10/8
pub const ML_KEM_768_V_SIZE: usize = 128; // 256*4/8
pub const ML_KEM_768_CT_SIZE: usize = 1088; // U_SIZE + V_SIZE
pub const ML_KEM_768_J_INPUT_SIZE: usize = 1120; // 32 + 1088

// Derived sizes for ML-KEM-1024 (k=4, du=11, dv=5)
pub const ML_KEM_1024_EK_SIZE: usize = 1568; // 4*384 + 32
pub const ML_KEM_1024_DK_PKE_SIZE: usize = 1536; // 4*384
pub const ML_KEM_1024_DK_SIZE: usize = 3168; // 1536 + 1568 + 32 + 32
pub const ML_KEM_1024_U_SIZE: usize = 1408; // 4*256*11/8
pub const ML_KEM_1024_V_SIZE: usize = 160; // 256*5/8
pub const ML_KEM_1024_CT_SIZE: usize = 1568; // U_SIZE + V_SIZE
pub const ML_KEM_1024_J_INPUT_SIZE: usize = 1600; // 32 + 1568

/// Rank-generic CPA ciphertext size, matching the Spec.MLKEM
/// `v_CPA_CIPHERTEXT_SIZE` shape.  Use this in rank-generic
/// `hax_lib::requires`/`ensures` annotations where threading a
/// `MlKemParams` value would be an architectural refactor.  For
/// fixed-rank consumers, prefer the `ML_KEM_{512,768,1024}_CT_SIZE`
/// constants directly; for `MlKemParams`-aware callers, prefer
/// `MlKemParams::ciphertext_size()`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn cpa_ciphertext_size(rank: usize) -> usize {
    if rank == 2 {
        ML_KEM_512_CT_SIZE
    } else if rank == 3 {
        ML_KEM_768_CT_SIZE
    } else {
        ML_KEM_1024_CT_SIZE
    }
}

/// Rank-to-`MlKemParams` lookup for rank-generic consumers that need to
/// invoke a `MlKemParams`-shape Hacspec function (e.g.
/// `Hacspec_ml_kem.Ind_cca.generate_keypair`) from an
/// `hax_lib::requires`/`ensures` annotation.  This is the canonical
/// adapter from the libcrux-side `const K: usize` shape to the
/// Hacspec-side `params: MlKemParams` shape.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn rank_to_params(rank: usize) -> MlKemParams {
    if rank == 2 {
        ML_KEM_512
    } else if rank == 3 {
        ML_KEM_768
    } else {
        ML_KEM_1024
    }
}

/// Rank predicate: ML-KEM is parameterised over rank ∈ {2, 3, 4}.
/// Use in `hax_lib::requires` to express rank refinement in pure Rust.
pub const fn is_rank(rank: usize) -> bool {
    rank == 2 || rank == 3 || rank == 4
}

// ─────────────────────────────────────────────────────────────────────────────
// Rank-generic free-function size helpers
//
// These mirror the `Spec.MLKEM.*` size constants that appear in the libcrux
// `hax_lib::requires`/`ensures` annotations.  All compose as a 3-way `if` on
// `rank ∈ {2,3,4}` (the same pattern as `cpa_ciphertext_size`/`rank_to_params`).
// They exist so that consumers can write pure-Rust annotations without having
// to thread a `MlKemParams` value through the call site (the methods on
// `MlKemParams` do the same job when an instance is in scope).
// ─────────────────────────────────────────────────────────────────────────────

/// Shared-secret length in bytes (FIPS 203 §7).
pub const SHARED_SECRET_SIZE: usize = 32;

/// CPA key-generation seed length in bytes (FIPS 203 §7).
pub const CPA_KEY_GENERATION_SEED_SIZE: usize = 32;

/// Rank-generic encoded-NTT-vector size: `rank * 384`.  Mirrors
/// `Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE` and
/// `Spec.MLKEM.v_RANKED_BYTES_PER_RING_ELEMENT`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn t_as_ntt_encoded_size(rank: usize) -> usize {
    rank * BYTES_PER_RING_ELEMENT
}

/// Synonym of `t_as_ntt_encoded_size` matching `Spec.MLKEM`'s
/// `v_RANKED_BYTES_PER_RING_ELEMENT` naming.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn ranked_bytes_per_ring_element(rank: usize) -> usize {
    rank * BYTES_PER_RING_ELEMENT
}

/// CPA encryption-key size: `rank * 384 + 32`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn cpa_public_key_size(rank: usize) -> usize {
    rank * BYTES_PER_RING_ELEMENT + 32
}

/// CPA decryption-key size: `rank * 384`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn cpa_private_key_size(rank: usize) -> usize {
    rank * BYTES_PER_RING_ELEMENT
}

/// CCA decapsulation-key size: `cpa_private_key + cpa_public_key + H_DIGEST + z`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn cca_private_key_size(rank: usize) -> usize {
    cpa_private_key_size(rank) + cpa_public_key_size(rank) + H_DIGEST_SIZE + 32
}

/// `du` compression factor for vector u: 10 for ranks 2,3 and 11 for rank 4.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn vector_u_compression_factor(rank: usize) -> usize {
    if rank == 4 {
        11
    } else {
        10
    }
}

/// `dv` compression factor for ring element v: 4 for ranks 2,3 and 5 for rank 4.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn vector_v_compression_factor(rank: usize) -> usize {
    if rank == 4 {
        5
    } else {
        4
    }
}

/// Per-block (per-ring-element) size of c1 in bytes: `(256 * du)/8`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn c1_block_size(rank: usize) -> usize {
    (COEFFICIENTS_IN_RING_ELEMENT * vector_u_compression_factor(rank)) / 8
}

/// Total c1 (encoded vector u) size in bytes: `rank * c1_block_size`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn c1_size(rank: usize) -> usize {
    rank * c1_block_size(rank)
}

/// Total c2 (encoded ring element v) size in bytes: `(256 * dv)/8`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn c2_size(rank: usize) -> usize {
    (COEFFICIENTS_IN_RING_ELEMENT * vector_v_compression_factor(rank)) / 8
}

/// `eta1` CBD parameter: 3 for rank 2, 2 for ranks 3,4.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn eta1(rank: usize) -> usize {
    if rank == 2 {
        3
    } else {
        2
    }
}

/// `eta2` CBD parameter: always 2.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn eta2(rank: usize) -> usize {
    let _ = rank;
    2
}

/// PRF output size for eta1 CBD sampling: `64 * eta1`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn eta1_randomness_size(rank: usize) -> usize {
    64 * eta1(rank)
}

/// PRF output size for eta2 CBD sampling: `64 * eta2`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn eta2_randomness_size(rank: usize) -> usize {
    64 * eta2(rank)
}

/// Implicit-rejection hash input length: `32 + cpa_ciphertext_size`.
#[hax_lib::requires(rank == 2 || rank == 3 || rank == 4)]
pub const fn implicit_rejection_hash_input_size(rank: usize) -> usize {
    32 + cpa_ciphertext_size(rank)
}

#[allow(non_snake_case)]
pub mod hash_functions {
    #[hax_lib::opaque]
    pub fn G(input: &[u8]) -> [u8; 64] {
        hacspec_sha3::sha3_512(input)
    }

    pub const H_DIGEST_SIZE: usize = 32;

    #[hax_lib::opaque]
    pub fn H(input: &[u8]) -> [u8; H_DIGEST_SIZE] {
        hacspec_sha3::sha3_256(input)
    }

    #[hax_lib::opaque]
    pub fn PRF<const LEN: usize>(input: &[u8]) -> [u8; LEN] {
        hacspec_sha3::shake256::<LEN>(input)
    }

    #[hax_lib::opaque]
    pub fn XOF<const LEN: usize>(input: &[u8]) -> [u8; LEN] {
        hacspec_sha3::shake128::<LEN>(input)
    }

    #[hax_lib::opaque]
    pub fn J<const LEN: usize>(input: &[u8]) -> [u8; LEN] {
        hacspec_sha3::shake256::<LEN>(input)
    }
}

/// An ML-KEM field element:
/// - after reduction modulo FIELD_MODULUS, it is an integer in the range [0, FIELD_MODULUS - 1]
/// - it is represented as a u16
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[hax_lib::attributes]
pub struct FieldElement {
    #[refine(val < FIELD_MODULUS)]
    pub val: u16,
}

#[hax_lib::attributes]
impl FieldElement {
    #[hax_lib::requires(val < FIELD_MODULUS)]
    pub const fn new(val: u16) -> Self {
        Self { val }
    }

    /// Reduce an arbitrary `i16` (e.g. an impl-side coefficient or
    /// Montgomery-domain value) into a canonical `FieldElement` in
    /// [0, FIELD_MODULUS).  Used by the impl→spec lift functions
    /// (`Libcrux_ml_kem.Vector.to_spec_*_t`) to bridge the trait-layer
    /// `i16` representation to the spec-layer `FieldElement` form.
    pub const fn from_i16(v: i16) -> FieldElement {
        let q = FIELD_MODULUS as i32;
        let r = (((v as i32) % q + q) % q) as u16;
        FieldElement::new(r)
    }

    /// Addition in ℤ/q.
    pub fn add(self, other: FieldElement) -> FieldElement {
        FieldElement::new(((self.val as u32 + other.val as u32) % FIELD_MODULUS as u32) as u16)
    }

    /// Subtraction in ℤ/q.  Adding q avoids unsigned underflow before reducing.
    pub fn sub(self, other: FieldElement) -> FieldElement {
        FieldElement::new(
            ((self.val as u32 + FIELD_MODULUS as u32 - other.val as u32) % FIELD_MODULUS as u32)
                as u16,
        )
    }

    /// Multiplication in ℤ/q.
    pub fn mul(self, other: FieldElement) -> FieldElement {
        FieldElement::new(((self.val as u32 * other.val as u32) % FIELD_MODULUS as u32) as u16)
    }

    /// Additive inverse in ℤ/q.  `a.neg()` = q − a   (0 when a = 0).
    pub fn neg(self) -> FieldElement {
        FieldElement::new((FIELD_MODULUS - self.val) % FIELD_MODULUS)
    }
}

/// An ML-KEM polynomial ring element
pub type Polynomial = [FieldElement; 256];

/// An ML-KEM vector
pub type Vector<const RANK: usize> = [Polynomial; RANK];

/// Am ML-KEM matrix
pub type Matrix<const RANK: usize> = [Vector<RANK>; RANK];

/// Utility function to create an array of size `N` by applying a function `f` to each index.
#[hax_lib::fstar::replace(
    r#"
assume val createi
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
    : t_Array v_T v_N

assume val createi_lemma 
      (#v_T: Type0)
      (v_N: usize)
      (#v_F: Type0)
      (f: (x:usize{x <. v_N}) -> v_T)
      (i: usize{i <. v_N})
     : Lemma (Seq.index (createi #v_T v_N #v_F f) (v i) == f i)
       [SMTPat (Seq.index (createi #v_T v_N #v_F f) (v i))]
"#
)]
pub fn createi<T, const N: usize, F: Fn(usize) -> T>(f: F) -> [T; N] {
    core::array::from_fn(f)
}

pub type BitVector<const N: usize> = [bool; N];
