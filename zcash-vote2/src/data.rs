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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct CandidateChoice {
    pub address: Option<String>,
    pub choice: String,
}

/// Details of an election, including metadata, candidates, and election parameters.
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
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
    use orchard::vote::OrchardHash;
    use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
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
        let task =
            tokio::spawn(async move { election.finalize(&mut *connection, tx).await.unwrap() });
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
        let mut e = task.await?;
        println!(
            "Finalized Election: {}",
            serde_json::to_string_pretty(&e).unwrap()
        );

        e.questions.clear();
        let expected = serde_json::from_str::<Election>(
            r#"
{
  "name": "jaja",
  "start_height": 2200000,
  "end_height": 2200100,
  "questions": [],
  "signature_required": false,
  "cmx": "b870f8e006bbb08dd5ff688386c4fba52148aa52840d857341d476751bcec835",
  "nf": "c9d8599cbd375bbcfa43d79b31813120eaae210baf6fc3570af25ab1e5245912",
  "cmx_frontier": {
    "position": 99,
    "leaf": "bb8fe205919617d739f08b1d5b0f036fb1669b68d41fbd4d411e93416933c004",
    "ommers": [
      "9a46a6bd7eeadbdf463fc6e6c53baa19df770aa1de82e135139e09c67640ad0f",
      "087d2e5053515e7c8d7be186213f13ce8368eb1247d484d5c7107cbb4f45441f",
      "c7413f4614cd64043abbab7cc1095c9bb104231cea89e2c3e0df83769556d030",
      "2111fc397753e5fd50ec74816df27d6ada7ed2a9ac3816aab2573c8fac794204",
      "806afbfeb45c64d4f2384c51eff30764b84599ae56a7ab3d4a46d9ce3aeab431",
      "c8097eb66364f6b23fe1423dae87d8c66dfe9227daa64b978f2f269e136d790f",
      "9c147ec100822a802b52f6ce75d75350b3658c97ccab8bfbd6e665e8f740d728",
      "4e14563df191a2a65b4b37113b5230680555051b22d74a8e1f1d706f90f3133b",
      "b3bbe4f993d18a0f4eb7f4174b1d8555ce3396855d04676f1ce4f06dda07371f",
      "4ef5bde9c6f0d76aeb9e27e93fba28c679dfcb991cbcb8395a2b57924cbd170e",
      "a3c02568acebf5ca1ec30d6a7d7cd217a47d6a1b8311bf9462a5f939c6b74307",
      "3ef9b30bae6122da1605bad6ec5d49b41d4d40caa96c1cf6302b66c5d2d10d39",
      "22ae2800cb93abe63b70c172de70362d9830e53800398884a7a64ff68ed99e0b",
      "187110d92672c24cedb0979cdfc917a6053b310d145c031c7292bb1d65b7661b",
      "3f98adbe364f148b0cc2042cafc6be1166fae39090ab4b354bfb6217b964453b",
      "63f8dbd10df936f1734973e0b3bd25f4ed440566c923085903f696bc6347ec0f",
      "2182163eac4061885a313568148dfae564e478066dcbe389a0ddb1ecb7f5dc34",
      "bd9dc0681918a3f3f9cd1f9e06aa1ad68927da63acc13b92a2578b2738a6d331",
      "ca2ced953b7fb95e3ba986333da9e69cd355223c929731094b6c2174c7638d2e",
      "55354b96b56f9e45aae1e0094d71ee248dabf668117778bdc3c19ca5331a4e1a",
      "7097b04c2aa045a0deffcaca41c5ac92e694466578f5909e72bb78d33310f705",
      "e81d6821ff813bd410867a3f22e8e5cb7ac5599a610af5c354eb392877362e01",
      "157de8567f7c4996b8c4fdc94938fd808c3b2a5ccb79d1a63858adaa9a6dd824",
      "fe1fce51cd6120c12c124695c4f98b275918fceae6eb209873ed73fe73775d0b",
      "1f91982912012669f74d0cfa1030ff37b152324e5b8346b3335a0aaeb63a0a2d",
      "5dec15f52af17da3931396183cbbbfbea7ed950714540aec06c645c754975522",
      "e8ae2ad91d463bab75ee941d33cc5817b613c63cda943a4c07f600591b088a25",
      "d53fdee371cef596766823f4a518a583b1158243afe89700f0da76da46d0060f",
      "15d2444cefe7914c9a61e829c730eceb216288fee825f6b3b6298f6f6b6bd62e",
      "4c57a617a0aa10ea7a83aa6b6b0ed685b6a3d9e5b8fd14f56cdc18021b12253f",
      "3fd4915c19bd831a7920be55d969b2ac23359e2559da77de2373f06ca014ba27",
      "87d063cd07ee4944222b7762840eb94c688bec743fa8bdf7715c8fe29f104c2a"
    ]
  }
}
        "#,
        )?;
        assert_eq!(e, expected);
        Ok(())
    }
}
