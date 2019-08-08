//! This tests writing directions for different languages.

use fonterator as font; // For parsing font file.
use footile::{FillRule, Plotter, Raster, Rgba8}; // For rendering font text.
use png_pong::{RasterBuilder, EncoderBuilder}; // For saving PNG

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
    let japanese = "フォント付きラスタテキスト"; // UP-DOWN, RIGHT-LEFT
    //
    let _hanunuo = "ᜱᜨᜳᜨᜳᜢ"; // LEFT-RIGHT, DOWN-UP

    // Init font, and paths.
    let font = font::monospace_font();

    // Init rendering.
    let mut p = Plotter::new(512, 512);
    let mut r = Raster::new(p.width(), p.height());

    // Render paths.
    let path = font.render(english2, (64.0, 0.0, 512.0 - 64.0, 512.0 - FONT_SIZE), (FONT_SIZE, FONT_SIZE), font::TextAlign::Left).0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    let path = font.render(nepali, (64.0, 32.0 * 1.0, 512.0 - 64.0, 512.0 - FONT_SIZE * 2.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Left).0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    let path = font.render(english, (64.0, 32.0 * 2.0, 512.0 - 64.0, 512.0 - 32.0 * 5.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Center).0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    let path = font.render(arabic, (64.0, 32.0 * 3.0, 512.0 - 64.0, 512.0 - FONT_SIZE * 3.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Right).0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    let path = font.render(hebrew, (64.0, 32.0 * 4.0, 512.0 - 64.0, 512.0 - 32.0 * 4.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Right).0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    let path = font.render(nepali, (64.0, 32.0 * 5.0, 512.0 - 64.0, 512.0 - 32.0 * 6.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Center).0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    let path = font.render(english, (64.0, 32.0 * 6.0, 512.0 - 64.0, 512.0 - FONT_SIZE), (FONT_SIZE, FONT_SIZE * 2.0), font::TextAlign::Right).0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    let path = font.render(korean, (0.0, 0.0, 512.0, 512.0 - 32.0 * 7.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Vertical).0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    let path = font.render(japanese, (32.0, 0.0, 512.0, 512.0 - 32.0 * 7.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Vertical).0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    // Save PNG
    let raster = RasterBuilder::new().with_u8_buffer(512, 512, r.as_u8_slice());
    let mut out_data = Vec::new();
    let mut encoder = EncoderBuilder::new();
    let mut encoder = encoder.encode_rasters(&mut out_data);
    encoder.add_frame(&raster, 0).expect("Failed to add frame");
    std::fs::write("dir.png", out_data).expect("Failed to save image");
}
