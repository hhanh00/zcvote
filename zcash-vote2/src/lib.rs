pub mod error;
pub mod db;
pub mod data;

pub type VoteError = crate::error::Error;
pub type VoteResult<T> = Result<T, VoteError>;
