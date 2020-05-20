use footile::{FillRule, Plotter};
use pix::rgb::{Rgba8p, SRgba8};
use pix::matte::Matte8;
use pix::Raster;
use pix::ops::SrcOver;
use png_pong::FrameEncoder; // For saving PNG

const FONT_SIZE: f32 = 200.0;

const STR: &str = "sphinx of black\nquartz, judge\nmy vow";

fn main() {
    let font = fonterator::monospace_font();

    // Init rendering
    let mut p = Plotter::new(Raster::with_clear(2048, 2048));
    let mut r = Raster::with_clear(2048, 2048);

    let mut start = 0;
    let mut row = 0;
    loop {
        let (path, left) = font.render(
            &STR[start..],
            2048.0,
            (FONT_SIZE, FONT_SIZE),
            fonterator::TextAlign::Center,
        );
        r.composite_matte(
            (0, row * 200, 2048, 2048),
            p.fill(FillRule::NonZero, path, Matte8::new(255)),
            (),
            Rgba8p::new(0, 0, 0, 255),
            SrcOver,
        );
        let mut pr = p.raster();
        pr.clear();
        p = Plotter::new(pr);
        // 
        if left == 0 {
            break;
        } else {
            start += left;
            row += 1;
        }
    }

    let mut start = 0;
    let mut row = 0;
    loop {
        let (path, left) = font.render(
            &STR[start..],
            2048.0,
            (FONT_SIZE, FONT_SIZE),
            fonterator::TextAlign::Right,
        );
        r.composite_matte(
            (0, 1024 + row * 200, 2048, 2048),
            p.fill(FillRule::NonZero, path, Matte8::new(255)),
            (),
            Rgba8p::new(0, 0, 0, 255),
            SrcOver,
        );
        let mut pr = p.raster();
        pr.clear();
        p = Plotter::new(pr);
        // 
        if left == 0 {
            break;
        } else {
            start += left;
            row += 1;
        }
    }

    // Save PNG
    let raster = Raster::<SRgba8>::with_raster(&r);
    let mut out_data = Vec::new();
    let mut encoder = FrameEncoder::new(&mut out_data);
    encoder.still(&raster).expect("Failed to add frame");
    std::fs::write("raster.png", out_data).expect("Failed to save image");
}
