# Fonterator
Fonterator is a pure Rust font loader.  When you want to render text, fonterator gives you an
iterator over [footile](https://crates.io/crates/footile) `PathOp`s, which you can easily pass
right into footile.

# Simple Example
In Cargo.toml,

```toml
[dependencies]
fonterator = "0.2.0"
```

In main.rs,
```rust
extern crate fonterator;
extern crate footile;

use fonterator::Font;
use footile::{FillRule, Plotter, Raster, Rgba8};

const FONT: &[u8] = include_bytes!("../font/LiberationSans-Regular.ttf");

fn main() {
    // This only succeeds if collection consists of one font
    let font = Font::new(FONT).expect("Failed to load font!");

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::new(p.width(), p.height());
 
    // Render the text
    let path = font.render(
        "Héllö,\nWørłd!", /*text*/
        (0.0, 0.0),       /*position*/
        (256.0, 256.0),   /*size*/
    );
    r.over(
        p.fill(path, FillRule::NonZero),
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

## TODO
* Support OpenType formatted fonts that are not just TrueType fonts (OpenType is
a superset of TrueType). Notably there is no support yet for cubic Bezier curves
used in glyphs.
* Support ligatures of any kind (‽, etc.).
* Support some less common TrueType sub-formats.

## Links
* [Website](https://free.plopgrizzly.com/fonterator)
* [Cargo](https://crates.io/crates/fonterator)
* [Documentation](https://docs.rs/fonterator)
* [Change Log](https://free.plopgrizzly.com/fonterator/changelog)
* [Contributing](https://plopgrizzly.com/contributing)
* [Code of Conduct](https://free.plopgrizzly.com/fonterator/codeofconduct)

---

[![Plop Grizzly](https://plopgrizzly.com/images/logo-bar.png)](https://plopgrizzly.com)