//! Declarations of many possible values

/* std use */
use rand::seq::SliceRandom;

/* crates use */

/* projet use */
use crate::constants;
use crate::error;

#[derive(Debug)]
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

#[derive(Debug)]
/// Fastq quality range, default: Illumina(1.8)
pub enum Quality {
    /// Sanger fastq quality range
    Sanger,
    /// Solexa fastq quality range
    Solexa,
    /// Illumina quality range 1.3 -> 13, 1.5 -> 15 and 1.8 -> 18 is availlable
    Illumina(u8),
}

impl core::convert::AsRef<[u8]> for Quality {
    fn as_ref(&self) -> &[u8] {
        match self {
            Quality::Sanger => &constants::ASCII_VISIBLE[0..40],
            Quality::Solexa => &constants::ASCII_VISIBLE[26..71],
            Quality::Illumina(13) => &constants::ASCII_VISIBLE[31..71],
            Quality::Illumina(15) => &constants::ASCII_VISIBLE[34..71],
            Quality::Illumina(18) => &constants::ASCII_VISIBLE[0..41],
            _ => &constants::ASCII_VISIBLE[0..41],
        }
    }
}

#[derive(Debug)]
/// Dna nucleotides
pub enum Dna {
    /// All
    All,
    /// Lower case only
    Lower,
    /// Upper case only
    Upper,
}

#[derive(Debug)]
/// Rna nucleotides
pub enum Rna {
    /// All
    All,
    /// Lower case only
    Lower,
    /// Upper case only
    Upper,
}

#[derive(Debug)]
/// Any nucleotides
pub enum Nucleotides {
    /// Dna
    Dna(Dna),

    /// Rna
    Rna(Rna),
}

impl core::convert::AsRef<[u8]> for Dna {
    fn as_ref(&self) -> &[u8] {
        match self {
            Dna::All => &constants::DNA_NUCLEOTIDES,
            Dna::Lower => &constants::DNA_NUCLEOTIDES[4..],
            Dna::Upper => &constants::DNA_NUCLEOTIDES[..4],
        }
    }
}

impl core::convert::AsRef<[u8]> for Rna {
    fn as_ref(&self) -> &[u8] {
        match self {
            Rna::All => &constants::RNA_NUCLEOTIDES,
            Rna::Lower => &constants::RNA_NUCLEOTIDES[4..],
            Rna::Upper => &constants::RNA_NUCLEOTIDES[..4],
        }
    }
}

impl core::convert::AsRef<[u8]> for Nucleotides {
    fn as_ref(&self) -> &[u8] {
        match self {
            Nucleotides::Dna(a) => a.as_ref(),
            Nucleotides::Rna(a) => a.as_ref(),
        }
    }
}

/// Trait use to generate random data from values Enum
pub trait Random
where
    Self: core::convert::AsRef<[u8]>,
{
    /// Generate one bytes
    fn one(&self, rng: &mut rand::rngs::StdRng) -> error::Result<u8> {
        self.as_ref()
            .choose(rng)
            .cloned()
            .ok_or(error::create_unreachable!())
    }

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

impl Random for Alphabet {}
impl Random for Quality {}
impl Random for Nucleotides {}

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
            Quality::Illumina(13).as_ref(),
            b"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefg"
        );
        assert_eq!(
            Quality::Illumina(15).as_ref(),
            b"CDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefg"
        );
        assert_eq!(
            Quality::Illumina(18).as_ref(),
            b"!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHI"
        );
        assert_eq!(
            Quality::Illumina(100).as_ref(),
            b"!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHI"
        );
    }

    #[test]
    fn nucleotides() {
        assert_eq!(Nucleotides::Dna(Dna::All).as_ref(), b"ACTGactg");
        assert_eq!(Nucleotides::Dna(Dna::Lower).as_ref(), b"actg");
        assert_eq!(Nucleotides::Dna(Dna::Upper).as_ref(), b"ACTG");

        assert_eq!(Nucleotides::Rna(Rna::All).as_ref(), b"ACUGacug");
        assert_eq!(Nucleotides::Rna(Rna::Lower).as_ref(), b"acug");
        assert_eq!(Nucleotides::Rna(Rna::Upper).as_ref(), b"ACUG");
    }
}
