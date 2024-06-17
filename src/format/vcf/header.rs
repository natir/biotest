//! VCF header generation

/* std use */

/* crates use */

/* projet use */
use crate::constants;
use crate::error;
use crate::values;

#[derive(typed_builder::TypedBuilder)]
/// Struct to generate header
pub struct Header {
    /// Value use for chromosomes
    #[builder(default = values::Chromosomes::Default)]
    contigs: values::Chromosomes,

    /// vcf version number
    #[builder(default = b"VCFv4.3".to_vec())]
    version: Vec<u8>,

    /// contig species
    #[builder(default = b"random".to_vec())]
    contig_species: Vec<u8>,

    /// contig length
    #[builder(default = u32::MAX)]
    contig_length: u32,

    /// filter range
    #[builder(default = values::Integer::UserDefine(0..3))]
    filter: values::Integer,

    /// filter prefix
    #[builder(default = b"filter_".to_vec())]
    filter_prefix: Vec<u8>,

    /// filter description
    #[builder(default = b"generated vcf filter field".to_vec())]
    filter_description: Vec<u8>,

    /// info prefix
    #[builder(default = b"info_".to_vec())]
    info_prefix: Vec<u8>,

    /// info description
    #[builder(default = b"generated vcf info field".to_vec())]
    info_description: Vec<u8>,

    /// InfoType
    #[builder(default = values::VcfInfoType::All)]
    info_type: values::VcfInfoType,

    /// InfoNumber
    #[builder(default = values::VcfInfoNumber::All)]
    info_number: values::VcfInfoNumber,

    /// format prefix
    #[builder(default = b"format_".to_vec())]
    format_prefix: Vec<u8>,

    /// format description
    #[builder(default = b"generated vcf format field".to_vec())]
    format_description: Vec<u8>,

    /// FormatType
    #[builder(default = values::VcfFormatType::All)]
    format_type: values::VcfFormatType,

    /// FormatNumber
    #[builder(default = values::VcfFormatNumber::All)]
    format_number: values::VcfFormatNumber,

    /// Number of sample
    #[builder(default = 3)]
    sample: usize,

    /// Sample prefix
    #[builder(default = b"sample_".to_vec())]
    sample_prefix: Vec<u8>,

    /// Sample suffix
    #[builder(default = b"".to_vec())]
    sample_suffix: Vec<u8>,
}

impl Header {
    /// Generate vcf header
    pub fn generate(&self, output: &mut dyn std::io::Write) -> error::Result<()> {
        // version
        output.write_all(b"##fileformat=")?;
        output.write_all(&self.version)?;
        output.write_all(b"\n")?;

        // contig
        for chr in self.contigs.as_ref() {
            output.write_all(b"##contig=<ID=")?;
            output.write_all(chr)?;
            output.write_all(b",length=")?;
            output.write_all(self.contig_length.to_string().as_bytes())?;
            output.write_all(b",species=\"")?;
            output.write_all(&self.contig_species)?;
            output.write_all(b"\">\n")?;
        }

        // filters
        for n in <values::Integer as core::convert::Into<core::ops::Range<i32>>>::into(
            <values::Integer as Clone>::clone(&self.filter),
        ) {
            output.write_all(b"##FILTER=<ID=")?;
            output.write_all(&self.filter_prefix)?;
            output.write_all(n.to_string().as_bytes())?;
            output.write_all(b",Description=\"")?;
            output.write_all(&self.filter_description)?;
            output.write_all(b"\">\n")?;
        }

        // infos
        for vcf_type in self.info_type.as_ref() {
            if vcf_type == b"Flag" {
                output.write_all(b"##INFO=<ID=")?;
                output.write_all(&self.info_prefix)?;
                output.write_all(b"Flag_0,")?;
                output.write_all(b"Number=0,Type=Flag,Description=\"")?;
                output.write_all(&self.info_description)?;
                output.write_all(b"\",Source=\"biotest\",Version=\"")?;
                output.write_all(constants::BIOTEST_VERSION)?;
                output.write_all(b"\">\n")?;
            } else {
                for vcf_number in self.info_number.as_ref() {
                    // IDentifiant
                    output.write_all(b"##INFO=<ID=")?;
                    output.write_all(&self.info_prefix)?;
                    output.write_all(vcf_type)?;
                    output.write_all(b"_")?;
                    output.write_all(vcf_number)?;

                    // Number
                    output.write_all(b",Number=")?;
                    output.write_all(vcf_number)?;

                    // Type
                    output.write_all(b",Type=")?;
                    output.write_all(vcf_type)?;

                    // Other
                    output.write_all(b",Description=\"")?;
                    output.write_all(&self.format_description)?;
                    output.write_all(b"\",Source=\"biotest\",")?;
                    output.write_all(b"Version=\"")?;
                    output.write_all(constants::BIOTEST_VERSION)?;
                    output.write_all(b"\">\n")?;
                }
            }
        }

        // formats
        for vcf_type in self.format_type.as_ref() {
            for vcf_number in self.format_number.as_ref() {
                // ID
                output.write_all(b"##FORMAT=<ID=")?;
                output.write_all(&self.format_prefix)?;
                output.write_all(vcf_type)?;
                output.write_all(b"_")?;
                output.write_all(vcf_number)?;

                // Number
                output.write_all(b",Number=")?;
                output.write_all(vcf_number)?;

                // Type
                output.write_all(b",Type=")?;
                output.write_all(vcf_type)?;

                // Other
                output.write_all(b",Description=\"")?;
                output.write_all(&self.format_description)?;
                output.write_all(b"\">\n")?;
            }
        }

        // column name
        output.write_all(b"#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO")?;

        if self.sample != 0 {
            output.write_all(b"\tFORMAT")?;
            for n in 0..self.sample {
                output.write_all(b"\t")?;
                output.write_all(&self.sample_prefix)?;
                output.write_all(n.to_string().as_bytes())?;
                output.write_all(&self.sample_suffix)?;
            }
        }

        output.write_all(b"\n")?;

        Ok(())
    }
}

