#![cfg_attr(not(feature = "check-secret-independence"), allow(dead_code))]
#![cfg_attr(not(valgrind_ct_test), allow(unused_variables))]
//! Helper functions for constant time tests using valgrind.
//!
//! When libcrux-secrets is compiled with the `valgrind_ct_test` cfg,
//! these functions will mark the respective memory as [`Undefined`] ([`ct_classify`])
//! or [`Defined`] ([`ct_declassify`]). Running the binary compiled with the cfg
//! under valgrind's memcheck tool will check for any code paths
//! dependent on the classified memory.
//!
//! When the `cfg`` is not enabled, these operations are no-ops.
//!
//! [`Undefined`]: https://docs.rs/crabgrind/latest/crabgrind/memcheck/enum.MemState.html#variant.Undefined
//! [`Defined`]: https://docs.rs/crabgrind/latest/crabgrind/memcheck/enum.MemState.html#variant.Defined

/// Mark memory as secret.
///
/// No-op if `valgrind_ct_test` cfg is not enabled.
#[inline(always)]
pub fn ct_classify<T: ?Sized>(val: &T) {
    #[cfg(valgrind_ct_test)]
    {
        use core::ffi::c_void;
        crabgrind::memcheck::mark_memory(
            val as *const _ as *const c_void,
            size_of_val(val),
            crabgrind::memcheck::MemState::Undefined,
        )
        .expect(
            "libcrux-secrets compiled with '--cfg valgrind_ct_test' must be run under valgrind",
        );
    }
    // Otherwise, no-op
}

/// Declassify secret memory.
///
/// No-op if `valgrind_ct_test` cfg is not enabled.
#[inline(always)]
pub fn ct_declassify<T: ?Sized>(val: &T) {
    #[cfg(valgrind_ct_test)]
    {
        use core::ffi::c_void;
        // If we expect the error, we get false positives after some uses
        // of ct_declassify.
        // If a valgrind_ct_test binary is not run under valgrind, the expect
        // in the classify should fail, so it is okay here to ignore the error
        let _ = crabgrind::memcheck::mark_memory(
            val as *const _ as *const c_void,
            size_of_val(val),
            crabgrind::memcheck::MemState::Defined,
        );
    }
    // Otherwise, no-op
}
