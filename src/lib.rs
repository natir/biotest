//! Generate random test data for bioinformatics
//!
//! There's a feature for every file format that can be generated:
//! - fasta
//! - fastq
//! - vcf

#![warn(missing_docs)]

/* std use */

/* crate use */
use rand::seq::SliceRandom;
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

/// Write a random sequence of length in output
fn sequence<W>(output: &mut W, rng: &mut rand::rngs::StdRng, length: usize) -> error::Result<()>
where
    W: std::io::Write,
{
    output.write_all(
        &(0..length)
            .map(|_| {
                constants::NUCLEOTIDES
                    .choose(rng)
                    .cloned()
                    .ok_or(create_unreachable!())
            })
            .collect::<error::Result<Vec<u8>>>()?,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    /* project use */
    use super::*;

    #[test]
    fn sequence_() -> error::Result<()> {
        let mut rng = rand();

        let mut output = Vec::new();
        sequence(&mut output, &mut rng, 20)?;

        assert_eq!(output, b"taTATgAAtCGCgtGTTAGT");

        Ok(())
    }
}
