//! Declarations of some constants value

/* std use */

/* crates use */

/* projet use */

pub(crate) const fn gen_array<const N: usize, const B: usize>() -> [u8; N] {
    let mut array = [0; N];

    let mut i = 0;
    while i < N {
        array[i] = (B + i) as u8;
        i += 1;
    }

    array
}

/// Fixed random seed
pub static SEED: [u8; 32] = [42; 32];

pub(crate) const ASCII_VISIBLE: [u8; 94] = gen_array::<94, 33>();

/// Nucleotides with any case
pub static NUCLEOTIDES: [u8; 8] = *b"ACTGactg";

pub(crate) const DNA_NUCLEOTIDES: [u8; 8] = *b"ACTGactg";
pub(crate) const RNA_NUCLEOTIDES: [u8; 8] = *b"ACUGacug";

/// Some different possible chromosomes name
pub static CHROMOSOMES: [&[u8]; 10] = [
    b"chr1",
    b"23",
    b"93",
    b"chrMT",
    b"X",
    b"NC_000015.10",
    b"ENA|LT795502|LT795502.1",
    b"NC_016845.1",
    b"YAR028W",
    b"1",
];

/// All vcf info type
pub static VCF_INFO_TYPE: [&[u8]; 5] = [b"Integer", b"Float", b"Flag", b"Character", b"String"];

/// All vcf info number
pub static VCF_INFO_NUMBER: [&[u8]; 6] = [b"1", b"2", b"A", b"R", b"G", b"."];

/// All vcf info type
pub static VCF_FORMAT_TYPE: [&[u8]; 4] = [b"Integer", b"Float", b"Character", b"String"];

/// All vcf info number
pub static VCF_FORMAT_NUMBER: [&[u8]; 6] = [b"1", b"2", b"A", b"R", b"G", b"."];

/// biotest version
pub const BIOTEST_VERSION: &[u8] = env!("CARGO_PKG_VERSION").as_bytes();

#[cfg(test)]
mod tests {
    /* project use */
    use super::*;

    #[test]
    fn ascii_visible() {
        assert_eq!(ASCII_VISIBLE, gen_array::<94, 33>())
    }
}
