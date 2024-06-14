//! GFF2 format

/* std use */

/* crates use */
use rand::seq::SliceRandom as _;
use rand::Rng as _;

/* project use */
use crate::error;
use crate::format;
use crate::values;

use crate::values::Generate as _;
use crate::values::Get as _;

/// Struct to generate gff record
#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Gff {
    /// Chromosome
    #[builder(default = "values::Chromosomes::Default")]
    contigs: values::Chromosomes,

    /// Feature
    #[builder(default = "values::GffFeature::All")]
    features: values::GffFeature,

    /// Position
    #[builder(default = "values::Integer::Position")]
    position: values::Integer,

    /// Feature length
    #[builder(default = "values::Integer::UserDefine(1..100_000)")]
    length: values::Integer,

    /// Score
    #[builder(default = "values::Float::Default")]
    score: values::Float,

    /// Strand
    #[builder(default = "values::Strand::All")]
    strand: values::Strand,

    /// Phase
    #[builder(default = "values::GffPhase::All")]
    phase: values::GffPhase,

    /// Id
    #[builder(default = "values::Alphabet::A2z")]
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

    /// Name
    #[builder(default = "values::Alphabet::Lower")]
    name: values::Alphabet,

    /// Length of name
    #[builder(default = "10")]
    name_len: usize,

    /// Name prefix
    #[builder(default = "b\"\".to_vec()")]
    name_prefix: Vec<u8>,

    /// Name suffix
    #[builder(default = "b\"\".to_vec()")]
    name_suffix: Vec<u8>,

    /// Alias
    #[builder(default = "values::Alphabet::A2z")]
    alias: values::Alphabet,

    /// Length of alias
    #[builder(default = "10")]
    alias_len: usize,

    /// Alias prefix
    #[builder(default = "b\"\".to_vec()")]
    alias_prefix: Vec<u8>,

    /// Alias suffix
    #[builder(default = "b\"\".to_vec()")]
    alias_suffix: Vec<u8>,

    /// Parent
    #[builder(default = "values::Alphabet::A2z")]
    parent: values::Alphabet,

    /// Length of parent
    #[builder(default = "10")]
    parent_len: usize,

    /// Parent prefix
    #[builder(default = "b\"\".to_vec()")]
    parent_prefix: Vec<u8>,

    /// Parent suffix
    #[builder(default = "b\"\".to_vec()")]
    parent_suffix: Vec<u8>,
}

impl Gff {
    /// Create a GffBuilder
    pub fn builder() -> GffBuilder {
        GffBuilder::default()
    }
}

impl core::default::Default for Gff {
    fn default() -> Self {
        GffBuilder::default().build().unwrap() // it's default no error
    }
}

impl format::Format for Gff {
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
        // seqid
        output.write_all(
            self.contigs
                .as_ref()
                .choose(rng)
                .ok_or(error::create_unreachable!())?,
        )?;
        output.write_all(b"\t")?;

        // source
        output.write_all(b"biotest\t")?;

        // type
        output.write_all(
            self.features
                .as_ref()
                .choose(rng)
                .ok_or(error::create_unreachable!())?,
        )?;
        output.write_all(b"\t")?;

        // start
        let start = rng.gen_range::<i32, core::ops::Range<i32>>(self.position.clone().into());
        output.write_all(&start.to_string().as_bytes().to_vec())?;
        output.write_all(b"\t")?;

        // end
        let end: i32 =
            start + rng.gen_range::<i32, core::ops::Range<i32>>(self.length.clone().into());
        output.write_all(&end.to_string().as_bytes().to_vec())?;
        output.write_all(b"\t")?;

        // score
        output.write_all(&self.score.clone().get(rng))?;
        output.write_all(b"\t")?;

        // strand
        output.write_all(
            self.strand
                .as_ref()
                .choose(rng)
                .ok_or(error::create_unreachable!())?,
        )?;
        output.write_all(b"\t")?;

