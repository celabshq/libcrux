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

[pre-verification]: https://img.shields.io/badge/pre_verification-orange.svg?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz48IS0tIFVwbG9hZGVkIHRvOiBTVkcgUmVwbywgd3d3LnN2Z3JlcG8uY29tLCBHZW5lcmF0b3I6IFNWRyBSZXBvIE1peGVyIFRvb2xzIC0tPg0KPHN2ZyB3aWR0aD0iODAwcHgiIGhlaWdodD0iODAwcHgiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4NCjxwYXRoIGQ9Ik05IDEySDE1TTIwIDEyQzIwIDE2LjQ2MTEgMTQuNTQgMTkuNjkzNyAxMi42NDE0IDIwLjY4M0MxMi40MzYxIDIwLjc5IDEyLjMzMzQgMjAuODQzNSAxMi4xOTEgMjAuODcxMkMxMi4wOCAyMC44OTI4IDExLjkyIDIwLjg5MjggMTEuODA5IDIwLjg3MTJDMTEuNjY2NiAyMC44NDM1IDExLjU2MzkgMjAuNzkgMTEuMzU4NiAyMC42ODNDOS40NTk5NiAxOS42OTM3IDQgMTYuNDYxMSA0IDEyVjguMjE3NTlDNCA3LjQxODA4IDQgNy4wMTgzMyA0LjEzMDc2IDYuNjc0N0M0LjI0NjI3IDYuMzcxMTMgNC40MzM5OCA2LjEwMDI3IDQuNjc3NjYgNS44ODU1MkM0Ljk1MzUgNS42NDI0MyA1LjMyNzggNS41MDIwNyA2LjA3NjQgNS4yMjEzNEwxMS40MzgyIDMuMjEwNjdDMTEuNjQ2MSAzLjEzMjcxIDExLjc1IDMuMDkzNzMgMTEuODU3IDMuMDc4MjdDMTEuOTUxOCAzLjA2NDU3IDEyLjA0ODIgMy4wNjQ1NyAxMi4xNDMgMy4wNzgyN0MxMi4yNSAzLjA5MzczIDEyLjM1MzkgMy4xMzI3MSAxMi41NjE4IDMuMjEwNjdMMTcuOTIzNiA1LjIyMTM0QzE4LjY3MjIgNS41MDIwNyAxOS4wNDY1IDUuNjQyNDMgMTkuMzIyMyA1Ljg4NTUyQzE5LjU2NiA2LjEwMDI3IDE5Ljc1MzcgNi4zNzExMyAxOS44NjkyIDYuNjc0N0MyMCA3LjAxODMzIDIwIDcuNDE4MDggMjAgOC4yMTc1OVYxMloiIHN0cm9rZT0iIzAwMDAwMCIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiLz4NCjwvc3ZnPg==
