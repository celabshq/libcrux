# Secret-Independence Utilities

![pre-verification]

This crate provides utilities to facilitate a secret-independent programming discipline.

## `no_std` support

This crate supports `no_std` targets and is free of heap allocations.

[pre-verification]: ../../../.assets/pre_verification-orange.svg

## Zeroization

The `Zeroize` trait and the `Zeroizing` wrapper are the entry points for
wiping secrets in libcrux. `Zeroize` is implemented for all machine integers,
SIMD register types, secret integers, and arrays and slices of these.
`Zeroizing<T>` wipes the wrapped value on drop.

Values are overwritten with plain writes, followed by the optimisation barrier
of the [`zeroize`](https://crates.io/crates/zeroize) crate, which keeps the
compiler from removing the writes. In order to reduce dependencies and ensure
completeness for libcrux use cases, this will be replaced by a libcrux
implementation behind the same API.

Zeroization is explicit and does not cover copies made by the compiler yet
(moves, by-value returns, register spills). Scrubbing the stack after secret
operations is planned in future.

Code extracted to C (via Eurydice) does not zeroize yet: wiping is a no-op
there, and there are no destructors that wipe on drop.

`Zeroize` is libcrux's own trait, not the `zeroize` crate's `Zeroize`. Code
that imports both needs qualified paths.

The `disable-zeroization` feature turns all zeroization into a no-op. Do not
enable it in builds that need FIPS 140-3 compliance. Because Cargo unifies
features, any crate in the dependency graph that enables it disables
zeroization for the whole build.

## Valgrind Integration

This crate integrates with valgrind when the `--cfg valgrind_ct_test` and the cargo
feature `check-secret-independence` are enabled. Then, each conversion of a value
into a `Secret` value (i.e. `From/Into`, `classify`, `classify_ref`, `classify_ref_mut`, `classify_mut_slice`)
will mark the memory as `Undefined` for valgrind.

Enabling only the `cfg`, but not the feature, will result in some valgrind requests being issued in the swap/select implementations.

Every `declassify`, `declassify_ref`, `declassify_ref_mut` and `declassify_mut_slice`
will mark the memory as `Defined` for valgrind. When running the resulting binary under
valgrind, it will track the uses of `Undefined` memory on a bit level and report
any operations operating on undefined bits that would result in observable behavior
differences. Concretely, these are:
> Checks on definedness only occur in three places: when a value is used to generate a memory address, when control flow decision needs to be made, and when a system call is detected, Memcheck checks definedness of parameters as required. [valgrind.org](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.value)

Note that this does not check for other side channels, such as variable time instructions (e.g., div) or cache side channels.

Running a binary compiled with `--cfg valgrind_ct_test` not under valgrind **will panic**.
