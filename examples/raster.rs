use footile::{FillRule, Plotter};
use pix::rgb::{Rgba8p, SRgba8};
use pix::Raster;
use png_pong::FrameEncoder; // For saving PNG

const FONT_SIZE: f32 = 200.0;

const STR: &str = "sphinx of black\nquartz, judge\nmy vow";

fn main() {
    let font = fonterator::monospace_font();

    // Init rendering
    let mut p = Plotter::new(Raster::with_clear(2048, 2048));

    let path = font
        .render(
            STR,
            (0.0, 0.0, 2048.0, 2048.0),
            (FONT_SIZE, FONT_SIZE),
            fonterator::TextAlign::Left,
        )
        .0;
    let path: Vec<footile::PathOp> = path.collect();

    p.fill(FillRule::NonZero, &path, Rgba8p::new(0, 0, 0, 255));

    let path = font
        .render(
            STR,
            (0.0, 1024.0, 2048.0, 1024.0),
            (FONT_SIZE, FONT_SIZE),
            fonterator::TextAlign::Right,
        )
        .0;
    let path: Vec<footile::PathOp> = path.collect();

    p.fill(FillRule::NonZero, &path, Rgba8p::new(0, 0, 0, 255));

    // Save PNG
    let raster = Raster::<SRgba8>::with_raster(&p.raster());
    let mut out_data = Vec::new();
    let mut encoder = FrameEncoder::new(&mut out_data);
    encoder.still(&raster).expect("Failed to add frame");
    std::fs::write("raster.png", out_data).expect("Failed to save image");
}
