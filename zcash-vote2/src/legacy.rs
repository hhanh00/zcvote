use bech32::{Bech32m, Hrp};
use orchard::{Address, vote::{Frontier, OrchardHash}};
use serde::{Deserialize, Serialize};

use crate::{IntoAnyhow, VoteResult};

pub struct VoteAddress(pub Address);


impl VoteAddress {
    pub fn decode(s: &str) -> VoteResult<Self> {
        let vote_hrp = Hrp::parse_unchecked("zvote");
        let (hrp, data) = bech32::decode(s).anyhow()?;
        if hrp != vote_hrp {
            return Err(anyhow::anyhow!("Invalid Address (incorrect prefix)").into());
        }
        if data.len() != 43 {
            return Err(anyhow::anyhow!("Invalid Address (incorrect length)").into());
        }
        let address = Address::from_raw_address_bytes(&data.try_into().unwrap());
        if address.is_none().into() {
            return Err(anyhow::anyhow!("Invalid Address (invalid data)").into());
        }
        let address = address.unwrap();
        Ok(VoteAddress(address))
    }

    pub fn encode(&self) -> String {
        let vote_hrp = Hrp::parse_unchecked("zvote");
        let address = &self.0;
        let address = address.to_raw_address_bytes();

        bech32::encode::<Bech32m>(vote_hrp, &address).unwrap()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LegacyCandidateChoice {
    pub address: String,
    pub choice: String,
}

impl LegacyCandidateChoice {
    pub fn new(address: Address, choice: &str) -> Self {
        LegacyCandidateChoice {
            address: VoteAddress(address).encode(),
            choice: choice.to_string(),
        }
    }
}

/// Details of an election, including metadata, candidates, and election parameters.
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct LegacyElection {
    pub name: String,
    pub start_height: u32,
    pub end_height: u32,
    pub question: String,
    pub candidates: Vec<LegacyCandidateChoice>,
    pub signature_required: bool,
    pub cmx: OrchardHash,
    pub nf: OrchardHash,
    pub cmx_frontier: Option<Frontier>,
}
