//! Sequence generation
//!
//! Usage:
//! ```no_run
//! use biotest::Format as _; // import Format trait is required
//!
//! # fn main() -> Result<(), biotest::error::Error> {
//! let mut rng = biotest::rand(); // Create a random generator with a fixed seed
//!
//! let mut output = Vec::new();
//! let generator = biotest::Sequence::default();
//!
//! generator.record(&mut output, &mut rng)?; // Write one sequence record in output
//! generator.records(&mut output, &mut rng, 5)?; // Write five sequence records in output
//!
//! generator.create("test.sequence", &mut rng, 5)?; // Write five sequence record in "test.sequence"
//! # Ok(())
//! # }
//! ```
//!
//! File generate follow this template
//! ```no_compile
//! {sequence}
//! {sequence}
//! .
//! .
//! ```
//!
//! Many think could be configurable with builder patern:
//! ```no_run
//! use rand;
//! use rand::SeedableRng;
//! use biotest::Format;
//!
//! # fn main() -> Result<(), biotest::error::Error> {
//! let mut rng = rand::rngs::StdRng::from_entropy(); // Create a random generator with a 'random' seed
//!
//! let generator = biotest::Sequence::builder()
//!     .sequence_len(50) // Set sequence length
//!     .build()?;
//!
//! generator.create("test.sequence", &mut rng, 5)?; // Write five sequence record in "test.sequence"
//! # Ok(())
//! # }
//! ```

/* std use */

/* crates use */

/* projet use */
use crate::error;
use crate::format;
use crate::values;

use crate::values::Generate as _;

/// Struct to generate random DNA sequence
#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Sequence {
    /// Alphabet use for sequence generation
    #[builder(default = "values::Nucleotides::Dna")]
    sequence: values::Nucleotides,

    /// Sequence length
    #[builder(default = "150")]
    sequence_len: usize,

    /// Sequence weights
    #[builder(default = "vec![1; 0]")]
    sequence_weights: Vec<u8>,
}

impl Sequence {
    /// Create a SequenceBuilder
    pub fn builder() -> SequenceBuilder {
        SequenceBuilder::default()
    }
}

impl core::default::Default for Sequence {
    fn default() -> Self {
        SequenceBuilder::default().build().unwrap() // it's default no error
    }
}

impl format::Format for Sequence {
    fn header(
        &self,
        _output: &mut dyn std::io::Write,
        _rng: &mut rand::rngs::StdRng,
    ) -> error::Result<()> {
        Ok(())
    }

    fn record(
        &self,
        output: &mut dyn std::io::Write,
        rng: &mut rand::rngs::StdRng,
    ) -> error::Result<()> {
        // sequence
        if self.sequence_weights.is_empty() {
            output.write_all(&self.sequence.generate(rng, self.sequence_len)?)?;
        } else {
            output.write_all(&self.sequence.weighted(
                rng,
                self.sequence_len,
                &self.sequence_weights,
            )?)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    /* std use */
    use std::io::Read as _;

    /* project use */
    use super::format::Format as _;
    use super::*;

    const TRUTH: &[u8] = b"taTATgAAtCGCgtGTTAGTTAagccAcggtAatGcTtgtaCgcAGgAta
TcgAAtTaTaGaTggttGCtCatGtctgCTGGTACtgTgcaaaagggGAG
acAtgCtGCAAtTacCGtTAAcaGGtatTCaTCctcTGgAActTgCGAca
AgaAAtaTCCcAgagggaCcttCcGcTTGcgAACcTtCttAacGtTtAtG
TgACAGCCaCGctGagattTGtgCttaAGggTcCTGcGTAGCTGTCCACg
";

    const DEFAULT: &[u8] = b"taTATgAAtCGCgtGTTAGTTAagccAcggtAatGcTtgtaCgcAGgAtaTcgAAtTaTaGaTggttGCtCatGtctgCTGGTACtgTgcaaaagggGAGacAtgCtGCAAtTacCGtTAAcaGGtatTCaTCctcTGgAActTgCGAca";

    const WEIGHTED_TRUTH: &[u8] = b"GCGGTCGGGACTGATGAAGGTCCTGCTGGGTCCGATCCATGTTGAGCCGG";

    #[test]
    fn default() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Sequence::default();

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, DEFAULT);

        Ok(())
    }

    #[test]
    fn record() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Sequence::builder().sequence_len(50).build()?;

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, TRUTH.to_vec()[..50]);

        Ok(())
    }

    #[test]
    fn weigthed_record() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Sequence::builder()
            .sequence_len(50)
            .sequence_weights(vec![1, 2, 3, 4])
            .build()?;

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, WEIGHTED_TRUTH.to_vec());

        Ok(())
    }

    #[test]
    fn records() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Sequence::builder().sequence_len(50).build()?;

        generator.records(&mut output, &mut rng, 5)?;

        assert_eq!(output, TRUTH.to_vec());

        Ok(())
    }

    #[test]
    fn create() -> error::Result<()> {
        let mut rng = crate::rand();

        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        let temp_file = temp_path.join("tmp.sequence");

        let generator = Sequence::builder().sequence_len(50).build()?;

        generator.create(&temp_file, &mut rng, 5)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, TRUTH.to_vec());

        Ok(())
    }
}
