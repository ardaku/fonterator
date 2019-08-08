# Fonterator

[![docs.rs](https://docs.rs/fonterator/badge.svg)](https://docs.rs/fonterator) [![build status](https://api.travis-ci.com/libcala/fonterator.svg?branch=master)](https://travis-ci.com/libcala/fonterator) [![crates.io](https://img.shields.io/crates/v/fonterator.svg)](https://crates.io/crates/fonterator) [![discord](https://img.shields.io/badge/discord-Cala%20Project-green.svg)](https://discord.gg/nXwF59K)

[About](https://libcala.github.io/fonterator) | [Source](https://github.com/libcala/fonterator) | [Changelog](https://libcala.github.io/fonterator/changelog) | [Cala Blog](https://libcala.github.io)

Load fonts as vector graphics in pure Rust with advanced text layout.  When you want to render text, fonterator gives you an iterator over [footile](https://crates.io/crates/footile) `PathOp`s, which you can easily pass right into footile.

# Simple Example
```rust
use fonterator as font; // For parsing font file.
use footile::{FillRule, Plotter, Raster, Rgba8}; // For rendering font text.
use png_pong::{RasterBuilder, EncoderBuilder}; // For saving PNG

const FONT_SIZE: f32 = 32.0;

fn main() {
    // Example Text
    let english = "Raster Text With Font";
    let korean = "글꼴로 래스터 텍스트 사용";
    let japanese = "フォント付きラスタテキスト";

    // Init font, and paths.
    let font = font::monospace_font();

    // Init rendering.
    let mut p = Plotter::new(512, 512);
    let mut r = Raster::new(p.width(), p.height());

    // Render English Left Aligned.
    let path = font.render(
        english,
        (64.0, 0.0, 512.0 - 64.0, 512.0 - FONT_SIZE),
        (FONT_SIZE, FONT_SIZE),
        font::TextAlign::Left
    );
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    // Render Korean Vertically
    let path = font.render(
        korean,
        (0.0, 0.0, 512.0, 512.0 - 32.0 * 7.0),
        (FONT_SIZE, FONT_SIZE),
        font::TextAlign::Vertical
    );
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    // Render Japanese Vertically
    let path = font.render(
        japanese,
        (32.0, 0.0, 512.0, 512.0 - 32.0 * 7.0),
        (FONT_SIZE, FONT_SIZE),
        font::TextAlign::Vertical
    );
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    // Save PNG
    let raster = RasterBuilder::new()
        .with_u8_buffer(512, 512, r.as_u8_slice());
    let mut out_data = Vec::new();
    let mut encoder = EncoderBuilder::new();
    let mut encoder = encoder.encode_rasters(&mut out_data);
    encoder.add_frame(&raster, 0).expect("Failed to add frame");
    std::fs::write("out.png", out_data).expect("Failed to save image");
}
```

## Features
- Load TTF/OTF fonts and font collections.
- Automatic kerning and font layout.
- Horizontal and vertical text layout.
- Left-to-right and right-to-left text layout.
- Use fallback fonts if a character is not available from one font.
- Align text left/center/right/vertical
- Line Wrapping

## TODO
- [Arabic and other script text shaping](https://github.com/plopgrizzly/fonterator/issues/3)
- Better interoperability for monospace when mixing scripts.

# Contributing
Contributors are always welcome!  Whether it is a bug report, bug fix, feature request, feature implementation or whatever.  Don't be shy about getting involved.  I always make time to fix bugs, so usually a patched version of the library will be out soon after a report.  Features take me longer, though.  I'll also always listen to any design critiques you have.  If you have any questions you can email me at jeronlau@plopgrizzly.com.  Otherwise, [here's a link to the issues on GitHub](https://github.com/libcala/fonterator/issues).

And, as always, make sure to always follow the [code of conduct](https://github.com/libcala/fonterator/blob/master/CODEOFCONDUCT.md).  Happy coding!

# License
This repository is licensed under either of the following:

- MIT License (MIT) - See accompanying file [LICENSE_MIT.txt](https://github.com/libcala/fonterator/blob/master/LICENSE_MIT.txt) or copy at https://opensource.org/licenses/MIT
- Boost Software License (BSL-1.0) - See accompanying file [LICENSE_BSL.txt](https://github.com/libcala/fonterator/blob/master/LICENSE_BSL.txt) or copy at https://www.boost.org/LICENSE_1_0.txt

at your option.

## Contribution Licensing
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above without any additional terms or conditions.
