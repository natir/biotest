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
#[macro_use]
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
                constants::ALPHABETS
                    .choose(rng)
                    .cloned()
                    .ok_or(create_unreachable!())
            })
            .collect::<error::Result<Vec<u8>>>()?,
    )?;

    Ok(())
}

/// Write a random sequence of length in output
pub fn sequence<W>(output: &mut W, rng: &mut rand::rngs::StdRng, length: usize) -> error::Result<()>
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

/// Write a random quality of length in output
pub fn quality<W>(output: &mut W, rng: &mut rand::rngs::StdRng, length: usize) -> error::Result<()>
where
    W: std::io::Write,
{
    output.write_all(
        &(0..length)
            .map(|_| {
                constants::PHRED33
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
    fn text_() -> error::Result<()> {
        let mut rng = rand();

        let mut output = Vec::new();
        text(&mut output, &mut rng, 20)?;

        assert_eq!(output, b"oNi_PdzwC[tBTlDDl[MK");

        Ok(())
    }

    #[test]
    fn sequence_() -> error::Result<()> {
        let mut rng = rand();

        let mut output = Vec::new();
        sequence(&mut output, &mut rng, 20)?;

        assert_eq!(output, b"taTATgAAtCGCgtGTTAGT");

        Ok(())
    }

    #[test]
    fn quality_() -> error::Result<()> {
        let mut rng = rand();

        let mut output = Vec::new();
        quality(&mut output, &mut rng, 20)?;

        assert_eq!(output, b"A*C69HD.##3)(0'4E@H+");

        Ok(())
    }
}
