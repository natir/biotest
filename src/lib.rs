//! Generate random test data for bioinformatics
//!
//! There's a feature for every file format that can be generated:
//! - [`fasta`](module@format::fasta)
//! - [`fastq`](module@format::fastq)
//! - [`vcf`](module@format::vcf)

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
pub mod format;

/* reexport */
pub use format::Format;

#[cfg(feature = "fasta")]
pub use format::fasta::Fasta;

#[cfg(feature = "fastq")]
pub use format::fastq::Fastq;

#[cfg(feature = "vcf")]
pub use format::vcf::Vcf;

/// Create a random generator with [constants::SEED]
pub fn rand() -> rand::rngs::StdRng {
    rand::rngs::StdRng::from_seed(constants::SEED)
}

/// Create a random generator with a user seed
pub fn seeded_rand(seed: u64) -> rand::rngs::StdRng {
    rand::rngs::StdRng::seed_from_u64(seed)
}
