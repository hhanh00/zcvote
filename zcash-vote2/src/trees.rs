use halo2_proofs::pasta::Fp;
use incrementalmerkletree::Hashable;
use num_integer::Integer;
use orchard::{
    tree::MerkleHashOrchard,
    vote::{Frontier, OrchardHash},
};
use pasta_curves::group::ff::PrimeField;

use crate::{ProgressReporter, VoteError, VoteResult};

pub const DEPTH: usize = 32;
pub const EMPTY: u64 = 2;

pub struct AuthPathFp {
    position: u32,
    value: Fp,
    path: Vec<Fp>,
    p: u32,
}

impl From<AuthPathFp> for Frontier {
    fn from(value: AuthPathFp) -> Self {
        Self {
            position: value.position,
            leaf: orchard_hash(value.value),
            ommers: value.path.into_iter().map(orchard_hash).collect(),
        }
    }
}

pub fn orchard_hash(value: Fp) -> OrchardHash {
    OrchardHash(value.to_repr())
}

// "Expand" nfs to ranges nfs[i]+1, nfs[i+1]-1, ...
pub fn make_nfs_ranges(nfs: &mut Vec<Fp>) {
    let len = nfs.len();
    nfs.resize((len + 1) * 2, Fp::zero());
    for i in (0..len).rev() {
        nfs[i * 2 + 2] = nfs[i];
    }
    // the probability of overflow is negligeable
    for i in 0..len {
        nfs[i*2+1] = nfs[i*2+2];
        nfs[i*2+1] -= Fp::one();
        nfs[i*2+2] += Fp::one();
    }
    *nfs.first_mut().unwrap() = Fp::zero(); // min
    *nfs.last_mut().unwrap() = Fp::zero() - Fp::one(); // max
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
    leaves: &[Fp],
    pos_reqs: &[u32],
    progress_reporter: PR,
) -> VoteResult<(Fp, Vec<AuthPathFp>)> {
    let mut paths = pos_reqs
        .iter()
        .map(|p| {
            let pos = *p as usize;
            AuthPathFp {
                value: leaves[pos],
                position: *p,
                path: vec![Fp::default(); DEPTH],
                p: *p,
            }
        })
        .collect::<Vec<_>>();
    let mut er = Fp::from(EMPTY);
    let mut layer = Vec::with_capacity(leaves.len() + 2);
    for i in 0..DEPTH {
        if i == 0 {
            layer.extend(leaves);
            if layer.is_empty() {
                layer.push(er);
            }
            if !layer.len().is_multiple_of(2) {
                layer.push(er);
            }
        }
        progress_reporter
            .submit(format!("Processing Layer #{i} with {} nodes", layer.len()))
            .await;

        for path in paths.iter_mut() {
            let idx = path.p;
            // copy the ommer to the merkle path
            if idx.is_multiple_of(2) {
                // use right node
                path.path[i] = layer[idx as usize + 1];
            } else {
                // use left node
                path.path[i] = layer[idx as usize - 1];
            }
            path.p /= 2;
        }

        let pairs = layer.len() / 2;
        let mut next_layer = Vec::with_capacity(pairs + 2);

        for j in 0..pairs {
            let h = cmx_hash(i as u8, layer[j * 2], layer[j * 2 + 1]);
            next_layer.push(h);
        }

        er = cmx_hash(i as u8, er, er);
        if !next_layer.len().is_multiple_of(2) {
            next_layer.push(er);
        }

        std::mem::swap(&mut layer, &mut next_layer);
    }

    let root = layer[0];
    Ok((root, paths))
}

fn cmx_hash(level: u8, left: Fp, right: Fp) -> Fp {
    let left = MerkleHashOrchard::from_base(left);
    let right = MerkleHashOrchard::from_base(right);
    let h = MerkleHashOrchard::combine(incrementalmerkletree::Altitude::from(level), &left, &right);
    h.inner()
}

#[cfg(test)]
mod tests {
    use pasta_curves::Fp;
    use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
    use tokio::sync::mpsc;

    use crate::{
        db::{list_cmxs, list_nfs}, tiu, trees::{compute_merkle_tree, make_nfs_ranges, orchard_hash}, VoteResult
    };
    use pasta_curves::group::ff::PrimeField;

    #[tokio::test]
    pub async fn test_nf_tree() -> VoteResult<()> {
        let options = SqliteConnectOptions::new().filename("zcvote.db");
        let pool = SqlitePool::connect_with(options).await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        let mut nfs = list_nfs(&mut connection, 2_200_000, 2_200_100).await?;
        make_nfs_ranges(&mut nfs);
        let (tx, mut rx) = mpsc::channel::<String>(1);
        tokio::spawn(async move {
            let (root, _) = compute_merkle_tree(&nfs, &[], tx).await.unwrap();
            let exp_root = Fp::from_repr(tiu!(hex::decode("c9d8599cbd375bbcfa43d79b31813120eaae210baf6fc3570af25ab1e5245912").unwrap())).unwrap();
            assert_eq!(root, exp_root);

            let root = orchard_hash(root);
            println!("ROOT: {}", hex::encode(&root.0));
        });
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
        Ok(())
    }

    #[tokio::test]
    pub async fn test_cmx_tree() -> VoteResult<()> {
        let options = SqliteConnectOptions::new().filename("zcvote.db");
        let pool = SqlitePool::connect_with(options).await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        let cmxs = list_cmxs(&mut connection, 2_200_000, 2_200_100).await?;
        let (tx, mut rx) = mpsc::channel::<String>(1);
        tokio::spawn(async move {
            let (root, _) = compute_merkle_tree(&cmxs, &[], tx).await.unwrap();
            let exp_root = Fp::from_repr(tiu!(hex::decode("b870f8e006bbb08dd5ff688386c4fba52148aa52840d857341d476751bcec835").unwrap())).unwrap();
            assert_eq!(root, exp_root);

            let root = orchard_hash(root);
            println!("ROOT: {}", hex::encode(&root.0));
        });
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
        Ok(())
    }
}
