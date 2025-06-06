//! Rust reimplementation of Nockchain's STARK prover.
//!
//! This mirrors portions of `hoon/common/stark/prover.hoon`.
//! In particular, `compute-table-polys` and `compute-lde` are
//! implemented following lines 115-139 of `ztd/eight.hoon`.

use serde::{Deserialize, Serialize};
use winterfell::{
    crypto::{hashers::Blake3_256, Digest, ElementHasher, MerkleTree},
    math::{fields::f64::BaseElement, polynom, get_power_series, StarkField},
};

pub type Felt = BaseElement;

/// STARK proof consisting of the merkle root of the extended trace.
#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    /// Commitment to the extended trace (see `bp-build-merk-heap` in
    /// `ztd/three.hoon` lines 1907-1914).
    pub root: [u8; 32],
}

/// Generate a STARK proof for a single-column trace.
/// This roughly corresponds to `generate-proof` in `prover.hoon`.
pub fn generate_proof(trace: &[Felt]) -> Proof {
    // Compute interpolation polynomial (see `compute-table-polys` in
    // `ztd/eight.hoon` lines 115-123).
    let n = trace.len();
    assert!(n.is_power_of_two());
    let g = Felt::get_root_of_unity(n.trailing_zeros());
    let xs = get_power_series::<Felt>(g, n);
    let poly = polynom::interpolate(&xs, trace, false);

    // Low degree extend the column (see `compute-lde` in `ztd/eight.hoon` lines
    // 124-137). We simply double the domain size.
    let domain = get_power_series::<Felt>(g, n * 2);
    let lde = polynom::eval_many(&poly, &domain);

    // Commit to rows using Blake3 merkle tree; mirrors `bp-build-merk-heap`.
    let leaves: Vec<_> = lde
        .iter()
        .map(|&v| Blake3_256::<Felt>::hash_elements(&[v]))
        .collect();
    let tree = MerkleTree::<Blake3_256<Felt>>::new(leaves).expect("tree");
    Proof { root: tree.root().as_bytes() }
}

/// Verify a STARK proof by recomputing the commitment.
/// This mirrors `verify-root` in `verify-root.hoon` which simply checks the
/// commitment root.
pub fn verify_proof(trace: &[Felt], proof: &Proof) -> bool {
    generate_proof(trace).root == proof.root
}
