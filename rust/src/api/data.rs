// These are FRB friendly structs

use struct_convert::Convert;

pub enum AppRole {
    Voter,
    Creator,
    Validator,
}

#[derive(Convert)]
#[convert(from = "zcash_vote2::data::CandidateChoice")]
#[convert(into = "zcash_vote2::data::CandidateChoice")]
pub struct CandidateChoice {
    pub address: String,
    pub choice: String,
}

/// Details of an election, including metadata, candidates, and election parameters.
pub struct Election {
    pub name: String,
    pub seed: Option<String>, // only available to the creator
    pub start_height: u32,
    pub end_height: u32,
    pub questions: Vec<Question>,
    pub signature_required: bool,
}

#[derive(Convert)]
#[convert(from = "zcash_vote2::data::Question")]
#[convert(into = "zcash_vote2::data::Question")]
pub struct Question {
    pub question: String,
    pub choices: Vec<CandidateChoice>,
}

impl From<AppRole> for zcash_vote2::data::AppRole {
    fn from(value: AppRole) -> Self {
        match value {
            AppRole::Voter => zcash_vote2::data::AppRole::Voter,
            AppRole::Creator => zcash_vote2::data::AppRole::Creator,
            AppRole::Validator => zcash_vote2::data::AppRole::Validator,
        }
    }
}

// Cannot use the macro Convert because of the extra fields in the
// into struct. (Convert supports extra fields in the from struct)
impl std::convert::From<zcash_vote2::data::Election> for Election {
    fn from(this: zcash_vote2::data::Election) -> Self {
        Election {
            name: this.name,
            seed: this.seed,
            start_height: this.start_height,
            end_height: this.end_height,
            questions: this.questions.into_iter().map(|a| a.into()).collect(),
            signature_required: this.signature_required,
        }
    }
}

impl std::convert::From<Election> for zcash_vote2::data::Election {
    fn from(val: Election) -> Self {
        let this = val;
        zcash_vote2::data::Election {
            name: this.name,
            seed: this.seed,
            start_height: this.start_height,
            end_height: this.end_height,
            questions: this.questions.into_iter().map(|a| a.into()).collect(),
            signature_required: this.signature_required,
            cmx: None,
            nf: None,
            cmx_frontier: None,
        }
    }
}

// impl From<Election> for zcash_vote2::data::Election {
//     fn from(value: Election) -> Self {
//         zcash_vote2::data::Election {
//             name: value.name,
//             seed: value.seed,
//             start_height: value.start_height,
//             end_height: value.end_height,
//             questions: value.questions.into_iter().map(|q| q.into()).collect(),
//             signature_required: value.signature_required,
//             ..Default::default()
//         }
//     }
// }

// impl From<Question> for zcash_vote2::data::Question {
//     fn from(value: Question) -> Self {
//         zcash_vote2::data::Question {
//             question: value.question,
//             choices: value.choices.into_iter().map(|c| c.into()).collect(),
//         }
//     }
// }

// impl From<CandidateChoice> for zcash_vote2::data::CandidateChoice {
//     fn from(value: CandidateChoice) -> Self {
//         zcash_vote2::data::CandidateChoice {
//             address: value.address,
//             choice: value.choice,
//         }
//     }
// }
