use pix::Raster;
use pix::ops::SrcOver;
use pix::rgb::{SRgba8, Rgba8p};
use footile::{FillRule, Plotter};
use png_pong::FrameEncoder; // For saving PNG

const FONT_SIZE: f32 = 200.0;

const STR: &'static str =
    "Hé\tllö,\nWørłd!\nW. And Me?\nHow go it‽\n||| 10 X.Y.Z.";

fn main() {
    let font = fonterator::monospace_font();

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::with_clear(p.width(), p.height());

    let path = font
        .render(
            STR,
            (0.0, 0.0, 2048.0, 2048.0),
            (FONT_SIZE, FONT_SIZE),
            fonterator::TextAlign::Left,
        )
        .0;
    let path: Vec<footile::PathOp> = path.collect();
    r.composite_matte((0, 0, 2048, 2048), 
        p.fill(&path, FillRule::NonZero),
        (),
        Rgba8p::new(0, 0, 0, 255), /*color*/
        SrcOver,
    );

    let path = font
        .render(
            STR,
            (0.0, 1024.0, 2048.0, 1024.0),
            (FONT_SIZE, FONT_SIZE),
            fonterator::TextAlign::Right,
        )
        .0;
    let path: Vec<footile::PathOp> = path.collect();
    r.composite_matte((0, 0, 2048, 2048), 
        p.fill(&path, FillRule::NonZero),
        (),
        Rgba8p::new(0, 0, 0, 255), /*color*/
        SrcOver,
    );

    // Save PNG
    let raster = Raster::<SRgba8>::with_raster(&r);
    let mut out_data = Vec::new();
    let mut encoder = FrameEncoder::new(&mut out_data);
    encoder.still(&raster).expect("Failed to add frame");
    std::fs::write("raster.png", out_data).expect("Failed to save image");
}
