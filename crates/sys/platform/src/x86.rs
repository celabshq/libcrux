//! Obtain particular CPU features for x86/x86_64

#![allow(non_upper_case_globals)]

#[cfg(target_arch = "x86")]
use core::arch::x86::{__cpuid, __cpuid_count, _xgetbv, CpuidResult};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv, CpuidResult};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) enum Feature {
    mmx,
    sse,
    sse2,
    sse3,
    pclmulqdq,
    ssse3,
    fma,
    movbe,
    sse4_1,
    sse4_2,
    popcnt,
    aes,
    xsave,
    osxsave,
    avx,
    rdrand,
    sgx,
    bmi1,
    avx2,
    bmi2,
    avx512f,
    avx512dq,
    rdseed,
    adx,
    avx512ifma,
    avx512pf,
    avx512er,
    avx512cd,
    sha,
    avx512bw,
    avx512vl,
}

/// Check hardware [`Feature`] support.
pub(super) fn supported(feature: Feature) -> bool {
    init();
    let leaf1_ecx = FEATURE_BITS.leaf1_ecx();
    let leaf1_edx = FEATURE_BITS.leaf1_edx();
    let leaf7_ebx = FEATURE_BITS.leaf7_ebx();
    let xcr0 = FEATURE_BITS.xcr0();
    // Without the corresponding XCR0 bits the OS does not preserve the wide
    // registers across context switches and the encodings raise #UD.
    let os_avx = xcr0 & 0b110 == 0b110;
    let os_avx512 = xcr0 & 0b1110_0110 == 0b1110_0110;
    match feature {
        Feature::mmx => leaf1_edx & (1 << 23) != 0,
        Feature::sse => leaf1_edx & (1 << 25) != 0,
        Feature::sse2 => leaf1_edx & (1 << 26) != 0,
        Feature::sse3 => leaf1_ecx & (1 << 0) != 0,
        Feature::pclmulqdq => leaf1_ecx & (1 << 1) != 0,
        Feature::ssse3 => leaf1_ecx & (1 << 9) != 0,
        Feature::fma => leaf1_ecx & (1 << 12) != 0,
        Feature::movbe => leaf1_ecx & (1 << 22) != 0,
        Feature::sse4_1 => leaf1_ecx & (1 << 19) != 0,
        Feature::sse4_2 => leaf1_ecx & (1 << 20) != 0,
        Feature::popcnt => leaf1_ecx & (1 << 23) != 0,
        Feature::aes => leaf1_ecx & (1 << 25) != 0,
        Feature::xsave => leaf1_ecx & (1 << 26) != 0,
        Feature::osxsave => leaf1_ecx & (1 << 27) != 0,
        Feature::avx => {
            leaf1_ecx & (1 << 28) != 0
                && supported(Feature::xsave)
                && supported(Feature::osxsave)
                && os_avx
        }
        Feature::rdrand => leaf1_ecx & (1 << 30) != 0,
        Feature::sgx => leaf7_ebx & (1 << 2) != 0,
        Feature::bmi1 => leaf7_ebx & (1 << 3) != 0,
        Feature::avx2 => {
            leaf7_ebx & (1 << 5) != 0
                && supported(Feature::avx)
                && supported(Feature::bmi1)
                && supported(Feature::bmi2)
                && supported(Feature::fma)
                && supported(Feature::movbe)
        }
        Feature::bmi2 => leaf7_ebx & (1 << 8) != 0,
        Feature::avx512f => os_avx512 && leaf7_ebx & (1 << 16) != 0,
        Feature::avx512dq => os_avx512 && leaf7_ebx & (1 << 17) != 0,
        Feature::rdseed => leaf7_ebx & (1 << 18) != 0,
        Feature::adx => leaf7_ebx & (1 << 19) != 0,
        Feature::avx512ifma => os_avx512 && leaf7_ebx & (1 << 21) != 0,
        Feature::avx512pf => os_avx512 && leaf7_ebx & (1 << 26) != 0,
        Feature::avx512er => os_avx512 && leaf7_ebx & (1 << 27) != 0,
        Feature::avx512cd => os_avx512 && leaf7_ebx & (1 << 28) != 0,
        Feature::sha => leaf7_ebx & (1 << 29) != 0,
        Feature::avx512bw => os_avx512 && leaf7_ebx & (1 << 30) != 0,
        Feature::avx512vl => os_avx512 && leaf7_ebx & (1 << 31) != 0,
    }
}

