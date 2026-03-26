# AES-based AEADs

![pre-verification]

This crate implements AES-GCM 128 and 256, as well as AES-CCM 128 and 256.

It provides 
- a portable, bit-sliced implementation
- an x64 optimised implementation using AES-NI
- an Aarch64 optimised implementation using the AES instructions

## Testing on RISC-V

If you want to run tests on the `riscv64gc-unknown-linux-gnu` target you can either follow the instructions provided in the [rustc book](https://doc.rust-lang.org/rustc/platform-support/riscv64gc-unknown-linux-gnu.html), or use the pre-configured `nix` shell in this directory.

In both cases you will have to install the target if you're working on a non-RISC-V host:
```sh
rustup target add riscv64gc-unknown-linux-gnu
```

### Using `nix`
If you have `nix` installed, you can now run
```sh
nix-shell --run "cargo test --target riscv64gc-unknown-linux-gnu"
```
to run tests on `qemu-riscv64`.

### Manual Setup
Alternatively, you can install the [RISC-V GNU toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain) and [qemu RISC-V emulator](https://www.qemu.org/docs/master/system/target-riscv.html) using the method of your choice.

Then you can run `cargo test` as follows:
```sh
cargo test --config riscv.toml --target riscv64gc-unknown-linux-gnu
```
where `riscv.toml` contains the linker and runner configuration (exact binary names depending on your installation):
```toml
[target.riscv64gc-unknown-linux-gnu]
linker = "riscv64-linux-gnu-gcc"
runner = "qemu-riscv64-static -L /usr/riscv64-linux-gnu -cpu rv64"
```

[pre-verification]: ../../../.assets/pre_verification-orange.svg
