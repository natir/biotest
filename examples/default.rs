//! Generate file with default value

#![warn(missing_docs)]

/* std use */

/* crate use */
use clap::Parser as _;
use rand::SeedableRng;

/* project use */
use biotest::error;

use biotest::Format as _;

/// Select type of file too generate
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Type {
    /// Generate a fasta file
    Fasta,

    /// Generate a fastq file
    Fastq,

    /// Generate a vcf file
    Vcf,
}

/// Example: {{project_description}}
#[derive(clap::Parser, std::fmt::Debug)]
#[clap(
    name = "biotest_default",
    version = "0.1",
    author = "Pierre Marijon <pierre@marijon.fr>"
)]
pub struct Command {
    /// Output path
    #[clap(short = 'o', long = "output")]
    pub output_path: std::path::PathBuf,

    /// Number of record
    #[clap(short = 'n', long = "number-record")]
    pub number_record: u64,

    /// Type of output
    #[clap(short = 't', long = "type")]
    pub out_type: Type,

    /// Silence all output
    #[clap(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[clap(short = 'v', long = "verbosity", action = clap::ArgAction::Count)]
    pub verbosity: u8,

    /// Timestamp (sec, ms, ns, none)
    #[clap(short = 'T', long = "timestamp")]
    pub ts: Option<stderrlog::Timestamp>,
}

fn main() -> error::Result<()> {
    // parse cli
    let params = Command::parse();

    // Setup logger
    stderrlog::new()
        .quiet(params.quiet)
        .verbosity(params.verbosity as usize)
        .timestamp(params.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();

    let mut rng = rand::rngs::StdRng::from_entropy();

    match params.out_type {
        Type::Fasta => biotest::Fasta::builder().build().unwrap().create(
            params.output_path,
            &mut rng,
            params.number_record as usize,
        ),
        Type::Fastq => biotest::Fastq::builder().build().unwrap().create(
            params.output_path,
            &mut rng,
            params.number_record as usize,
        ),
        Type::Vcf => biotest::Vcf::builder().build().unwrap().create(
            params.output_path,
            &mut rng,
            params.number_record as usize,
        ),
    }
}
