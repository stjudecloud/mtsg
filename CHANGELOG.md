# Changelog

## [2.3.0] - 2020-09-21

### Added

  * main: Add context to error messages.

  * visualizations: Show signature etiologies.

### Changed

  * vcf: Skip the genotype when it is fully missing (i.e., `.`) or every field
    is missing (e.g., `.:.`).

## [2.2.0] - 2020-05-11

### Added

  * vcf: When splitting a VCF, warn when the output VCF already exists.

  * sample-sheet: When creating a sample sheet, warn when no VCFs are loaded.

### Changed

  * Raise mininum supported Rust version to 1.40.0.

  * visualize: Link to COSMIC Mutational Signatures v2.

## [2.1.0] - 2019-05-16

### Changed

  * Rather than the default set of ten repeated colors, the colors used in the
    signature contributions stacked bar chart are distinct.

## [2.0.0] - 2019-03-20

### Changed

  * [BREAKING] Renamed project to Mutational Signatures (mtsg). Update bin
    references from `mutspec` to `mtsg`. The CLI commands remain the same.

  * Return an error instead of crashing when the default COSMIC signature
    probabilities cannot be downloaded.

### Fixed

  * Both variants of SJIDs are parsed to extract the disease code for the
    sample sheet. This fixes codes like AMLM7 previously being extracted as
    AMLM.

## 1.0.0 - 2018-09-04

  * Initial release

[2.3.0]: https://github.com/stjude/mtsg/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/stjude/mtsg/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/stjude/mtsg/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/stjude/mtsg/compare/v1.0.0...v2.0.0
