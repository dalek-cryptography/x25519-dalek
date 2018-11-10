// -*- mode: rust; -*-
//
// This file is part of x25519-dalek.
// Copyright (c) 2017 Isis Lovecruft
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>

//! x25519 Diffie-Hellman key exchange
//!
//! A pure-Rust implementation of x25519 elliptic curve Diffie-Hellman key
//! exchange as specified by Mike Hamburg and Adam Langley in
//! [RFC7748](https://tools.ietf.org/html/rfc7748).
//!
//! # Examples
//!
//! [![](https://raw.githubusercontent.com/isislovecruft/x25519-dalek/master/res/bubblesort-zines-secret-messages-cover.jpeg)](https://shop.bubblesort.io)
//!
//! "Secret Messages" cover image and [zine](https://shop.bubblesort.io/products/secret-messages-zine)
//! copyright © Amy Wibowo ([@sailorhg](https://twitter.com/sailorhg))
//!
//! Alice and Bob are two adorable kittens who have lost their mittens, and they
//! wish to be able to send secret messages to each other to coordinate finding
//! them, otherwise—if their caretaker cat finds out—they will surely be called
//! naughty kittens and be given no pie!
//!
//! But the two kittens are quite clever.  Even though their paws are still too
//! big and the rest of them is 90% fuzziness, these clever kittens have been
//! studying up on modern public key cryptography and have learned a nifty trick
//! called *elliptic curve Diffie-Hellman key exchange*.  With the right
//! incantations, the kittens will be able to secretly organise to find their
//! mittens, and then spend the rest of the afternoon nomming some yummy pie!
//!
//! First, Alice uses `x25519_dalek::generate_secret()` and
//! `x25519_dalek::generate_public()` to produce her secret and public keys:
//!
//! ```
//! extern crate x25519_dalek;
//! extern crate rand;
//!
//! # fn main() {
//! use x25519_dalek::SecretKey;
//! use x25519_dalek::PublicKey;
//! use rand::thread_rng;
//!
//! let mut alice_csprng = thread_rng();
//! let     alice_secret = SecretKey::generate(&mut alice_csprng);
//! let     alice_public = PublicKey::generate(&alice_secret);
//! # }
//! ```
//!
//! Bob does the same:
//!
//! ```
//! # extern crate x25519_dalek;
//! # extern crate rand;
//! #
//! # fn main() {
//! # use x25519_dalek::SecretKey;
//! # use x25519_dalek::PublicKey;
//! # use rand::thread_rng;
//! #
//! let mut bob_csprng = thread_rng();
//! let     bob_secret = SecretKey::generate(&mut bob_csprng);
//! let     bob_public = PublicKey::generate(&bob_secret);
//! # }
//! ```
//!
//! Alice meows across the room, telling `alice_public` to Bob, and Bob
//! loudly meows `bob_public` back to Alice.  Alice now computes her
//! shared secret with Bob by doing:
//!
//! ```
//! # extern crate x25519_dalek;
//! # extern crate rand;
//! #
//! # fn main() {
//! # use x25519_dalek::SecretKey;
//! # use x25519_dalek::PublicKey;
//! # use rand::thread_rng;
//! #
//! # let mut alice_csprng = thread_rng();
//! # let     alice_secret = SecretKey::generate(&mut alice_csprng);
//! # let     alice_public = PublicKey::generate(&alice_secret);
//! #
//! # let mut bob_csprng = thread_rng();
//! # let     bob_secret = SecretKey::generate(&mut bob_csprng);
//! # let     bob_public = PublicKey::generate(&bob_secret);
//! #
//! use x25519_dalek::diffie_hellman;
//!
//! let shared_secret = diffie_hellman(&alice_secret, &bob_public);
//! # }
//! ```
//!
//! Similarly, Bob computes the same shared secret by doing:
//!
//! ```
//! # extern crate x25519_dalek;
//! # extern crate rand;
//! #
//! # fn main() {
//! # use x25519_dalek::diffie_hellman;
//! # use x25519_dalek::SecretKey;
//! # use x25519_dalek::PublicKey;
//! # use rand::thread_rng;
//! #
//! # let mut alice_csprng = thread_rng();
//! # let     alice_secret = SecretKey::generate(&mut alice_csprng);
//! # let     alice_public = PublicKey::generate(&alice_secret);
//! #
//! # let mut bob_csprng = thread_rng();
//! # let     bob_secret = SecretKey::generate(&mut bob_csprng);
//! # let     bob_public = PublicKey::generate(&bob_secret);
//! #
//! let shared_secret = diffie_hellman(&bob_secret, &alice_public);
//! # }
//! ```
//!
//! Voilá!  Alice and Bob can now use their shared secret to encrypt their
//! meows, for example, by using it to generate a key and nonce for an
//! authenticated-encryption cipher.

#![no_std]
#![cfg_attr(feature = "bench", feature(test))]
#![deny(missing_docs)]

extern crate curve25519_dalek;

extern crate rand_core;

extern crate clear_on_drop;

#[cfg(test)]
extern crate rand;

mod x25519;

pub use x25519::*;
