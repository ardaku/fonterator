# Fonterator
Load fonts as vector graphics in pure Rust - based on RustType

[Cargo](https://crates.io/crates/fonterator) /
[Documentation](https://docs.rs/fonterator) /
[Change Log](http://plopgrizzly.com/fonterator/changelog.html)

## Features
**fonterator**'s current features:
* Load TTF fonts and font collections.
* Load some OTF fonts and font collections.
* Automatic kerning and font layout.
* Horizontal and vertical text layout.
* Left-to-right and right-to-left text layout.

**fonterator**'s planned features:
* Support OpenType formatted fonts that are not just TrueType fonts (OpenType is
a superset of TrueType). Notably there is no support yet for cubic Bezier curves
used in glyphs.
* Support ligatures of any kind (‽, etc.).
* Support some less common TrueType sub-formats.
