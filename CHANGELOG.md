# Changelog

## Unreleased

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
