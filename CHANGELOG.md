# Changelog

## Unreleased

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

[2.0.0]: https://github.com/stjude/mtsg/compare/v1.0.0...v2.0.0
