// Copyright Jeron Lau 2018-2019.
// Copyright Dylan Ede 2016.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

//! Fonterator is a pure Rust font loader.  When you want to render text, fonterator gives you an
//! iterator over [footile](https://crates.io/crates/footile) `PathOp`s, which you can easily pass
//! right into footile.
//!
//! # Simple Example
//! In Cargo.toml,
//!
//! ```toml
//! [dependencies]
//! fonterator = "0.4.0"
//! ```
//!
//! In main.rs,
//! ```rust
//! use fonterator::FontGroup;
//! use footile::{FillRule, Plotter, Raster, Rgba8};
//!
//! fn main() {
//!     // Load the default FontGroup (font and fallbacks).
//!     let font = FontGroup::default();
//!
//!     // Init rendering
//!     let mut p = Plotter::new(2048, 2048);
//!     let mut r = Raster::new(p.width(), p.height());
//!
//!     // Render the text
//!     let mut path = font.render(
//!         "Héllö,\nWørłd!‽i", /*text*/
//!         (0.0, 0.0),         /*position*/
//!         (256.0, 256.0),     /*size*/
//!     );
//!     r.over(
//!         p.fill(&mut path, FillRule::NonZero),
//!         Rgba8::rgb(0, 0, 0), /*color*/
//!     );
//!     r.write_png("main.png").unwrap(); /*save as PNG*/
//! }
//! ```

pub use footile;

mod tt;

/// PathOp from Footile.
pub use footile::PathOp;
use unicode_normalization::UnicodeNormalization;

use std::fmt;
use std::sync::Arc;

/// A 2D vector
#[derive(Copy, Clone)]
struct Vec2(pub f32, pub f32);

/// A `FontGroup` is a collection of fonts that together should cover all of the unicode codepoints.
pub struct FontGroup<'a> {
    fonts: Vec<Font<'a>>,
    mono: Option<f32>,
}

#[cfg(feature = "builtin-font")]
impl<'a> Default for FontGroup<'a> {
    fn default() -> Self {
        const FONTA: &[u8] = include_bytes!("font/dejavu/DejaVuSansMono.ttf");
        const FONTB: &[u8] = include_bytes!("font/wqy-microhei/WenQuanYiMicroHeiMono.ttf");

        FontGroup::new()
            .add(FONTA)
            .unwrap()
            .add(FONTB)
            .unwrap()
            .multilingual_mono(1)
    }
}

impl<'a> FontGroup<'a> {
    /// Create a new FontGroup
    pub fn new() -> Self {
        FontGroup {
            fonts: vec![],
            mono: None,
        }
    }

    /// Add a Font or FontCollection to the FontGroup
    pub fn add<B: Into<SharedBytes<'a>>>(mut self, bytes: B) -> Result<Self, Error> {
        let collection = FontCollection::new(bytes)?;

        if tt::get_font_offset_for_index(&collection.0, 1).is_some() {
            // multiple fonts
            let mut fonts = collection.into_fonts();
            self.fonts.append(&mut fonts);
        } else {
            // one font
            let font = collection.into_font()?;
            self.fonts.push(font);
        }

        Ok(self)
    }

    /// Get an iterator over the glyphs in a string.
    fn glyphs<T: ToString>(
        &'a self,
        text: T,
        scale: (f32, f32),
        mono: Option<f32>,
    ) -> GlyphIterator<'a> {
        let (scale_x, scale_y) = {
            let scale_y = self.fonts[0].info.scale_for_pixel_height(scale.1);
            let scale_x = scale_y * scale.0 / scale.1;
            (scale_x, scale_y)
        };

        GlyphIterator {
            font: &self.fonts,
            api_scale: scale,
            scale: Vec2(scale_x, scale_y),
            string: text.to_string().nfc().collect::<Vec<char>>(),
            cursor: 0,
            last: None,
            mono,
        }
    }

    /// Render a string.
    pub fn render<T: ToString>(&self, text: T, xy: (f32, f32), wh: (f32, f32)) -> PathIterator {
        PathIterator::new(self.glyphs(text, wh, self.mono), xy)
    }

    /// Enable Multi-Lingual Monospace (2 Latin Letters per CJK Character).
    ///
    /// The index refers to which font to get the CJK character width from, so it should be a font
    /// with CJK support (otherwise it won't work).
    pub fn multilingual_mono(mut self, index: usize) -> Self {
        let glyphb = self.fonts[index].glyph('野', Vec2(1.0, 1.0), None).0;
        let glyphb = self.fonts[index]
            .info
            .get_glyph_h_metrics(glyphb.id().0)
            .advance_width as f32;
        self.mono = Some(glyphb);
        self
    }
}

