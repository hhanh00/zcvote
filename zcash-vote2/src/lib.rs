use tokio::sync::mpsc;
use tonic::transport::Channel;

pub mod error;
pub mod seed;
pub mod db;
pub mod legacy;
pub mod data;
pub mod download;
pub mod trees;
pub mod sync;
pub mod pb;
pub mod builder;

#[path = "cash.z.wallet.sdk.rpc.rs"]
pub mod lwd_prc;

pub type VoteError = crate::error::Error;
pub type VoteResult<T> = Result<T, VoteError>;
pub type Client = lwd_prc::compact_tx_streamer_client::CompactTxStreamerClient<Channel>;

pub trait ProgressReporter: Send + Sync {
    fn submit(&self, message: String) -> impl std::future::Future<Output = ()> + Send;
}

impl ProgressReporter for () {
    async fn submit(&self, _message: String) {
    }
}

impl ProgressReporter for mpsc::Sender<String> {
    async fn submit(&self, message: String) {
        let _ = self.send(message).await;
    }
}

#[macro_export]
macro_rules! tiu {
    ($x: expr) => {
        $x.try_into().unwrap()
    };
}

pub trait IntoAnyhow<T> {
    fn anyhow(self) -> Result<T, anyhow::Error>;
}

impl<T, E> IntoAnyhow<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn anyhow(self) -> Result<T, anyhow::Error> {
        self.map_err(anyhow::Error::new)
    }
}
