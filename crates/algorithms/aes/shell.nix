{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-23.11.tar.gz") {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # The cross-compiler (gcc-riscv64-linux-gnu equivalent)
    pkgsCross.riscv64.buildPackages.gcc
    # QEMU for running the binaries
    qemu
    # Rustup or cargo if not already on the runner
    rustup
  ];

  shellHook = ''
    # Tell C-build scripts (cc-rs) which compiler to use for the RISC-V target
    export CC_riscv64gc_unknown_linux_gnu="riscv64-unknown-linux-gnu-gcc"
    export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER="riscv64-unknown-linux-gnu-gcc"
    export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUNNER="qemu-riscv64 -L ${pkgs.pkgsCross.riscv64.glibc}/riscv64-unknown-linux-gnu"
  '';
}
