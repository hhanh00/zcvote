use orchard::vote::{Frontier, OrchardHash};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CandidateChoice {
    pub address: String,
    pub choice: String,
}

impl CandidateChoice {
    pub fn new(address: String, choice: String) -> Self {
        CandidateChoice {
            address,
            choice,
        }
    }
}

/// Details of an election, including metadata, candidates, and election parameters.
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Election {
    pub name: String,
    pub start_height: u32,
    pub end_height: u32,
    pub question: String,
    pub candidates: Vec<CandidateChoice>,
    pub signature_required: bool,
    pub cmx: OrchardHash,
    pub nf: OrchardHash,
    pub cmx_frontier: Option<Frontier>,
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
