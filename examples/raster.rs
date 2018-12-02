// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

extern crate fonterator;
extern crate footile;

use fonterator::Font;
use footile::{FillRule, Plotter, Raster, Rgba8};

const FONT: &[u8] = include_bytes!("../font/LiberationSans-Regular.ttf");
const FONT_SIZE: f32 = 200.0;

const STR: &'static str = "Hé\tllö,\nWørłd!\nW. And Me?\nHow go it‽\n||| 10 A.D.I.";

fn main() {
    // This only succeeds if collection consists of one font
    let font = Font::new(FONT).expect("Failed to load font!");

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::new(p.width(), p.height());

    //    let path = font.render(STR, (1024.0, 0.0), (FONT_SIZE, FONT_SIZE)).vertical().right_to_left();
    //    r.over(p.fill(path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    //    let path = font.render(STR, (1024.0, 0.0), (FONT_SIZE, FONT_SIZE)).vertical();
    //    r.over(p.fill(path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    let path = font.render(STR, (0.0, 0.0), (FONT_SIZE, FONT_SIZE));
    r.over(p.fill(path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    let path = font
        .render(STR, (2048.0, 1024.0), (FONT_SIZE, FONT_SIZE))
        .right_to_left();
    r.over(p.fill(path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    r.write_png("raster.png").unwrap();
}
