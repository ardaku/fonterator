use fonterator::FontGroup;
use footile::{FillRule, Plotter, Raster, Rgba8};

const FONT_SIZE: f32 = 200.0;

const STR: &'static str = "Hé\tllö,\nWørłd!\nW. And Me?\nHow go it‽\n||| 10 X.Y.Z.";

fn main() {
    let font = FontGroup::default();

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::new(p.width(), p.height());

    //    let path = font.render(STR, (1024.0, 0.0), (FONT_SIZE, FONT_SIZE)).vertical().right_to_left();
    //    r.over(p.fill(path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    //    let path = font.render(STR, (1024.0, 0.0), (FONT_SIZE, FONT_SIZE)).vertical();
    //    r.over(p.fill(path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    let mut path = font.render(STR, (0.0, 0.0), (FONT_SIZE, FONT_SIZE));
    r.over(p.fill(&mut path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    let mut path = font
        .render(STR, (2048.0, 1024.0), (FONT_SIZE, FONT_SIZE))
        .right_to_left();
    r.over(p.fill(&mut path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    r.write_png("raster.png").unwrap();
}
