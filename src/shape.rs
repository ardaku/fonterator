//! Fonterator's text shaping with rustybuzz

use rustybuzz::{Face, UnicodeBuffer, GlyphBuffer};
use footile::PathOp;

fn glyph_buffer_with_text(face: &Face<'_>, glyph_buffer: GlyphBuffer, text: &str) -> GlyphBuffer {
    let mut unicode_buffer = glyph_buffer.clear();
    unicode_buffer.push_str(text);
    rustybuzz::shape(&face, &[], unicode_buffer)
}

fn new_glyph_buffer(face: &Face<'_>) -> GlyphBuffer {
    let unicode_buffer = UnicodeBuffer::new();
    let glyph_buffer = rustybuzz::shape(&face, &[], unicode_buffer);

    glyph_buffer
}

/// A TTF font
pub struct Font<'a> {
    face: Face<'a>,
    glyph_buffer: GlyphBuffer,
}

impl<'a> Font<'a> {
    /// Load a font from a TTF file
    pub fn new(ttf: &'a [u8]) -> Result<Self, ()> {
        let face = Face::from_slice(ttf, 0).ok_or(())?;
        let glyph_buffer = new_glyph_buffer(&face);

        Ok(Self { face, glyph_buffer })
    }

    /// Render text including shaping and layout
    ///
    /// # Parameters
    ///  - `text`: The UTF-8 text to render
    ///  - `row`: The available rendering width in ems
    pub fn render<'b: 'a>(&'b mut self, text: &mut &str, row: f32)
        -> impl Iterator<Item = PathOp> + 'b + 'a
    {
        // FIXME: Lookahead
        let consumed = (*text).len();
        let piece = &(*text)[..consumed];
   
        let mut glyph_buffer = new_glyph_buffer(&self.face);
        std::mem::swap(&mut glyph_buffer, &mut self.glyph_buffer);
        self.glyph_buffer = glyph_buffer_with_text(&self.face, glyph_buffer, piece);

        *text = &text[consumed..];

        Render { font: self, glyph_index: 0 }
    }
}

struct Render<'a, 'b> {
    font: &'b mut Font<'a>,
    glyph_index: usize,
}

impl Iterator for Render<'_, '_> {
    type Item = PathOp;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
