use ttf_parser as ttf;

pub use footile;
pub use footile::PathOp;

/// A collection of TTF/OTF fonts used as a single font.
#[derive(Default)]
pub struct Font<'a> {
    fonts: Vec<(ttf::Font<'a>, f32)>,
}

impl<'a> Font<'a> {
    /// Create a new `Font`.  Add glyphs with `add()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a TTF or OTF font's glyphs to this `Font`.
    pub fn add<B: Into<&'a [u8]>>(mut self, bytes: B)
        -> Result<Self, Box<std::error::Error>>
    {
        let font = ttf::Font::from_data(bytes.into(), 0)?;
        let ems_per_unit = (font.units_per_em().ok_or("em")? as f32).recip();
        self.fonts.push((font, ems_per_unit));
        Ok(self)
    }

    /// Render a string.
    pub fn render(&'a self, text: &'a str, xy: (f32, f32), wh: (f32, f32))
        -> TextPathIterator<'a>
    {
        TextPathIterator {
            text: text.chars(),
            path: CharPathIterator::new(self, xy, wh),
            op: PathOp::Close(),
        }
    }
}

/*pub fn process() {
    let units_per_em = font.units_per_em().ok_or("invalid units per em")?;
    let scale = font_size / units_per_em;

    let cell_size = font.height() as f64 * FONT_SIZE / units_per_em as f64;
    let rows = (font.number_of_glyphs() as f64 / COLUMNS as f64).ceil() as u32;

    let mut path_buf = Vec::with_capacity(64);
    let mut row = 0;
    let mut column = 0;
    for id in 0..font.number_of_glyphs() {
        glyph_to_path(
            column as f64 * cell_size,
            row as f64 * cell_size,
            &font,
            ttf::GlyphId(id),
            cell_size,
            scale,
            &mut svg,
            &mut path_buf,
        );

        column += 1;
        if column == COLUMNS {
            column = 0;
            row += 1;
        }
    }

    Ok(())
}*/

/*fn glyph_to_path(
    x: f64,
    y: f64,
    font: &ttf::Font,
    glyph_id: ttf::GlyphId,
    cell_size: f64,
    scale: f64,
    svg: &mut xmlwriter::XmlWriter,
    path_buf: &mut svgtypes::Path,
) {
    svg.start_element("text");
    svg.write_attribute("x", &(x + 2.0));
    svg.write_attribute("y", &(y + cell_size - 4.0));
    svg.write_attribute("font-size", "36");
    svg.write_attribute("fill", "gray");
    svg.write_text_fmt(format_args!("{}", glyph_id.0));
    svg.end_element();

    path_buf.clear();
    let mut builder = Builder(path_buf);
    let bbox = match font.outline_glyph(glyph_id, &mut builder) {
        Ok(v) => v,
        Err(ttf::Error::NoOutline) => return,
        Err(ttf::Error::NoGlyph) => return,
        Err(e) => {
            eprintln!("Warning (glyph {}): {}.", glyph_id.0, e);
            return;
        }
    };

    let metrics = match font.glyph_hor_metrics(glyph_id) {
        Ok(v) => v,
        Err(_) => return,
    };

    let dx = (cell_size - metrics.advance as f64 * scale) / 2.0;
    let y = y + cell_size + font.descender() as f64 * scale;

    let mut ts = svgtypes::Transform::default();
    ts.translate(x + dx, y);
    ts.scale(1.0, -1.0);
    ts.scale(scale, scale);

    svg.start_element("path");
    svg.write_attribute_raw("d", |buf| path_buf.write_buf(buf));
    svg.write_attribute_raw("transform", |buf| ts.write_buf(buf));
    svg.end_element();

    {
        let bbox_w = (bbox.x_max as f64 - bbox.x_min as f64) * scale;
        let bbox_h = (bbox.y_max as f64 - bbox.y_min as f64) * scale;
        let bbox_x = x + dx + bbox.x_min as f64 * scale;
        let bbox_y = y - bbox.y_min as f64 * scale - bbox_h;

        svg.start_element("rect");
        svg.write_attribute("x", &bbox_x);
        svg.write_attribute("y", &bbox_y);
        svg.write_attribute("width", &bbox_w);
        svg.write_attribute("height", &bbox_h);
        svg.write_attribute("fill", "none");
        svg.write_attribute("stroke", "green");
        svg.end_element();
    }
}*/

