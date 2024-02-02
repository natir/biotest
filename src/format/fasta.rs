//! Fasta generation

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
    output.write_all(&[b'>'])?;
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

    const TRUTH: &[u8] = b">oNi_P dzwC[tBTlD
tCGCgtGTTAGTTAagccAcggtAatGcTtgtaCgcAGgAtaTcgAAtTa
>rQ_[V S^RtSvzMeT
ttGCtCatGtctgCTGGTACtgTgcaaaagggGAGacAtgCtGCAAtTac
>HYNm[ QBCgL`Scxx
GGtatTCaTCctcTGgAActTgCGAcaAgaAAtaTCCcAgagggaCcttC
>gNXcb hRd]QWyFOg
gAACcTtCttAacGtTtAtGTgACAGCCaCGctGagattTGtgCttaAGg
>ppugI LwOFhYRxBZ
CTGTCCACgTTTGagtGaGCatAGGACAAaacTaTTagagGtatAGCcTa
";

    #[test]
    fn record_() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        record(&mut output, &mut rng, 5, 10, 50)?;

        assert_eq!(output, TRUTH.to_vec()[..68]);

        Ok(())
    }

    #[test]
    fn records_() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        records(&mut output, &mut rng, 5, 10, 50, 5)?;

        assert_eq!(output, TRUTH);

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

        assert_eq!(data, TRUTH);

        Ok(())
    }
}
