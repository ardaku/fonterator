use footile::{FillRule, Plotter, Raster, Rgba8};

fn main() {
    // Load the default FontGroup (font and fallbacks).
    let font = fonterator::monospace_font();

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::new(p.width(), p.height());

    // Render the text
    let path = font.render(
        "Héllö, Wørłd!‽i 野ウサギ WW野WWウ サWWギWW",     /*text*/
        (0.0, 0.0, 2048.0, 2048.0),                     /*bbox*/
        (256.0, 256.0),                                 /*size*/
        fonterator::TextAlign::Left,
    );
    let path: Vec<footile::PathOp> = path.collect();
    r.over(
        p.fill(&path, FillRule::NonZero),
        Rgba8::rgb(0, 0, 0), /*color*/
    );
    r.write_png("main.png").unwrap(); /*save as PNG*/
}
