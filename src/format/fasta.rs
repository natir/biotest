//! Fasta generation
//!
//! Usage:
//! ```
//! use biotest::Format as _; // import Format trait is required
//!
//! # fn main() -> Result<(), biotest::error::Error> {
//! let mut rng = biotest::rand(); // Create a random generator with a fixed seed
//!
//! let mut output = Vec::new();
//! let generator = biotest::Fasta::default();
//!
//! generator.record(&mut output, &mut rng)?; // Write one fasta record in output
//! generator.records(&mut output, &mut rng, 5)?; // Write five fasta records in output
//!
//! generator.create("test.fasta", &mut rng, 5)?; // Write five fasta record in "test.fasta"
//! # Ok(())
//! # }
//! ```
//!
//! Read generate follow this template
//! ```ignore
//! >{id_prefix}{id}{id_suffix} {comment_prefix}{comment}{comment_suffix}
//! {sequence}
//! ```
//!
//! Many think could be configurable with builder patern:
//! ```
//! use rand;
//! use rand::SeedableRng;
//! use biotest::Format;
//!
//! # fn main() -> Result<(), biotest::error::Error> {
//! let mut rng = rand::rngs::StdRng::from_entropy(); // Create a random generator with a 'random' seed
//!
//! let generator = biotest::Fasta::builder()
//!     .id(biotest::values::Alphabet::Lower) // Set alphabet use to generate sequence id
//!     .id_len(10) // Set length of id
//!     .id_prefix(b"prefix".to_vec()) // Set read id prefix
//!     .id_suffix(b"suffix".to_vec()) // Set read id prefix
//!     .comment(biotest::values::Alphabet::Upper) // Set alphabet use to generate sequence comment
//!     .comment_len(0) // If comment length is set to 0 prefix and suffix isn't write
//!     .comment_prefix(b"prefix".to_vec()) // Set read id prefix
//!     .comment_suffix(b"suffix".to_vec()) // Set read id prefix
//!     .build()?;
//!
//! generator.create("test.fasta", &mut rng, 5)?; // Write five fasta record in "test.fasta"
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

/// Struct to generate random fastq record
#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Fasta {
    /// Alphabet use for id generation
    #[builder(default = "values::Alphabet::Upper")]
    id: values::Alphabet,

    /// Length of id
    #[builder(default = "10")]
    id_len: usize,

    /// Id prefix
    #[builder(default = "b\"\".to_vec()")]
    id_prefix: Vec<u8>,

    /// Id suffix
    #[builder(default = "b\"\".to_vec()")]
    id_suffix: Vec<u8>,

    /// Alphapet use for comment generation
    #[builder(default = "values::Alphabet::Lower")]
    comment: values::Alphabet,

    /// Comment length
    #[builder(default = "20")]
    comment_len: usize,

    /// Comment prefix
    #[builder(default = "b\"\".to_vec()")]
    comment_prefix: Vec<u8>,

    /// Comment suffix
    #[builder(default = "b\"\".to_vec()")]
    comment_suffix: Vec<u8>,

    /// Alphabet use for sequence generation
    #[builder(default = "values::Nucleotides::Dna")]
    sequence: values::Nucleotides,

    /// Sequence length
    #[builder(default = "150")]
    sequence_len: usize,
}

impl Fasta {
    /// Create a FastaBuilder
    pub fn builder() -> FastaBuilder {
        FastaBuilder::default()
    }
}

impl core::default::Default for Fasta {
    fn default() -> Self {
        FastaBuilder::default().build().unwrap() // it's default no error
    }
}

impl format::Format for Fasta {
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
        // id
        output.write_all(&[b'>'])?;
        output.write_all(&self.id_prefix)?;
        output.write_all(&self.id.generate(rng, self.id_len)?)?;
        output.write_all(&self.id_suffix)?;
        if self.id_prefix.len() + self.id_len + self.id_suffix.len() != 0 {
            output.write_all(&[b' '])?;
        }

        // comment
        output.write_all(&self.comment_prefix)?;
        output.write_all(&self.comment.generate(rng, self.comment_len)?)?;
        output.write_all(&self.comment_suffix)?;
        output.write_all(b"\n")?;

        // sequence
        output.write_all(&self.sequence.generate(rng, self.sequence_len)?)?;

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

    const TRUTH: &[u8] = b">GSWNP zybhlatbbu
CGCgtGTTAGTTAagccAcggtAatGcTtgtaCgcAGgAtaTcgAAtTaT
>NJWIN icfqqisulj
CtCatGtctgCTGGTACtgTgcaaaagggGAGacAtgCtGCAAtTacCGt
>LHABR foipykuoug
CaTCctcTGgAActTgCGAcaAgaAAtaTCCcAgagggaCcttCcGcTTG
>GZCGR xtisataesr
TtCttAacGtTtAtGTgACAGCCaCGctGagattTGtgCttaAGggTcCT
>CKRPH yaldfvgykz
TCCACgTTTGagtGaGCatAGGACAAaacTaTTagagGtatAGCcTatTt
";

    #[test]
    fn record() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Fasta::builder()
            .id_len(5)
            .comment_len(10)
            .sequence_len(50)
            .build()?;

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, TRUTH.to_vec()[..68]);

        Ok(())
    }

    #[test]
    fn pre_suf_ix() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Fasta::builder()
            .id_len(5)
            .comment_len(10)
            .sequence_len(50)
            .id_prefix(b"id_prefix_".to_vec())
            .id_suffix(b"_id_suffix".to_vec())
            .comment_prefix(b"comment_prefix_".to_vec())
            .comment_suffix(b"_comment_suffix".to_vec())
            .build()?;

        generator.record(&mut output, &mut rng)?;

        assert_eq!(
            output,
            b">id_prefix_GSWNP_id_suffix comment_prefix_zybhlatbbu_comment_suffix
CGCgtGTTAGTTAagccAcggtAatGcTtgtaCgcAGgAtaTcgAAtTaT"
                .to_vec()
        );

        Ok(())
    }

    #[test]
    fn records() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Fasta::builder()
            .id_len(5)
            .comment_len(10)
            .sequence_len(50)
            .build()?;

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

        let generator = Fasta::builder()
            .id_len(5)
            .comment_len(10)
            .sequence_len(50)
            .build()?;

        generator.create(&temp_file, &mut rng, 5)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, TRUTH.to_vec());

        Ok(())
    }
}
