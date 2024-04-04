//! Error struct of project biotest

/* crate use */
use thiserror;

/// Enum to manage error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
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
}

macro_rules! create_unreachable {
    () => {
        crate::error::Error::Unreachable {
            line: std::line!(),
            file: std::file!(),
        }
    };
}

/// Alias of result
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn unreachable_macro() {
        assert_matches::assert_matches!(
            create_unreachable!(),
            crate::error::Error::Unreachable {
                line: 40,
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
