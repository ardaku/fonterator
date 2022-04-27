// Copyright © 2018-2022 The Fonterator Contributors.
//                                                                               
// Licensed under any of:                                                        
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)   
// - MIT License (https://mit-license.org/)                                      
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt) 
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,              
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use crate::direction::{direction, Direction};
use footile::{PathOp, Pt};
use rustybuzz::{
    Face as FaceShaper, GlyphBuffer, GlyphInfo, GlyphPosition, UnicodeBuffer,
};
use ttf_parser::{kern::Subtable, Face, GlyphId, OutlineBuilder};

struct LangFont<'a>(Face<'a>, FaceShaper<'a>);

struct Outliner<'a> {
    // Path to write out to.
    path: &'a mut Vec<PathOp>,
    // How tall the font is (used to invert the Y axis).
    ascender: f32,
    // Translated X and Y positions.
    offset: (f32, f32),
    // Font scaling.
    scale: f32,
}

impl OutlineBuilder for Outliner<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        let x = x + self.offset.0;
        let y = self.ascender - (y + self.offset.1);
        self.path
            .push(PathOp::Move(Pt(x * self.scale, y * self.scale)));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let x = x + self.offset.0;
        let y = self.ascender - (y + self.offset.1);
        self.path
            .push(PathOp::Line(Pt(x * self.scale, y * self.scale)));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let x = x + self.offset.0;
        let x1 = x1 + self.offset.0;
        let y = self.ascender - (y + self.offset.1);
        let y1 = self.ascender - (y1 + self.offset.1);
        self.path.push(PathOp::Quad(
            Pt(x1 * self.scale, y1 * self.scale),
            Pt(x * self.scale, y * self.scale),
        ));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let x = x + self.offset.0;
        let x1 = x1 + self.offset.0;
        let x2 = x2 + self.offset.0;
        let y = self.ascender - (y + self.offset.1);
        let y1 = self.ascender - (y1 + self.offset.1);
        let y2 = self.ascender - (y2 + self.offset.1);

        self.path.push(PathOp::Cubic(
            Pt(x1 * self.scale, y1 * self.scale),
            Pt(x2 * self.scale, y2 * self.scale),
            Pt(x * self.scale, y * self.scale),
        ));
    }

    fn close(&mut self) {
        self.path.push(PathOp::Close());
    }
}

struct StyledFont<'a> {
    // Buffer associated with this font.
    glyph_buffer: Option<GlyphBuffer>,
    // Required
    none: LangFont<'a>,
}

impl<'a> StyledFont<'a> {
    fn path(
        &self,
        index: usize,
        path: &mut Vec<PathOp>,
        offset: &mut (i32, i32),
    ) {
        let GlyphInfo {
            glyph_id,
            cluster: _,
            ..
        } = self.glyph_buffer.as_ref().unwrap().glyph_infos()[index];
        let GlyphPosition {
            x_advance,
            y_advance,
            x_offset,
            y_offset,
            ..
        } = self.glyph_buffer.as_ref().unwrap().glyph_positions()[index];

        let glyph_id = GlyphId(glyph_id as u16);
        let scale = (self.none.0.height() as f32).recip();

        // let xy = (xy.0 + x_offset as f32 * scale, -xy.1 - y_offset as f32 * scale);
        let ascender = self.none.0.ascender() as f32 * scale;
        let x_offset = x_offset + offset.0;
        let y_offset = y_offset + offset.1;
        offset.0 += x_advance;
        offset.1 += y_advance;
        let offset = (
            x_offset as f32,
            (y_offset - self.none.0.ascender() as i32) as f32,
        );

        self.none.0.outline_glyph(
            glyph_id,
            &mut Outliner {
                path,
                ascender,
                scale,
                offset,
            },
        );
    }
}

/// A collection of TTF/OTF fonts used as a single font.
#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct Font<'a> {
    paths: Vec<PathOp>,
    fonts: Vec<StyledFont<'a>>,
}

impl<'a> Font<'a> {
    /// Create a new `Font`.  Add glyphs with `push()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a TTF or OTF font's glyphs to this `Font`.
    pub fn push<B: Into<&'a [u8]>>(mut self, font_data: B) -> Option<Self> {
        let font_data = font_data.into();
        let face = (
            Face::from_slice(font_data, 0).ok()?,
            FaceShaper::from_slice(font_data, 0)?,
        );
        let none = LangFont(face.0, face.1);

