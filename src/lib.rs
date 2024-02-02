//! Generate random test data for bioinformatics

#![warn(missing_docs)]

/* std use */

/* crate use */
use rand::seq::SliceRandom;
use rand::SeedableRng;

/* project use */
#[cfg(feature = "derive")]
pub use biotest_derive as derive;

/* mod declaration */
pub mod constants;
pub mod error;
pub mod format;

/// Generate random generator
pub fn rand() -> rand::rngs::StdRng {
    rand::rngs::StdRng::from_seed(constants::SEED)
}

/// Write random text of length in output
pub fn text<W>(output: &mut W, rng: &mut rand::rngs::StdRng, length: usize) -> error::Result<()>
where
    W: std::io::Write,
{
    output.write_all(
        &(0..length)
            .map(|_| {
                *constants::ALPHABETS
                    .choose(rng)
                    .unwrap_or_else(|| unreachable!())
            })
            .collect::<Vec<u8>>(),
    )?;

    Ok(())
}

fn sequence<W>(output: &mut W, rng: &mut rand::rngs::StdRng, length: usize) -> error::Result<()>
where
    W: std::io::Write,
{
    output.write_all(
        &(0..length)
            .map(|_| {
                *constants::NUCLEOTIDES
                    .choose(rng)
                    .unwrap_or_else(|| unreachable!())
            })
            .collect::<Vec<u8>>(),
    )?;

    Ok(())
}

fn quality<W>(output: &mut W, rng: &mut rand::rngs::StdRng, length: usize) -> error::Result<()>
where
    W: std::io::Write,
{
    output.write_all(
        &(0..length)
            .map(|_| {
                *constants::PHRED33
                    .choose(rng)
                    .unwrap_or_else(|| unreachable!())
            })
            .collect::<Vec<u8>>(),
    )?;

    Ok(())
}
