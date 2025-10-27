use halo2_proofs::pasta::Fp;
use orchard::vote::OrchardHash;
use sqlx::SqliteConnection;
use num_integer::Integer;

use crate::{ProgressReporter, VoteError, VoteResult};

pub type AuthPath = Vec<OrchardHash>;

// "Expand" nfs to ranges nfs[i]+1, nfs[i+1]-1, ...
pub fn make_nfs_ranges(nfs: &mut Vec<Fp>) {
    nfs.resize(nfs.len() * 2, Fp::zero());
    for i in (0..nfs.len()).rev() {
        nfs[i * 2] = nfs[i] + Fp::one(); // the probability of overflow is negligeable
        nfs[i * 2 + 1] = nfs[i] - Fp::one();
    }
    *nfs.first_mut().unwrap() = Fp::zero(); // min
    *nfs.last_mut().unwrap() = Fp::one() - Fp::one(); // max
}

// Given a list of nullifiers and a sorted list of ranges
// flatten as [a_i, b_i]
// always return the lower bound a_i, positions must be even
pub fn locate_nullifiers(nfs: &[Fp], ranges: &[Fp]) -> VoteResult<Vec<u32>> {
    let mut poss = vec![];
    for nf in nfs {
        let pos = match ranges.binary_search(nf) {
            Ok(p) => p.prev_multiple_of(&2),
            Err(p) => {
                if p.is_multiple_of(2) {
                    // fallen between ranges, nf is not in any range
                    return Err(VoteError::DuplicateNullifier);
                }
                p - 1
            }
        };
        poss.push(pos as u32);
    }
    Ok(poss)
}

pub async fn compute_merkle_tree<PR: ProgressReporter>(
    connection: &mut SqliteConnection,
    query: String,
    pos_reqs: Vec<u32>,
    progress_reporter: PR,
) -> VoteResult<(OrchardHash, Vec<AuthPath>)> {
    Ok((OrchardHash([0u8; 32]), vec![]))
}
