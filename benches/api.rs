use criterion::{criterion_group, criterion_main, Criterion};
use enc_rust::kem::*;

pub fn gen_key_benchmark_512(c: &mut Criterion) {
    c.bench_function("key_gen_bench_512", |b| {
        b.iter(|| generate_keypair_512(None))
    });
}

pub fn gen_key_benchmark_768(c: &mut Criterion) {
    c.bench_function("key_gen_bench_768", |b| {
        b.iter(|| generate_keypair_768(None))
    });
}

pub fn gen_key_benchmark_1024(c: &mut Criterion) {
    c.bench_function("key_gen_bench_1024", |b| {
        b.iter(|| generate_keypair_1024(None))
    });
}

pub fn encap_benchmark_512(c: &mut Criterion) {
    let (pk, _) = generate_keypair_512(None).unwrap();
    c.bench_function("encap_benchmark_512", |b| {
        b.iter(|| pk.encapsulate(None, None))
    });
}

pub fn encap_benchmark_768(c: &mut Criterion) {
    let (pk, _) = generate_keypair_768(None).unwrap();
    c.bench_function("encap_benchmark_768", |b| {
        b.iter(|| pk.encapsulate(None, None))
    });
}

pub fn encap_benchmark_1024(c: &mut Criterion) {
    let (pk, _) = generate_keypair_1024(None).unwrap();
    c.bench_function("encap_benchmark_1024", |b| {
        b.iter(|| pk.encapsulate(None, None))
    });
}

pub fn decap_benchmark_512(c: &mut Criterion) {
    let (pk, sk) = generate_keypair_512(None).unwrap();
    let (ciphertext_obj, _) = pk.encapsulate(None, None).unwrap();
    c.bench_function("decap_benchmark_512", |b| {
        b.iter(|| sk.decapsulate(ciphertext_obj.as_bytes()))
    });
}

pub fn decap_benchmark_768(c: &mut Criterion) {
    let (pk, sk) = generate_keypair_768(None).unwrap();
    let (ciphertext_obj, _) = pk.encapsulate(None, None).unwrap();
    c.bench_function("decap_benchmark_768", |b| {
        b.iter(|| sk.decapsulate(ciphertext_obj.as_bytes()))
    });
}

pub fn decap_benchmark_1024(c: &mut Criterion) {
    let (pk, sk) = generate_keypair_1024(None).unwrap();
    let (ciphertext_obj, _) = pk.encapsulate(None, None).unwrap();
    c.bench_function("decap_benchmark_1024", |b| {
        b.iter(|| sk.decapsulate(ciphertext_obj.as_bytes()))
    });
}

criterion_group!(
    benches,
    gen_key_benchmark_512,
    gen_key_benchmark_768,
    gen_key_benchmark_1024,
    encap_benchmark_512,
    encap_benchmark_768,
    encap_benchmark_1024,
    decap_benchmark_512,
    decap_benchmark_768,
    decap_benchmark_1024
);
criterion_main!(benches);
