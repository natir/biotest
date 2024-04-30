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

    /// biotest::format::fasta::FastaBuilderError
    #[cfg(feature = "fasta")]
    #[error(transparent)]
    FastaBuilderError(#[from] crate::format::fasta::FastaBuilderError),

    /// crate::format::fastq::FastqBuilderError
    #[cfg(feature = "fastq")]
    #[error(transparent)]
    FastqBuilderError(#[from] crate::format::fastq::FastqBuilderError),

    /// crate::format::vcf::VcfBuilderError
    #[cfg(feature = "vcf")]
    #[error(transparent)]
    VcfBuilderError(#[from] crate::format::vcf::VcfBuilderError),

    /// crate::format::vcf::record::RecordBuilderError
    #[cfg(feature = "vcf")]
    #[error(transparent)]
    VcfRecordBuilderError(#[from] crate::format::vcf::record::RecordBuilderError),

    /// crate::format::vcf::header::HeaderBuilderError
    #[cfg(feature = "vcf")]
    #[error(transparent)]
    VcfHeaderBuilderError(#[from] crate::format::vcf::header::HeaderBuilderError),
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
    #[test]
    fn unreachable_macro() {
        assert_matches::assert_matches!(
            create_unreachable!(),
            crate::error::Error::Unreachable {
                line: 67,
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
