use thiserror::Error;
use tracing::subscriber::SetGlobalDefaultError;

/// A [`Result`] alias where the [`Err`] case is [`crate::Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// The Errors that may occur when using the crate.
#[derive(Error, Debug)]
#[error("{0}")]
pub enum Error {
    Reqwest(#[from] reqwest::Error),
    Tracing(#[from] SetGlobalDefaultError),
    Tokio(#[from] tokio::task::JoinError),
}
