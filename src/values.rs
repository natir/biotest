//! Declarations of many possible values

/* std use */
use rand::seq::SliceRandom;

/* crates use */

/* projet use */
use crate::constants;
use crate::error;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "tools", derive(clap::ValueEnum))]
/// Differente generic ascii alphabet
pub enum Alphabet {
    /// Any visible ascii character
    Visible,
    /// Upper case latin alphabet
    Upper,
    /// Lower case latin alphapet
    Lower,
    /// Ascii character between A-z
    A2z,
}

impl core::convert::AsRef<[u8]> for Alphabet {
    fn as_ref(&self) -> &[u8] {
        match self {
            Alphabet::Visible => &constants::ASCII_VISIBLE,
            Alphabet::Upper => &constants::ASCII_VISIBLE[32..58],
            Alphabet::Lower => &constants::ASCII_VISIBLE[64..90],
            Alphabet::A2z => &constants::ASCII_VISIBLE[32..90],
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "tools", derive(clap::ValueEnum))]
/// Fastq quality range, default: Illumina(1.8)
pub enum Quality {
    /// Sanger fastq quality range
    Sanger,
    /// Solexa fastq quality range
    Solexa,
    /// Illumina quality range version 1.3
    Illumina13,
    /// Illumina quality range version 1.5
    Illumina15,
    /// Illumina quality range version 1.8
    Illumina18,
    /// Illumina quality range version 1.8
    Illumina,
}

impl core::convert::AsRef<[u8]> for Quality {
    fn as_ref(&self) -> &[u8] {
        match self {
            Quality::Sanger => &constants::ASCII_VISIBLE[0..40],
            Quality::Solexa => &constants::ASCII_VISIBLE[26..71],
            Quality::Illumina13 => &constants::ASCII_VISIBLE[31..71],
            Quality::Illumina15 => &constants::ASCII_VISIBLE[34..71],
            Quality::Illumina18 | Quality::Illumina => &constants::ASCII_VISIBLE[0..41],
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "tools", derive(clap::ValueEnum))]
/// Any nucleotides
pub enum Nucleotides {
    /// Dna any case
    Dna,
    /// Dna lower case
    DnaLower,
    /// Dna upper case
    DnaUpper,
    /// Rna any case
    Rna,
    /// Rna lower case
    RnaLower,
    /// Rna upper case
    RnaUpper,
}

impl core::convert::AsRef<[u8]> for Nucleotides {
    fn as_ref(&self) -> &[u8] {
        match self {
            Nucleotides::Dna => &constants::DNA_NUCLEOTIDES,
            Nucleotides::DnaLower => &constants::DNA_NUCLEOTIDES[4..],
            Nucleotides::DnaUpper => &constants::DNA_NUCLEOTIDES[..4],
            Nucleotides::Rna => &constants::RNA_NUCLEOTIDES,
            Nucleotides::RnaLower => &constants::RNA_NUCLEOTIDES[4..],
            Nucleotides::RnaUpper => &constants::RNA_NUCLEOTIDES[..4],
        }
    }
}

/// Trait use to generate random data from values Enum
pub trait RandomBytes
where
    Self: core::convert::AsRef<[u8]>,
{
    /// Generate n bytes
    fn n(&self, rng: &mut rand::rngs::StdRng, n: usize) -> error::Result<Vec<u8>> {
        (0..n)
            .map(|_| {
                self.as_ref()
                    .choose(rng)
                    .cloned()
                    .ok_or(error::create_unreachable!())
            })
            .collect::<error::Result<Vec<u8>>>()
    }
}

impl RandomBytes for Alphabet {}
impl RandomBytes for Quality {}
impl RandomBytes for Nucleotides {}

#[cfg(test)]
mod tests {
    /* project use */
    use super::*;

    #[test]
    fn alphabet() {
        assert_eq!(Alphabet::Visible.as_ref(), b"!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
        assert_eq!(Alphabet::Upper.as_ref(), b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!(Alphabet::Lower.as_ref(), b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(
            Alphabet::A2z.as_ref(),
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz"
        );
    }

    #[test]
    fn quality() {
        assert_eq!(
            Quality::Sanger.as_ref(),
            b"!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGH"
        );
        assert_eq!(
            Quality::Solexa.as_ref(),
            b";<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefg"
        );
        assert_eq!(
            Quality::Illumina13.as_ref(),
            b"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefg"
        );
        assert_eq!(
            Quality::Illumina15.as_ref(),
            b"CDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefg"
        );
        assert_eq!(
            Quality::Illumina18.as_ref(),
            b"!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHI"
        );
        assert_eq!(
            Quality::Illumina.as_ref(),
            b"!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHI"
        );
    }

    #[test]
    fn nucleotides() {
        assert_eq!(Nucleotides::Dna.as_ref(), b"ACTGactg");
        assert_eq!(Nucleotides::DnaLower.as_ref(), b"actg");
        assert_eq!(Nucleotides::DnaUpper.as_ref(), b"ACTG");

        assert_eq!(Nucleotides::Rna.as_ref(), b"ACUGacug");
        assert_eq!(Nucleotides::RnaLower.as_ref(), b"acug");
        assert_eq!(Nucleotides::RnaUpper.as_ref(), b"ACUG");
    }
}
