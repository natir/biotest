//! Error struct of project biotest

/* crate use */
use thiserror;

/// Enum to manage error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    /// std::io::Error error
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
}

/// Alias of result
pub type Result<T> = core::result::Result<T, Error>;
