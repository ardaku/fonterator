extern crate fonterator;
extern crate footile;

use fonterator::FontChain;
use footile::{FillRule, Plotter, Raster, Rgba8};

fn main() {
    // Load the default FontChain (font and fallbacks).
    let font = FontChain::default();

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::new(p.width(), p.height());

    // Render the text
    let path = font.render(
        "Héllö,\nWørłd!‽i\n野ウサギ\nWW野WWウ\nサWWギWW", /*text*/
        (0.0, 0.0),                                                     /*position*/
        (256.0, 256.0),                                                 /*size*/
    );
    r.over(
        p.fill(path, FillRule::NonZero),
        Rgba8::rgb(0, 0, 0), /*color*/
    );
    r.write_png("main.png").unwrap(); /*save as PNG*/
}