/// A collection of fonts read straight from a font file's data. The data in the
/// collection is not validated. This structure may or may not own the font
/// data.
#[derive(Clone, Debug)]
struct FontCollection<'a>(SharedBytes<'a>);
/// A single font. This may or may not own the font data.
#[derive(Clone)]
struct Font<'a> {
    info: tt::FontInfo<SharedBytes<'a>>,
}

/// `SharedBytes` handles the lifetime of font data used in Fonterator. The data
/// is either a shared reference to externally owned data, or managed by
/// reference counting. `SharedBytes` can be conveniently used with `From` and
/// `Into`, and dereferences to the contained bytes.
#[derive(Clone, Debug)]
pub enum SharedBytes<'a> {
    ByRef(&'a [u8]),
    ByArc(Arc<[u8]>),
}

impl<'a> ::std::ops::Deref for SharedBytes<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match *self {
            SharedBytes::ByRef(bytes) => bytes,
            SharedBytes::ByArc(ref bytes) => &**bytes,
        }
    }
}
impl<'a> From<&'a [u8]> for SharedBytes<'a> {
    fn from(bytes: &'a [u8]) -> SharedBytes<'a> {
        SharedBytes::ByRef(bytes)
    }
}
impl From<Arc<[u8]>> for SharedBytes<'static> {
    fn from(bytes: Arc<[u8]>) -> SharedBytes<'static> {
        SharedBytes::ByArc(bytes)
    }
}
impl From<Box<[u8]>> for SharedBytes<'static> {
    fn from(bytes: Box<[u8]>) -> SharedBytes<'static> {
        SharedBytes::ByArc(bytes.into())
    }
}
impl From<Vec<u8>> for SharedBytes<'static> {
    fn from(bytes: Vec<u8>) -> SharedBytes<'static> {
        SharedBytes::ByArc(bytes.into())
    }
}

/// Represents a Unicode code point.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Codepoint(pub u32);
/// Represents a glyph identifier for a particular font. This identifier will not necessarily
/// correspond to the correct glyph in a font other than the one that it was obtained from.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GlyphId(pub u32);
/// A single glyph of a font. this is a thin wrapper referring to the font,
/// glyph id and scaling information.
#[derive(Clone)]
struct Glyph<'a> {
    inner: GlyphInner<'a>,
    v: Vec2,
}

#[derive(Clone)]
struct GlyphInner<'a>(Font<'a>, u32);

/// The "horizontal metrics" of a glyph. This is useful for calculating the
/// horizontal offset of a glyph from the previous one in a string when laying a
/// string out horizontally.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct HMetrics {
    /// The horizontal offset that the origin of the next glyph should be from
    /// the origin of this glyph.
    pub advance_width: f32,
    /// The horizontal offset between the origin of this glyph and the leftmost
    /// edge/point of the glyph.
    pub left_side_bearing: f32,
}
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
/// The "vertical metrics" of a font at a particular scale. This is useful for
/// calculating the amount of vertical space to give a line of text, and for
/// computing the vertical offset between successive lines.
struct VMetrics {
    /// The highest point that any glyph in the font extends to above the
    /// baseline. Typically positive.
    pub ascent: f32,
    /// The lowest point that any glyph in the font extends to below the
    /// baseline. Typically negative.
    pub descent: f32,
    /// The gap to leave between the descent of one line and the ascent of the
    /// next. This is of course only a guideline given by the font's designers.
    pub line_gap: f32,
}

