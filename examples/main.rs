use footile::{FillRule, Plotter, Raster, Rgba8};

fn main() {
    // Load the default FontGroup (font and fallbacks).
    let font = fonterator::monospace_font();

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::new(p.width(), p.height());

    // Render the text
    let text = "Héllö, Wørłd‽ 野ウサギ a WW野WWウ a wa サW WギWW";
    let mut begin = 0;
    let mut line = 0;
    loop {
        let (path, l) = font.render(
            &text[begin..],     /*text*/
            (0.0, line as f32 * 256.0, 2048.0, 2048.0),                     /*bbox*/
            (256.0, 256.0),                                 /*size*/
            fonterator::TextAlign::Left,
        );
        println!("{} {}", begin, begin + l);
        let path: Vec<footile::PathOp> = path.collect();
        r.over(
            p.fill(&path, FillRule::NonZero),
            Rgba8::rgb(0, 0, 0), /*color*/
        );
        begin += l;
        line += 1;
        if l == 0 { break }
    }
    r.write_png("main.png").unwrap(); /*save as PNG*/
}
