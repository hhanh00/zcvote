use bech32::{Bech32m, Hrp};
use bip39::Mnemonic;
use orchard::{
    keys::{FullViewingKey, Scope, SpendingKey},
    vote::{Frontier, OrchardHash},
};
use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;

use crate::{
    IntoAnyhow, ProgressReporter, VoteError, VoteResult,
    db::{list_cmxs, list_nfs},
    trees::{compute_merkle_tree, make_nfs_ranges, orchard_hash},
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CandidateChoice {
    pub address: Option<String>,
    pub choice: String,
}

/// Details of an election, including metadata, candidates, and election parameters.
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Election {
    pub name: String,
    #[serde(skip)]
    pub seed: Option<String>, // only available to the creator
    pub start_height: u32,
    pub end_height: u32,
    pub questions: Vec<Question>,
    pub signature_required: bool,
    pub cmx: Option<OrchardHash>,
    pub nf: Option<OrchardHash>,
    pub cmx_frontier: Option<Frontier>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Question {
    pub question: String,
    pub choices: Vec<CandidateChoice>,
}

impl CandidateChoice {
    pub fn new(address: String, choice: String) -> Self {
        CandidateChoice {
            address: Some(address),
            choice,
        }
    }
}

impl Election {
    pub async fn finalize<PR: ProgressReporter + 'static>(
        self,
        connection: &mut SqliteConnection,
        progress_reporter: PR,
    ) -> VoteResult<Self> {
        let (seed,): (Option<String>,) =
            sqlx::query_as("SELECT seed FROM election_defs WHERE name = ?1")
                .bind(&self.name)
                .fetch_one(&mut *connection)
                .await?;
        let seed = seed.ok_or_else(|| anyhow::anyhow!("Missing seed phrase"))?;
        let hrp = Hrp::parse("zv").unwrap();
        // derive addresses as zip32/question #/answer #
        let questions = self
            .questions
            .into_iter()
            .enumerate()
            .map(|(i, q)| {
                let choices = q
                    .choices
                    .into_iter()
                    .enumerate()
                    .map(|(j, c)| {
                        let sk = derive_vote_key(&seed, i as u32, j as u32).unwrap();
                        let fvk = FullViewingKey::from(&sk);
                        let address = fvk.address_at(0u64, Scope::External);
                        let address =
                            bech32::encode::<Bech32m>(hrp, &address.to_raw_address_bytes())
                                .unwrap();
                        CandidateChoice {
                            address: Some(address),
                            choice: c.choice,
                        }
                    })
                    .collect();
                Question {
                    question: q.question,
                    choices,
                }
            })
            .collect();

        let start = self.start_height;
        let end = self.end_height;
        let mut nfs = list_nfs(&mut *connection, start, end).await?;
        let cmxs = list_cmxs(&mut *connection, start, end).await?;

        let (nf, cmx, frontier) = tokio::spawn(async move {
            make_nfs_ranges(&mut nfs);
            let (root, _) = compute_merkle_tree(&nfs, &[], &progress_reporter)
                .await
                .unwrap();
            let nf = orchard_hash(root);

            let end_pos = (cmxs.len() - 1) as u32;
            let (root, path) = compute_merkle_tree(&cmxs, &[end_pos], &progress_reporter)
                .await
                .unwrap();
            let frontier_path = &path[0];
            let frontier = Frontier::from(frontier_path);
            let cmx = orchard_hash(root);
            Ok::<_, VoteError>((nf, cmx, frontier))
        })
        .await
        .anyhow()??;

        let nf = Some(nf);
        let cmx = Some(cmx);
        let cmx_frontier = Some(frontier);

        Ok(Election {
            name: self.name,
            seed: None,
            start_height: self.start_height,
            end_height: self.end_height,
            questions,
            signature_required: self.signature_required,
            cmx,
            nf,
            cmx_frontier,
        })
    }
}

pub fn derive_vote_key(seed: &str, i: u32, j: u32) -> VoteResult<SpendingKey> {
    let seed = Mnemonic::parse(seed).anyhow()?;
    let seed = seed.to_seed("VoteOrchard");
    let sk = SpendingKey::from_zip32_seed(&seed, i, j).unwrap();
    Ok(sk)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use sqlx::{
        SqlitePool,
        sqlite::SqliteConnectOptions,
    };
    use tokio::sync::mpsc;

    use crate::data::Election;

    #[tokio::test]
    pub async fn test_finalize_election() -> Result<()> {
        let options = SqliteConnectOptions::new().filename("zcvote.db");
        let pool = SqlitePool::connect_with(options).await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        let (data,): (String,) =
            sqlx::query_as("SELECT definition FROM election_defs WHERE name = 'jaja'")
                .fetch_one(&mut *connection)
                .await?;
        let election: Election = serde_json::from_str(&data).unwrap();
        let (tx, mut rx) = mpsc::channel::<String>(1);
        let task = tokio::spawn(async move {
            election.finalize(&mut *connection, tx).await.unwrap()
        });
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
        let e = task.await?;
        println!("Finalized Election: {}", serde_json::to_string_pretty(&e).unwrap());
        Ok(())
    }
}
