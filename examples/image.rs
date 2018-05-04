// image.rs -- Fonterator
// Copyright (c) 2018  Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

// extern crate svg;
extern crate fonterator;

use fonterator::{point, Font, Scale, PathOp};

fn main() {
	// Load the font
	let font_data = include_bytes!("../font/DejaVuSansMono.ttf");
	// This only succeeds if collection consists of one font
	let font =
		Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

	// The font size to use
	let size = (1.0f32).ceil();
	let scale = Scale { x: size, y: size };
	let v_metrics = font.v_metrics(scale);
	let offset = point(0.0, v_metrics.ascent);

	// The text to render
	let text = "S";

	// Use a dark red colour
	let colour = (150, 0, 0);

	// Loop through the glyphs in the text, positing each one on a line
	for glyph in font.layout(text, scale, offset) {
		if let Some(bounding_box) = glyph.pixel_bounding_box() {
			// Draw the glyph into the image per-pixel by using the draw closure
			for i in glyph.draw() {
				match i {
					PathOp::MoveTo(x, y) => println!("Move({}, {})", x, y),
					PathOp::LineTo(x, y) => println!("Line({}, {})", x, y),
					PathOp::QuadTo(x, y, cx, cy) => println!("Quad({}, {}, {}, {})", x, y, cx, cy),
					PathOp::LineClose => println!("Line Close"),
					PathOp::QuadClose(cx, cy) => println!("Quad Close({} {})", cx, cy),
				}
			}
		}
	}

	// Save the image to a png file
	// image.save("image_example.png").unwrap();
}
