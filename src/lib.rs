//! Generate random test data for bioinformatics
//!
//! There's a feature for every file format that can be generated:
//! - fasta
//! - fastq
//! - vcf

#![warn(missing_docs)]

/* std use */

/* crate use */
use rand::SeedableRng;

/* project use */
//#[cfg(feature = "derive")]
//pub use biotest_derive as derive;

/* mod declaration */
pub mod constants;
pub mod values;
#[macro_use]
pub mod error;
mod format;

/* reexport */
#[cfg(feature = "fasta")]
pub use format::fasta;

#[cfg(feature = "fastq")]
pub use format::fastq;

#[cfg(feature = "vcf")]
pub use format::vcf;

/// Create a random generator with [constants::SEED]
pub fn rand() -> rand::rngs::StdRng {
    rand::rngs::StdRng::from_seed(constants::SEED)
}