impl From<tt::VMetrics> for VMetrics {
    fn from(vm: tt::VMetrics) -> Self {
        Self {
            ascent: vm.ascent as f32,
            descent: vm.descent as f32,
            line_gap: vm.line_gap as f32,
        }
    }
}
/// A trait for types that can be converted into a `GlyphId`, in the context of
/// a specific font.
///
/// Many `fonterator` functions that operate on characters accept values of any
/// type that implements `IntoGlyphId`. Such types include `char`, `Codepoint`,
/// and obviously `GlyphId` itself.
trait IntoGlyphId {
    /// Convert `self` into a `GlyphId`, consulting the index map of `font` if
    /// necessary.
    fn into_glyph_id(self, a: &Font) -> GlyphId;
}
impl IntoGlyphId for char {
    fn into_glyph_id(self, font: &Font) -> GlyphId {
        GlyphId(font.info.find_glyph_index(self as u32))
    }
}
impl IntoGlyphId for Codepoint {
    fn into_glyph_id(self, font: &Font) -> GlyphId {
        GlyphId(font.info.find_glyph_index(self.0))
    }
}
impl IntoGlyphId for GlyphId {
    fn into_glyph_id(self, _font: &Font) -> GlyphId {
        self
    }
}
impl<'a> FontCollection<'a> {
    /// Constructs a font collection from an array of bytes, typically loaded
    /// from a font file, which may be a single font or a TrueType Collection
    /// holding a number of fonts. This array may be owned (e.g. `Vec<u8>`), or
    /// borrowed (`&[u8]`). As long as `From<T>` is implemented for `Bytes` for
    /// some type `T`, `T` can be used as input.
    ///
    /// This returns an error if `bytes` does not seem to be font data in a
    /// format we recognize.
    pub fn new<B: Into<SharedBytes<'a>>>(bytes: B) -> Result<FontCollection<'a>, Error> {
        let bytes = bytes.into();
        // We should use tt::is_collection once it lands in stb_truetype-rs:
        // https://github.com/redox-os/stb_truetype-rs/pull/15
        if !tt::is_font(&bytes) && &bytes[0..4] != b"ttcf" {
            return Err(Error::UnrecognizedFormat);
        }

        Ok(FontCollection(bytes))
    }

    /// If this `FontCollection` holds a single font, or a TrueType Collection
    /// containing only one font, return that as a `Font`. The `FontCollection`
    /// is consumed.
    ///
    /// If this `FontCollection` holds multiple fonts, return a
    /// `CollectionContainsMultipleFonts` error.
    ///
    /// If an error occurs, the `FontCollection` is lost, since this function
    /// takes ownership of it, and the error values don't give it back. If that
    /// is a problem, use the `font_at` or `into_fonts` methods instead, which
    /// borrow the `FontCollection` rather than taking ownership of it.
    pub fn into_font(self) -> Result<Font<'a>, Error> {
        let offset = if tt::is_font(&self.0) {
            0
        } else if tt::get_font_offset_for_index(&self.0, 1).is_some() {
            return Err(Error::CollectionContainsMultipleFonts);
        } else {
            // We now know that either a) `self.0` is a collection with only one
            // font, or b) `get_font_offset_for_index` found data it couldn't
            // recognize. Request the first font's offset, distinguishing
            // those two cases.
            match tt::get_font_offset_for_index(&self.0, 0) {
                None => return Err(Error::IllFormed),
                Some(offset) => offset,
            }
        };
        let info = tt::FontInfo::new(self.0, offset as usize).ok_or(Error::IllFormed)?;
        Ok(Font { info })
    }

    /// Gets the font at index `i` in the font collection, if it exists and is
    /// valid. The produced font borrows the font data that is either borrowed
    /// or owned by this font collection.
    pub fn font_at(&self, i: usize) -> Result<Font<'a>, Error> {
        let offset = tt::get_font_offset_for_index(&self.0, i as i32)
            .ok_or(Error::CollectionIndexOutOfBounds)?;
        let info = tt::FontInfo::new(self.0.clone(), offset as usize).ok_or(Error::IllFormed)?;
        Ok(Font { info })
    }

    /// Converts `self` into an `Iterator` yielding each `Font` that exists
    /// within the collection.
    pub fn into_fonts(self) -> Vec<Font<'a>> {
        let mut fonts = vec![];
        let mut index = 0;

        loop {
            let result = self.font_at(index);
            if let Err(Error::CollectionIndexOutOfBounds) = result {
                break;
            }
            index += 1;
            fonts.push(result.unwrap());
        }

        fonts
    }
}

