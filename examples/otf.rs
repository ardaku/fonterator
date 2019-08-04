use fonterator::FontGroup;
use footile::{FillRule, Plotter, Raster, Rgba8};

const FONT: &'static [u8] = include_bytes!("scorlatti-26.otf");

fn main() {
    // Load the default FontGroup (font and fallbacks).
    let font = FontGroup::new().add(FONT).unwrap();

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
