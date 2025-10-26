use halo2_proofs::pasta::Fp;
use orchard::vote::OrchardHash;
use sqlx::SqliteConnection;

use crate::{ProgressReporter, VoteResult};

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

pub async fn compute_merkle_tree<PR: ProgressReporter>(
    connection: &mut SqliteConnection,
    query: String,
    paths_req: Vec<Fp>,
    progress_reporter: PR,
) -> VoteResult<(OrchardHash, Vec<AuthPath>)> {
    Ok((OrchardHash([0u8; 32]), vec![]))
}