/// An iterator created by `FontGroup.render()` for `PathOp`s.
pub struct PathIterator<'a> {
    glyph_iter: GlyphIterator<'a>,
    glyph: Option<Glyph<'a>>,
    x: f32,  // initial x (for Return and Vertical Tab)
    y: f32,  // initial y (for Vertical Alignment for East Asian languages)
    cx: f32, // current x
    cy: f32, // current y
    oc: usize,
    vt: bool,     // Should text be written vertically?
    rl: bool,     // Should text be written right to left?
    ch: char,     // Current character.
    advance: f32, // Advance character width
    f: f32, // When drawing vertically, how much of square character is taken up by latin letters.
}

static mut OP: PathOp = PathOp::Close();

impl<'a> PathIterator<'a> {
    fn new(glyph_iter: GlyphIterator<'a>, xy: (f32, f32)) -> Self {
        PathIterator {
            glyph_iter,
            glyph: None,
            x: xy.0,
            y: xy.1,
            cx: xy.0,
            cy: xy.1,
            oc: 0,
            vt: false,
            rl: false,
            ch: '\0',
            advance: 0.0,
            f: 0.0,
        }
    }

    /// Modify the iterator to align characters vertically rather than horizontally.
    pub fn vertical(mut self) -> Self {
        self.vt = true;
        self
    }

    /// Modify the iterator to align characters right to left.
    pub fn right_to_left(mut self) -> Self {
        self.rl = true;
        self
    }

    /// Consume the iterator and get the current x and y positions.
    pub fn xy(self) -> (f32, f32) {
        (self.cx, self.cy)
    }
}

