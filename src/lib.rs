//! # Fonterator
//! Fonterator is a pure Rust font renderer.  When you want to render text, fonterator gives you an iterator over [footile](https://crates.io/crates/footile) `PathOp`s, which you can easily pass right into footile.
//! 
//! # Simple Example
//! ```rust
//! use fonterator as font; // For parsing font file.
//! use footile::{FillRule, Plotter, Raster, Rgba8}; // For rendering font text.
//! use png_pong::{RasterBuilder, EncoderBuilder}; // For saving PNG
//! 
//! const FONT_SIZE: f32 = 32.0;
//! 
//! fn main() {
//!     // Most common
//!     let english = "Raster Text With Font"; // LEFT-RIGHT
//!     let korean = "글꼴로 래스터 텍스트 사용"; // UP-DOWN, RIGHT-LEFT
//!     let japanese = "フォント付きラスタテキスト"; // UP-DOWN, RIGHT-LEFT
//! 
//!     // Init font, and paths.
//!     let font = font::monospace_font();
//! 
//!     // Init rendering.
//!     let mut p = Plotter::new(512, 512);
//!     let mut r = Raster::new(p.width(), p.height());
//! 
//!     // Render English Left Aligned.
//!     let path = font.render(english, (64.0, 0.0, 512.0 - 64.0, 512.0 - FONT_SIZE), (FONT_SIZE, FONT_SIZE), font::TextAlign::Left);
//!     let path: Vec<font::PathOp> = path.collect();
//!     r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
//! 
//!     // Render Korean Vertically
//!     let path = font.render(korean, (0.0, 0.0, 512.0, 512.0 - 32.0 * 7.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Vertical);
//!     let path: Vec<font::PathOp> = path.collect();
//!     r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
//! 
//!     // Render Japanese Vertically
//!     let path = font.render(japanese, (32.0, 0.0, 512.0, 512.0 - 32.0 * 7.0), (FONT_SIZE, FONT_SIZE), font::TextAlign::Vertical);
//!     let path: Vec<font::PathOp> = path.collect();
//!     r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
//! 
//!     // Save PNG
//!     let raster = RasterBuilder::new().with_u8_buffer(512, 512, r.as_u8_slice());
//!     let mut out_data = Vec::new();
//!     let mut encoder = EncoderBuilder::new();
//!     let mut encoder = encoder.encode_rasters(&mut out_data);
//!     encoder.add_frame(&raster, 0).expect("Failed to add frame");
//!     std::fs::write("dir.png", out_data).expect("Failed to save image");
//! }
//! ```

use ttf_parser as ttf;

pub use footile;
pub use footile::PathOp;

mod direction;

use direction::Direction;

/// Text alignment.
pub enum TextAlign {
    /// Align text to the left.
    Left,
    /// Align text to the center.
    Center,
    /// Align text to the right.
    Right,
    /// Justify text.
    Justified,
    /// Vertical text.
    Vertical,
}

/// No emphasis
pub const NONE: char = '\x01'; // Start of Heading
/// Bold
pub const BOLD: char = '\x02'; // Start of Text
/// Italic
pub const ITALIC: char = '\x03'; // End of Text

struct LangFont<'a>(ttf::Font<'a>, f32);

struct StyledFont<'a> {
    // Required
    none: LangFont<'a>,
}

/// A collection of TTF/OTF fonts used as a single font.
#[derive(Default)]
pub struct Font<'a> {
    fonts: Vec<StyledFont<'a>>,
}

impl<'a> Font<'a> {
    /// Create a new `Font`.  Add glyphs with `add()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a TTF or OTF font's glyphs to this `Font`.
    pub fn add<B: Into<&'a [u8]>>(mut self, none: B)
        -> Result<Self, Box<std::error::Error>>
    {
        let none = ttf::Font::from_data(none.into(), 0)?;
        let em_per_unit = (none.units_per_em().ok_or("em")? as f32).recip();
        let none = LangFont(none, em_per_unit);

        self.fonts.push(StyledFont {
            none,
        });
        Ok(self)
    }

