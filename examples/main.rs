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