struct CharPathIterator<'a> {
    // The font to use.
    font: &'a Font<'a>,
    // Path of the current character.
    path: Vec<PathOp>,
    // W & H
    size: (f32, f32),
    // X & Y
    xy: (f32, f32),
    // Multiplied wh.
    wh: (f32, f32),
}

impl<'a> CharPathIterator<'a> {
    fn new(font: &'a Font<'a>, xy: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            font,
            path: vec![],
            xy,
            size,
            wh: (0.0, 0.0),
        }
    }

    fn set(&mut self, c: char) -> Result<(), ttf::Error> {
        let mut index = 0;
        let glyph_id = loop {
            match self.font.fonts[index].0.glyph_index(c) {
                Ok(v) => break v,
                Err(e) => {
                    index += 1;
                    if index == self.font.fonts.len() {
                        eprintln!("No Glyph for \"{}\" ({})", c, c as u32);
                        return Err(e);
                    }
                }
            }
        };

        self.path.clear();
        self.wh = (self.size.0 * self.font.fonts[index].1, self.size.1 * self.font.fonts[index].1);
        let bbox = match self.font.fonts[index].0.outline_glyph(glyph_id, self) {
            Ok(v) => v,
            Err(ttf::Error::NoOutline) => return Err(ttf::Error::NoOutline),
            Err(ttf::Error::NoGlyph) => return Err(ttf::Error::NoGlyph),
            Err(e) => {
                eprintln!("Warning (glyph {}): {}.", glyph_id.0, e);
                return Err(e);
            }
        };

        self.path.reverse();

        Ok(())
    }
}

impl ttf::OutlineBuilder for CharPathIterator<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.push(PathOp::Move(x * self.wh.0 + self.xy.0, y * self.wh.1 + self.xy.1));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.push(PathOp::Line(x * self.wh.0 + self.xy.0, y * self.wh.1 + self.xy.1));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path.push(PathOp::Quad(x1 * self.wh.0 + self.xy.0, y1 * self.wh.1 + self.xy.1, x * self.wh.0 + self.xy.0, y * self.wh.1 + self.xy.1));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path.push(PathOp::Cubic(x1 * self.wh.0 + self.xy.0, y1 * self.wh.1 + self.xy.1, x2 * self.wh.0 + self.xy.0, y2 * self.wh.1 + self.xy.1, x * self.wh.0 + self.xy.0, y * self.wh.1 + self.xy.1));
    }

    fn close(&mut self) {
        self.path.push(PathOp::Close());
    }
}

impl<'a> Iterator for CharPathIterator<'a> {
    type Item = PathOp;

    fn next(&mut self) -> Option<Self::Item> {
        self.path.pop()
    }
}

pub struct TextPathIterator<'a> {
    // The text that we're parsing.
    text: std::str::Chars<'a>,
    // Path for the current character.
    path: CharPathIterator<'a>,
    //
    op: PathOp,
}

impl<'a> TextPathIterator<'a> {
    fn test(&'a self) -> &'a PathOp {
        &self.op
    }
}

impl<'a> Iterator for &'a mut TextPathIterator<'a> {
    type Item = PathOp;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(op) = self.path.next() {
            Some(op)
        } else {
            if let Some(c) = self.text.next() {
                self.path.set(c);
                self.next()
            } else {
                None
            }
        }
    }
}

/// Get a monospace font.  Requires feature = "builtin-font".
#[cfg(feature = "builtin-font")]
pub fn monospace_font() -> Font<'static> {
    const FONTA: &[u8] = include_bytes!("font/dejavu/DejaVuSansMono.ttf");
    const FONTB: &[u8] = include_bytes!("font/wqy-microhei/WenQuanYiMicroHeiMono.ttf");

    Font::new()
        .add(FONTA)
        .unwrap()
        .add(FONTB)
        .unwrap()
}
