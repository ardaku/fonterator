use footile::{FillRule, Plotter, Raster, Rgba8};

const FONT_SIZE: f32 = 200.0;

const STR: &'static str = "Hé\tllö,\nWørłd!\nW. And Me?\nHow go it‽\n||| 10 X.Y.Z.";

fn main() {
    let font = fonterator::monospace_font();

    // Init rendering
    let mut p = Plotter::new(2048, 2048);
    let mut r = Raster::new(p.width(), p.height());

    let path = font.render(STR, (0.0, 0.0, 2048.0, 2048.0), (FONT_SIZE, FONT_SIZE), fonterator::TextAlign::Left);
    let path: Vec<footile::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    let path = font
        .render(STR, (0.0, 1024.0, 2048.0, 1024.0), (FONT_SIZE, FONT_SIZE), fonterator::TextAlign::Right);
    let path: Vec<footile::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(0, 0, 0));
    r.write_png("raster.png").unwrap();
}
