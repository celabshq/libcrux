//! Obtain particular CPU features for x86/x86_64

#![allow(non_upper_case_globals)]

#[cfg(target_arch = "x86")]
use core::arch::x86::{CpuidResult, __cpuid, __cpuid_count};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{CpuidResult, __cpuid, __cpuid_count};
use core::sync::atomic::{AtomicBool, Ordering};

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
    let cpu_id_0 = unsafe { CPU_ID[0] };
    let cpu_id_1 = unsafe { CPU_ID[1] };
    match feature {
        Feature::mmx => cpu_id_0.edx & (1 << 23) != 0,
        Feature::sse => cpu_id_0.edx & (1 << 25) != 0,
        Feature::sse2 => cpu_id_0.edx & (1 << 26) != 0,
        Feature::sse3 => cpu_id_0.ecx & (1 << 0) != 0,
        Feature::pclmulqdq => cpu_id_0.ecx & (1 << 1) != 0,
        Feature::ssse3 => cpu_id_0.ecx & (1 << 9) != 0,
        Feature::fma => cpu_id_0.ecx & (1 << 12) != 0,
        Feature::movbe => cpu_id_0.ecx & (1 << 22) != 0,
        Feature::sse4_1 => cpu_id_0.ecx & (1 << 19) != 0,
        Feature::sse4_2 => cpu_id_0.ecx & (1 << 20) != 0,
        Feature::popcnt => cpu_id_0.ecx & (1 << 23) != 0,
        Feature::aes => cpu_id_0.ecx & (1 << 25) != 0,
        Feature::xsave => cpu_id_0.ecx & (1 << 26) != 0,
        Feature::osxsave => cpu_id_0.ecx & (1 << 27) != 0,
        Feature::avx => {
            cpu_id_0.ecx & (1 << 28) != 0
                && supported(Feature::xsave)
                && supported(Feature::osxsave)
        }
        Feature::rdrand => cpu_id_0.ecx & (1 << 30) != 0,
        Feature::sgx => cpu_id_1.ebx & (1 << 2) != 0,
        Feature::bmi1 => cpu_id_1.ebx & (1 << 3) != 0,
        Feature::avx2 => {
            cpu_id_1.ebx & (1 << 5) != 0
                && supported(Feature::bmi1)
                && supported(Feature::bmi2)
                && supported(Feature::fma)
                && supported(Feature::movbe)
        }
        Feature::bmi2 => cpu_id_1.ebx & (1 << 8) != 0,
        Feature::avx512f => cpu_id_1.ebx & (1 << 16) != 0,
        Feature::avx512dq => cpu_id_1.ebx & (1 << 17) != 0,
        Feature::rdseed => cpu_id_1.ebx & (1 << 18) != 0,
        Feature::adx => cpu_id_1.ebx & (1 << 19) != 0,
        Feature::avx512ifma => cpu_id_1.ebx & (1 << 21) != 0,
        Feature::avx512pf => cpu_id_1.ebx & (1 << 26) != 0,
        Feature::avx512er => cpu_id_1.ebx & (1 << 27) != 0,
        Feature::avx512cd => cpu_id_1.ebx & (1 << 28) != 0,
        Feature::sha => cpu_id_1.ebx & (1 << 29) != 0,
        Feature::avx512bw => cpu_id_1.ebx & (1 << 30) != 0,
        Feature::avx512vl => cpu_id_1.ebx & (1 << 31) != 0,
    }
}

// Guarded by INITIALIZED; always use load-acquire and store-release access to guarantee atomicity.
static mut CPU_ID: [CpuidResult; 2] = [
    CpuidResult {
        eax: 0,
        ebx: 0,
        ecx: 0,
        edx: 0,
    },
    CpuidResult {
        eax: 0,
        ebx: 0,
        ecx: 0,
        edx: 0,
    },
];
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize CPU detection.
#[inline(always)]
pub(super) fn init() {
    if INITIALIZED.load(Ordering::Acquire) {
        return;
    }

    // XXX: https://github.com/rust-lang/rust/issues/101346
    #[inline(never)]
    unsafe fn cpuid(leaf: u32) -> CpuidResult {
        __cpuid(leaf)
    }

    #[inline(never)]
    unsafe fn cpuid_count(leaf: u32, sub_leaf: u32) -> CpuidResult {
        __cpuid_count(leaf, sub_leaf)
    }

    // XXX[no_std]: no good way to do this in no_std
    // std::panic::catch_unwind(|| {
    // If there's no CPU ID because we're in SGX or whatever other reason,
    // we'll consider the hw detection as initialized but always return false.
    unsafe {
        CPU_ID = [cpuid(1), cpuid_count(7, 0)];
    }
    // });
    INITIALIZED.store(true, Ordering::Release);
}
