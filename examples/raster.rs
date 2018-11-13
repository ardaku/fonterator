// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

extern crate footile;
extern crate fonterator;

use footile::{PathBuilder, Plotter, FillRule, Color};
use fonterator::{Font, PathOp};

const FONT: &[u8] = include_bytes!("../font/LiberationSans-Regular.ttf");
const FONT_SIZE: f32 = 256.0;

fn main() {
	// This only succeeds if collection consists of one font
	let font = Font::new(FONT).expect("Failed to load font!");

	// Initialize variables need to write to SVG
	let mut x = 0.0;

    let mut path = PathBuilder::new().pen_width(0f32).absolute();

	// Loop through the glyphs in the text, adding to the SVG.
	for g in font.glyphs("Hello, World! ¿é?"/*CWXY4\%æ*/, (FONT_SIZE, FONT_SIZE)) {
		// Draw the glyph
		for i in g.0.draw(x, 0.0) {
			match i {
				PathOp::Move(x, y) => {
                    path = path.move_to(x, y);
				}
				PathOp::Line(x, y) => {
                    path = path.line_to(x, y);
				}
				PathOp::Quad(cx, cy, x, y) => {
                    path = path.quad_to(cx, cy, x, y);
				}
                PathOp::Close() => {
                    path = path.close();
                }
				_ => { unimplemented!() }
			}
		}

		// Position next glyph
		x += g.1;
	}

    let path = path.build();
    let mut p = Plotter::new(x as u32, 256);
    p.fill(&path, FillRule::NonZero).color_over(Color::rgb(0, 0, 0));
//    p.stroke(&path).color_over(Color::rgb(0, 0, 0));
    p.write_png("raster.png").unwrap();

	// Save the image to an SVG file
//	let style = Style::new("path { fill: 0x000000; stroke: black; stroke-width: 3; }");
//	let document = Document::new().set("width", x).set("height", 256.0)
//		.add(style).add(group);
//	svg::save("image_example.svg", &document).unwrap();
}
