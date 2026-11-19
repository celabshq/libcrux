//! `zeroize` benchmarks.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use libcrux_secrets::Zeroize;

fn bench_slices(c: &mut Criterion) {
    let mut group = c.benchmark_group("slice/u8");
    for len in [15, 16, 17, 32, 63, 64, 65, 128, 4096, 65536] {
        let mut bytes = vec![0xa5u8; len];
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_function(BenchmarkId::from_parameter(len), |b| {
            b.iter(|| bytes.as_mut_slice().zeroize());
        });
    }
    group.finish();
}

/// Benchmarks zeroizing `[$t; $n]` arrays through the `[T; N]` impl, so the
/// length is a compile-time constant at the call site.
///
/// No `black_box` is needed: the wipe uses volatile stores, which cannot be
/// elided.
macro_rules! bench_fixed {
    ($c:expr, $t:ty, $init:expr, [$($n:literal),*]) => {{
        let mut group = $c.benchmark_group(concat!("fixed/", stringify!($t)));
        $(
            let mut arr: [$t; $n] = [$init; $n];
            group.throughput(Throughput::Bytes(core::mem::size_of::<[$t; $n]>() as u64));
            group.bench_function(BenchmarkId::from_parameter($n), |b| {
                b.iter(|| arr.zeroize());
            });
        )*
        group.finish();
    }};
}

fn bench_fixed(c: &mut Criterion) {
    // Buffers, tags, keys, digests, Keccak rates (136 = SHAKE256, 168 =
    // SHAKE128) and SHA-2 final blocks.
    bench_fixed!(c, u8, 0xa5, [16, 28, 32, 48, 64, 128, 136, 168, 256]);
    // SHA-256 state and message schedule.
    bench_fixed!(c, u32, 0xa5a5_a5a5, [8, 16]);
    // SHA-512 state and message schedule, P-384 limbs, Keccak state.
    bench_fixed!(c, u64, 0xa5a5_a5a5_a5a5_a5a5, [6, 8, 16, 25]);
}

criterion_group!(benches, bench_slices, bench_fixed);
criterion_main!(benches);