impl<'a> Iterator for &mut PathIterator<'a> {
    type Item = &'static PathOp;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ch == '\0' {
            self.ch = '\x01';
            return Some(&PathOp::PenWidth(0.0));
        }

        // If no glyph, then get next.
        if self.glyph.is_none() {
            let (glyph, advance) = if let Some(ch) = self.glyph_iter.string.get(self.glyph_iter.cursor)
            {
                self.ch = *ch;
                self.glyph_iter.next().unwrap() // was ?
            } else {
                // IF-ELSE for ADVANCE TODO: IS COPY-ISH
                if self.vt {
                    // Vertical text
                    self.f += self.advance;
                    self.cy += 0.0; // TODO glyph.font().v_metrics(glyph.v);
                } else {
                    // Horizontal text
                    self.cx += if self.rl { -self.advance } else { self.advance };
                }
                // END IF-ELSE for ADVANCE
                return None;
            };

            if self.ch == '\n' {
                self.advance = 0.0;
                if self.vt {
                    // Vertical text
                    self.cy = self.y;
                    self.cx += if self.rl { -1.0 } else { 1.0 } * glyph.font().v_metrics(glyph.v);
                    self.f = 0.0;
                } else {
                    // Horizontal text
                    self.cx = self.x;
                    self.cy += glyph.font().v_metrics(glyph.v);
                }
                return Self::next(self);
            } else if self.ch == ' ' && self.vt {
                self.advance = 0.0;
                self.f = 0.0;
                self.cy += glyph.font().v_metrics(glyph.v) * 2.0;
                return Self::next(self);
            } else if self.ch == '\t' {
                if self.vt {
                    self.advance = 0.0;
                    self.f = 0.0;
                    self.cy += glyph.font().v_metrics(glyph.v);
                } else {
                    self.cx += glyph.font().v_metrics(glyph.v) * if self.rl { -4.0 } else { 4.0 };
                }
                return Self::next(self);
            }
            self.oc = 0;
            // IF-ELSE for ADVANCE TODO: HAS COPY
            if self.vt {
                // Vertical text
                self.f += self.advance;
                if self.ch == '.' {
                    self.f = glyph.font().v_metrics(glyph.v) - advance;
                } else if self.f + advance >= glyph.font().v_metrics(glyph.v)
                    && advance <= glyph.font().v_metrics(glyph.v)
                {
                    self.f = 0.0;
                    self.cy += glyph.font().v_metrics(glyph.v);
                }
            } else {
                // Horizontal text
                self.cx += if self.rl { -self.advance } else { self.advance };
            }
            // END IF-ELSE for ADVANCE
            self.advance = if self.glyph_iter.mono.is_some() {
                if unicode_width::UnicodeWidthChar::width(self.ch) == Some(2) {
                    glyph.font().v_metrics(glyph.v)
                } else {
                    glyph.font().v_metrics(glyph.v) / 2.0
                }
            } else {
                advance
            };
            self.glyph = Some(glyph);
        }

        let shape = {
            let glyph = self.glyph.as_ref().unwrap();
            let (font, id) = (glyph.font(), glyph.id());

            font.info.get_glyph_shape(id.0).unwrap_or_else(Vec::new)
        };

        let v = if let Some(v) = shape.get(self.oc) {
            v
        } else {
            self.glyph = None;
            return Self::next(self);
        };

        let glyph = self.glyph.as_ref().unwrap();
        let ay = glyph.font().v_metrics(glyph.v);

        let x = v.x as f32 * glyph.v.0
            + self.cx
            + if self.rl {
                -self.advance - self.f
            } else {
                self.f
            };
        let y = -v.y as f32 * glyph.v.1 + self.cy + ay;

        use crate::tt::VertexType;

        match v.vertex_type() {
            VertexType::LineTo => unsafe { OP = PathOp::Line(x, y) },
            VertexType::CurveTo => {
                let cx = v.cx as f32 * glyph.v.0
                    + self.cx
                    + if self.rl {
                        -self.advance - self.f
                    } else {
                        self.f
                    };
                let cy = -v.cy as f32 * glyph.v.1 + self.cy + ay;

                unsafe { OP = PathOp::Quad(cx, cy, x, y) };
            }
            VertexType::MoveTo => unsafe { OP = PathOp::Move(x, y) },
        }

        self.oc += 1;

        unsafe { Some(&OP) }
    }
}

/// An iterator over glyphs in a string.
struct GlyphIterator<'a> {
    // The font
    font: &'a Vec<Font<'a>>,
    // Scaling info
    api_scale: (f32, f32),
    // ...
    scale: Vec2,
    // Normalized string
    string: Vec<char>,
    // Which character in the string
    cursor: usize,
    // The previous glyph
    last: Option<(Glyph<'a>, &'a Font<'a>)>,
    //
    mono: Option<f32>,
}

impl<'a> Iterator for GlyphIterator<'a> {
    type Item = (Glyph<'a>, f32);

    fn next(&mut self) -> Option<(Glyph<'a>, f32)> {
        let c = self.string.get(self.cursor);

        if let Some(c) = c {
            let mut i = 0;
            let glyph: Glyph<'a> = loop {
                let mono = if self.mono.is_some()
                    && unicode_width::UnicodeWidthChar::width(*c) == Some(1)
                {
                    self.mono
                } else {
                    None
                };
                let (glyph, hit): (Glyph<'a>, bool) = self.font[i].glyph(*c, self.scale, mono);
                if hit || i == self.font.len() - 1 {
                    break glyph;
                }
                i += 1;
            };

            let mut advance = self.font[i]
                .info
                .get_glyph_h_metrics(glyph.id().0)
                .advance_width as f32
                * self.scale.0;

            if self.cursor != 0 {
                advance += self.font[i].kerning(
                    self.api_scale,
                    self.scale,
                    self.last.as_ref().unwrap(),
                    &glyph,
                );
            }

            self.last = Some((glyph.clone(), &self.font[i]));
            self.cursor += 1;
            Some((glyph, advance))
        } else {
            None
        }
    }
}

impl<'a> Font<'a> {
    /// The "vertical metrics" for this font at a given scale. These metrics are
    /// shared by all of the glyphs in the font. See `VMetrics` for more detail.
    fn v_metrics(&self, scale: Vec2) -> f32 {
        let vm = self.info.get_v_metrics();
        let scale = scale.1;
        (vm.ascent as f32) * scale
    }