        self.fonts.push(StyledFont {
            none,
            glyph_buffer: None,
        });
        Some(self)
    }

    /// Render some text.  Returns an iterator and index within the `&str` where
    /// rendering stopped.
    ///  - `text`: text to render.
    ///  - `row`: x (Left/Right align) or y (Up/Down align) offset where to stop
    ///    rendering.
    ///
    ///  Returns an iterator which generates the path from characters (see
    ///  [`TextPathIterator`]) and a number indicating how many characters are
    ///  leftover (not rendered).
    pub fn render<'b>(
        &'b mut self,
        text: &str,
        row: f32,
    ) -> (TextPathIterator<'a, 'b>, Option<usize>) {
        let row: f64 = row.into();
        let row: i32 = (u16::MAX as f64 * row) as i32;
        let mut text = text;

        // Look for newlines and spaces to handle specially.
        let mut left_over = None;
        for (i, c) in text.char_indices() {
            match c {
                // ' ' => last_space = i,
                '\n' => {
                    left_over = Some(i + 1);
                    text = &text[..i];
                    break;
                }
                _ => {}
            }
        }

        // Replace glyph buffer using text.
        // FIXME: Currently only using first font.
        self.fonts[0].glyph_buffer = Some({
            let mut unicode_buffer =
                if let Some(buf) = self.fonts[0].glyph_buffer.take() {
                    buf.clear()
                } else {
                    UnicodeBuffer::new()
                };
            unicode_buffer.push_str(text);
            rustybuzz::shape(&self.fonts[0].none.1, &[], unicode_buffer)
        });

        // Pass over glyphs, looking for where to stop.
        let positions = self.fonts[0]
            .glyph_buffer
            .as_ref()
            .unwrap()
            .glyph_positions();
        let infos = self.fonts[0].glyph_buffer.as_ref().unwrap().glyph_infos();
        let mut until = positions.len();
        'crop: for (index, glyph) in positions.iter().enumerate() {
            if glyph.x_offset > row {
                left_over = Some(infos[index].cluster as usize);
                until = index;
                break 'crop;
            }
        }

        // Return iterator over PathOps and index to start on next call.
        (
            TextPathIterator {
                fontc: self,
                until,
                index: 0,
                path_i: 0,
                offset: (0, 0),
            },
            left_over,
        )

        /*let mut pixel_length = 0.0;
        let mut last = None;
        let mut left_over = None;
        let mut last_space = 0;

        // First Pass: Get pixel length
        for (i, c) in text.char_indices() {
            match c {
                ' ' => last_space = i,
                '\n' => {
                    left_over = Some(i + 1);
                    break;
                }
                _ => {}
            }

            let mut index = 0;
            let glyph_id = loop {
                match self.fonts[index].none.0.glyph_index(c) {
                    Some(v) => break v,
                    None => {
                        index += 1;
                        if index == self.fonts.len() {
                            // eprintln!("No Glyph for \"{}\" ({})", c, c as u32);
                            index = 0;
                            break self.fonts[0]
                                .none
                                .0
                                .glyph_index('�')
                                .unwrap();
                        }
                    }
                }
            };

            let selected_font = &self.fonts[index].none;

            // Transform font size.
            let fh = selected_font.0.height() as f32;
            let font_size = (fh.recip(), fh.recip());

            let advance = match selected_font.0.glyph_hor_advance(glyph_id) {
                Some(adv) => {
                    font_size.0
                        * (f32::from(adv)
                            + if let Some(last) = last {
                                selected_font
                                    .0
                                    .kerning_subtables()
                                    .next()
                                    .unwrap_or_else(Subtable::default)
                                    .glyphs_kerning(glyph_id, last)
                                    .unwrap_or(0)
                                    .into()
                            } else {
                                0f32
                            })
                }
                None => 0.0,
            };

            pixel_length += advance;

            // Extends past the width of the bounding box.
            if pixel_length > row {
                if last_space != 0 {
                    left_over = Some(last_space + 1);
                    break;
                } else {
                    left_over = Some(i + 1);
                    break;
                }
            }

            last = Some(glyph_id);
        }

        let mut xy = (0.0, 0.0);
        let mut vertical = false;

        // Second Pass: Get `PathOp`s
        (
            TextPathIterator {
                temp: vec![],
                back: false,
                path: CharPathIterator::new(self, xy, vertical),
            },
            left_over.unwrap_or_else(|| text.bytes().len()),
        )*/
    }
}

struct CharPathIterator<'a> {
    // The font to use.
    font: &'a Font<'a>,
    // Path of the current character.
    path: Vec<PathOp>,
    // Return position for X.
    xy: (f32, f32),
    // General direction of the text.
    direction: Direction,
    // Last character
    last: Option<GlyphId>,
    //
    vertical: bool,
    //
    font_ascender: f32,
    //
    font_size: (f32, f32),
}

impl<'a> CharPathIterator<'a> {
    fn new(font: &'a Font<'a>, xy: (f32, f32), vertical: bool) -> Self {
        Self {
            font,
            path: vec![],
            xy,
            direction: Direction::CheckNext,
            last: None,
            vertical,
            font_ascender: 0.0,
            font_size: (0.0, 0.0),
        }
    }

