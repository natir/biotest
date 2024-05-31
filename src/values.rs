//! Declarations of many possible values

/* std use */

/* crates use */
use rand::distributions::Distribution as _;
use rand::seq::SliceRandom as _;
use rand::Rng as _;

/* projet use */
use crate::constants;
use crate::error;

#[derive(Debug, Clone, Default)]
/// Differente generic ascii alphabet
pub enum Alphabet {
    #[default]
    /// Any visible ascii character
    Visible,

    /// Upper case latin alphabet
    Upper,

    /// Lower case latin alphapet
    Lower,

    /// Ascii character between A-z
    A2z,

    /// Vcf default value
    VcfDefault,
}

impl core::convert::AsRef<[u8]> for Alphabet {
    fn as_ref(&self) -> &[u8] {
        match self {
            Alphabet::Visible => &constants::ASCII_VISIBLE,
            Alphabet::Upper => &constants::ASCII_VISIBLE[32..58],
            Alphabet::Lower => &constants::ASCII_VISIBLE[64..90],
            Alphabet::A2z => &constants::ASCII_VISIBLE[32..90],
            Alphabet::VcfDefault => &constants::ASCII_VISIBLE[13..14],
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Fastq quality range
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

    #[default]
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

#[derive(Debug, Clone, Default)]
/// Any nucleotides
pub enum Nucleotides {
    #[default]
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
pub trait Generate
where
    Self: core::convert::AsRef<[u8]>,
{
    /// Generate n bytes
    fn generate(&self, rng: &mut rand::rngs::StdRng, n: usize) -> error::Result<Vec<u8>> {
        (0..n)
            .map(|_| {
                self.as_ref()
                    .choose(rng)
                    .cloned()
                    .ok_or(error::create_unreachable!())
            })
            .collect::<error::Result<Vec<u8>>>()
    }

    /// Generate n bytes with a weigthed distributions
    fn weighted<I, X>(
        &self,
        rng: &mut rand::rngs::StdRng,
        n: usize,
        weights: I,
    ) -> error::Result<Vec<u8>>
    where
        I: core::iter::IntoIterator,
        I::Item: rand::distributions::uniform::SampleBorrow<X>,
        X: rand::distributions::uniform::SampleUniform
            + PartialOrd
            + for<'a> core::ops::AddAssign<&'a X>
            + Clone
            + Default,
    {
        let dist = rand::distributions::WeightedIndex::new(weights)?;

        (0..n)
            .map(|_| {
                self.as_ref()
                    .get(dist.sample(rng))
                    .cloned()
                    .ok_or(error::Error::WeightArrayLargerValueArray)
            })
            .collect::<error::Result<Vec<u8>>>()
    }
}

impl Generate for Alphabet {}
impl Generate for Quality {}
impl Generate for Nucleotides {}

/// Range of integer value
#[derive(Debug, Clone, Default)]
pub enum Integer {
    /// Vcf possible position
    Position,

    /// Vcf integer possible value
    Vcf,

    /// Quality
    Quality,

    #[default]
    /// Full i32 range
    Full,

    /// UserDefine
    UserDefine(core::ops::Range<i32>),
}

impl core::convert::From<Integer> for core::ops::Range<i32> {
    fn from(val: Integer) -> Self {
        match val {
            Integer::Position => 0..i32::MAX,
            Integer::Vcf => (i32::MIN + 7)..i32::MAX,
            Integer::Quality => 0..255,
            Integer::Full => i32::MIN..i32::MAX,
            Integer::UserDefine(x) => x,
        }
    }
}

/// Range of float value
#[derive(Debug, Clone, Default)]
pub enum Float {
    #[default]
    /// between -100.0 and 100.0
    Default,

    /// Full f32 range
    Full,

    /// UserDefine
    UserDefine(core::ops::Range<f32>),
}

impl core::convert::From<Float> for core::ops::Range<f32> {
    fn from(val: Float) -> Self {
        match val {
            Float::Default => -100.0..100.0,
            Float::Full => f32::MIN..f32::MAX,
            Float::UserDefine(x) => x,
        }
    }
}

/// Trait to choose a random value in range and convert it in ASCII string
pub trait Get<T>
where
    Self: core::convert::Into<core::ops::Range<T>>,
    T: std::string::ToString + rand::distributions::uniform::SampleUniform + core::cmp::PartialOrd,
{
    /// Get a value
    fn get(self, rng: &mut rand::rngs::StdRng) -> Vec<u8> {
        rng.gen_range::<T, core::ops::Range<T>>(self.into())
            .to_string()
            .as_bytes()
            .to_vec()
    }
}

impl Get<i32> for Integer {}
impl Get<f32> for Float {}

#[derive(Debug, Clone, Default)]
/// Possible chromosomes names
pub enum Chromosomes {
    #[default]
    /// Default chromosomes names
    Default,

    /// UserDefine
    UserDefine(Vec<&'static [u8]>),
}

impl core::convert::AsRef<[&'static [u8]]> for Chromosomes {
    fn as_ref(&self) -> &[&'static [u8]] {
        match self {
            Chromosomes::Default => &constants::CHROMOSOMES,
            Chromosomes::UserDefine(a) => a.as_ref(),
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Possible vcf info type
pub enum VcfInfoType {
    #[default]
    /// All possible Vcf info type
    All,

    /// UserDefine
    UserDefine(Vec<&'static [u8]>),
}

impl core::convert::AsRef<[&'static [u8]]> for VcfInfoType {
    fn as_ref(&self) -> &[&'static [u8]] {
        match self {
            VcfInfoType::All => &constants::VCF_INFO_TYPE,
            VcfInfoType::UserDefine(a) => a.as_ref(),
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Possible vcf info type
pub enum VcfInfoNumber {
    #[default]
    /// All possible Vcf info type
    All,

    /// UserDefine
    UserDefine(Vec<&'static [u8]>),
}

impl core::convert::AsRef<[&'static [u8]]> for VcfInfoNumber {
    fn as_ref(&self) -> &[&'static [u8]] {
        match self {
            VcfInfoNumber::All => &constants::VCF_INFO_NUMBER,
            VcfInfoNumber::UserDefine(a) => a.as_ref(),
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Possible vcf format type
pub enum VcfFormatType {
    #[default]
    /// All possible Vcf format type
    All,

    /// UserDefine
    UserDefine(Vec<&'static [u8]>),
}

impl core::convert::AsRef<[&'static [u8]]> for VcfFormatType {
    fn as_ref(&self) -> &[&'static [u8]] {
        match self {
            VcfFormatType::All => &constants::VCF_FORMAT_TYPE,
            VcfFormatType::UserDefine(a) => a.as_ref(),
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Possible vcf format type
pub enum VcfFormatNumber {
    #[default]
    /// All possible Vcf format type
    All,

    /// UserDefine
    UserDefine(Vec<&'static [u8]>),
}

impl core::convert::AsRef<[&'static [u8]]> for VcfFormatNumber {
    fn as_ref(&self) -> &[&'static [u8]] {
        match self {
            VcfFormatNumber::All => &constants::VCF_FORMAT_NUMBER,
            VcfFormatNumber::UserDefine(a) => a.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    /* project use */
    use super::*;

    #[test]
    fn alphabet() -> error::Result<()> {
        assert_eq!(Alphabet::Visible.as_ref(), b"!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
        assert_eq!(Alphabet::Upper.as_ref(), b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!(Alphabet::Lower.as_ref(), b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(
            Alphabet::A2z.as_ref(),
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz"
        );
        assert_eq!(Alphabet::VcfDefault.as_ref(), b".");

        let mut rng = crate::rand();
        assert_eq!(Alphabet::Visible.generate(&mut rng, 5)?, b"l7bR:".to_vec());

        assert_eq!(
            Alphabet::Visible.weighted(&mut rng, 5, [1, 1, 1, 1])?,
            b"#$$!\"".to_vec()
        );

        Ok(())
    }

    #[test]
    fn quality() -> error::Result<()> {
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

        let mut rng = crate::rand();
        assert_eq!(Quality::Illumina.generate(&mut rng, 5)?, b"=DI3E".to_vec());

        assert_eq!(
            Quality::Sanger.weighted(&mut rng, 5, [2, 0, 2, 1, 1])?,
            b"!#$!!".to_vec()
        );
        assert_eq!(
            Quality::Solexa.weighted(&mut rng, 5, [1, 0, 1, 0, 1])?,
            b"??=;;".to_vec()
        );
        assert_eq!(
            Quality::Illumina13.weighted(&mut rng, 5, [5, 2, 1, 3, 1])?,
            b"@D@AC".to_vec()
        );
        assert_eq!(
            Quality::Illumina15.weighted(&mut rng, 5, [50, 25, 10, 1, 2])?,
            b"DGCCC".to_vec()
        );
        assert_eq!(
            Quality::Illumina18.weighted(&mut rng, 5, [1, 2, 3, 4, 5])?,
            b"%!###".to_vec()
        );
        assert_eq!(
            Quality::Illumina.weighted(&mut rng, 5, [1, 0, 2, 1, 1])?,
            b"!#%##".to_vec()
        );

        Ok(())
    }

    #[test]
    fn nucleotides() -> error::Result<()> {
        assert_eq!(Nucleotides::Dna.as_ref(), b"ACTGactg");
        assert_eq!(Nucleotides::DnaLower.as_ref(), b"actg");
        assert_eq!(Nucleotides::DnaUpper.as_ref(), b"ACTG");

        assert_eq!(Nucleotides::Rna.as_ref(), b"ACUGacug");
        assert_eq!(Nucleotides::RnaLower.as_ref(), b"acug");
        assert_eq!(Nucleotides::RnaUpper.as_ref(), b"ACUG");

        let mut rng = crate::rand();
        assert_eq!(
            Nucleotides::RnaUpper.generate(&mut rng, 5)?,
            b"GGUCU".to_vec()
        );

        assert!(matches!(
            Nucleotides::DnaUpper.weighted(&mut rng, 1, [0, 0, 0, 0, 1]),
            Err(error::Error::WeightArrayLargerValueArray)
        ));

        assert_eq!(
            Nucleotides::Dna.weighted(&mut rng, 5, [1, 1, 1, 1, 2, 2, 2, 2])?,
            b"gAGag".to_vec()
        );
        assert_eq!(
            Nucleotides::DnaLower.weighted(&mut rng, 5, [1, 1, 1, 1])?,
            b"actaa".to_vec()
        );
        assert_eq!(
            Nucleotides::DnaUpper.weighted(&mut rng, 5, [1, 1, 5, 5])?,
            b"GGTTT".to_vec()
        );
        assert_eq!(
            Nucleotides::Rna.weighted(&mut rng, 5, [1, 1, 1, 1, 2, 2, 2, 2])?,
            b"agCag".to_vec()
        );
        assert_eq!(
            Nucleotides::RnaLower.weighted(&mut rng, 5, [1, 5, 5, 1])?,
            b"ugccc".to_vec()
        );
        assert_eq!(
            Nucleotides::RnaUpper.weighted(&mut rng, 5, [1, 1, 5, 5])?,
            b"GAUUU".to_vec()
        );

        Ok(())
    }

    #[test]
    fn chromosomes() {
        assert_eq!(Chromosomes::Default.as_ref(), constants::CHROMOSOMES);
        assert_eq!(Chromosomes::UserDefine(vec![b"A"]).as_ref(), &[b"A"]);
    }

    #[test]
    fn info() {
        assert_eq!(VcfInfoType::All.as_ref(), constants::VCF_INFO_TYPE);
        assert_eq!(VcfInfoType::UserDefine(vec![b"A"]).as_ref(), &[b"A"]);

        assert_eq!(VcfInfoNumber::All.as_ref(), constants::VCF_INFO_NUMBER);
        assert_eq!(VcfInfoNumber::UserDefine(vec![b"A"]).as_ref(), &[b"A"]);
    }

    #[test]
    fn format() {
        assert_eq!(VcfFormatType::All.as_ref(), constants::VCF_FORMAT_TYPE);
        assert_eq!(VcfFormatType::UserDefine(vec![b"A"]).as_ref(), &[b"A"]);

        assert_eq!(VcfFormatNumber::All.as_ref(), constants::VCF_FORMAT_NUMBER);
        assert_eq!(VcfFormatNumber::UserDefine(vec![b"A"]).as_ref(), &[b"A"]);
    }

    #[test]
    fn interger() {
        assert_eq!(
            core::ops::Range::<i32>::from(Integer::Position),
            0..i32::MAX
        );
        assert_eq!(
            core::ops::Range::<i32>::from(Integer::Vcf),
            (i32::MIN + 7)..i32::MAX
        );
        assert_eq!(
            core::ops::Range::<i32>::from(Integer::Full),
            i32::MIN..i32::MAX
        );
        assert_eq!(
            core::ops::Range::<i32>::from(Integer::UserDefine(-92..108)),
            -92..108
        );

        let mut rng = crate::rand();
        assert_eq!(Integer::Position.get(&mut rng,), b"1720731148".to_vec());
    }

    #[test]
    fn float() {
        assert_eq!(
            <Float as Into<core::ops::Range<f32>>>::into(Float::Full),
            f32::MIN..f32::MAX
        );
        assert_eq!(
            <Float as Into<core::ops::Range<f32>>>::into(Float::UserDefine(-1023.3..3002.5)),
            -1023.3..3002.5
        );

        let mut rng = crate::rand();
        assert_eq!(
            Float::UserDefine(-1023.3..3002.5).get(&mut rng,),
            b"2202.4844".to_vec()
        );
    }
}
