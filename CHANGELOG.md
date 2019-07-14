# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://free.plopgrizzly.com/semver/).

## [Unreleased]
### Added
### Removed
### Changed
### Fixed

## [0.4.1] - 2019-07-14
### Fixed
- Multi-lingual monospace not spacing correctly.

## [0.4.0] - 2019-05-13
### Added
- You can now use footile from fonterator with `use fonterator::footile;`
- Method `xy()` on `PathIterator`

### Changed
- Upgrade to newer version of footile (`0.3`).
- Renamed `FontChain` to `FontGroup`.
- `PathIterator` now iterates by mutable reference.

### Fixed
- Not all examples working.

## [0.3.0] - 2018-12-09
### Added
- `FontChain` for default and fallback fonts.  `FontChain` is an abstraction over the old `Font` and `FontCollection` structs, which are no longer part of the public API.
- Proper support for multi-lingual monospace with `FontChain::multilingual_mono()`.

## [0.2.1] - 2018-12-08
### Fixed
- Fix README.

## [0.2.0] - 2018-12-02
### Added
- Added vertical text layout support.
- Added right-to-left text layout support.

### Changed
- Depend on Footile for `PathOp`s rather than afi.
- Simpler `render` API replaces old nested iterator stuff.

## [0.1.0] - 2018-05-25
### Added
- Added to crates.io
