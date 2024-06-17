//! Error struct of project biotest

/* crate use */
use thiserror;

/// Enum to manage error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    /// WeightedDistribution is larger than value
    #[error("Weight array is larger than value array")]
    WeightArrayLargerValueArray,

    /// unreachable
    #[error("Unreachable error from file {file} in line {line}")]
    Unreachable {
        /// line number
        line: u32,
        /// file name
        file: &'static str,
    },

    /// std::io::Error error
    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    /// rand::distributions::weighted::WeightedError
    #[error(transparent)]
    RandWeightedError(#[from] rand::distributions::weighted::WeightedError),
}

macro_rules! create_unreachable {
    () => {
        crate::error::Error::Unreachable {
            line: std::line!(),
            file: std::file!(),
        }
    };
}

pub(crate) use create_unreachable;

/// Alias of result
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    /* local use */
    use super::*;

    #[test]
    fn unreachable_macro() {
        assert_matches::assert_matches!(
            create_unreachable!(),
            crate::error::Error::Unreachable {
                line: 53,
                #[cfg(target_family = "windows")]
                file: "src\\error.rs",
                #[cfg(target_family = "unix")]
                file: "src/error.rs",
                #[cfg(target_family = "wasm")]
                file: "src/error.rs",
            }
        );
    }
}
