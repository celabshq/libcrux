# Secret-Independence Utilities

![pre-verification]

This crate provides utilities to facilitate a secret-independent programming discipline.

## `no_std` support

This crate supports `no_std` targets and is free of heap allocations.

[pre-verification]: ../../../.assets/pre_verification-orange.svg

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
