//! Fastq generation

/* std use */

/* crates use */

/* projet use */
use crate::error;

fn description<W>(
    output: &mut W,
    rng: &mut rand::rngs::StdRng,
    id: usize,
    comment: usize,
) -> error::Result<()>
where
    W: std::io::Write,
{
    output.write_all(&[b'@'])?;
    crate::text(output, rng, id)?;
    output.write_all(&[b' '])?;
    crate::text(output, rng, comment)?;

    Ok(())
}

/// Write record
pub fn record<W>(
    output: &mut W,
    rng: &mut rand::rngs::StdRng,
    id: usize,
    comment: usize,
    seq_len: usize,
) -> error::Result<()>
where
    W: std::io::Write,
{
    description(output, rng, id, comment)?;
    output.write_all(&[b'\n'])?;
    crate::sequence(output, rng, seq_len)?;
    output.write_all(b"\n+\n")?;
    crate::quality(output, rng, seq_len)?;

    Ok(())
}

/// Write multiple record
pub fn records<W>(
    output: &mut W,
    rng: &mut rand::rngs::StdRng,
    id: usize,
    comment: usize,
    seq_len: usize,
    num_record: usize,
) -> error::Result<()>
where
    W: std::io::Write,
{
    for _ in 0..num_record {
        record(output, rng, id, comment, seq_len)?;
        output.write_all(&[b'\n'])?;
    }

    Ok(())
}

/// Create a fasta file
pub fn create<P>(
    path: P,
    rng: &mut rand::rngs::StdRng,
    id: usize,
    comment: usize,
    seq_len: usize,
    num_record: usize,
) -> error::Result<()>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let mut output = std::fs::File::create(&path)?;

    records(&mut output, rng, id, comment, seq_len, num_record)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    /* std use */
    use std::io::Read;

    /* project use */
    use super::*;

    const TRUTH: &[u8] = b"@oNi_P dzwC[tBTlD
tCGCgtGTTAGTTAagccAcggtAatGcTtgtaCgcAGgAtaTcgAAtTa
+
,30C5-D.$.=A@2/&=\'6A0A$@D&4,1+=!/\'@ED:C577DF%\"%>.0
@k_sGD gZcCc]tIGE
tGCAAtTacCGtTAAcaGGtatTCaTCctcTGgAActTgCGAcaAgaAAt
+
6+9#(7E7<??C;*184,;E>-\"=BH3?\"6;%13=A-?!2FH!>\"1\'%))
@K`HVk goY`vkxarZ
ttTGtgCttaAGggTcCTGcGTAGCTGTCCACgTTTGagtGaGCatAGGA
+
\'!H\":,=$*$6*-95FH5D2?BA,+@58%75BH0D?G0+@E&?D>\")&,B
@F_fww GspJRS\\aPw
TCAGgCtaGTtcCCTcgcTgAgGgAtCAAatTCTATTGTaggcGCaCcCG
+
A=(@9!DA+-D/,:*B7C+\'=07$C&&C9%H;B=!6&>1\"AD6+2#?54/
@Uz^rc VndZg_IpyM
tGcTAGCCAgaTTgcAaTtaTGgACTTagGgtATACCtcTctCAtgCGCa
+
D,\',GB55&(!**$F=@0?3G183F?>6<.C$$6AB2FH4#E<1?-@$.+
";

    #[test]
    fn record_() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        record(&mut output, &mut rng, 5, 10, 50)?;

        assert_eq!(output, TRUTH.to_vec()[..121]);

        Ok(())
    }

    #[test]
    fn records_() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        records(&mut output, &mut rng, 5, 10, 50, 5)?;

        assert_eq!(output, TRUTH.to_vec());

        Ok(())
    }

    #[test]
    fn create_() -> error::Result<()> {
        let mut rng = crate::rand();

        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        let temp_file = temp_path.join("tmp.fasta");

        create(&temp_file, &mut rng, 5, 10, 50, 5)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, TRUTH.to_vec());

        Ok(())
    }
}
