# rust-stark

This crate reimplements portions of Nockchain's STARK prover in Rust.
It mirrors the algorithms defined in `hoon/common/stark/prover.hoon`
and `hoon/common/ztd/eight.hoon`. Currently only a single column trace
is supported. The trace is interpolated to a polynomial, extended to a
larger domain and hashed into a Merkle tree using Blake3.

An example is provided demonstrating proof generation and verification
with the original Hoon code. The Rust example computes a proof, builds
the `verify-root.hoon` generator, and executes it via the Nock
interpreter. The script simply prints the proof root, showing that the
Hoon environment can consume data produced by the Rust prover. The same
root can also be checked via the `verify_proof` helper.
