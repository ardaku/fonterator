use footile::{PathOp, Pt};
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
    let mut path = font
        .render(
            "…hello‽É¿?üæ 2⸘", /*text*/
            2048.0,                      /*width*/
            fonterator::TextAlign::Left,
        )
        .0;
    data = Data::new();

    for i in &mut path {
        match i {
            PathOp::Move(Pt(x, y)) => {
                data = data.move_to((x * FONT_SIZE, y * FONT_SIZE));
            }
            PathOp::Line(Pt(x, y)) => {
                data = data.line_to((x * FONT_SIZE, y * FONT_SIZE));
            }
            PathOp::Quad(Pt(cx, cy), Pt(x, y)) => {
                data = data.quadratic_curve_to((
                    cx * FONT_SIZE,
                    cy * FONT_SIZE,
                    x * FONT_SIZE,
                    y * FONT_SIZE,
                ));
            }
            PathOp::Cubic(Pt(ax, ay), Pt(bx, by), Pt(x, y)) => {
                data = data.cubic_curve_to((
                    ax * FONT_SIZE,
                    ay * FONT_SIZE,
                    bx * FONT_SIZE,
                    by * FONT_SIZE,
                    x * FONT_SIZE,
                    y * FONT_SIZE,
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