    fn set(&mut self, c: char) {
        /*match c {
            '\n' => return,
            _ => {}
        }

        if self.direction == Direction::CheckNext {
            self.direction = direction(c);
        }

        let mut index = 0;
        let glyph_id = loop {
            match self.font.fonts[index].none.0.glyph_index(c) {
                Some(v) => break v,
                None => {
                    index += 1;
                    if index == self.font.fonts.len() {
                        index = 0;
                        break self.font.fonts[0]
                            .none
                            .0
                            .glyph_index('�')
                            .unwrap();
                    }
                }
            }
        };

        let selected_font = &self.font.fonts[index].none;

        self.path.clear();

        /*        if self.bold {
            self.path.push(PathOp::PenWidth(self.size.0 / 10.0));
        }*/

        let font_height = selected_font.0.height() as f32;
        self.font_ascender = selected_font.0.ascender() as f32;
        self.font_size = (font_height.recip(), font_height.recip());

        selected_font.0.outline_glyph(glyph_id, self);

        if self.vertical {
            self.xy.1 += 1.0;
        } else {
            let advance = match selected_font.0.glyph_hor_advance(glyph_id) {
                Some(adv) => {
                    self.font_size.0
                        * (f32::from(adv)
                            + if let Some(last) = self.last {
                                selected_font
                                    .0
                                    .kerning_subtables()
                                    .next()
                                    .unwrap_or_else(Subtable::default)
                                    .glyphs_kerning(glyph_id, last)
                                    .unwrap_or(0)
                                    .into()
                            } else {
                                0f32
                            })
                }
                None => 0.0,
            };
            self.xy.0 += advance;
        }

        self.path.reverse();

        self.last = Some(glyph_id);*/
    }
}

impl Iterator for CharPathIterator<'_> {
    type Item = PathOp;

    fn next(&mut self) -> Option<Self::Item> {
        self.path.pop()
    }
}

/// Iterator that generates a path from characters.
#[allow(missing_debug_implementations)]
pub struct TextPathIterator<'a, 'b> {
    // Contains reusable glyph and path buffers.
    fontc: &'b mut Font<'a>,
    // Index to stop rendering at.
    until: usize,
    // Current glyph index.
    index: usize,
    // Index for `PathOp`s.
    path_i: usize,
    // x and y offset.
    offset: (i32, i32),
}

impl Iterator for TextPathIterator<'_, '_> {
    type Item = PathOp;

    fn next(&mut self) -> Option<PathOp> {
        // First, check for remaining PathOp's in the glyph path buffer.
        if self.path_i != self.fontc.paths.len() {
            let path_op = self.fontc.paths[self.path_i];
            self.path_i += 1;
            return Some(path_op);
        }
        // Because no path ops were left, clear buffer for reuse.
        self.fontc.paths.clear();
        self.path_i = 0;
        // Check for remaining glyphs in the GlyphBuffer.
        if self.index != self.until {
            self.fontc.fonts[0].path(
                self.index,
                &mut self.fontc.paths,
                &mut self.offset,
            );
            self.index += 1;
            self.next()
        } else {
            None
        }
    }

    /*  if let Some(op) = self.path.next() {
            Some(op)
        } else if let Some(c) = self.text.peek() {
            let dir = direction(*c);
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
            } else if let Some(c) = self.temp.pop() {
                self.path.set(c);
            } else {
                let c = self.text.next().unwrap();
                self.path.set(c);
            }
            self.next()
        } else if let Some(c) = self.temp.pop() {
            self.path.set(c);
            self.next()
        } else {
            None
        }
    }*/
}

/// Get a monospace font.  Requires feature = "monospace-font", enabled by default.
#[cfg(feature = "monospace-font")]
pub fn monospace_font() -> Font<'static> {
    const FONTA: &[u8] = include_bytes!("font/dejavu/SansMono.ttf");
    const FONTB: &[u8] = include_bytes!("font/noto/SansDevanagari.ttf");
    const FONTC: &[u8] = include_bytes!("font/noto/SansHebrew.ttf");
    const FONTD: &[u8] = include_bytes!("font/droid/SansFallback.ttf");

    Font::new()
        .push(FONTA)
        .unwrap()
        .push(FONTB)
        .unwrap()
        .push(FONTC)
        .unwrap()
        .push(FONTD)
        .unwrap()
}

/// Get a normal font.  Requires feature = "normal-font".
#[cfg(feature = "normal-font")]
pub fn normal_font() -> Font<'static> {
    const FONTA: &[u8] = include_bytes!("font/dejavu/Sans.ttf");
    const FONTB: &[u8] = include_bytes!("font/noto/SansDevanagari.ttf");
    const FONTC: &[u8] = include_bytes!("font/noto/SansHebrew.ttf");
    const FONTD: &[u8] = include_bytes!("font/droid/SansFallback.ttf");

    Font::new()
        .push(FONTA)
        .unwrap()
        .push(FONTB)
        .unwrap()
        .push(FONTC)
        .unwrap()
        .push(FONTD)
        .unwrap()
}

#[cfg(any(feature = "monospace-font", feature = "normal-font"))]
/// Get a text string of the licenses that must be included in a binary program
/// for using the font.  Requires either feature = "monospace-font" or feature
/// = "normal-font"
pub fn licenses() -> &'static str {
    include_str!("bin-licenses.txt")
}