    /// The number of glyphs present in this font. Glyph identifiers for this
    /// font will always be in the range `0..self.glyph_count()`
    fn glyph_count(&self) -> usize {
        self.info.get_num_glyphs() as usize
    }

    /// Returns the corresponding glyph for a Unicode code point or a glyph id
    /// for this font.
    ///
    /// If `id` is a `GlyphId`, it must be valid for this font; otherwise, this
    /// function panics. `GlyphId`s should always be produced by looking up some
    /// other sort of designator (like a Unicode code point) in a font, and
    /// should only be used to index the font they were produced for.
    ///
    /// Note that code points without corresponding glyphs in this font map to
    /// the ".notdef" glyph, glyph 0.
    fn glyph<C: IntoGlyphId>(&self, id: C, mut v: Vec2, mono: Option<f32>) -> (Glyph<'a>, bool) {
        let gid = id.into_glyph_id(self);

        if let Some(glyphb) = mono {
            let glypha = self.glyph('a', v, None).0;
            let glypha = self.info.get_glyph_h_metrics(glypha.id().0).advance_width as f32;

            v.0 *= 0.5 * glyphb / glypha;
        }

        assert!((gid.0 as usize) < self.glyph_count());
        // font clone either a reference clone, or arc clone
        (Glyph::new(GlyphInner(self.clone(), gid.0), v), gid.0 != 0)
    }

    /// Returns additional kerning to apply as well as that given by HMetrics
    /// for a particular pair of glyphs.
    fn pair_kerning<A, B>(
        &self,
        scale: (f32, f32),
        v: Vec2,
        first: A,
        second: B,
        old: &'a Font<'a>,
    ) -> f32
    where
        A: IntoGlyphId,
        B: IntoGlyphId,
    {
        let (first, second) = (old.glyph(first, v, None).0, self.glyph(second, v, None).0);
        let factor = self.info.scale_for_pixel_height(scale.1) * (scale.0 / scale.1);
        let kern = self
            .info
            .get_glyph_kern_advance(first.id().0, second.id().0);
        factor * kern as f32
    }
    /// Get the proper spacing from the start of one character to the next.
    fn kerning(
        &self,
        scale: (f32, f32),
        v: Vec2,
        first: &(Glyph<'a>, &'a Font<'a>),
        second: &Glyph<'a>,
    ) -> f32 {
        self.pair_kerning(scale, v, first.0.id(), second.id(), first.1)
    }
}
impl<'a> Glyph<'a> {
    fn new(inner: GlyphInner<'a>, v: Vec2) -> Glyph<'a> {
        Glyph { inner, v }
    }

    /// The font to which this glyph belongs.
    fn font(&self) -> &Font<'a> {
        &self.inner.0
    }

    /// The glyph identifier for this glyph.
    fn id(&self) -> GlyphId {
        GlyphId(self.inner.1)
    }
}

/// The type for errors returned by Fonterator.
#[derive(Debug)]
pub enum Error {
    /// Font data presented to Fonterator is not in a format that the
    /// library recognizes.
    UnrecognizedFormat,

    /// Font data presented to Fonterator was ill-formed (lacking necessary
    /// tables, for example).
    IllFormed,

    /// The caller tried to access the `i`'th font from a `FontCollection`,
    /// but the collection doesn't contain that many fonts.
    CollectionIndexOutOfBounds,

    /// The caller tried to convert a `FontCollection` into a font via
    /// `into_font`, but the `FontCollection` contains more than one font.
    CollectionContainsMultipleFonts,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        f.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            UnrecognizedFormat => "Font data in unrecognized format",
            IllFormed => "Font data is ill-formed",
            CollectionIndexOutOfBounds => "Font collection has no font at the given index",
            CollectionContainsMultipleFonts => {
                "Attempted to convert collection into a font, \
                 but collection contais more than one font"
            }
        }
    }
}

impl std::convert::From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error)
    }
}
