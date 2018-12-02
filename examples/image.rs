// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

extern crate fonterator;
extern crate svg;

use fonterator::{Font, PathOp};
use svg::{
    node::element::{path::Data, Group, Path, Style},
    Document, Node,
};

const FONT: &[u8] = include_bytes!("../font/LiberationSans-Regular.ttf");
const FONT_SIZE: f32 = 256.0;

fn main() {
    // This only succeeds if collection consists of one font
    let font = Font::new(FONT).expect("Failed to load font!");

    // Initialize variables need to write to SVG
    let mut group = Group::new();
    let mut data; //= Data::new().move_to(vec![0.0, 0.0]);
    let mut x = 0.0;

    // Loop through the glyphs in the text, adding to the SVG.
    for g in font.glyphs("Splat And…    ‽é¿?üæ", (FONT_SIZE, FONT_SIZE)) {
        data = Data::new();

        let mut first = true;

        // Draw the glyph
        for i in g.0.draw(x, 0.0) {
            match i {
                PathOp::Move(x, y) => {
                    if first {
                        first = false;
                    } else {
                        data = data.close();
                    }
                    data = data.move_to(vec![x, y]);
                }
                PathOp::Line(x, y) => {
                    data = data.line_to(vec![x, y]);
                }
                PathOp::Quad(cx, cy, x, y) => {
                    data = data.quadratic_curve_to(vec![cx, cy, x, y]);
                }
                _ => unimplemented!(),
            }
        }

        data = data.close();

        group.append(Path::new().set("d", data.clone()));

        // Position next glyph
        x += g.1;
    }

    // Save the image to an SVG file
    let style = Style::new("path { fill: 0x000000; stroke: black; stroke-width: 3; }");
    let document = Document::new()
        .set("width", x)
        .set("height", 256.0)
        .add(style)
        .add(group);
    svg::save("image_example.svg", &document).unwrap();
}
