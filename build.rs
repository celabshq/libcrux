//! A simple build script that ensure that the correct cfg flags are set.

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LIBCRUX_DISABLE_SIMD128");
    println!("cargo:rerun-if-env-changed=LIBCRUX_DISABLE_SIMD256");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    let disable_simd128 = match env::var("LIBCRUX_DISABLE_SIMD128") {
        Ok(s) => s == "1" || s == "y" || s == "Y",
        Err(_) => false,
    };

    let disable_simd256 = match env::var("LIBCRUX_DISABLE_SIMD256") {
        Ok(s) => s == "1" || s == "y" || s == "Y",
        Err(_) => false,
    };

    // We don't enable any features when cross compiling for now.
    let cross = target != host;

    if !cross
        && libcrux_platform::simd128_support()
        && target_arch != "x86"
        && !target_arch.contains("wasm")
        && !disable_simd128
    {
        println!("cargo:rustc-cfg=simd128");
    }
    if !cross
        && libcrux_platform::simd256_support()
        && target_arch != "x86"
        && !target_arch.contains("wasm")
        && !disable_simd256
    {
        println!("cargo:rustc-cfg=simd256");
    }
    if !cross && libcrux_platform::bmi2_adx_support() {
        println!("cargo:rustc-cfg=bmi2");
        println!("cargo:rustc-cfg=adx");
    }
    if !cross && libcrux_platform::aes_ni_support() && target_arch == "x86_64" {
        println!("cargo:rustc-cfg=aes_ni");
    }
}
