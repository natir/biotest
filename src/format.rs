//! Format data generation

/* std use */

/* crates use */

/* module declaration */
#[cfg(feature = "fasta")]
pub mod fasta;

#[cfg(feature = "fastq")]
pub mod fastq;

#[cfg(feature = "vcf")]
pub mod vcf;

#[cfg(feature = "sequence")]
pub mod sequence;

#[cfg(feature = "quality")]
pub mod quality;

/* projet use */
use crate::error;

/// Trait of Format
pub trait Format {
    /// Write header of format in output
    fn header(
        &self,
        output: &mut dyn std::io::Write,
        rng: &mut rand::rngs::StdRng,
    ) -> error::Result<()>;

    /// Write a record in output
    fn record(
        &self,
        output: &mut dyn std::io::Write,
        rng: &mut rand::rngs::StdRng,
    ) -> error::Result<()>;

    /// Write multiple record in output
    fn records(
        &self,
        output: &mut dyn std::io::Write,
        rng: &mut rand::rngs::StdRng,
        number: usize,
    ) -> error::Result<()> {
        for _ in 0..number {
            self.record(output, rng)?;
            output.write_all(&[b'\n'])?;
        }
        Ok(())
    }

    /// Create a file at path with header and multiple records
    fn create<P>(&self, path: P, rng: &mut rand::rngs::StdRng, number: usize) -> error::Result<()>
    where
        P: core::convert::AsRef<std::path::Path>,
    {
        let mut output = std::io::BufWriter::new(std::fs::File::create(path)?);

        self.header(&mut output, rng)?;
        self.records(&mut output, rng, number)?;

        Ok(())
    }
}
