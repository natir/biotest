<h1 style="text-align: center;">biotest</h1>

[![License](https://img.shields.io/badge/license-MIT-green)](https://github.com/natir/biotest/blob/master/LICENSE)
![Test](https://github.com/natir/biotest/workflows/Test/badge.svg)
![Lints](https://github.com/natir/biotest/workflows/Lints/badge.svg)
![MSRV](https://github.com/natir/biotest/workflows/MSRV/badge.svg)
[![codecov](https://codecov.io/gh/natir/biotest/graph/badge.svg?token=7KY1Z4RHDB)](https://codecov.io/gh/natir/biotest)
[![Documentation](https://github.com/natir/biotest/workflows/Documentation/badge.svg)](https://natir.github.io/biotest/biotest)

Generate random test data for bioinformatics

## Usage

In your Cargo.toml add
```toml
biotest = { version = "0.1", features = ["fasta", "fastq", "vcf"] }
```

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.74.