    /// Render a string.
    ///
    /// `bbox`: x, y, width, height.
    pub fn render(&'a self, text: &'a str, bbox: (f32, f32, f32, f32), wh: (f32, f32), text_align: TextAlign)
        -> TextPathIterator<'a>
    {
        let mut pixel_length = 0.0;
        let mut last = None;

        // First Pass: Get pixel length
        for c in text.chars() {
            let mut index = 0;
            let glyph_id = loop {
                match self.fonts[index].none.0.glyph_index(c) {
                    Ok(v) => break v,
                    Err(_e) => {
                        index += 1;
                        if index == self.fonts.len() {
                            eprintln!("No Glyph for \"{}\" ({})", c, c as u32);
                            break self.fonts[index].none.0.glyph_index('�').unwrap();
                        }
                    }
                }
            };

            let selected_font = &self.fonts[index].none;
            let wh = (wh.0 * selected_font.1, -wh.1 * selected_font.1);

            let advance = match selected_font.0.glyph_hor_metrics(glyph_id) {
                Ok(v) => (v.advance as f32 + if let Some(last) = last { selected_font.0.glyphs_kerning(glyph_id, last).unwrap_or(0) as f32 } else { 0f32 }) * wh.0,
                Err(_) => 0.0,
            };

            pixel_length += advance;

            last = Some(glyph_id);
        }

        let mut bbox = (bbox.0, bbox.1, bbox.0 + bbox.2, bbox.1 + bbox.3);
        let mut vertical = false;

        match text_align {
            TextAlign::Left => { /* don't adjust */ },
            TextAlign::Right => bbox.0 = bbox.2 - pixel_length,
            TextAlign::Center => bbox.0 = (bbox.0 + bbox.2 - pixel_length) * 0.5,
            TextAlign::Justified => { /* don't adjust */ },
            TextAlign::Vertical => vertical = true,
        }

        bbox.1 += wh.1;

        // Second Pass: Get `PathOp`s
        TextPathIterator {
            text: text.chars().peekable(),
            temp: vec![],
            back: false,
            path: CharPathIterator::new(self, bbox, wh, vertical),
        }
    }
}

struct CharPathIterator<'a> {
    // The font to use.
    font: &'a Font<'a>,
    // Path of the current character.
    path: Vec<PathOp>,
    // W & H
    size: (f32, f32),
    // X, Y, X2, Y2
    bbox: (f32, f32, f32, f32),
    // Multiplied wh.
    wh: (f32, f32),
    // Return position for X.
    xy: (f32, f32),
    // General direction of the text.
    direction: Direction,
    // Last character
    last: Option<ttf_parser::GlyphId>,
    //
    vertical: bool,
    //
    bold: bool,
    //
    italic: bool,
}

impl<'a> CharPathIterator<'a> {
    fn new(font: &'a Font<'a>, bbox: (f32, f32, f32, f32), size: (f32, f32), vertical: bool) -> Self {
        Self {
            font,
            path: vec![],
            bbox,
            size,
            wh: (0.0, 0.0),
            xy: (bbox.0, bbox.1),
            direction: Direction::CheckNext,
            last: None,
            vertical,
            bold: false,
            italic: false,
        }
    }

    fn set(&mut self, c: char) -> Result<(), ttf::Error> {
        if c == BOLD {
            self.bold = true;
            return Ok(());
        } else if c == ITALIC {
            self.italic = true;
            return Ok(());
        } else if c == NONE {
            self.bold = false;
            self.italic = false;
            return Ok(());
        }

        if self.direction == Direction::CheckNext {
            self.direction = direction::direction(c);
            self.xy = (self.bbox.0, self.bbox.1);
        }

        let mut index = 0;
        let glyph_id = loop {
            match self.font.fonts[index].none.0.glyph_index(c) {
                Ok(v) => break v,
                Err(e) => {
                    index += 1;
                    if index == self.font.fonts.len() {
                        eprintln!("No Glyph for \"{}\" ({})", c, c as u32);
                        return Err(e);
                    }
                }
            }
        };

        let selected_font = &self.font.fonts[index].none;

        self.path.clear();

/*        if self.bold {
            self.path.push(PathOp::PenWidth(self.size.0 / 10.0));
        }*/

        self.wh = (self.size.0 * selected_font.1, -self.size.1 * selected_font.1);
        match selected_font.0.outline_glyph(glyph_id, self) {
            Ok(_v) => {},
            Err(ttf::Error::NoOutline) => { /* whitespace */ },
            Err(ttf::Error::NoGlyph) => { /* unknown glyph */
                let id = self.font.fonts[0].none.0.glyph_index('�')?;
                selected_font.0.outline_glyph(id, self).unwrap();
            },
            Err(e) => {
                eprintln!("Warning (glyph {}): {}.", glyph_id.0, e);
                return Err(e);
            }
        };

        if self.vertical {
            self.xy.1 += self.size.1;
        } else {
            let advance = match selected_font.0.glyph_hor_metrics(glyph_id) {
                Ok(v) => (v.advance as f32 + if let Some(last) = self.last { selected_font.0.glyphs_kerning(glyph_id, last).unwrap_or(0) as f32 } else { 0f32 }) * self.wh.0,
                Err(_) => 0.0,
            };
            self.xy.0 += advance;
        }

        self.path.reverse();

        self.last = Some(glyph_id);

        Ok(())
    }
}

