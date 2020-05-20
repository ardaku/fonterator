# Changelog
All notable changes to `fonterator` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://jeronlau.tk/semver/).

## [0.8.0] - 2020-05-20
### Changed
- Update to footile 0.5
- Update to ttf-parser 0.6
- Remove arguments on `Font.render()`, recommend using footile `Transform`s
  instead.

### Fixed
- Some text layout algorithms that were completely wrong

## [0.7.0] - 2020-04-30
### Changed
- Updated to footile 0.4
- `monospace-font` feature is no longer enabled by default.

### Fixed
- Unknown unicode character crashing the program with a panic

## [0.6.0] - 2019-10-24
### Fixed
- `dyn` Warnings
- Clippy Warnings
- No longer panics when character can't be found

### Changed
- `add()` on `Font` is renamed `push()`.
- Updated dependencies

## [0.5.1] - 2019-08-10
### Fixed
- Text being drawn below the bounding box instead of inside.

## [0.5.0] - 2019-08-08
### Added
- `TextAlign` enum (replaces `vertical()`).
- `licenses` function: returns a string of the embedded fonts' licenses.
- `BOLD`, `ITALIC`, `NONE` constants, don't do anything yet.
- Automatic Right to Left detection and glyph reordering.  Gets rid of need for `right_to_left()`, so removed.
- Text wrapping (see `examples/main.rs` for example on how to use)
- `#![forbid(unsafe_code)]`

### Changed
- Use `ttf-parser` crate to support more fonts.
- Use `monospace-font` & `normal-font` features to enable functions `monospace_font()` and `normal_font()`
- WQY MicroHei -> DroidSans Fallback, fixes some Korean text rendering issues.
- `render()` now takes a bounding box.  Makes `xy()` unneeded, so removed.

### Removed
- `multilingual_mono()` because it did bad typefacing.

## [0.4.2] - 2019-07-15
### Fixed
- Release mode renders correctly now.

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
