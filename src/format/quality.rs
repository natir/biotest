//! quality generation
//!
//! Usage:
//! ```no_run
//! use biotest::Format as _; // import Format trait is required
//!
//! # fn main() -> Result<(), biotest::error::Error> {
//! let mut rng = biotest::rand(); // Create a random generator with a fixed seed
//!
//! let mut output = Vec::new();
//! let generator = biotest::Quality::default();
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
//! let generator = biotest::Quality::builder()
//!     .quality_len(50) // Set quality length
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
pub struct Quality {
    /// Alphabet use for sequence generation
    #[builder(default = "values::Quality::Illumina")]
    quality: values::Quality,

    /// quality length
    #[builder(default = "150")]
    quality_len: usize,

    /// quality weights
    #[builder(default = "vec![1; 0]")]
    quality_weights: Vec<u8>,
}

impl Quality {
    /// Create a QualityBuilder
    pub fn builder() -> QualityBuilder {
        QualityBuilder::default()
    }
}

impl core::default::Default for Quality {
    fn default() -> Self {
        QualityBuilder::default().build().unwrap() // it's default no error
    }
}

impl format::Format for Quality {
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
        // quality
        if self.quality_weights.is_empty() {
            output.write_all(&self.quality.generate(rng, self.quality_len)?)?;
        } else {
            output.write_all(&self.quality.weighted(
                rng,
                self.quality_len,
                &self.quality_weights,
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

    const TRUTH: &[u8] = b"=DI3E\"?#?3(\'5FAI2C+,\"E*=)#=G4A%H53A1).<FA7\'G0##2EI
!A6,=C8.>!83@9-660D.5-E.F/$*::>A2A>)\'=0B<$E/&411+!
)AF,E;7.8.3GF2%\"%:4%#<399BE%$8900(08#,.;&2*@3,\"\"<)
79HH127*A+:%7,(<2H3F*!)H#BH<3?=@/-%%&<CH38\"G63H!!B
+\':<>3#.EFG@@D\'*98<-:,1+F?>\"?(<C,IA+\'3@\"<%=3>B4-?C
";

    const DEFAULT: &[u8] = b"=DI3E\"?#?3(\'5FAI2C+,\"E*=)#=G4A%H53A1).<FA7\'G0##2EI!A6,=C8.>!83@9-660D.5-E.F/$*::>A2A>)\'=0B<$E/&411+!)AF,E;7.8.3GF2%\"%:4%#<399BE%$8900(08#,.;&2*@3,\"\"<)";

    const WEIGHTED_TRUTH: &[u8] = b"&$&&%$%&&!$%&!$&##&&%$#$&#%&&&$$$&!$$$!%&%%&#%#$&&";

    #[test]
    fn default() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Quality::default();

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, DEFAULT);

        Ok(())
    }

    #[test]
    fn record() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Quality::builder().quality_len(50).build()?;

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, TRUTH.to_vec()[..50]);

        Ok(())
    }

    #[test]
    fn weigthed_record() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Quality::builder()
            .quality_len(50)
            .quality_weights(vec![1, 0, 3, 4, 5, 6])
            .build()?;

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, WEIGHTED_TRUTH.to_vec());

        Ok(())
    }

    #[test]
    fn records() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Quality::builder().quality_len(50).build()?;

        generator.records(&mut output, &mut rng, 5)?;

        assert_eq!(output, TRUTH.to_vec());

        Ok(())
    }

    #[test]
    fn create() -> error::Result<()> {
        let mut rng = crate::rand();

        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        let temp_file = temp_path.join("tmp.quality");

        let generator = Quality::builder().quality_len(50).build()?;

        generator.create(&temp_file, &mut rng, 5)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, TRUTH.to_vec());

        Ok(())
    }
}