impl ttf::OutlineBuilder for CharPathIterator<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.push(PathOp::Move(x * self.wh.0 + self.xy.0, y * self.wh.1 + self.xy.1));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.push(PathOp::Line(x * self.wh.0 + self.xy.0, y * self.wh.1 + self.xy.1));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path.push(PathOp::Quad(x1 * self.wh.0 + self.xy.0, y1 * self.wh.1 + self.xy.1, x * self.wh.0 + self.xy.0, y * self.wh.1 + self.xy.1));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path.push(PathOp::Cubic(x1 * self.wh.0 + self.xy.0, y1 * self.wh.1 + self.xy.1, x2 * self.wh.0 + self.xy.0, y2 * self.wh.1 + self.xy.1, x * self.wh.0 + self.xy.0, y * self.wh.1 + self.xy.1));
    }

    fn close(&mut self) {
        self.path.push(PathOp::Close());
    }
}

impl<'a> Iterator for CharPathIterator<'a> {
    type Item = PathOp;

    fn next(&mut self) -> Option<Self::Item> {
        self.path.pop()
    }
}

/// Iterator that generates path from characters.
pub struct TextPathIterator<'a> {
    // The text that we're parsing.
    text: std::iter::Peekable<std::str::Chars<'a>>,
    // Temporary text.
    temp: Vec<char>,
    // Backwards text.
    back: bool,
    // Path for the current character.
    path: CharPathIterator<'a>,
}

impl<'a> Iterator for TextPathIterator<'a> {
    type Item = PathOp;

    fn next(&mut self) -> Option<PathOp> {
        if let Some(op) = self.path.next() {
            Some(op)
        } else {
            if let Some(c) = self.text.peek() {
                let dir = direction::direction(*c);
                let dir = if dir == Direction::CheckNext {
                    if self.back {
                        Direction::RightLeft
                    } else {
                        Direction::LeftRight
                    }
                } else {
                    dir
                };
                if dir == Direction::RightLeft {
                    let c = self.text.next().unwrap();
                    if !self.back {
                        self.back = true;
                    }
                    self.temp.push(c);
                } else {
                    if let Some(c) = self.temp.pop() {
                        let _ = self.path.set(c);
                    } else {
                        let c = self.text.next().unwrap();
                        let _ = self.path.set(c);
                    }
                }
                self.next()
            } else {
                if let Some(c) = self.temp.pop() {
                    let _ = self.path.set(c);
                    self.next()
                } else {
                    None
                }
            }
        }
    }
}

/// Get a monospace font.  Requires feature = "builtin-font".
#[cfg(feature = "monospace-font")]
pub fn monospace_font() -> Font<'static> {
    const FONTA: &[u8] = include_bytes!("font/dejavu/SansMono.ttf");
    const FONTB: &[u8] = include_bytes!("font/noto/SansDevanagari.ttf");
    const FONTC: &[u8] = include_bytes!("font/noto/SansHebrew.ttf");
    const FONTD: &[u8] = include_bytes!("font/droid/SansFallback.ttf");

    Font::new()
        .add(FONTA)
        .unwrap()
        .add(FONTB)
        .unwrap()
        .add(FONTC)
        .unwrap()
        .add(FONTD)
        .unwrap()
}

/// Get a monospace font.  Requires feature = "builtin-font".
#[cfg(feature = "normal-font")]
pub fn normal_font() -> Font<'static> {
    const FONTA: &[u8] = include_bytes!("font/dejavu/Sans.ttf");
    const FONTB: &[u8] = include_bytes!("font/noto/SansDevanagari.ttf");
    const FONTC: &[u8] = include_bytes!("font/noto/SansHebrew.ttf");
    const FONTD: &[u8] = include_bytes!("font/droid/SansFallback.ttf");

    Font::new()
        .add(FONTA)
        .unwrap()
        .add(FONTB)
        .unwrap()
        .add(FONTC)
        .unwrap()
        .add(FONTD)
        .unwrap()
}

#[cfg(any(feature = "monospace-font", feature = "normal-font"))]
/// Get a text string of the licenses that must be included in a binary program
/// for using the font.  Assumes BSL-1.0 for fonterator, so fonterator license
/// does not need to be included.
pub fn licenses() -> &'static str {
    include_str!("bin-licenses.txt")
}
