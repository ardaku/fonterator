// Copyright Jeron Lau 2018.
// Copyright Dylan Ede 2016.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

use byteorder::BigEndian as BE;
use byteorder::ByteOrder;
use std::ops::Deref;
#[derive(Clone, Debug)]
pub(crate) struct FontInfo<Data: Deref<Target = [u8]>> {
    data: Data, // pointer to .ttf file
    // fontstart: usize,	   // offset of start of font
    num_glyphs: u32, // number of glyphs, needed for range checking
    loca: u32,
    head: u32,
    glyf: u32,
    hhea: u32,
    hmtx: u32,
    name: u32,
    kern: u32,                // table locations as offset from start of .ttf
    index_map: u32,           // a cmap mapping for our chosen character encoding
    index_to_loc_format: u32, // format needed to map from glyph index to glyph
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Vertex {
    pub(crate) x: i16,
    pub(crate) y: i16,
    pub(crate) cx: i16,
    pub(crate) cy: i16,
    type_: u8,
}

impl Vertex {
    pub(crate) fn vertex_type(&self) -> VertexType {
        unsafe { ::std::mem::transmute(self.type_) }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub(crate) enum VertexType {
    MoveTo = 1,
    LineTo = 2,
    CurveTo = 3,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Rect<T> {
    pub(crate) x0: T,
    pub(crate) y0: T,
    pub(crate) x1: T,
    pub(crate) y1: T,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct HMetrics {
    pub(crate) advance_width: i32,
    pub(crate) left_side_bearing: i32,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct VMetrics {
    pub(crate) ascent: i32,
    pub(crate) descent: i32,
    pub(crate) line_gap: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum PlatformId {
    // platformID
    Unicode = 0,
    Mac = 1,
    Iso = 2,
    Microsoft = 3,
}

fn platform_id(v: u16) -> Option<PlatformId> {
    use tt::PlatformId::*;
    match v {
        0 => Some(Unicode),
        1 => Some(Mac),
        2 => Some(Iso),
        3 => Some(Microsoft),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum MicrosoftEid {
    // encodingID for PLATFORM_ID_MICROSOFT
    Symbol = 0,
    UnicodeBMP = 1,
    Shiftjis = 2,
    UnicodeFull = 10,
}

fn microsoft_eid(v: u16) -> Option<MicrosoftEid> {
    use tt::MicrosoftEid::*;
    match v {
        0 => Some(Symbol),
        1 => Some(UnicodeBMP),
        2 => Some(Shiftjis),
        10 => Some(UnicodeFull),
        _ => None,
    }
}

// # accessors to parse data from file

// on platforms that don't allow misaligned reads, if we want to allow
// truetype fonts that aren't padded to alignment, define ALLOW_UNALIGNED_TRUETYPE

pub(crate) fn is_font(font: &[u8]) -> bool {
    if font.len() >= 4 {
        let tag = &font[0..4];
        tag == [b'1', 0, 0, 0] || tag == b"typ1" || tag == b"OTTO" || tag == [0, 1, 0, 0]
    } else {
        false
    }
}

fn find_table(data: &[u8], fontstart: usize, tag: &[u8]) -> u32 {
    let num_tables = BE::read_u16(&data[fontstart + 4..]);
    let tabledir = fontstart + 12;
    for i in 0..num_tables {
        let loc = tabledir + 16 * (i as usize);
        if &data[loc..loc + 4] == tag {
            return BE::read_u32(&data[loc + 8..]);
        }
    }
    return 0;
}

/// Each .ttf/.ttc file may have more than one font. Each font has a sequential
/// index number starting from 0. Call this function to get the font offset for
/// a given index; it returns None if the index is out of range. A regular .ttf
/// file will only define one font and it always be at offset 0, so it will
/// return Some(0) for index 0, and None for all other indices. You can just skip
/// this step if you know it's that kind of font.
pub(crate) fn get_font_offset_for_index(font_collection: &[u8], index: i32) -> Option<u32> {
    // if it's just a font, there's only one valid index
    if is_font(font_collection) {
        return if index == 0 { Some(0) } else { None };
    }
    // check if it's a TTC
    if &font_collection[0..4] == b"ttcf" {
        // version 1?
        if BE::read_u32(&font_collection[4..]) == 0x00010000
            || BE::read_u32(&font_collection[4..]) == 0x00020000
        {
            let n = BE::read_i32(&font_collection[8..]);
            if index >= n {
                return None;
            }
            return Some(BE::read_u32(&font_collection[12 + (index as usize) * 4..]));
        }
    }
    return None;
}

impl<Data: Deref<Target = [u8]>> FontInfo<Data> {
    /// Given an offset into the file that defines a font, this function builds
    /// the necessary cached info for the rest of the system.
    pub(crate) fn new(data: Data, fontstart: usize) -> Option<FontInfo<Data>> {
        let cmap = find_table(&data, fontstart, b"cmap"); // required
        let loca = find_table(&data, fontstart, b"loca"); // required
        let head = find_table(&data, fontstart, b"head"); // required
        let glyf = find_table(&data, fontstart, b"glyf"); // required
        let hhea = find_table(&data, fontstart, b"hhea"); // required
        let hmtx = find_table(&data, fontstart, b"hmtx"); // required
        let name = find_table(&data, fontstart, b"name"); // not required
        let kern = find_table(&data, fontstart, b"kern"); // not required
        if cmap == 0 || loca == 0 || head == 0 || glyf == 0 || hhea == 0 || hmtx == 0 {
            return None;
        }
        let t = find_table(&data, fontstart, b"maxp");
        let num_glyphs = if t != 0 {
            BE::read_u16(&data[t as usize + 4..])
        } else {
            0xffff
        };

        // find a cmap encoding table we understand *now* to avoid searching
        // later. (todo: could make this installable)
        // the same regardless of glyph.
        let num_tables = BE::read_u16(&data[cmap as usize + 2..]);
        let mut index_map = 0;
        for i in 0..num_tables {
            let encoding_record = (cmap + 4 + 8 * (i as u32)) as usize;
            // find an encoding we understand:
            match platform_id(BE::read_u16(&data[encoding_record..])) {
                Some(PlatformId::Microsoft) => {
                    match microsoft_eid(BE::read_u16(&data[encoding_record + 2..])) {
                        Some(MicrosoftEid::UnicodeBMP) | Some(MicrosoftEid::UnicodeFull) => {
                            // MS/Unicode
                            index_map = cmap + BE::read_u32(&data[encoding_record + 4..]);
                        }
                        _ => (),
                    }
                }
                Some(PlatformId::Unicode) => {
                    // Mac/iOS has these
                    // all the encodingIDs are unicode, so we don't bother to check it
                    index_map = cmap + BE::read_u32(&data[encoding_record + 4..]);
                }
                _ => (),
            }
        }
        if index_map == 0 {
            return None;
        }
        let index_to_loc_format = BE::read_u16(&data[head as usize + 50..]) as u32;
        Some(FontInfo {
            // fontstart: fontstart,
            data: data,
            loca: loca,
            head: head,
            glyf: glyf,
            hhea: hhea,
            hmtx: hmtx,
            name: name,
            kern: kern,
            num_glyphs: num_glyphs as u32,
            index_map: index_map,
            index_to_loc_format: index_to_loc_format,
        })
    }

    pub(crate) fn get_num_glyphs(&self) -> u32 {
        self.num_glyphs
    }

    /// If you're going to perform multiple operations on the same character
    /// and you want a speed-up, call this function with the character you're
    /// going to process, then use glyph-based functions instead of the
    /// codepoint-based functions.
    pub(crate) fn find_glyph_index(&self, unicode_codepoint: u32) -> u32 {
        let data = &self.data;
        let index_map = &data[self.index_map as usize..]; //self.index_map as usize;

        let format = BE::read_u16(index_map);
        match format {
            0 => {
                // apple byte encoding
                let bytes = BE::read_u16(&index_map[2..]);
                if unicode_codepoint < bytes as u32 - 6 {
                    return index_map[6 + unicode_codepoint as usize] as u32;
                }
                return 0;
            }
            6 => {
                let first = BE::read_u16(&index_map[6..]) as u32;
                let count = BE::read_u16(&index_map[8..]) as u32;
                if unicode_codepoint >= first && unicode_codepoint < first + count {
                    return BE::read_u16(&index_map[10 + (unicode_codepoint - first) as usize * 2..])
                        as u32;
                }
                return 0;
            }
            2 => {
                // @TODO: high-byte mapping for japanese/chinese/korean
                panic!("Index map format unsupported: 2");
            }
            4 => {
                // standard mapping for windows fonts: binary search collection of ranges
                let segcount = BE::read_u16(&index_map[6..]) as usize >> 1;
                let mut search_range = BE::read_u16(&index_map[8..]) as usize >> 1;
                let mut entry_selector = BE::read_u16(&index_map[10..]);
                let range_shift = BE::read_u16(&index_map[12..]) as usize >> 1;

                // do a binary search of the segments
                let end_count = self.index_map as usize + 14;
                let mut search = end_count;

                if unicode_codepoint > 0xffff {
                    return 0;
                }

                // they lie from endCount .. endCount + segCount
                // but searchRange is the nearest power of two, so...
                if unicode_codepoint >= BE::read_u16(&data[search + range_shift * 2..]) as u32 {
                    search += range_shift * 2;
                }

                // now decrement to bias correctly to find smallest
                search -= 2;
                while entry_selector != 0 {
                    search_range >>= 1;
                    let end = BE::read_u16(&data[search + search_range * 2..]) as u32;
                    if unicode_codepoint > end {
                        search += search_range * 2;
                    }
                    entry_selector -= 1;
                }
                search += 2;

                {
                    let item = (search - end_count) >> 1;
                    assert!(
                        unicode_codepoint <= BE::read_u16(&data[end_count + 2 * item..]) as u32
                    );
                    let start = BE::read_u16(&index_map[14 + segcount * 2 + 2 + 2 * item..]) as u32;
                    if unicode_codepoint < start {
                        return 0;
                    }
                    let offset =
                        BE::read_u16(&index_map[14 + segcount * 6 + 2 + 2 * item..]) as usize;
                    if offset == 0 {
                        return (unicode_codepoint as i32 + BE::read_i16(
                            &index_map[14 + segcount * 4 + 2 + 2 * item..],
                        ) as i32) as u16 as u32;
                    }
                    return BE::read_u16(
                        &index_map[offset
                                       + (unicode_codepoint - start) as usize * 2
                                       + 14
                                       + segcount * 6
                                       + 2
                                       + 2 * item..],
                    ) as u32;
                }
            }
            12 | 13 => {
                let ngroups = BE::read_u32(&index_map[12..]) as usize;
                let mut low = 0;
                let mut high = ngroups;
                // Binary search of the right group
                while low < high {
                    let mid = low + ((high - low) >> 1); // rounds down, so low <= mid < high
                    let start_char = BE::read_u32(&index_map[16 + mid * 12..]);
                    let end_char = BE::read_u32(&index_map[16 + mid * 12 + 4..]);
                    if unicode_codepoint < start_char {
                        high = mid;
                    } else if unicode_codepoint > end_char {
                        low = mid + 1;
                    } else {
                        let start_glyph = BE::read_u32(&index_map[16 + mid * 12 + 8..]);
                        if format == 12 {
                            return start_glyph + unicode_codepoint - start_char;
                        } else {
                            return start_glyph;
                        }
                    }
                }
                return 0;
            }
            n => panic!("Index map format unsupported: {}", n),
        }
    }

    fn get_glyf_offset(&self, glyph_index: u32) -> Option<u32> {
        let g1;
        let g2;
        if glyph_index >= self.num_glyphs || self.index_to_loc_format >= 2 {
            // glyph index out of range or unknown index->glyph map format
            return None;
        }

        if self.index_to_loc_format == 0 {
            g1 = self.glyf
                + BE::read_u16(&self.data[(self.loca + glyph_index * 2) as usize..]) as u32 * 2;
            g2 = self.glyf
                + BE::read_u16(&self.data[(self.loca + glyph_index * 2 + 2) as usize..]) as u32 * 2;
        } else {
            g1 = self.glyf + BE::read_u32(&self.data[(self.loca + glyph_index * 4) as usize..]);
            g2 = self.glyf + BE::read_u32(&self.data[(self.loca + glyph_index * 4 + 4) as usize..]);
        }
        if g1 == g2 {
            None
        } else {
            Some(g1)
        }
    }

    /// Like `get_codepoint_shape`, but takes a glyph index instead. Use this if you have cached the
    /// glyph index for a codepoint.
    pub(crate) fn get_glyph_shape(&self, glyph_index: u32) -> Option<Vec<Vertex>> {
        use tt::VertexType::*;
        fn close_shape(
            vertices: &mut [Vertex],
            num_vertices: &mut usize,
            was_off: bool,
            start_off: bool,
            sx: i32,
            sy: i32,
            scx: i32,
            scy: i32,
            cx: i32,
            cy: i32,
        ) {
            use tt::VertexType::*;
            if start_off {
                if was_off {
                    vertices[*num_vertices] = Vertex {
                        type_: CurveTo as u8,
                        x: ((cx + scx) >> 1) as i16,
                        y: ((cy + scy) >> 1) as i16,
                        cx: cx as i16,
                        cy: cy as i16,
                    };
                    *num_vertices += 1;
                }
                vertices[*num_vertices] = Vertex {
                    type_: CurveTo as u8,
                    x: sx as i16,
                    y: sy as i16,
                    cx: scx as i16,
                    cy: scy as i16,
                };
            } else {
                vertices[*num_vertices] = if was_off {
                    Vertex {
                        type_: CurveTo as u8,
                        x: sx as i16,
                        y: sy as i16,
                        cx: cx as i16,
                        cy: cy as i16,
                    }
                } else {
                    Vertex {
                        type_: LineTo as u8,
                        x: sx as i16,
                        y: sy as i16,
                        cx: 0,
                        cy: 0,
                    }
                };
            }
            *num_vertices += 1;
        }

        let g = match self.get_glyf_offset(glyph_index) {
            Some(g) => &self.data[g as usize..],
            None => return None,
        };

        let number_of_contours = BE::read_i16(g);
        let vertices: Vec<Vertex> = if number_of_contours > 0 {
            let number_of_contours = number_of_contours as usize;
            let mut start_off = false;
            let mut was_off = false;
            let end_points_of_contours = &g[10..];
            let ins = BE::read_u16(&g[10 + number_of_contours * 2..]) as usize;
            let mut points = &g[10 + number_of_contours * 2 + 2 + ins..];

            let n =
                1 + BE::read_u16(&end_points_of_contours[number_of_contours * 2 - 2..]) as usize;

            let m = n + 2 * number_of_contours; // a loose bound on how many vertices we might need
            let mut vertices: Vec<Vertex> = Vec::with_capacity(m);
            unsafe { vertices.set_len(m) };

            let mut next_move = 0;
            let mut flagcount = 0;

            // in first pass, we load uninterpreted data into the allocated array
            // above, shifted to the end of the array so we won't overwrite it when
            // we create our final data starting from the front

            // starting offset for uninterpreted data, regardless of how m ends up being calculated
            let off = m - n;

            // first load flags
            let mut flags = 0;
            for i in 0..n {
                if flagcount == 0 {
                    flags = points[0];
                    points = &points[1..];
                    if flags & 8 != 0 {
                        flagcount = points[0];
                        points = &points[1..];
                    }
                } else {
                    flagcount -= 1;
                }
                vertices[off + i].type_ = flags;
            }

            // now load x coordinates
            let mut x = 0i32;
            for i in 0..n {
                let flags = vertices[off + i].type_;
                if flags == 255 {
                    println!("{:?}", flags);
                }
                if flags & 2 != 0 {
                    let dx = points[0] as i32;
                    points = &points[1..];
                    if flags & 16 != 0 {
                        // ???
                        x += dx;
                    } else {
                        x -= dx;
                    }
                } else {
                    if flags & 16 == 0 {
                        x += points[0] as i32 * 256 + points[1] as i32;
                        points = &points[2..];
                    }
                }
                vertices[off + i].x = x as i16;
            }

            // now load y coordinates
            let mut y = 0i32;
            for i in 0..n {
                let flags = vertices[off + i].type_;
                if flags & 4 != 0 {
                    let dy = points[0] as i32;
                    points = &points[1..];
                    if flags & 32 != 0 {
                        y += dy;
                    } else {
                        y -= dy;
                    }
                } else {
                    if flags & 32 == 0 {
                        y += points[0] as i32 * 256 + points[1] as i32;
                        points = &points[2..];
                    }
                }
                vertices[off + i].y = y as i16;
            }

            // now convert them to our format
            let mut num_vertices = 0;
            let mut sx = 0;
            let mut sy = 0;
            let mut cx = 0;
            let mut cy = 0;
            let mut scx = 0;
            let mut scy = 0;
            let mut i = 0;
            let mut j = 0;
            while i < n {
                let flags = vertices[off + i].type_;
                x = vertices[off + i].x as i32;
                y = vertices[off + i].y as i32;

                if next_move == i {
                    if i != 0 {
                        close_shape(
                            &mut vertices[..],
                            &mut num_vertices,
                            was_off,
                            start_off,
                            sx,
                            sy,
                            scx,
                            scy,
                            cx,
                            cy,
                        );
                    }

                    // now start the new one
                    start_off = flags & 1 == 0;
                    if start_off {
                        // if we start off with an off-curve point, then when we need to find a point on the curve
                        // where we can start, and we need to save some state for when we wraparound.
                        scx = x;
                        scy = y;
                        if vertices[off + i + 1].type_ as u8 & 1 == 0 {
                            // next point is also a curve point, so interpolate an on-point curve
                            sx = (x + vertices[off + i + 1].x as i32) >> 1;
                            sy = (y + vertices[off + i + 1].y as i32) >> 1;
                        } else {
                            // otherwise just use the next point as our start point
                            sx = vertices[off + i + 1].x as i32;
                            sy = vertices[off + i + 1].y as i32;
                            i += 1; // we're using point i+1 as the starting point, so skip it
                        }
                    } else {
                        sx = x;
                        sy = y;
                    }
                    vertices[num_vertices] = Vertex {
                        type_: MoveTo as u8,
                        x: sx as i16,
                        y: sy as i16,
                        cx: 0,
                        cy: 0,
                    };
                    num_vertices += 1;
                    was_off = false;
                    next_move = 1 + BE::read_u16(&end_points_of_contours[j * 2..]) as usize;
                    j += 1;
                } else {
                    if flags & 1 == 0 {
                        // if it's a curve
                        if was_off {
                            // two off-curve control points in a row means interpolate an on-curve midpoint
                            vertices[num_vertices] = Vertex {
                                type_: CurveTo as u8,
                                x: ((cx + x) >> 1) as i16,
                                y: ((cy + y) >> 1) as i16,
                                cx: cx as i16,
                                cy: cy as i16,
                            };
                            num_vertices += 1;
                        }
                        cx = x;
                        cy = y;
                        was_off = true;
                    } else {
                        if was_off {
                            vertices[num_vertices] = Vertex {
                                type_: CurveTo as u8,
                                x: x as i16,
                                y: y as i16,
                                cx: cx as i16,
                                cy: cy as i16,
                            }
                        } else {
                            vertices[num_vertices] = Vertex {
                                type_: LineTo as u8,
                                x: x as i16,
                                y: y as i16,
                                cx: 0 as i16,
                                cy: 0 as i16,
                            }
                        }
                        num_vertices += 1;
                        was_off = false;
                    }
                }
                i += 1;
            }
            close_shape(
                &mut vertices[..],
                &mut num_vertices,
                was_off,
                start_off,
                sx,
                sy,
                scx,
                scy,
                cx,
                cy,
            );
            assert!(num_vertices <= vertices.len());
            unsafe { vertices.set_len(num_vertices) };
            vertices
        } else if number_of_contours == -1 {
            // Compound shapes
            let mut more = true;
            let mut comp = &g[10..];
            let mut vertices = Vec::new();
            while more {
                let mut mtx = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

                let flags = BE::read_i16(comp);
                comp = &comp[2..];
                let gidx = BE::read_u16(comp);
                comp = &comp[2..];

                if flags & 2 != 0 {
                    // XY values
                    if flags & 1 != 0 {
                        // shorts
                        mtx[4] = BE::read_i16(comp) as f32;
                        comp = &comp[2..];
                        mtx[5] = BE::read_i16(comp) as f32;
                        comp = &comp[2..];
                    } else {
                        mtx[4] = comp[0] as f32;
                        comp = &comp[1..];
                        mtx[5] = comp[0] as f32;
                        comp = &comp[1..];
                    }
                } else {
                    panic!("Matching points not supported.");
                }
                if flags & (1 << 3) != 0 {
                    // WE_HAVE_A_SCALE
                    mtx[0] = BE::read_i16(comp) as f32 / 16384.0;
                    comp = &comp[2..];
                    mtx[1] = 0.0;
                    mtx[2] = 0.0;
                    mtx[3] = mtx[0];
                } else if flags & (1 << 6) != 0 {
                    // WE_HAVE_AN_X_AND_YSCALE
                    mtx[0] = BE::read_i16(comp) as f32 / 16384.0;
                    comp = &comp[2..];
                    mtx[1] = 0.0;
                    mtx[2] = 0.0;
                    mtx[3] = BE::read_i16(comp) as f32 / 16384.0;
                    comp = &comp[2..];
                } else if flags & (1 << 7) != 0 {
                    // WE_HAVE_A_TWO_BY_TWO
                    mtx[0] = BE::read_i16(comp) as f32 / 16384.0;
                    comp = &comp[2..];
                    mtx[1] = BE::read_i16(comp) as f32 / 16384.0;
                    comp = &comp[2..];
                    mtx[2] = BE::read_i16(comp) as f32 / 16384.0;
                    comp = &comp[2..];
                    mtx[3] = BE::read_i16(comp) as f32 / 16384.0;
                    comp = &comp[2..];
                }

                // Find transformation scales.
                let m = (mtx[0] * mtx[0] + mtx[1] * mtx[1]).sqrt();
                let n = (mtx[2] * mtx[2] + mtx[3] * mtx[3]).sqrt();

                // Get indexed glyph.
                let mut comp_verts = self
                    .get_glyph_shape(gidx as u32)
                    .unwrap_or_else(|| Vec::new());
                if comp_verts.len() > 0 {
                    // Transform vertices
                    for v in &mut *comp_verts {
                        let (x, y, cx, cy) = (v.x as f32, v.y as f32, v.cx as f32, v.cy as f32);
                        *v = Vertex {
                            type_: v.type_,
                            x: (m * (mtx[0] * x + mtx[2] * y + mtx[4])) as i16,
                            y: (n * (mtx[1] * x + mtx[3] * y + mtx[5])) as i16,
                            cx: (m * (mtx[0] * cx + mtx[2] * cy + mtx[4])) as i16,
                            cy: (n * (mtx[1] * cx + mtx[3] * cy + mtx[5])) as i16,
                        };
                    }
                    // Append vertices.
                    vertices.append(&mut comp_verts);
                }
                // More components ?
                more = flags & (1 << 5) != 0;
            }
            vertices
        } else if number_of_contours < 0 {
            panic!("Contour format not supported.")
        } else {
            return None;
        };
        Some(vertices)
    }

    /// like `get_codepoint_h_metrics`, but takes a glyph index instead. Use this if you have cached the
    /// glyph index for a codepoint.
    pub(crate) fn get_glyph_h_metrics(&self, glyph_index: u32) -> HMetrics {
        let num_of_long_hor_metrics = BE::read_u16(&self.data[self.hhea as usize + 34..]) as usize;
        if (glyph_index as usize) < num_of_long_hor_metrics {
            HMetrics {
                advance_width: BE::read_i16(
                    &self.data[self.hmtx as usize + 4 * glyph_index as usize..],
                ) as i32,
                left_side_bearing: BE::read_i16(
                    &self.data[self.hmtx as usize + 4 * glyph_index as usize + 2..],
                ) as i32,
            }
        } else {
            HMetrics {
                advance_width: BE::read_i16(
                    &self.data[self.hmtx as usize + 4 * (num_of_long_hor_metrics - 1)..],
                ) as i32,
                left_side_bearing: BE::read_i16(
                    &self.data[self.hmtx as usize
                                   + 4 * num_of_long_hor_metrics
                                   + 2 * (glyph_index as isize - num_of_long_hor_metrics as isize)
                                       as usize..],
                ) as i32,
            }
        }
    }

    /// like `get_codepoint_kern_advance`, but takes glyph indices instead. Use this if you have cached the
    /// glyph indices for the codepoints.
    pub(crate) fn get_glyph_kern_advance(&self, glyph_1: u32, glyph_2: u32) -> i32 {
        let kern = &self.data[self.kern as usize..];
        // we only look at the first table. it must be 'horizontal' and format 0
        if self.kern == 0 || BE::read_u16(&kern[2..]) < 1 || BE::read_u16(&kern[8..]) != 1 {
            // kern not present, OR
            // no tables (need at least one), OR
            // horizontal flag not set in format
            return 0;
        }

        let mut l: i32 = 0;
        let mut r: i32 = BE::read_u16(&kern[10..]) as i32 - 1;
        let needle = glyph_1 << 16 | glyph_2;
        while l <= r {
            let m = (l + r) >> 1;
            let straw = BE::read_u32(&kern[18 + (m as usize) * 6..]); // note: unaligned read
            if needle < straw {
                r = m - 1;
            } else if needle > straw {
                l = m + 1;
            } else {
                return BE::read_i16(&kern[22 + (m as usize) * 6..]) as i32;
            }
        }
        0
    }

    /// `ascent` is the coordinate above the baseline the font extends; descent
    /// is the coordinate below the baseline the font extends (i.e. it is typically negative)
    /// `line_gap` is the spacing between one row's descent and the next row's ascent...
    /// so you should advance the vertical position by `ascent - descent + line_gap`
    /// these are expressed in unscaled coordinates, so you must multiply by
    /// the scale factor for a given size
    pub(crate) fn get_v_metrics(&self) -> VMetrics {
        let hhea = &self.data[self.hhea as usize..];
        VMetrics {
            ascent: BE::read_i16(&hhea[4..]) as i32,
            descent: BE::read_i16(&hhea[6..]) as i32,
            line_gap: BE::read_i16(&hhea[8..]) as i32,
        }
    }

    /// computes a scale factor to produce a font whose "height" is 'pixels' tall.
    /// Height is measured as the distance from the highest ascender to the lowest
    /// descender; in other words, it's equivalent to calling GetFontVMetrics
    /// and computing:
    ///	   scale = pixels / (ascent - descent)
    /// so if you prefer to measure height by the ascent only, use a similar calculation.
    pub(crate) fn scale_for_pixel_height(&self, height: f32) -> f32 {
        let hhea = &self.data[self.hhea as usize..];
        let fheight = BE::read_i16(&hhea[4..]) as f32 - BE::read_i16(&hhea[6..]) as f32;
        height / fheight
    }

    /// Returns the units per EM square of this font.
    pub(crate) fn units_per_em(&self) -> u16 {
        BE::read_u16(&self.data[self.head as usize + 18..])
    }
}
