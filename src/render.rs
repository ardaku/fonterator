// Copyright © 2018-2022 The Fonterator Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt, 
// LICENSE_BOOST_1_0.txt, and LICENSE_MIT.txt).
//! Rendering TTF glyphs with footile

use footile::PathOp;
use pointy::Pt;
use rustybuzz::Face;
use ttf_parser::{GlyphId, OutlineBuilder};

struct Outliner<'a> {
    // Path to write out to.
    path: &'a mut Vec<PathOp>,
    // Scale to 1.0 = 1 em
    scale: f32,
    // Where to draw the glyph X
    glyph_x: f32,
    // Where to draw the glyph Y
    glyph_y: f32,
}

impl OutlineBuilder for Outliner<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        let x = x + self.glyph_x;
        let y = y + self.glyph_y;
        self.path
            .push(PathOp::Move(Pt::new(x * self.scale, y * self.scale)));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let x = x + self.glyph_x;
        let y = y + self.glyph_y;
        self.path
            .push(PathOp::Line(Pt::new(x * self.scale, y * self.scale)));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let x1 = x1 + self.glyph_x;
        let y1 = y1 + self.glyph_y;
        let x = x + self.glyph_x;
        let y = y + self.glyph_y;
        self.path.push(PathOp::Quad(
            Pt::new(x1 * self.scale, y1 * self.scale),
            Pt::new(x * self.scale, y * self.scale),
        ));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let x1 = x1 + self.glyph_x;
        let y1 = y1 + self.glyph_y;
        let x2 = x2 + self.glyph_x;
        let y2 = y2 + self.glyph_y;
        let x = x + self.glyph_x;
        let y = y + self.glyph_y;
        self.path.push(PathOp::Cubic(
            Pt::new(x1 * self.scale, y1 * self.scale),
            Pt::new(x2 * self.scale, y2 * self.scale),
            Pt::new(x * self.scale, y * self.scale),
        ));
    }

    fn close(&mut self) {
        self.path.push(PathOp::Close());
    }
}

/// Build a path
pub(crate) fn build_path(
    path: &mut Vec<PathOp>,
    face: &Face<'_>,
    glyph_x: f32,
    glyph_y: f32,
    glyph_id: u16,
    scale: f32,
) {
    let mut outliner = Outliner {
        path,
        scale,
        glyph_x,
        glyph_y,
    };
    face.outline_glyph(GlyphId(glyph_id), &mut outliner);
}
