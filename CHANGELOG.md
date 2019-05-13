# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://free.plopgrizzly.com/semver/).

## [Unreleased]
### Added
### Removed
### Changed
### Fixed

## [0.3.0]
### Added
- `FontChain` for default and fallback fonts.  `FontChain` is an abstraction over the old `Font` and `FontCollection` structs, which are no longer part of the public API.
- Proper support for multi-lingual monospace with `FontChain::multilingual_mono()`.

## [0.2.1]
### Fixed
- Fix README.

## [0.2.0]
### Added
- Added vertical text layout support.
- Added right-to-left text layout support.

### Changed
- Depend on Footile for `PathOp`s rather than afi.
- Simpler `render` API replaces old nested iterator stuff.

## [0.1.0] - 2019-05-13
### Added
- Added to crates.io
