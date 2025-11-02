// These are FRB friendly structs

use anyhow::Result;
use flutter_rust_bridge::frb;
use sqlx::SqliteConnection;
use struct_convert::Convert;
use tracing::info;
use zcash_vote2::download::{connect, download_blocks};
pub use zcash_vote2::legacy::LegacyElection;

use crate::{api::app::App, frb_generated::StreamSink};

#[derive(Convert)]
#[convert(from = "zcash_vote2::data::CandidateChoice")]
#[frb(dart_metadata = ("freezed"))]
pub struct CandidateChoice {
    pub choice: String,
}

/// Details of an election, including metadata, candidates, and election parameters.
#[frb(dart_metadata = ("freezed"))]
pub struct Election {
    pub name: String,
    pub seed: Option<String>, // only available to the creator
    pub start_height: u32,
    pub end_height: u32,
    pub questions: Vec<Question>,
    pub signature_required: bool,
    pub locked: bool,
}

#[derive(Convert)]
#[convert(from = "zcash_vote2::data::Question")]
#[convert(into = "zcash_vote2::data::Question")]
#[frb(dart_metadata = ("freezed"))]
pub struct Question {
    pub question: String,
    pub choices: Vec<CandidateChoice>,
}

impl std::convert::From<CandidateChoice> for zcash_vote2::data::CandidateChoice {
    fn from(val: CandidateChoice) -> zcash_vote2::data::CandidateChoice {
        zcash_vote2::data::CandidateChoice {
            address: None,
            choice: val.choice,
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
            locked: this.cmx_frontier.is_some(),
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

#[frb(opaque)]
pub struct OldElection {
    pub(crate) inner: LegacyElection,
}

impl OldElection {
    #[frb(sync)]
    pub fn start(&self) -> u32 { self.inner.start_height }
    #[frb(sync)]
    pub fn end(&self) -> u32 { self.inner.end_height }

    pub async fn download_blocks(&self, app: &App, url: &str, progress_reporter: StreamSink<String>) -> Result<()> {
        let mut connection = app.db_connect().await?;
        let start = self.inner.start_height;
        let end = self.inner.end_height;
        info!("download blocks {} {}", start, end);
        let mut client = connect(url.to_string()).await?;

        download_blocks(&mut client, &mut *connection, start, end, &progress_reporter).await?;

        Ok(())
    }
}
