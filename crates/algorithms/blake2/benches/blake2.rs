use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use libcrux_blake2::{Blake2bBuilder, Blake2sBuilder};

pub fn randombytes(n: usize) -> Vec<u8> {
    use rand::{rngs::SysRng, TryRng};

    let mut bytes = vec![0u8; n];
    SysRng.try_fill_bytes(&mut bytes).unwrap();
    bytes
}

pub fn fmt(x: usize) -> String {
    let base = (x as f64).log(1024f64).floor() as usize;
    let suffix = ["", "KB", "MB", "GB"];
    format!("{} {}", x >> (10 * base), suffix[base])
}

fn benchmarks(c: &mut Criterion) {
    const PAYLOAD_SIZES: [usize; 7] = [32, 64, 128, 256, 512, 1024, 1024 * 1024 * 10];

    let mut group = c.benchmark_group("blake2");

    for payload_size in PAYLOAD_SIZES.iter() {
        group.throughput(Throughput::Bytes(*payload_size as u64));

        let payload = randombytes(*payload_size);

        group.bench_function(BenchmarkId::new("b", fmt(*payload_size)), |b| {
            b.iter(|| {
                let mut digest = [0; 32];
                let mut hasher = Blake2bBuilder::new_unkeyed().build_const_digest_len();
                hasher.update(&payload).unwrap();
                hasher.finalize(&mut digest);
            })
        });
    }

    for payload_size in PAYLOAD_SIZES.iter() {
        group.throughput(Throughput::Bytes(*payload_size as u64));

        let payload = randombytes(*payload_size);

        group.bench_function(BenchmarkId::new("s", fmt(*payload_size)), |b| {
            b.iter(|| {
                let mut digest = [0; 32];
                let mut hasher = Blake2sBuilder::new_unkeyed().build_const_digest_len();
                hasher.update(&payload).unwrap();
                hasher.finalize(&mut digest);
            })
        });
    }
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
