# fonterator

#### Load fonts as vector graphics in pure Rust with advanced text layout.

[![Build Status](https://api.travis-ci.org/libcala/fonterator.svg?branch=master)](https://travis-ci.org/libcala/fonterator)
[![Docs](https://docs.rs/fonterator/badge.svg)](https://docs.rs/fonterator)
[![crates.io](https://img.shields.io/crates/v/fonterator.svg)](https://crates.io/crates/fonterator)

When you want to render text, fonterator gives you an iterator over
[footile](https://crates.io/crates/footile) `PathOp`s, which you can easily
pass right into footile.

- Loads TTF/OTF fonts and font collections.
- Automatic kerning and font layout.
- Horizontal and vertical text layout.
- Left-to-right and right-to-left text layout.
- Uses fallback fonts if a character is not available from one font.
- Can Align text left/center/right/vertical
- Line Wrapping

### Todo
- [Arabic and other script text shaping](https://github.com/plopgrizzly/fonterator/issues/3)
- Better interoperability for monospace when mixing scripts.

## Table of Contents
- [Getting Started](#getting-started)
   - [Example](#example)
   - [API](#api)
   - [Features](#features)
- [Upgrade](#upgrade)
- [License](#license)
   - [Contribution](#contribution)

## Getting Started
Add the following to your `Cargo.toml`.

```toml
[dependencies]
fonterator = "0.7"
```

### Example
```rust,no_run
use fonterator as font; // For parsing font file.
use footile::{FillRule, Plotter, PathOp}; // For rendering font text.
use png_pong::FrameEncoder; // For saving PNG
use pix::{
    Raster,
    rgb::{Rgba8p, SRgba8},
    matte::{Matte8},
    ops::{SrcOver}
};

const FONT_SIZE: f32 = 32.0;

fn main() {
    // Example Text
    let english = "Raster Text With Font";
    let korean = "글꼴로 래스터 텍스트 사용";
    let japanese = "フォント付きラスタテキスト";

    // Init font, and paths.
    let font = font::monospace_font();

    // Init rendering.
    let mut p = Plotter::new(Raster::with_clear(512, 512));
    let mut r = Raster::with_clear(512, 512);

    // Render English Left Aligned.
    let path = font.render(
        english,
        512.0 - 64.0,
        (FONT_SIZE, FONT_SIZE),
        font::TextAlign::Left
    ).0;
    r.composite_matte(
        (64, 0, 512, 512),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );

    // Render Korean Vertically
    let path = font.render(
        korean,
        512.0,
        (FONT_SIZE, FONT_SIZE),
        font::TextAlign::Vertical
    ).0;
    r.composite_matte(
        (0, 0, 512, 512),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );

    // Render Japanese Vertically
    let path = font.render(
        japanese,
        512.0 - 32.0 * 7.0,
        (FONT_SIZE, FONT_SIZE),
        font::TextAlign::Vertical
    ).0;
    r.composite_matte(
        (32, 0, 512, 512),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );

    // Save PNG
    let raster = Raster::<SRgba8>::with_raster(&r);
    let mut out_data = Vec::new();
    let mut encoder = FrameEncoder::new(&mut out_data);
    encoder.still(&raster).expect("Failed to add frame");
    std::fs::write("out.png", out_data).expect("Failed to save image");
}
```

### API
API documentation can be found on [docs.rs](https://docs.rs/fonterator).

### Features
#### `monospace-font`
Embeds a monospace font accessible with the `monospace_font()` public API in
the root of the crate.

#### `normal-font`
Embeds a variable-width font accessible with the `normal_font()` public API in
the root of the crate.

## Upgrade
You can use the
[changelog](https://github.com/libcala/fonterator/blob/master/CHANGELOG.md)
to facilitate upgrading this crate as a dependency.

## License
Licensed under either of
 - Apache License, Version 2.0,
   ([LICENSE-APACHE](https://github.com/libcala/fonterator/blob/master/LICENSE-APACHE) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
 - Zlib License,
   ([LICENSE-ZLIB](https://github.com/libcala/fonterator/blob/master/LICENSE-ZLIB) or
   [https://opensource.org/licenses/Zlib](https://opensource.org/licenses/Zlib))

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Contributors are always welcome (thank you for being interested!), whether it
be a bug report, bug fix, feature request, feature implementation or whatever.
Don't be shy about getting involved.  I always make time to fix bugs, so usually
a patched version of the library will be out a few days after a report.
Features requests will not complete as fast.  If you have any questions, design
critques, or want me to find you something to work on based on your skill level,
you can email me at [jeronlau@plopgrizzly.com](mailto:jeronlau@plopgrizzly.com).
Otherwise,
[here's a link to the issues on GitHub](https://github.com/libcala/fonterator/issues).
Before contributing, check out the
[contribution guidelines](https://github.com/libcala/fonterator/blob/master/CONTRIBUTING.md),
and, as always, make sure to follow the
[code of conduct](https://github.com/libcala/fonterator/blob/master/CODE_OF_CONDUCT.md).
