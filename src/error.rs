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

    #[cfg(feature = "fasta")]
    /// biotest::format::fasta::FastaBuilderError
    #[error(transparent)]
    FastaBuilderError(crate::format::fasta::FastaBuilderError),

    #[cfg(feature = "fastq")]
    /// crate::format::fastq::FastqBuilderError
    #[error(transparent)]
    FastqBuilderError(crate::format::fastq::FastqBuilderError),

    #[cfg(feature = "vcf")]
    /// crate::format::vcf::VcfBuilderError
    #[error(transparent)]
    VcfBuilderError(crate::format::vcf::VcfBuilderError),

    #[cfg(feature = "vcf")]
    /// crate::format::vcf::record::RecordBuilderError
    #[error(transparent)]
    VcfRecordBuilderError(crate::format::vcf::record::RecordBuilderError),

    #[cfg(feature = "vcf")]
    /// crate::format::vcf::header::HeaderBuilderError
    #[error(transparent)]
    VcfHeaderBuilderError(crate::format::vcf::header::HeaderBuilderError),
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
