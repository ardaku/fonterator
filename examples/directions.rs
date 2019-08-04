//! This tests writing directions for different languages.

use fonterator as font; // For parsing font file.
use footile::{FillRule, Plotter, Raster, Rgba8}; // For rendering font text.
use png_pong::{RasterBuilder, EncoderBuilder}; // For saving PNG

const FONT_SIZE: f32 = 64.0;

fn main() {
    // Most common
    let english = "Raster Text With Font"; // LEFT-RIGHT
    let _nepali = "फन्टको साथ रास्टर पाठ"; // LEFT-RIGHT
    let _arabic = "النقطية النص مع الخط"; // RIGHT-LEFT
    let _hebrew = "טקסט רסטר עם גופן"; // RIGHT-LEFT
    // Note that any direction works for these languages, but traditionally
    // up-down, right-left.  Commonly LEFT-RIGHT
    let _korean = "글꼴로 래스터 텍스트 사용"; // UP-DOWN, RIGHT-LEFT
    let _japanese = "フォント付きラスタテキスト"; // UP-DOWN, RIGHT-LEFT
    //
    let _hanunuo = "ᜱᜨᜳᜨᜳᜢ"; // LEFT-RIGHT, DOWN-UP

    // Init font, and paths.
    let font = font::monospace_font();
    let mut path = font.render(english, (64.0, 64.0), (FONT_SIZE, FONT_SIZE));

    // Init rendering.
    let mut p = Plotter::new(1024, 1024);
    let mut r = Raster::new(p.width(), p.height());

    // Render paths.
    r.over(p.fill(&mut path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));

    // Save PNG
    let raster = RasterBuilder::new().with_u8_buffer(1024, 1024, r.as_u8_slice());
    let mut out_data = Vec::new();
    let mut encoder = EncoderBuilder::new();
    let mut encoder = encoder.encode_rasters(&mut out_data);
    encoder.add_frame(&raster, 0).expect("Failed to add frame");
    std::fs::write("dir.png", out_data).expect("Failed to save image");
}
