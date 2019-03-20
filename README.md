# Mutational Signatures

[![Build Status](https://travis-ci.org/stjude/mtsg.svg?branch=master)](https://travis-ci.org/stjude/mtsg)

**Mutational Signatures** (abbreviated as **mtsg**) finds and quantifies COSMIC
mutational signatures across samples.

This is done by finding the optimal non-negative linear combination of
mutation signatures to reconstruct a mutation matrix. It builds the initial
mutation matrix from multiple single-sample VCFs and, by default, fits it to
[mutational signatures from COSMIC].

mtsg supports both hg19 (GRCh37) and hg38 (GRCh38).

[mutational signatures from COSMIC]: https://cancer.sanger.ac.uk/cosmic/signatures

## Prerequisites

  * [R] ^3.4.4
    * [BSgenome] ~1.48.0
    * [BSgenome.Hsapiens.UCSC.hg19] ~1.4.0
    * [BSgenome.Hsapiens.UCSC.hg38] ~1.4.1
    * [GenomicRanges] ~1.32.6
    * [MutationalPatterns] ~1.6.1
    * [rtracklayer] ~1.40.3
  * [Rust] ^1.31.0

[R]: https://www.r-project.org/
[MutationalPatterns]: https://bioconductor.org/packages/release/bioc/html/MutationalPatterns.html
[BSgenome]: https://bioconductor.org/packages/release/bioc/html/BSgenome.html
[BSgenome.Hsapiens.UCSC.hg19]: https://bioconductor.org/packages/release/data/annotation/html/BSgenome.Hsapiens.UCSC.hg19.html
[BSgenome.Hsapiens.UCSC.hg38]: https://bioconductor.org/packages/release/data/annotation/html/BSgenome.Hsapiens.UCSC.hg38.html
[rtracklayer]: https://bioconductor.org/packages/release/bioc/html/rtracklayer.html
[GenomicRanges]: https://bioconductor.org/packages/release/bioc/html/GenomicRanges.html
[Rust]: https://rust-lang.org/

## Installation

Install [Rust] and use `cargo` to install `mtsg`.

```
$ cargo install --git https://github.com/stjude/mtsg.git
```

[Rust]: https://www.rust-lang.org/tools/install

## Usage

```
mtsg 2.0.0

USAGE:
    mtsg [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Use verbose logging

SUBCOMMANDS:
    download-signatures      Downloads and preprocesses known mutational signatures (COSMIC)
    generate-sample-sheet    Generates a sample sheet from a directory of VCFs
    help                     Prints this message or the help of the given subcommand(s)
    run                      Finds the linear combination of mutation signatures that reconstructs the mutation
                             matrix
    split-vcf                Splits a multi-sample VCF to multiple single-sample VCFs
    visualize                Creates an interactive visualization for the given cancer signatures
```

Run `mtsg <subcommand> --help` for more information about each subcommand.

A common workflow would be to 1) split a multi-sample VCF into single-sample
VCFs, 2) create a sample sheet mapping sample IDs to diseases, 3) run
mutational patterns, and 4) visualize the results.

```
$ mtsg split-vcf --output-directory vcfs in.vcf
$ mtsg generate-sample-sheet --output sample-sheet.txt vcfs
$ mtsg run --output-directory results vcfs sample-sheet.txt
$ mtsg visualize --output results/signatures.html results/signatures.txt
```

## Docker

Mutational Signatures has a `Dockerfile` to create a Docker image, which sets
up and installs all the required dependencies. To build and use this image,
[install Docker](https://docs.docker.com/install) for your platform.

### Build

In the Mutational Signatures project directory, build the Docker image.

```
$ docker build --tag mtsg .
```

### Run

The image uses `mtsg` as its entrypoint, giving access to all commands.

```
$ docker run mtsg <args...>
```

The following example mounts the necessary files/directories and runs the
`run` command.

```
$ docker run \
    --mount type=bind,source=$DATA_DIR,target=/data,readonly \
    --mount type=bind,source=$RESULTS_DIR,target=/results \
    mtsg \
    --verbose \
    run \
    --output-directory /results \
    /data/vcfs \
    /data/sample-sheet.txt
```

## References

  * Blokzijl F, Janssen R, van Boxtel R, Cuppen E (2018). "MutationalPatterns:
    comprehensive genome-wide analysis of mutational processes." _Genome
    Medicine_. doi: [10.1186/s13073-018-0539-0].

[10.1186/s13073-018-0539-0]: https://doi.org/10.1186/s13073-018-0539-0
