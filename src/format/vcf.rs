//! VCF generation

/* std use */

/* crates use */
use rand::seq::SliceRandom as _;
use rand::Rng as _;

/* projet use */
use crate::constants;
use crate::error;

/// Generate vcf header
fn header<W>(output: &mut W) -> error::Result<()>
where
    W: std::io::Write,
{
    output.write_all(b"##fileformat=VCFv4.3\n")?;
    for chr in constants::CHROMOSOMES {
        output.write_all(b"##contig=<ID=")?;
        output.write_all(chr)?;
        output.write_all(b",length=2147483648,species=\"random\">\n")?;
    }

    output.write_all(b"#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\n")?;

    Ok(())
}

/// Generate vcf record
fn record<W>(output: &mut W, rng: &mut rand::rngs::StdRng) -> error::Result<()>
where
    W: std::io::Write,
{
    // chromosomes
    output.write_all(constants::CHROMOSOMES.choose(rng).unwrap())?;
    output.write_all(b"\t")?;

    // position
    output.write_all(
        &rng.gen_range(0..i32::MAX)
            .to_string()
            .bytes()
            .collect::<Vec<u8>>(),
    )?;

    // identifiant
    output.write_all(b"\t.\t")?;

    // reference
    output.write_all(&[*constants::NUCLEOTIDES.choose(rng).unwrap()])?;
    output.write_all(b"\t")?;

    // alternatif
    let alt_len = rng.gen_range(1..5);
    crate::sequence(output, rng, alt_len)?;
    output.write_all(b"\t")?;

    // quality
    output.write_all(
        &rng.gen_range(0..255)
            .to_string()
            .bytes()
            .collect::<Vec<u8>>(),
    )?;
    output.write_all(b"\t")?;

    // filter
    output.write_all(b".\t")?;

    // info
    output.write_all(b"AC=")?;
    output.write_all(
        &rng.gen_range(0..i32::MAX)
            .to_string()
            .bytes()
            .collect::<Vec<u8>>(),
    )?;

    Ok(())
}

/// Write multiple record
pub fn records<W>(
    output: &mut W,
    rng: &mut rand::rngs::StdRng,
    num_record: usize,
) -> error::Result<()>
where
    W: std::io::Write,
{
    for _ in 0..num_record {
        record(output, rng)?;
        output.write_all(&[b'\n'])?;
    }

    Ok(())
}

/// Create a vcf file
pub fn create<P>(path: P, rng: &mut rand::rngs::StdRng, num_record: usize) -> error::Result<()>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let mut output = std::fs::File::create(&path)?;

    header(&mut output)?;
    records(&mut output, rng, num_record)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    /* std use */
    use std::io::Read as _;

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
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO
YAR028W	509242864	.	a	ATg	6	.	AC=730431288
NC_016845.1	127722615	.	t	GTTA	97	.	AC=549947617
chr1	993087666	.	a	cAcg	122	.	AC=1985475776
chr1	223251961	.	a	cT	204	.	AC=1940638485
23	1676586806	.	a	AG	248	.	AC=1909766224
";

    #[test]
    fn header_() -> error::Result<()> {
        let mut output = Vec::new();

        header(&mut output)?;

        assert_eq!(output, TRUTH[..628]);

        Ok(())
    }

    #[test]
    fn record_() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        record(&mut output, &mut rng)?;

        assert_eq!(output, TRUTH[628..670]);

        Ok(())
    }

    #[test]
    fn records_() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        records(&mut output, &mut rng, 5)?;

        assert_eq!(output, TRUTH[628..]);

        Ok(())
    }

    #[test]
    fn create_() -> error::Result<()> {
        let mut rng = crate::rand();

        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        let temp_file = temp_path.join("tmp.vcf");

        create(&temp_file, &mut rng, 5)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, TRUTH);

        Ok(())
    }
}
