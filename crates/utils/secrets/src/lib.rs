//! # Libcrux Secrets
//!
//! This crate implements classification and declassification operations over
//! machine integers and arrays/slices of machine integers
//!
//! To check your code for secret independence, you first identify all the secret
//! values in your code and swap their types to use secret integers:
//! - u8 -> U8, i16 -> I16 etc.
//! - [u8] -> U8, [i16; N] -> [I16; N], etc
//!
//! You should be able to run your code as before with no performance impact.
//!
//! Then you can turn on the feature `check-secret-independence` to check
//! whether your code obeys the secret independent coding discipline:
//! - does it branch on comparisons over secret values?
//! - does it access arrays on secret indices?
//! - does it use non-constant-time operations like division or modulus?
//!
//! To convince the typechecker, you will need to convert some public values to secret
//! using `.classify()` operations.
//!
//! In some cases, you may decide that a certain declassification of secret values to
//! public values is safe, and in this case you may use a `.declassify()` operation.
//! However, note that every use of `.declassify()` is at the responsibility of the
//! programmer and represents a potential side-channel leak
//!
//! ## Valgrind Integration
//!
//! When both the feature `check-secret-independence` and the cfg `valgrind_ct_test` are
//! enabled, this crate issues valgrind client requests for each classify and declassify
//! operation. Each `classify` operation will mark the memory as [`Undefined`] for valgrind
//! and each `declassify` as [`Defined`].
//!
//! Running the resulting binary under valgrind's memcheck
//! tool will flag any use of a value containing undefined bits which would result in
//! observable differences in program behavior[^observable], indicating a potential timing side channel.
//!
//! Example Usage:
//! ```shell
//! RUSTFLAGS='--cfg valgrind_ct_test' cargo build \
//!     --features check-secret-independence --profile release-debug
//! valgrind --tool=memcheck --error-exitcode=1 --track-origins=yes \
//!     target/release-debug/<BINARY>
//! ```
//!
//! [`Undefined`]: https://docs.rs/crabgrind/latest/crabgrind/memcheck/enum.MemState.html#variant.Undefined
//! [`Defined`]: https://docs.rs/crabgrind/latest/crabgrind/memcheck/enum.MemState.html#variant.Defined
//! [^observable]: > Checks on definedness only occur in three places: when a value is used to
//! generate a memory address, when control flow decision needs to be made, and when a system call is detected,
//! Memcheck checks definedness of parameters as required. <https://valgrind.org/docs/manual/mc-manual.html#mc-manual.value>
#![no_std]

mod traits;
pub use traits::*;
mod int;
pub use int::*;

pub mod mem_requests;
