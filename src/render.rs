//! Rendering TTF glyphs with footile

use footile::PathOp;
use rustybuzz::Face;

/// Build a path
pub(crate) fn build_path(path_buffer: &mut Vec<PathOp>, face: &Face<'_>, glyph_x: f32, glyph_y: f32, glyph_id: u16) {
}
