use svg::node::element::Rectangle;
use svg::node::element::{Group, Line};
use svg::Document;

pub struct Image {
    lines: Vec<Line>,
}

impl Image {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }

    pub fn add_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.lines.push(
            Line::new()
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2)
                .set("stroke", "black"),
        );
    }

    pub fn save(&self, file_path: &str) {
        let mut group = Group::new();
        for line in &self.lines {
            group = group.add(line.clone());
        }

        let square = Rectangle::new()
            .set("x", -400)
            .set("y", -400)
            .set("width", 1200)
            .set("height", 1200)
            .set("fill", "white");

        let document = Document::new()
            .set("viewBox", (-400, -400, 800, 800))
            .add(square)
            .add(group);

        svg::save(file_path, &document).expect("Unable to save SVG file");
    }
}