/// The `cpuid` register words holding the feature bits tested by [`supported`].
///
/// Only the three registers + xcr0 we actually read are kept, so that the bit tests
/// can name the register they look at.
///
/// # Synchronization
///
/// `cpuid` output is a property of the machine, so every thread running
/// [`init`] writes the very same values here. Racing writers atomically overwrite
/// [`FEATURE_BITS`] with the same data.
struct FeatureBits {
    /// Whether the registers below have been written by [`init`] yet.
    initialized: AtomicBool,
    /// Leaf 1, register ECX.
    leaf1_ecx: AtomicU32,
    /// Leaf 1, register EDX.
    leaf1_edx: AtomicU32,
    /// Leaf 7 sub-leaf 0, register EBX.
    leaf7_ebx: AtomicU32,
    /// Low half of `XCR0`, naming the state the OS saves across context
    /// switches. Zero if `XGETBV` was not safe to execute.
    xcr0: AtomicU32,
}

impl FeatureBits {
    /// All registers zeroed, i.e. reporting no feature as supported.
    const fn uninit() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            leaf1_ecx: AtomicU32::new(0),
            leaf1_edx: AtomicU32::new(0),
            leaf7_ebx: AtomicU32::new(0),
            xcr0: AtomicU32::new(0),
        }
    }

    /// Whether the registers hold their final values.
    fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }

    /// Store the registers and mark them as initialized.
    fn publish(&self, leaf1: CpuidResult, leaf7: CpuidResult, xcr0: u32) {
        self.leaf1_ecx.store(leaf1.ecx, Ordering::Relaxed);
        self.leaf1_edx.store(leaf1.edx, Ordering::Relaxed);
        self.leaf7_ebx.store(leaf7.ebx, Ordering::Relaxed);
        self.xcr0.store(xcr0, Ordering::Relaxed);
        // This makes the preceding relaxed stores visible for any other
        // thread that observes a true from `is_initialized`.
        self.initialized.store(true, Ordering::Release);
    }

    // The reads below are `Relaxed`: callers reach them through `init`, whose
    // load-acquire on `initialized` provides the happens-before edge.

    fn leaf1_ecx(&self) -> u32 {
        self.leaf1_ecx.load(Ordering::Relaxed)
    }

    fn leaf1_edx(&self) -> u32 {
        self.leaf1_edx.load(Ordering::Relaxed)
    }

    fn leaf7_ebx(&self) -> u32 {
        self.leaf7_ebx.load(Ordering::Relaxed)
    }

    fn xcr0(&self) -> u32 {
        self.xcr0.load(Ordering::Relaxed)
    }
}

/// Feature bits of the running CPU, filled in by [`init`].
static FEATURE_BITS: FeatureBits = FeatureBits::uninit();

/// Initialize CPU detection.
#[inline(always)]
pub(super) fn init() {
    // Implementation partially based on:
    // https://github.com/rust-lang/rust/blob/e5b95097d9a14bdec7cd9101dde67ee3aad2578a/library/std_detect/src/detect/os/x86.rs#L27

    // No cpuid support on Intel SGX
    if cfg!(target_env = "sgx") {
        // We can save ourselves publishing anything here; on this target every
        // caller just immediately returns, `FEATURE_BITS` stays zeroed, and the bit
        // tests for the features will return false.
        return;
    }

    if FEATURE_BITS.is_initialized() {
        return;
    }

    // If `FEATURE_BITS.initialized` is false, multiple threads might call `init_slow`
    // at the same time. This is fine, as the stores to FEATURE_BITS are all atomic
    // and different threads will always write the same results.
    //
    // Put the slow path into an inline(never) function so we don't
    // bloat the code size at each usage site of init.
    #[inline(never)]
    fn init_slow() {
        /// Stand-in for a leaf the CPU does not implement; reports no feature.
        const UNSUPPORTED_LEAF: CpuidResult = CpuidResult {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
        };

        // EAX = 0: Queries the highest basic leaf this CPU implements.
        let CpuidResult {
            eax: max_basic_leaf,
            ..
        } = __cpuid(0);

        // EAX = 1, ECX = 0: Queries "Processor Info and Feature Bits";
        // Contains information about most x86 features.
        let leaf1 = if max_basic_leaf >= 1 {
            __cpuid(1)
        } else {
            UNSUPPORTED_LEAF
        };
        // EAX = 7, ECX = 0: Queries "Extended Features";
        // Contains information about bmi1, bmi2, and avx2 support.
        let leaf7 = if max_basic_leaf >= 7 {
            __cpuid_count(7, 0)
        } else {
            UNSUPPORTED_LEAF
        };

        // XGETBV is only executable once the OS has set CR4.OSXSAVE, which is
        // what CPUID.1:ECX[27] reports.
        let xcr0 = if leaf1.ecx & (1 << 27) != 0 {
            unsafe { _xgetbv(0) as u32 }
        } else {
            0
        };

        FEATURE_BITS.publish(leaf1, leaf7, xcr0);
    }

    init_slow();
}
