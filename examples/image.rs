use fonterator::{FontGroup, PathOp};
use svg::{
    node::element::{path::Data, Group, Path, Style},
    Document, Node,
};

const FONT_SIZE: f32 = 256.0;

fn main() {
    let font = FontGroup::default();

    // Initialize variables need to write to SVG
    let mut group = Group::new();
    let mut data; //= Data::new().move_to(vec![0.0, 0.0]);

    // Loop through the glyphs in the text, adding to the SVG.
    let mut path = font.render(
        "DIVE and…    ‽é¿?üæ", /*text*/
        (0.0, 0.0),                                                     /*position*/
        (FONT_SIZE, FONT_SIZE),                                                 /*size*/
    );
    data = Data::new();

    for i in &mut path {
            match i {
                PathOp::Move(x, y) => {
                    data = data.move_to((*x, *y));
                }
                PathOp::Line(x, y) => {
                    data = data.line_to((*x, *y));
                }
                PathOp::Quad(cx, cy, x, y) => {
                    data = data.quadratic_curve_to((*cx, *cy, *x, *y));
                }
                PathOp::Cubic(ax, ay, bx, by, x, y) => {
                    data = data.cubic_curve_to((*ax, *ay, *bx, *by, *x, *y));
                }
                PathOp::Close() => {
                    data = data.close();
                }
                PathOp::PenWidth(_) => {}
            }
    }

    group.append(Path::new().set("d", data.clone()));
    let (x, _y) = path.xy();

    // Save the image to an SVG file
    let style = Style::new("path { fill: 0x000000; stroke: black; stroke-width: 3; }");
    let document = Document::new()
        .set("width", x)
        .set("height", 256.0)
        .add(style)
        .add(group);
    svg::save("image_example.svg", &document).unwrap();
}
