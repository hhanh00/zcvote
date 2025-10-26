use orchard::vote::{Frontier, OrchardHash};
use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;

use crate::ProgressReporter;

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
    pub async fn finalize<PR: ProgressReporter>(self, connection: &mut SqliteConnection, progress_reporter: PR) -> Self {
        let questions = vec![];
        let cmx = None;
        let nf = None;
        let cmx_frontier = None;

        Election {
            name: self.name,
            seed: None,
            start_height: self.start_height,
            end_height: self.end_height,
            questions,
            signature_required: self.signature_required,
            cmx,
            nf,
            cmx_frontier,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use anyhow::Result;

    #[test]
    pub fn load_election() -> Result<()> {
        let f = File::open("test-election.json")?;
        let e = serde_json::from_reader::<_, super::Election>(&f)?;
        println!("{:?}", &e);
        Ok(())
    }
}
