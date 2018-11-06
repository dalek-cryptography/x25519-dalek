// -*- mode: rust; -*-
//
// This file is part of x25519-dalek.
// Copyright (c) 2017 Isis Lovecruft
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>

//! Benchmark the Diffie-Hellman operation.

#[macro_use]
extern crate criterion;
extern crate rand;
extern crate x25519_dalek;

use criterion::Criterion;

use rand::OsRng;

use x25519_dalek::SecretKey;
use x25519_dalek::PublicKey;
use x25519_dalek::diffie_hellman;

fn bench_diffie_hellman(c: &mut Criterion) {
    let mut csprng: OsRng = OsRng::new().unwrap();
    let alice_secret: SecretKey = SecretKey::generate(&mut csprng);
    let bob_secret: SecretKey = SecretKey::generate(&mut csprng);
    let bob_public: PublicKey = PublicKey::generate(&bob_secret).to_bytes();

    c.bench_function("diffie_hellman", move |b| {
        b.iter(||
               diffie_hellman(&alice_secret, &bob_public)
        )
    });
}

criterion_group!{
    name = x25519_benches;
    config = Criterion::default();
    targets =
        bench_diffie_hellman,
}
criterion_main!{
    x25519_benches,
}
