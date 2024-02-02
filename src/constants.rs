//! Declarations of some constants value

/* std use */

/* crates use */

/* projet use */

const fn gen_array<const N: usize, const B: usize>() -> [u8; N] {
    let mut array = [0; N];

    let mut i = 0;
    while i < N {
        array[i] = (B + i) as u8;
        i += 1;
    }

    array
}

/// Fixed random seed
pub const SEED: [u8; 32] = [42; 32];

/// Nucleotides with any case
pub const NUCLEOTIDES: [u8; 8] = *b"ACTGactg";

/// Nucleotides lower
pub const NUCLEOTIDES_LOWER: [u8; 4] = *b"actg";

/// Nucleotides upper
pub const NUCLEOTIDES_UPPER: [u8; 4] = *b"ACTG";

/// All possible phred 33 value
pub const PHRED33: [u8; 40] = gen_array::<40, 33>();

/// All possible phred 64 value
pub const PHRED64: [u8; 40] = gen_array::<40, 64>();

/// Alphabets with [ \ ] ^ _ `
pub const ALPHABETS: [u8; 58] = gen_array::<58, 65>();

/// Some different possible chromosomes name
pub const CHROMOSOMES: [&[u8]; 10] = [
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

#[cfg(test)]
mod tests {
    /* project use */
    use super::*;

    #[test]
    fn phred33() {
        assert_eq!(
            gen_array::<40, 33>().to_vec(),
            b"!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGH".to_vec()
        );
    }

    #[test]
    fn phred64() {
        assert_eq!(
            gen_array::<40, 64>().to_vec(),
            b"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefg".to_vec()
        );
    }

    #[test]
    fn alphapets() {
        assert_eq!(
            gen_array::<58, 65>().to_vec(),
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz".to_vec()
        );
    }
}
