<!-- dx-header -->
# St. Jude Mutational Signatures

Find and quantify COSMIC mutational signatures across samples
<!-- /dx-header -->

This is done by finding the optimal non-negative linear combination of
mutation signatures to reconstruct a mutation matrix. It builds the initial
mutation matrix from multiple single-sample VCFs and, by default, fits it to
[mutational signatures from COSMIC].

mtsg supports both hg19 (GRCh37) and hg38 (GRCh38).

[mutational signatures from COSMIC]: https://cancer.sanger.ac.uk/cosmic/signatures

## Inputs

  * `vcfs`: List of VCF inputs. This can be any list of single or multi-sample
    VCFs. The files can be either uncompressed or gzipped.

  * `sample_sheet`: A tab-delimited file (no headers) with two columns: the
    sample ID and a tag. The tag is any arbitrary identifier, typically a
    disease abbreviation or tissue of origin. If not given, a sample sheet will
    be generated from the sample IDs extracted from the multi-sample VCF.
    [optional]

  * `genome_build`: The genome build used as reference. This can be
    either "GRCh37" (hg19) or "GRCh38" (hg38). [default: "GRCh38"]

  * `min_burden`: Minimum number of somatic SNVs a sample must have to be
    considered. [default: 9]

  * `min_contribution`: Minimum number of mutations attributable to a single
    signature. [default: 9]

  * `prefix`: Prefix to prepend to the output filenames. If blank, the basename
    of `multi_sample_vcf` is used. [optional]

  * `disable_column`: VCF column index (starting from samples, zero-based) to
    ignore when reading VCFs. If not set, all samples are considered. Applies
    to all input VCFs.

    For example, in a VCF with samples `SJEPD003_D` and `SJEPD003_G`, the
    germline sample (`SJEPD003_G`) can be discarded by setting
    `disable_column` to `1`. [optional]

## Outputs

  * `signatures_txt`: A tab-delimited file of the raw results with sample
    contributions for each signature.

  * `signatures_html`: An HTML file that imports `signatures_txt` for
    interactive plotting.

  * `sample_sheet_out`: A tab-delimited file (no headers) with two columns: the
    sample ID and a tag. The tag is any arbitrary identifier, typically a
    disease abbreviation or tissue of origin. If a sample sheet was not
    originally given as an input, this file is the one that was automatically
    generated. It can be reused in subsequent runs. If a sample was given, this
    is a copy of the input.

## Process

Mutational Signatures runs four steps using subcommands of `mtsg`.

  1. split VCFs (single or multi-sample) to multiple single-sample VCFs
  2. generate a sample sheet from the directory of single-sample VCFs
  3. run mutational patterns
  4. create a visualization file using the fitted signatures

## Instance Types

Mutational Signatures is a CPU-bound workflow, spending a majority of its time
reading VCFs and creating the initial mutation matrix. Both of these
operations are multithreaded, meaning that adding more CPUs can improve
performance.

The following table is an example of price vs performance for 652 - 11
filtered samples with `mem1_ssd1_x{2,4,8,16}` instances (as of 2018-08-08).

| cpus |  read | mutmat | total |    cost |
|-----:|------:|-------:|------:|--------:|
|    2 | 14:18 |   3:25 | 37:30 | $0.0841 |
|    4 |  7:17 |   1:42 | 20:52 | $0.0959 |
|    8 |  4:05 |   0:55 | 14:36 | $0.1342 |
|   16 |  2:29 |   0:30 | 11:10 | $0.2052 |

## References

  * Blokzijl F, Janssen R, van Boxtel R, Cuppen E (2018). "MutationalPatterns:
    comprehensive genome-wide analysis of mutational processes." _Genome
    Medicine_. doi: [10.1186/s13073-018-0539-0].

[10.1186/s13073-018-0539-0]: https://doi.org/10.1186/s13073-018-0539-0