impl core::default::Default for Header {
    fn default() -> Self {
        Header::builder().build()
    }
}

#[cfg(test)]
mod tests {
    /* project use */
    use super::*;

    const DEFAULT: &[u8] = b"##fileformat=VCFv4.3
##contig=<ID=chr1,length=4294967295,species=\"random\">
##contig=<ID=23,length=4294967295,species=\"random\">
##contig=<ID=93,length=4294967295,species=\"random\">
##contig=<ID=chrMT,length=4294967295,species=\"random\">
##contig=<ID=X,length=4294967295,species=\"random\">
##contig=<ID=NC_000015.10,length=4294967295,species=\"random\">
##contig=<ID=ENA|LT795502|LT795502.1,length=4294967295,species=\"random\">
##contig=<ID=NC_016845.1,length=4294967295,species=\"random\">
##contig=<ID=YAR028W,length=4294967295,species=\"random\">
##contig=<ID=1,length=4294967295,species=\"random\">
##FILTER=<ID=filter_0,Description=\"generated vcf filter field\">
##FILTER=<ID=filter_1,Description=\"generated vcf filter field\">
##FILTER=<ID=filter_2,Description=\"generated vcf filter field\">
##INFO=<ID=info_Integer_1,Number=1,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Integer_2,Number=2,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Integer_A,Number=A,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Integer_R,Number=R,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Integer_G,Number=G,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Integer_.,Number=.,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Float_1,Number=1,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Float_2,Number=2,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Float_A,Number=A,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Float_R,Number=R,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Float_G,Number=G,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Float_.,Number=.,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Flag_0,Number=0,Type=Flag,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Character_1,Number=1,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Character_2,Number=2,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Character_A,Number=A,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Character_R,Number=R,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Character_G,Number=G,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_Character_.,Number=.,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_String_1,Number=1,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_String_2,Number=2,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_String_A,Number=A,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_String_R,Number=R,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_String_G,Number=G,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=info_String_.,Number=.,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.2.0\">
##FORMAT=<ID=format_Integer_1,Number=1,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_2,Number=2,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_A,Number=A,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_R,Number=R,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_G,Number=G,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_.,Number=.,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_1,Number=1,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_2,Number=2,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_A,Number=A,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_R,Number=R,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_G,Number=G,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_.,Number=.,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_1,Number=1,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_2,Number=2,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_A,Number=A,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_R,Number=R,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_G,Number=G,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_.,Number=.,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_1,Number=1,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_2,Number=2,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_A,Number=A,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_R,Number=R,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_G,Number=G,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_.,Number=.,Type=String,Description=\"generated vcf format field\">
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO	FORMAT	sample_0	sample_1	sample_2
";

    const SET: &[u8] = b"##fileformat=VCFv4.3
##contig=<ID=A,length=4223,species=\"alphabet\">
##contig=<ID=B,length=4223,species=\"alphabet\">
##contig=<ID=C,length=4223,species=\"alphabet\">
##INFO=<ID=INFO_Integer_1,Number=1,Type=Integer,Description=\"description\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=INFO_Integer_2,Number=2,Type=Integer,Description=\"description\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=INFO_Float_1,Number=1,Type=Float,Description=\"description\",Source=\"biotest\",Version=\"0.2.0\">
##INFO=<ID=INFO_Float_2,Number=2,Type=Float,Description=\"description\",Source=\"biotest\",Version=\"0.2.0\">
##FORMAT=<ID=FORMAT_Integer_1,Number=1,Type=Integer,Description=\"description\">
##FORMAT=<ID=FORMAT_Integer_2,Number=2,Type=Integer,Description=\"description\">
##FORMAT=<ID=FORMAT_Float_1,Number=1,Type=Float,Description=\"description\">
##FORMAT=<ID=FORMAT_Float_2,Number=2,Type=Float,Description=\"description\">
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO
";

    #[test]
    fn default() -> error::Result<()> {
        let mut output = Vec::new();

        let generator = Header::builder().build();

        generator.generate(&mut output)?;

        assert_eq!(output, DEFAULT);

        Ok(())
    }

    #[test]
    fn set() -> error::Result<()> {
        let mut output = Vec::new();

        let generator = Header::builder()
            .contigs(values::Chromosomes::UserDefine(vec![b"A", b"B", b"C"]))
            .contig_species(b"alphabet".to_vec())
            .contig_length(4223)
            .filter(values::Integer::UserDefine(0..0))
            .filter_prefix(b"filter_".to_vec())
            .filter_description(b"description".to_vec())
            .info_prefix(b"INFO_".to_vec())
            .info_description(b"description".to_vec())
            .info_type(values::VcfInfoType::UserDefine(vec![b"Integer", b"Float"]))
            .info_number(values::VcfInfoNumber::UserDefine(vec![b"1", b"2"]))
            .format_prefix(b"FORMAT_".to_vec())
            .format_description(b"description".to_vec())
            .format_type(values::VcfFormatType::UserDefine(vec![
                b"Integer", b"Float",
            ]))
            .format_number(values::VcfFormatNumber::UserDefine(vec![b"1", b"2"]))
            .sample(0)
            .sample_prefix(b"individual_".to_vec())
            .sample_suffix(b" auie".to_vec())
            .build();

        generator.generate(&mut output)?;

        assert_eq!(output, SET);

        Ok(())
    }
}
