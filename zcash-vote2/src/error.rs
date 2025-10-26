use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Duplicate Nullifier")]
    DuplicateNullifier,
    #[error(transparent)]
    DbError(#[from] sqlx::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
