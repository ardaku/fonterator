use footile::PathOp;
use svg::{
    node::element::{path::Data, Group, Path, Style},
    Document, Node,
};

const FONT_SIZE: f32 = 256.0;

fn main() {
    let mut font = fonterator::monospace_font();

    // Initialize variables need to write to SVG
    let mut group = Group::new();
    let mut data; //= Data::new().move_to(vec![0.0, 0.0]);

    // Loop through the glyphs in the text, adding to the SVG.
    let mut path = font.render("…hello‽É¿?üæ 2⸘" /*text*/, 2048.0 /*width*/).0;
    data = Data::new();

    for i in &mut path {
        match i {
            PathOp::Move(pt) => {
                data = data.move_to((pt.x() * FONT_SIZE, pt.y() * FONT_SIZE));
            }
            PathOp::Line(pt) => {
                data = data.line_to((pt.x() * FONT_SIZE, pt.y() * FONT_SIZE));
            }
            PathOp::Quad(cpt, pt) => {
                data = data.quadratic_curve_to((
                    cpt.x() * FONT_SIZE,
                    cpt.y() * FONT_SIZE,
                    pt.x() * FONT_SIZE,
                    pt.y() * FONT_SIZE,
                ));
            }
            PathOp::Cubic(apt, bpt, pt) => {
                data = data.cubic_curve_to((
                    apt.x() * FONT_SIZE,
                    apt.y() * FONT_SIZE,
                    bpt.x() * FONT_SIZE,
                    bpt.y() * FONT_SIZE,
                    pt.x() * FONT_SIZE,
                    pt.y() * FONT_SIZE,
                ));
            }
            PathOp::Close() => {
                data = data.close();
            }
            PathOp::PenWidth(_) => {}
        }
    }

    group.append(Path::new().set("d", data));

    // Save the image to an SVG file
    let style =
        Style::new("path { fill: 0x000000; stroke: black; stroke-width: 3; }");
    let document = Document::new()
        .set("width", 2048.0)
        .set("height", 256.0)
        .add(style)
        .add(group);
    svg::save("image_example.svg", &document).unwrap();
}
