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
struct LangFont<'a>(ttf_parser::Font<'a>, f32);

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
    /// Create a new `Font`.  Add glyphs with `add()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a TTF or OTF font's glyphs to this `Font`.
    pub fn push<B: Into<&'a [u8]>>(mut self, none: B) -> Option<Self> {
        let none = ttf_parser::Font::from_data(none.into(), 0)?;
        let em_per_height = f32::from(none.height()).recip();
        let none = LangFont(none, em_per_height);

        self.fonts.push(StyledFont { none });
        Some(self)
    }

    /// Render some text.  Returns an iterator and how many characters were
    /// rendered inside the bounding box.
    /// - `text`: text to render.
    /// - `bbox`: x, y, width, height.
    /// - `wh`: the size of each character in X & Y dimensions.
    /// - `text_align`: how the text is aligned
    pub fn render(
        &'a self,
        text: &'a str,
        bbox: (f32, f32, f32, f32),
        wh: (f32, f32),
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
                    left_over = Some(i);
                    break;
                }
                _ if c == BOLD => continue,
                _ if c == ITALIC => continue,
                _ if c == NONE => continue,
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
            let wh = (wh.0 * selected_font.1, -wh.1 * selected_font.1);

            let advance = match selected_font.0.glyph_hor_advance(glyph_id) {
                Some(adv) => {
                    (f32::from(adv)
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
                        * wh.0
                }
                None => 0.0,
            };

            pixel_length += advance;

            // Extends past the width of the bounding box.
            if pixel_length > bbox.2 {
                if last_space != 0 {
                    left_over = Some(last_space);
                    break;
                } else {
                    left_over = Some(i);
                    break;
                }
            }

            last = Some(glyph_id);
        }

        let mut bbox = (bbox.0, bbox.1, bbox.0 + bbox.2, bbox.1 + bbox.3);
        let mut vertical = false;

        match text_align {
            TextAlign::Left => { /* don't adjust */ }
            TextAlign::Right => bbox.0 = bbox.2 - pixel_length,
            TextAlign::Center => {
                bbox.0 = (bbox.0 + bbox.2 - pixel_length) * 0.5
            }
            TextAlign::Justified => { /* don't adjust */ }
            TextAlign::Vertical => vertical = true,
        }

        bbox.1 += wh.1;

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
                path: CharPathIterator::new(self, bbox, wh, vertical),
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
    offset: f32,
}

impl<'a> CharPathIterator<'a> {
    fn new(
        font: &'a Font<'a>,
        bbox: (f32, f32, f32, f32),
        size: (f32, f32),
        vertical: bool,
    ) -> Self {
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
            offset: 0.0,
        }
    }

    fn set(&mut self, c: char) {
        if c == BOLD {
            self.bold = true;
            return;
        } else if c == ITALIC {
            self.italic = true;
            return;
        } else if c == NONE {
            self.bold = false;
            self.italic = false;
            return;
        }

        if self.direction == Direction::CheckNext {
            self.direction = direction(c);
            self.xy = (self.bbox.0, self.bbox.1);
        }

        let mut index = 0;
        let glyph_id = loop {
            match self.font.fonts[index].none.0.glyph_index(c) {
                Some(v) => break v,
                None => {
                    index += 1;
                    if index == self.font.fonts.len() {
                        // eprintln!("No Glyph for \"{}\" ({})", c, c as u32);
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

        self.wh = (
            self.size.0 * selected_font.1,
            -self.size.1 * selected_font.1,
        );
        let em_per_unit =
            f32::from(selected_font.0.units_per_em().ok_or("em").unwrap())
                .recip();
        let h = selected_font.1 * self.size.1 / em_per_unit;
        self.offset = -self.size.1 + h;
        selected_font.0.outline_glyph(glyph_id, self);

        if self.vertical {
            self.xy.1 += self.size.1;
        } else {
            let advance = match selected_font.0.glyph_hor_advance(glyph_id) {
                Some(adv) => {
                    (f32::from(adv)
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
                        * self.wh.0
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
        self.path.push(PathOp::Move(Pt(
            x * self.wh.0 + self.xy.0,
            y * self.wh.1 + self.xy.1 + self.offset,
        )));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.push(PathOp::Line(Pt(
            x * self.wh.0 + self.xy.0,
            y * self.wh.1 + self.xy.1 + self.offset,
        )));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path.push(PathOp::Quad(
            Pt(
                x1 * self.wh.0 + self.xy.0,
                y1 * self.wh.1 + self.xy.1 + self.offset,
            ),
            Pt(
                x * self.wh.0 + self.xy.0,
                y * self.wh.1 + self.xy.1 + self.offset,
            ),
        ));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path.push(PathOp::Cubic(
            Pt(
                x1 * self.wh.0 + self.xy.0,
                y1 * self.wh.1 + self.xy.1 + self.offset,
            ),
            Pt(
                x2 * self.wh.0 + self.xy.0,
                y2 * self.wh.1 + self.xy.1 + self.offset,
            ),
            Pt(
                x * self.wh.0 + self.xy.0,
                y * self.wh.1 + self.xy.1 + self.offset,
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

/// Iterator that generates path from characters.
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

/// Get a monospace font.  Requires feature = "normal-font".
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