        // phase
        output.write_all(
            self.phase
                .as_ref()
                .choose(rng)
                .ok_or(error::create_unreachable!())?,
        )?;
        output.write_all(b"\t")?;

        // attributes
        // id
        output.write_all(b"ID=")?;
        output.write_all(&self.id_prefix)?;
        output.write_all(&self.id.generate(rng, self.id_len)?)?;
        output.write_all(&self.id_suffix)?;
        output.write_all(b";")?;

        // name
        output.write_all(b"Name=")?;
        output.write_all(&self.name_prefix)?;
        output.write_all(&self.name.generate(rng, self.name_len)?)?;
        output.write_all(&self.name_suffix)?;
        output.write_all(b";")?;

        // alias
        output.write_all(b"Alias=")?;
        output.write_all(&self.alias_prefix)?;
        output.write_all(&self.alias.generate(rng, self.alias_len)?)?;
        output.write_all(&self.alias_suffix)?;
        output.write_all(b";")?;

        // parent
        output.write_all(b"Parent=")?;
        output.write_all(&self.parent_prefix)?;
        output.write_all(&self.parent.generate(rng, self.parent_len)?)?;
        output.write_all(&self.parent_suffix)?;

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

    const TRUTH: &[u8] = b"YAR028W\tbiotest\texon\t6057\t6155\t9.429573\t.\t0\tID=[tBTlDDl[M;Name=emxuzgaghm;Alias=s^[teLMir[;Parent=gMDhw\\voCG
YAR028W\tbiotest\trepeat\t4903\t4948\t7.9373302\t+\t0\tID=gQouVGn`Jw;Name=qjbbjlzxpz;Alias=Any[_POshs;Parent=qbSjAdbZcR
YAR028W\tbiotest\trepeat\t5211\t5297\t3.2389307\t+\t0\tID=tSvzMeTjon;Name=ljdusfsrcu;Alias=tTH\\QXXOiA;Parent=LJLnuPtf`S
YAR028W\tbiotest\trepeat\t5617\t5673\t3.21298\t+\t2\tID=vYGCzkT\\Wk;Name=nwbrlpbpvm;Alias=tIGEbcnVWJ;Parent=VaDBnQSHYN
X\tbiotest\ttranscript\t5944\t6040\t9.520424\t+\t2\tID=`NnOG[K`QK;Name=rrilpylxga;Alias=MyDqpgZliS;Parent=mUzRvGGXBg
";

    const DEFAULT: &[u8] = b"YAR028W\tbiotest\texon\t1133862760\t1133889429\t21.144531\t.\t0\tID=[tBTlDDl[M;Name=emxuzgaghm;Alias=s^[teLMir[;Parent=gMDhw\\voCG";

    #[test]
    fn default() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Gff::default();

        generator.record(&mut output, &mut rng)?;

        assert_eq!(output, DEFAULT);

        Ok(())
    }

    #[test]
    fn record() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Gff::builder()
            .position(values::Integer::UserDefine(0..10_000))
            .length(values::Integer::UserDefine(2..100))
            .score(values::Float::UserDefine(0.0..10.0))
            .build()?;

        generator.record(&mut output, &mut rng)?;

        println!("{:?} {}", String::from_utf8(output.to_vec()), output.len());
        assert_eq!(output, TRUTH.to_vec()[..108]);

        Ok(())
    }

    #[test]
    fn records() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Gff::builder()
            .position(values::Integer::UserDefine(0..10_000))
            .length(values::Integer::UserDefine(2..100))
            .score(values::Float::UserDefine(0.0..10.0))
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

        let generator = Gff::builder()
            .position(values::Integer::UserDefine(0..10_000))
            .length(values::Integer::UserDefine(2..100))
            .score(values::Float::UserDefine(0.0..10.0))
            .build()?;

        generator.create(&temp_file, &mut rng, 5)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, TRUTH.to_vec());

        Ok(())
    }
}
