//! CIGAR

/* std use */

/* crates use */
use rand::Rng as _;

/* project use */
use crate::error;
use crate::format;
use crate::values;

use crate::values::Generate as _;

/// Struct to generate cigar record
#[derive(typed_builder::TypedBuilder)]
pub struct Cigar {
    /// Cigar length
    #[builder(default = 20)]
    length: u64,

    /// Cigar Alphabet
    #[builder(default = values::Cigar::Sam)]
    alphabet: values::Cigar,

    /// Cigar weights
    #[builder(default = vec![1; 0])]
    alphabet_weights: Vec<u8>,
}

impl core::default::Default for Cigar {
    fn default() -> Self {
        Cigar::builder().build()
    }
}

impl format::Format for Cigar {
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
        let mut len = 0;
        while len < self.length {
            let size = if self.length - len > 1 {
                rng.gen_range::<usize, core::ops::Range<usize>>(1..(self.length - len) as usize)
            } else {
                1
            };

            let letter = if self.alphabet_weights.is_empty() {
                self.alphabet.generate(rng, 1)
            } else {
                self.alphabet.weighted(rng, 1, &self.alphabet_weights)
            };

            output.write_all(size.to_string().as_bytes())?;
            output.write_all(&letter?)?;

            len += size as u64;
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

    const TRUTH: &[u8] = b"12F20I1M13D1D1I1D1R
23R17I1F7M1D1I
33I15M1M1M
44F1D2I2R1R
16F29D2D1I1F1F
";

    const DEFAULT: &[u8] = b"5P8D1S1N1S1N1S1D1D";

    const WEIGHTED_TRUTH: &[u8] = b"12R20D1D13R1I1D1M1F";

    #[test]
    fn default() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Cigar::default();

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, DEFAULT);

        Ok(())
    }

    #[test]
    fn record() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Cigar::builder()
            .length(50)
            .alphabet(values::Cigar::Gff)
            .build();

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, TRUTH.to_vec()[..19]);

        Ok(())
    }

    #[test]
    fn weigthed_record() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Cigar::builder()
            .length(50)
            .alphabet(values::Cigar::Gff)
            .alphabet_weights(vec![1, 2, 3, 4, 5])
            .build();

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, WEIGHTED_TRUTH.to_vec());

        Ok(())
    }

    #[test]
    fn records() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Cigar::builder()
            .length(50)
            .alphabet(values::Cigar::Gff)
            .build();

        generator.records(&mut output, &mut rng, 5)?;

        assert_eq!(output, TRUTH.to_vec());

        Ok(())
    }

    #[test]
    fn create() -> error::Result<()> {
        let mut rng = crate::rand();

        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        let temp_file = temp_path.join("tmp.fasta");

        let generator = Cigar::builder()
            .length(50)
            .alphabet(values::Cigar::Gff)
            .build();

        generator.create(&temp_file, &mut rng, 5)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, TRUTH.to_vec());

        Ok(())
    }
}
