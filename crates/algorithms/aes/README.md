# AES-based AEADs

![pre-verification]

This crate implements AES-GCM 128 and 256, as well as AES-CCM 128 and 256.

It provides 
- a portable, bit-sliced implementation
- an x64 optimised implementation using AES-NI
- an Aarch64 optimised implementation using the AES instructions

## Testing on RISC-V & ARMv7

If you want to run tests on the `riscv64gc-unknown-linux-gnu` or `armv7-unknown-linux-gnueabihf` targets you can either follow the instructions provided in the rustc book (for [RISC-V](https://doc.rust-lang.org/rustc/platform-support/riscv64gc-unknown-linux-gnu.html) and for [ARMv7](https://doc.rust-lang.org/rustc/platform-support/armv7-unknown-linux-gnueabi.html)), or use the pre-configured `nix` shell in this directory.

In both cases you will have to install the necessary target if you're working on a host different from the target:
```sh
rustup target add riscv64gc-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
```

### Using `nix`
If you have `nix` installed, you can now run
```sh
nix-shell --run "cargo test --target <target>"
```
to run tests on `qemu-riscv64` or `qemu-arm` depending on choice of `<target>`.

### Manual Setup
Alternatively, you can install 
- for RISC-V: the [RISC-V GNU toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain) and [qemu RISC-V emulator](https://www.qemu.org/docs/master/system/target-riscv.html), or
- for ARMv7: the [ARM GNU toolchain](http://developer.arm.com/downloads/-/gnu-a) and [qemu ARM emulator](https://www.qemu.org/docs/master/system/target-arm.html)
 using the method of your choice.

Then you can run `cargo test` as follows:
```sh
cargo test --config config.toml --target <target>
```
where `config.toml` contains the linker and runner configuration (exact binaries/paths depending on your installation and may have to be adapted).

[pre-verification]: ../../../.assets/pre_verification-orange.svg
