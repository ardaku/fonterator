use pix::Raster;
use pix::ops::SrcOver;
use pix::rgb::{SRgba8, Rgba8p};
use footile::{FillRule, Plotter};
use png_pong::FrameEncoder; // For saving PNG

fn main() {
    // Load the default FontGroup (font and fallbacks).
    let font = fonterator::monospace_font();

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::with_clear(p.width(), p.height());

    // Render the text
    let text = "Héllö, Wørłd‽ 野ウサギ a WW野WWウ a wa サW WギWW";
    let mut begin = 0;
    let mut line = 0;
    loop {
        let (path, l) = font.render(
            &text[begin..],                             /*text*/
            (0.0, line as f32 * 256.0, 2048.0, 2048.0), /*bbox*/
            (256.0, 256.0),                             /*size*/
            fonterator::TextAlign::Left,
        );
        println!("{} {}", begin, begin + l);
        let path: Vec<footile::PathOp> = path.collect();
        r.composite_matte((0, 0, 2048, 2048), 
            p.fill(&path, FillRule::NonZero),
            (),
            Rgba8p::new(0, 0, 0, 255), /*color*/
            SrcOver,
        );
        begin += l;
        line += 1;
        if l == 0 {
            break;
        }
    }

    // Save PNG
    let raster = Raster::<SRgba8>::with_raster(&r);
    let mut out_data = Vec::new();
    let mut encoder = FrameEncoder::new(&mut out_data);
    encoder.still(&raster).expect("Failed to add frame");
    std::fs::write("main.png", out_data).expect("Failed to save image");
}
