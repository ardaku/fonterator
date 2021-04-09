// fonterator
//
// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::direction::{direction, Direction};
use footile::{PathOp, Pt};
use ttf_parser::kern::Subtable;

/// Text alignment.
#[derive(Copy, Clone, Debug)]
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

#[derive(Debug)]
struct LangFont<'a>(ttf_parser::Face<'a>);

#[derive(Debug)]
struct StyledFont<'a> {
    // Required
    none: LangFont<'a>,
}

/// A collection of TTF/OTF fonts used as a single font.
#[derive(Default, Debug)]
pub struct Font<'a> {
    fonts: Vec<StyledFont<'a>>,
}

impl<'a> Font<'a> {
    /// Create a new `Font`.  Add glyphs with `push()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a TTF or OTF font's glyphs to this `Font`.
    pub fn push<B: Into<&'a [u8]>>(mut self, none: B) -> Option<Self> {
        let none = LangFont(ttf_parser::Face::from_slice(none.into(), 0).ok()?);

        self.fonts.push(StyledFont { none });
        Some(self)
    }

    /// Render some text.  Returns an iterator and how many characters were
    /// rendered inside the bounding box.
    ///  - `text`: text to render.
    ///  - `row`: x (Left/Right align) or y (Up/Down align) offset where to stop
    ///    rendering.
    ///  - `text_align`: how the text is aligned
    /// 
    ///  Returns an iterator which generates the path from characters (see [`TextPathIterator`])
    ///  and a number indicating what was leftover
    pub fn render(
        &'a self,
        text: &'a str,
        row: f32,
        text_align: TextAlign,
    ) -> (TextPathIterator<'a>, usize) {
        let mut pixel_length = 0.0;
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
                chr if chr == BOLD => continue,
                chr if chr == ITALIC => continue,
                chr if chr == NONE => continue,
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

        match text_align {
            TextAlign::Left => { /* don't adjust */ }
            TextAlign::Right => xy.0 = row - pixel_length,
            TextAlign::Center => xy.0 = (row - pixel_length) * 0.5,
            TextAlign::Justified => { /* don't adjust */ }
            TextAlign::Vertical => vertical = true,
        }

        // Second Pass: Get `PathOp`s
        (
            TextPathIterator {
                text: if let Some(i) = left_over {
                    text[..i].chars().peekable()
                } else {
                    text.chars().peekable()
                },
                temp: vec![],
                back: false,
                path: CharPathIterator::new(self, xy, vertical),
            },
            left_over.unwrap_or_else(|| text.bytes().len()),
        )
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
    last: Option<ttf_parser::GlyphId>,
    //
    vertical: bool,
    //
    bold: bool,
    //
    italic: bool,
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
            bold: false,
            italic: false,
            font_ascender: 0.0,
            font_size: (0.0, 0.0),
        }
    }

    fn set(&mut self, c: char) {
        match c {
            BOLD => {
                self.bold = true;
                return;
            }
            ITALIC => {
                self.italic = true;
                return;
            }
            NONE => {
                self.bold = false;
                self.italic = false;
                return;
            }
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

        self.last = Some(glyph_id);
    }
}

impl ttf_parser::OutlineBuilder for CharPathIterator<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        let y = self.font_ascender - y;
        self.path.push(PathOp::Move(Pt(
            x * self.font_size.0 + self.xy.0,
            y * self.font_size.1 + self.xy.1,
        )));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let y = self.font_ascender - y;
        self.path.push(PathOp::Line(Pt(
            x * self.font_size.0 + self.xy.0,
            y * self.font_size.1 + self.xy.1,
        )));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let y1 = self.font_ascender - y1;
        let y = self.font_ascender - y;
        self.path.push(PathOp::Quad(
            Pt(
                x1 * self.font_size.0 + self.xy.0,
                y1 * self.font_size.1 + self.xy.1,
            ),
            Pt(
                x * self.font_size.0 + self.xy.0,
                y * self.font_size.1 + self.xy.1,
            ),
        ));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let y1 = self.font_ascender - y1;
        let y2 = self.font_ascender - y2;
        let y = self.font_ascender - y;

        self.path.push(PathOp::Cubic(
            Pt(
                x1 * self.font_size.0 + self.xy.0,
                y1 * self.font_size.1 + self.xy.1,
            ),
            Pt(
                x2 * self.font_size.0 + self.xy.0,
                y2 * self.font_size.1 + self.xy.1,
            ),
            Pt(
                x * self.font_size.0 + self.xy.0,
                y * self.font_size.1 + self.xy.1,
            ),
        ));
    }

    fn close(&mut self) {
        self.path.push(PathOp::Close());
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

impl Iterator for TextPathIterator<'_> {
    type Item = PathOp;

    fn next(&mut self) -> Option<PathOp> {
        if let Some(op) = self.path.next() {
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
    }
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
