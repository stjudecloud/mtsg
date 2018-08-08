<!-- dx-header -->
# St. Jude Mutational Spectrum

Find the optimal non-negative linear combination of mutation signatures to
reconstruct a mutation matrix
<!-- /dx-header -->

**Mutational Spectrum** (abbreviated syllabically as **mutspec**) finds the
optimal non-negative linear combination of mutation signatures to reconstruct
a mutation matrix. It builds the initial mutation matrix from multiple
single-sample VCFs and, by default, fits it to [mutational signatures from
COSMIC].

mutspec supports both hg19 (GRCh37) and hg38 (GRCh38).

[mutational signatures from COSMIC]: https://cancer.sanger.ac.uk/cosmic/signatures

## Inputs

  * `multi_sample_vcf`: A multi-sample VCF.

  * `sample-sheet`: A tab-delimited file (no headers) with two columns: the
    sample ID and a tag. The tag is any arbitrary identifier, typically a
    disease abbreviation or tissue of origin. If not given, a sample sheet will
    be generated from sample IDs extracted from the multi-sample VCF.
    [optional]

  * `genome_build`: The genome build used to align the input. This can be
    either "GRCh37" (hg19) or "GRCh38" (hg38) [default: "GRCh38"].

  * `min_burden`: Threshold to exclude mutations with low burden [default: 9].

  * `min_contribution`: Threshold to exclude signatures with low contribution [default: 9].

## Outputs

  * `signatures_txt`: A tab-delimited file of the raw results with sample
    contributions for each signature.

  * `signatures_html`: An HTML file that imports `signatures_txt` for
    interactive plotting.

## Process

Mutational Spectrum runs four steps using subcommands of `mutspec`.

  1. split a multi-sample VCF to multiple single-sample VCFs
  2. generate a sample sheet from the directory of single-sample VCFs
  3. run mutational patterns
  4. create a visualization file using the fitted signatures

## Instance Types

Mutational Spectrum is a CPU-bound workflow, spending a majority of its time
reading VCFs and creating the initial mutation matrix. Both of these
operations are multithreaded, meaning that adding more CPUs can improve
performance.

The following table is an example of price vs performance for 652 - 11
filtered samples with `mem1_ssd1_x{2,4,8,16}` instances.

| cpus |  read | mutmat | total |    cost |
|-----:|------:|-------:|------:|--------:|
|    2 | 14:18 |   3:25 | 37:30 | $0.0841 |
|    4 |  7:17 |   1:42 | 20:52 | $0.0959 |
|    8 |  4:05 |   0:55 | 14:36 | $0.1342 |
|   16 |  2:29 |   0:30 | 11:10 | $0.2052 |
