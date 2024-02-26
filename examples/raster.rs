use footile::{FillRule, Plotter};
use pix::matte::Matte8;
use pix::ops::SrcOver;
use pix::rgb::{Rgba8p, SRgba8};
use pix::Raster;
use png_pong::{Encoder, PngRaster};
use pointy::Transform; // For saving PNG

const FONT_SIZE: f32 = 200.0;

const STR: &str = "sphinx of black\nquartz, judge\nmy vow";

fn main() {
    let mut font = fonterator::monospace_font();

    // Init rendering
    let mut p = Plotter::new(Raster::with_clear(2048, 2048));
    let mut r = Raster::with_clear(2048, 2048);
    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));

    // FIXME: Align Center
    let mut start = 0;
    let mut row = 0;
    loop {
        let (path, left) = font.render(&STR[start..], 2048.0 / FONT_SIZE);
        r.composite_matte(
            (0, row * 200, 2048, 2048),
            p.fill(FillRule::NonZero, path, Matte8::new(255)),
            (),
            Rgba8p::new(0, 0, 0, 255),
            SrcOver,
        );
        // Reset plotter
        let mut pr = p.into_raster();
        pr.clear();
        p = Plotter::new(pr);
        p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));
        //
        if let Some(left) = left {
            start += left;
            row += 1;
        } else {
            break;
        }
    }

    // FIXME: Align Right
    let mut start = 0;
    let mut row = 0;
    loop {
        let (path, left) = font.render(&STR[start..], 2048.0 / FONT_SIZE);
        r.composite_matte(
            (0, 1024 + row * 200, 2048, 2048),
            p.fill(FillRule::NonZero, path, Matte8::new(255)),
            (),
            Rgba8p::new(0, 0, 0, 255),
            SrcOver,
        );
        // Reset plotter
        let mut pr = p.into_raster();
        pr.clear();
        p = Plotter::new(pr);
        p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));
        //
        if let Some(left) = left {
            start += left;
            row += 1;
        } else {
            break;
        }
    }

    // Save PNG
    let raster = PngRaster::Rgba8(Raster::<SRgba8>::with_raster(&r));
    let mut out_data = Vec::new();
    let mut encoder = Encoder::new(&mut out_data).into_step_enc();
    encoder.still(&raster).expect("Failed to add frame");
    std::fs::write("raster.png", out_data).expect("Failed to save image");
}
