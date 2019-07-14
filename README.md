# Fonterator
Fonterator is a pure Rust font loader.  When you want to render text, fonterator gives you an
iterator over [footile](https://crates.io/crates/footile) `PathOp`s, which you can easily pass
right into footile.

# Simple Example
In Cargo.toml,

```toml
[dependencies]
fonterator = "0.4"
```

In main.rs,
```rust
use fonterator::FontGroup;
use footile::{FillRule, Plotter, Raster, Rgba8};

fn main() {
    // Load the default FontGroup (font and fallbacks).
    let font = FontGroup::default();

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::new(p.width(), p.height());

    // Render the text
    let mut path = font.render(
        "Héllö,\nWørłd!‽i", /*text*/
        (0.0, 0.0),         /*position*/
        (256.0, 256.0),     /*size*/
    );
    r.over(
        p.fill(&mut path, FillRule::NonZero),
        Rgba8::rgb(0, 0, 0), /*color*/
    );
    r.write_png("main.png").unwrap(); /*save as PNG*/
}
```

## Features
* Load TTF fonts and font collections.
* Load some OTF fonts and font collections.
* Automatic kerning and font layout.
* Horizontal and vertical text layout.
* Left-to-right and right-to-left text layout.
* Use fallback fonts if a character is not available from one font.
* Optional multilingual monospace (CJK is rendered exactly twice the width of Latin).

## TODO
* Support OpenType fonts that aren't TrueType (Supporting cubic bezier curves).
* Support ligatures (‽,æ etc.).
* Support other TrueType variants.

## Links
* [Website](https://code.plopgrizzly.com/fonterator)
* [Cargo](https://crates.io/crates/fonterator)
* [Documentation](https://docs.rs/fonterator)
* [Change Log](https://code.plopgrizzly.com/fonterator/CHANGELOG)
* [Contributors](https://code.plopgrizzly.com/fonterator/CONTRIBUTORS)
* [Code of Conduct](https://code.plopgrizzly.com/fonterator/CODEOFCONDUCT)
