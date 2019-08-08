# Fonterator
Fonterator is a pure Rust font renderer.  When you want to render text, fonterator gives you an iterator over [footile](https://crates.io/crates/footile) `PathOp`s, which you can easily pass right into footile.

# Simple Example
```rust
use fonterator as font; // For parsing font file.
use footile::{FillRule, Plotter, Raster, Rgba8}; // For rendering font text.
use png_pong::{RasterBuilder, EncoderBuilder}; // For saving PNG

const FONT_SIZE: f32 = 32.0;

fn main() {
    // Most common
    let english = "Raster Text With Font"; // LEFT-RIGHT
    let korean = "글꼴로 래스터 텍스트 사용"; // UP-DOWN, RIGHT-LEFT
    let japanese = "フォント付きラスタテキスト"; // UP-DOWN, RIGHT-LEFT

    // Init font, and paths.
    let font = font::monospace_font();

    // Init rendering.
    let mut p = Plotter::new(512, 512);
    let mut r = Raster::new(p.width(), p.height());

    // Render English Left Aligned.
    let path = font.render(english, (64.0, 0.0, 512.0 - 64.0, 512.0 - FONT_SIZE), (FONT_SIZE, FONT_SIZE), font::TextAlign::Left);
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    // Render Korean Vertically
    let path = font.render(korean, (0.0, 0.0, 512.0, 512.0 - 32.0 * 7.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Vertical);
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    // Render Japanese Vertically
    let path = font.render(japanese, (32.0, 0.0, 512.0, 512.0 - 32.0 * 7.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Vertical);
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    // Save PNG
    let raster = RasterBuilder::new().with_u8_buffer(512, 512, r.as_u8_slice());
    let mut out_data = Vec::new();
    let mut encoder = EncoderBuilder::new();
    let mut encoder = encoder.encode_rasters(&mut out_data);
    encoder.add_frame(&raster, 0).expect("Failed to add frame");
    std::fs::write("dir.png", out_data).expect("Failed to save image");
}
```

## Features
* Load TTF/OTF fonts and font collections.
* Automatic kerning and font layout.
* Horizontal and vertical text layout.
* Left-to-right and right-to-left text layout.
* Use fallback fonts if a character is not available from one font.
* Align text left/center/right/vertical 

## TODO
- [Arabic and other script text shaping](https://github.com/plopgrizzly/fonterator/issues/3)
- Better interoperability for monospace when mixing scripts.

## Links
* [Website](https://code.plopgrizzly.com/fonterator)
* [Cargo](https://crates.io/crates/fonterator)
* [Documentation](https://docs.rs/fonterator)
* [Change Log](https://code.plopgrizzly.com/fonterator/CHANGELOG)
* [Contributors](https://code.plopgrizzly.com/fonterator/CONTRIBUTORS)
* [Code of Conduct](https://code.plopgrizzly.com/fonterator/CODEOFCONDUCT)
