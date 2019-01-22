/// See https://bheisler.github.io/criterion.rs/book/getting_started.html to add more benchmarks.
#[macro_use]
extern crate criterion;

use criterion::Criterion;
use crypto::group::dummy::DummyRSA;
use crypto::proof::poe::{prove_poe, verify_poe};
use num::BigUint;

fn bench_poe() {
  let base = DummyRSA::elem_of(2);
  let exp = BigUint::from(20 as u8);
  let result = DummyRSA::elem_of(1_048_576);
  let proof = prove_poe::<DummyRSA>(&base, &exp, &result);
  verify_poe::<DummyRSA>(&base, &exp, &result, &proof);
}

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("test_poe", |b| b.iter(|| bench_poe()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);