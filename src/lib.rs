//! Generate random test data for bioinformatics
//!
//! There's a feature for every file format that can be generated:
//! - [`fasta`](module@format::fasta)
//! - [`fastq`](module@format::fastq)
//! - [`vcf`](module@format::vcf)
//! - [`sequence`](module@format::sequence)

#![warn(missing_docs)]

/* std use */

/* crate use */
use rand::SeedableRng;

/* project use */

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

#[cfg(feature = "sequence")]
pub use format::sequence::Sequence;

#[cfg(feature = "quality")]
pub use format::quality::Quality;

#[cfg(feature = "cigar")]
pub use format::cigar::Cigar;

/// Create a random generator with [constants::SEED]
pub fn rand() -> rand::rngs::StdRng {
    rand::rngs::StdRng::from_seed(constants::SEED)
}

/// Create a random generator with a user seed
pub fn seeded_rand(seed: u64) -> rand::rngs::StdRng {
    rand::rngs::StdRng::seed_from_u64(seed)
}

#[cfg(test)]
mod tests {
    /* crate use */
    use rand::Rng;

    /* local use */
    use super::*;

    #[test]
    fn check_rand() {
        let mut rng = rand();

        assert_eq!(rng.gen::<u8>(), 27);
    }

    #[test]
    fn check_seeded_rand() {
        let mut rng = seeded_rand(42);

        assert_eq!(rng.gen::<u8>(), 162);
    }
}
