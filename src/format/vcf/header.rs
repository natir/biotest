//! VCF header generation

/* std use */

/* crates use */

/* projet use */
use crate::constants;
use crate::error;

/// Generate version
fn version<W>(output: &mut W) -> error::Result<()>
where
    W: std::io::Write,
{
    output.write_all(b"##fileformat=VCFv4.3\n")?;

    Ok(())
}

/// Generate contigs header
fn contigs<W>(output: &mut W) -> error::Result<()>
where
    W: std::io::Write,
{
    for chr in constants::CHROMOSOMES {
        output.write_all(b"##contig=<ID=")?;
        output.write_all(chr)?;
        output.write_all(b",length=2147483648,species=\"random\">\n")?;
    }

    Ok(())
}

/// Generate all filter
fn filters<W>(output: &mut W, number: u8) -> error::Result<()>
where
    W: std::io::Write,
{
    for n in 0..number {
        output.write_all(b"##FILTER=<ID=Filter_")?;
        output.write_all(n.to_string().as_bytes())?;
        output.write_all(b",Description=\"generated vcf filter field\">\n")?;
    }

    Ok(())
}

/// Generate all possible info header
fn infos<W>(output: &mut W) -> error::Result<()>
where
    W: std::io::Write,
{
    for vcf_type in constants::VCF_INFO_TYPE {
        if vcf_type == b"Flag" {
            output.write_all(b"##INFO=<ID=info_Flag_0,Number=0,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"")?;
            output.write_all(constants::BIOTEST_VERSION)?;
            output.write_all(b"\">\n")?;
        } else {
            for vcf_number in constants::VCF_INFO_NUMBER {
                // ID
                output.write_all(b"##INFO=<ID=info_")?;
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
                output.write_all(b",Description=\"generated vcf info field\",")?;
                output.write_all(b"Source=\"biotest\",")?;
                output.write_all(b"Version=\"")?;
                output.write_all(constants::BIOTEST_VERSION)?;
                output.write_all(b"\">\n")?;
            }
        }
    }

    Ok(())
}

/// Generate all possible format header
fn formats<W>(output: &mut W) -> error::Result<()>
where
    W: std::io::Write,
{
    for vcf_type in constants::VCF_FORMAT_TYPE {
        for vcf_number in constants::VCF_FORMAT_NUMBER {
            // ID
            output.write_all(b"##FORMAT=<ID=format_")?;
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
            output.write_all(b",Description=\"generated vcf info field\">\n")?;
        }
    }

    Ok(())
}

/// Column name
fn columns_name<W>(output: &mut W, genotypes: Option<u8>) -> error::Result<()>
where
    W: std::io::Write,
{
    output.write_all(b"#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO")?;

    if let Some(number_genotype) = genotypes {
        output.write_all(b"\tFORMAT")?;
        for n in 0..number_genotype {
            output.write_all(b"\tsample_")?;
            output.write_all(n.to_string().as_bytes())?;
        }
    }

    output.write_all(b"\n")?;

    Ok(())
}

/// Generate vcf header
pub fn write<W>(
    output: &mut W,
    nb_filters: Option<u8>,
    add_infos: bool,
    nb_samples: Option<u8>,
) -> error::Result<()>
where
    W: std::io::Write,
{
    version(output)?;
    contigs(output)?;

    if let Some(n) = nb_filters {
        filters(output, n)?;
    }

    if add_infos {
        infos(output)?;
    }

    if nb_samples.is_some() {
        formats(output)?;
    }

    columns_name(output, nb_samples)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    /* project use */
    use super::*;

    const TRUTH: &[u8] = b"##fileformat=VCFv4.3
##contig=<ID=chr1,length=2147483648,species=\"random\">
##contig=<ID=23,length=2147483648,species=\"random\">
##contig=<ID=93,length=2147483648,species=\"random\">
##contig=<ID=chrMT,length=2147483648,species=\"random\">
##contig=<ID=X,length=2147483648,species=\"random\">
##contig=<ID=NC_000015.10,length=2147483648,species=\"random\">
##contig=<ID=ENA|LT795502|LT795502.1,length=2147483648,species=\"random\">
##contig=<ID=NC_016845.1,length=2147483648,species=\"random\">
##contig=<ID=YAR028W,length=2147483648,species=\"random\">
##contig=<ID=1,length=2147483648,species=\"random\">
##FILTER=<ID=Filter_0,Description=\"generated vcf filter field\">
##FILTER=<ID=Filter_1,Description=\"generated vcf filter field\">
##INFO=<ID=info_Integer_1,Number=1,Type=Integer,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_2,Number=2,Type=Integer,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_A,Number=A,Type=Integer,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_R,Number=R,Type=Integer,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_G,Number=G,Type=Integer,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_.,Number=.,Type=Integer,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_1,Number=1,Type=Float,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_2,Number=2,Type=Float,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_A,Number=A,Type=Float,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_R,Number=R,Type=Float,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_G,Number=G,Type=Float,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_.,Number=.,Type=Float,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Flag_0,Number=0,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_1,Number=1,Type=Character,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_2,Number=2,Type=Character,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_A,Number=A,Type=Character,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_R,Number=R,Type=Character,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_G,Number=G,Type=Character,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_.,Number=.,Type=Character,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_1,Number=1,Type=String,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_2,Number=2,Type=String,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_A,Number=A,Type=String,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_R,Number=R,Type=String,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_G,Number=G,Type=String,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_.,Number=.,Type=String,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##FORMAT=<ID=format_Integer_1,Number=1,Type=Integer,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Integer_2,Number=2,Type=Integer,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Integer_A,Number=A,Type=Integer,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Integer_R,Number=R,Type=Integer,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Integer_G,Number=G,Type=Integer,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Integer_.,Number=.,Type=Integer,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Float_1,Number=1,Type=Float,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Float_2,Number=2,Type=Float,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Float_A,Number=A,Type=Float,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Float_R,Number=R,Type=Float,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Float_G,Number=G,Type=Float,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Float_.,Number=.,Type=Float,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Character_1,Number=1,Type=Character,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Character_2,Number=2,Type=Character,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Character_A,Number=A,Type=Character,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Character_R,Number=R,Type=Character,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Character_G,Number=G,Type=Character,Description=\"generated vcf info field\">
##FORMAT=<ID=format_Character_.,Number=.,Type=Character,Description=\"generated vcf info field\">
##FORMAT=<ID=format_String_1,Number=1,Type=String,Description=\"generated vcf info field\">
##FORMAT=<ID=format_String_2,Number=2,Type=String,Description=\"generated vcf info field\">
##FORMAT=<ID=format_String_A,Number=A,Type=String,Description=\"generated vcf info field\">
##FORMAT=<ID=format_String_R,Number=R,Type=String,Description=\"generated vcf info field\">
##FORMAT=<ID=format_String_G,Number=G,Type=String,Description=\"generated vcf info field\">
##FORMAT=<ID=format_String_.,Number=.,Type=String,Description=\"generated vcf info field\">
#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT\tsample_0\tsample_1\tsample_2
";

    const COLUMNS_NO_FORMAT: &[u8] = b"#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\n";

    #[test]
    fn version_() -> error::Result<()> {
        let mut output = Vec::new();

        version(&mut output)?;

        assert_eq!(output, TRUTH[..21]);

        Ok(())
    }

    #[test]
    fn contigs_() -> error::Result<()> {
        let mut output = Vec::new();

        contigs(&mut output)?;
        assert_eq!(output, TRUTH[21..589]);

        Ok(())
    }

    #[test]
    fn filters_() -> error::Result<()> {
        let mut output = Vec::new();

        filters(&mut output, 2)?;
        assert_eq!(output, TRUTH[589..717]);

        Ok(())
    }

    #[test]
    fn infos_() -> error::Result<()> {
        let mut output = Vec::new();

        infos(&mut output)?;
        assert_eq!(output, TRUTH[717..3714]);

        Ok(())
    }

    #[test]
    fn formats_() -> error::Result<()> {
        let mut output = Vec::new();

        formats(&mut output)?;

        assert_eq!(output, TRUTH[3714..5910]);

        Ok(())
    }

    #[test]
    fn columns_name_() -> error::Result<()> {
        let mut output = Vec::new();

        columns_name(&mut output, Some(3))?;

        assert_eq!(output, TRUTH[5910..]);

        Ok(())
    }

    #[test]
    fn columns_name_no_format() -> error::Result<()> {
        let mut output = Vec::new();

        columns_name(&mut output, None)?;

        assert_eq!(output, COLUMNS_NO_FORMAT);

        Ok(())
    }

    #[test]
    fn write_() -> error::Result<()> {
        let mut output = Vec::new();

        write(&mut output, Some(2), true, Some(3))?;

        assert_eq!(output, TRUTH);
        Ok(())
    }
}
