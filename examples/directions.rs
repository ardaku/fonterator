//! This tests writing directions for different languages.

use fonterator as font; // For parsing font file.
use footile::{FillRule, Plotter, Transform}; // For rendering font text.
use pix::matte::Matte8;
use pix::ops::SrcOver;
use pix::rgb::{Rgba8p, SRgba8};
use pix::Raster;
use png_pong::{PngRaster, Encoder}; // For saving PNG

const FONT_SIZE: f32 = 32.0;

fn main() {
    // Most common
    let english = "Raster Text With Font"; // LEFT-RIGHT
    let english2 = &format!("{}Raster Text With Font", font::BOLD); // LEFT-RIGHT
    let nepali = "फन्टको साथ रास्टर पाठ"; // LEFT-RIGHT
    let arabic = "النقطية النص مع الخط"; // RIGHT-LEFT
    let hebrew = "טקסט רסטר עם גופן"; // RIGHT-LEFT

    // Note that any direction works for these languages, but traditionally
    // up-down, right-left.  Commonly LEFT-RIGHT
    let korean = "글꼴로 래스터 텍스트 사용"; // UP-DOWN, RIGHT-LEFT
    let japanese = "フォント 付きラス タテキス ト"; // UP-DOWN, RIGHT-LEFT

    // LEFT-RIGHT, DOWN-UP
    let _hanunuo = "ᜱᜨᜳᜨᜳᜢ";

    // Init font, and paths.
    let font = font::monospace_font();

    // Init rendering.
    let mut p = Plotter::new(Raster::with_clear(512, 512));
    let mut r = Raster::with_clear(512, 512);

    // Render paths.
    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));
    let path = font
        .render(english2, (512.0 - 64.0) / FONT_SIZE, font::TextAlign::Left)
        .0;
    r.composite_matte(
        (64, 0, 512 - 64, 512),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );
    // Reset plotter
    let mut pr = p.raster();
    pr.clear();
    p = Plotter::new(pr);
    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));

    let path = font
        .render(nepali, (512.0 - 64.0) / FONT_SIZE, font::TextAlign::Left)
        .0;
    // Composite
    r.composite_matte(
        (64, 32, 512, 512),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );
    // Reset plotter
    let mut pr = p.raster();
    pr.clear();
    p = Plotter::new(pr);
    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));

    let path = font
        .render(english, (512.0 - 64.0) / FONT_SIZE, font::TextAlign::Center)
        .0;
    // Composite
    r.composite_matte(
        (64, 64, 512, 512),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );
    // Reset plotter
    let mut pr = p.raster();
    pr.clear();
    p = Plotter::new(pr);
    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));

    let path = font
        .render(arabic, (512.0 - 64.0) / FONT_SIZE, font::TextAlign::Right)
        .0;
    // Composite
    r.composite_matte(
        (64, 96, 512, 512),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );
    // Reset plotter
    let mut pr = p.raster();
    pr.clear();
    p = Plotter::new(pr);
    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));

    let path = font
        .render(hebrew, (512.0 - 64.0) / FONT_SIZE, font::TextAlign::Right)
        .0;
    // Composite
    r.composite_matte(
        (64, 128, 512 - 64, 512 - 128),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );
    // Reset plotter
    let mut pr = p.raster();
    pr.clear();
    p = Plotter::new(pr);
    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));

    let path = font
        .render(nepali, (512.0 - 64.0) / FONT_SIZE, font::TextAlign::Center)
        .0;
    // Composite
    r.composite_matte(
        (64, 32 * 5, 512 - 64, 512 - 32 * 5),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );
    // Reset plotter
    let mut pr = p.raster();
    pr.clear();
    p = Plotter::new(pr);

    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE * 2.0));
    let path = font
        .render(english, (512.0 - 64.0) / FONT_SIZE, font::TextAlign::Right)
        .0;
    // Composite
    r.composite_matte(
        (64, 32 * 6, 512 - 64, 512 - 32 * 6),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );
    // Reset plotter
    let mut pr = p.raster();
    pr.clear();
    p = Plotter::new(pr);
    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));

    let path = font
        .render(
            korean,
            (512.0 - 32.0 * 7.0) / FONT_SIZE,
            font::TextAlign::Vertical,
        )
        .0;
    // Composite
    r.composite_matte(
        (0, 0, 512, 512),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );
    // Reset plotter
    let mut pr = p.raster();
    pr.clear();
    p = Plotter::new(pr);
    p.set_transform(Transform::with_scale(FONT_SIZE, FONT_SIZE));

    let path = font
        .render(
            japanese,
            (512.0 - 32.0 * 7.0) / FONT_SIZE,
            font::TextAlign::Vertical,
        )
        .0;
    // Composite
    r.composite_matte(
        (32, 0, 512, 512),
        p.fill(FillRule::NonZero, path, Matte8::new(255)),
        (),
        Rgba8p::new(0, 0, 0, 255),
        SrcOver,
    );

    // Save PNG
    let raster = PngRaster::Rgba8(Raster::<SRgba8>::with_raster(&r));
    let mut out_data = Vec::new();
    let mut encoder = Encoder::new(&mut out_data).into_step_enc();
    encoder.still(&raster).expect("Failed to add frame");
    std::fs::write("dir.png", out_data).expect("Failed to save image");
}
