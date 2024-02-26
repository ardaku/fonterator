use footile::{FillRule, Plotter};
use pix::matte::Matte8;
use pix::ops::SrcOver;
use pix::rgb::{Rgba8p, SRgba8};
use pix::Raster;
use png_pong::{Encoder, PngRaster};
use pointy::Transform; // For saving PNG

fn main() {
    // Load the default FontGroup (font and fallbacks).
    let mut font = fonterator::monospace_font();

    // Init rendering
    let mut p = Plotter::new(Raster::with_clear(2048, 2048));
    let mut r = Raster::with_clear(2048, 2048);

    // Render the text
    let text = "Héllö, Wørłd‽ 野ウサギ a WW野WWウ a wa サW WギWW";
    let mut begin = 0;
    let mut line = 0;
    loop {
        // Get path iterator
        let (path, l) = font.render(
            &text[begin..], /*text*/
            2048.0 / 256.0,
            // fonterator::TextAlign::Left,
        );
        // Clear plotter
        let mut pr = p.into_raster();
        pr.clear();
        p = Plotter::new(pr);
        p.set_transform(Transform::with_scale(256.0, 256.0));
        // Composite
        r.composite_matte(
            (0, line * 256, 2048, 2048),
            p.fill(FillRule::NonZero, path, Matte8::new(255)),
            (),
            Rgba8p::new(0, 0, 0, 255), /*color*/
            SrcOver,
        );
        if let Some(l) = l {
            begin += l;
        }
        line += 1;
        if l.is_none() {
            break;
        }
    }

    // Save PNG
    let raster = PngRaster::Rgba8(Raster::<SRgba8>::with_raster(&r));
    let mut out_data = Vec::new();
    let mut encoder = Encoder::new(&mut out_data).into_step_enc();
    encoder.still(&raster).expect("Failed to add frame");
    std::fs::write("main.png", out_data).expect("Failed to save image");
}
