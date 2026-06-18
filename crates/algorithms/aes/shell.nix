{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-23.11.tar.gz") {} }:

let
  riscvPkgs = pkgs.pkgsCross.riscv64.buildPackages;
  armv7Pkgs = pkgs.pkgsCross.armv7l-hf-multiplatform.buildPackages;
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # The cross-compilers (gcc-riscv64-linux-gnu equivalent)
    riscvPkgs.gcc
    armv7Pkgs.gcc

    # QEMU for running the binaries
    qemu
    # Rustup or cargo if not already on the runner
    rustup
  ];

  shellHook = ''
    # RISC-V
    # Tell C-build scripts (cc-rs) which compiler to use for the RISC-V target
    export CC_riscv64gc_unknown_linux_gnu="riscv64-unknown-linux-gnu-gcc"
    export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER="riscv64-unknown-linux-gnu-gcc"
    export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUNNER="qemu-riscv64 -L ${pkgs.pkgsCross.riscv64.glibc}/riscv64-unknown-linux-gnu"

    # ARMv7
    export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER="armv7l-unknown-linux-gnueabihf-gcc"
    export CC_armv7_unknown_linux_gnueabihf="armv7l-unknown-linux-gnueabihf-gcc"
    export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUNNER="qemu-arm -L ${pkgs.pkgsCross.armv7l-hf-multiplatform.glibc}/arm-unknown-linux-gnueabihf"
  '';
}
