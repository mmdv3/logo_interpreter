use svg::node::element::Rectangle;
use svg::node::element::{Group, Line};
use svg::Document;

#[derive(Debug)]
enum Command {
    Forward(f64),
    Turn(f64),
    Repeat(u32, Vec<Command>),
}

impl Command {
    fn parse(input: &str) -> Vec<Command> {
        let tokens: Vec<&str> = input.split_whitespace().collect();
        Self::parse_tokens(&tokens[..])
    }

    fn parse_tokens(tokens: &[&str]) -> Vec<Command> {
        let mut commands = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            match tokens[i] {
                "forward" => {
                    if let Ok(value) = tokens[i + 1].parse::<f64>() {
                        commands.push(Command::Forward(value));
                        i += 2;
                    } else {
                        panic!("Invalid forward command: expected a number");
                    }
                }
                "turn" => {
                    if let Ok(value) = tokens[i + 1].parse::<f64>() {
                        commands.push(Command::Turn(value));
                        i += 2;
                    } else {
                        panic!("Invalid turn command: expected a number");
                    }
                }
                "repeat" => {
                    if let Ok(repeats) = tokens[i + 1].parse::<u32>() {
                        if tokens.get(i + 2) != Some(&"[") {
                            panic!(
                                "Expected '[' after 'repeat {}', but found {:?} or nothing.",
                                repeats,
                                tokens.get(i + 2)
                            );
                        }

                        let start = i + 3;
                        let end = tokens
                            .iter()
                            .rposition(|&w| w == "]")
                            .expect("No matching ']' found for '[' after 'repeat' command");

                        if start >= end {
                            panic!("Malformed repeat block: '[' found but no commands before ']'");
                        }

                        let nested_commands = &tokens[start..end];
                        commands.push(Command::Repeat(
                            repeats,
                            Command::parse_tokens(nested_commands),
                        ));
                        i = end + 1;
                    } else {
                        panic!("Invalid repeat command");
                    }
                }
                _ => panic!("Unknown command {}", tokens[i]),
            }
        }

        commands
    }
}

#[derive(Debug)]
struct Turtle {
    x: f64,
    y: f64,
    angle: f64, // In degrees
}

impl Turtle {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 270.0,
        }
    }

    fn execute(&mut self, command: &Command, image: &mut Image) {
        match command {
            Command::Forward(distance) => {
                let radians = self.angle.to_radians();
                let new_x = self.x + distance * radians.cos();
                let new_y = self.y + distance * radians.sin();

                // Add a line directly to the image
                image.add_line(self.x, self.y, new_x, new_y);

                self.x = new_x;
                self.y = new_y;
            }
            Command::Turn(angle) => {
                self.angle = (self.angle + angle) % 360.0;
            }
            Command::Repeat(times, commands) => {
                for _ in 0..*times {
                    for command in commands.iter() {
                        self.execute(command, image);
                    }
                }
            }
        }
    }
}

struct Image {
    // lines: Group, // Stores lines for the SVG document
    lines: Vec<Line>, // Stores SVG lines
}

impl Image {
    fn new() -> Self {
        Self {
            // lines: Group::new(),
            lines: vec![],
        }
    }

    /// Adds a line to the image
    fn add_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        // let line = Line::new()
        //     .set("x1", x1)
        //     .set("y1", y1)
        //     .set("x2", x2)
        //     .set("y2", y2)
        //     .set("stroke", "black");
        // self.lines = self.lines.add(line);

        self.lines.push(
            Line::new()
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2)
                .set("stroke", "black"),
        );
    }

    /// Saves the image to a file
    fn save(&self, file_path: &str) {
        // lepiej dodawać pojedynczo, bez groupy
        let mut group = Group::new();
        for line in &self.lines {
            group = group.add(line.clone()); // !!!
        }

        let square = Rectangle::new()
            .set("x", -100)
            .set("y", -100)
            .set("width", 300)
            .set("height", 300)
            .set("fill", "white");

        // Wywal klona!!!!
        let document = Document::new()
            .set("viewBox", (-100, -100, 200, 200))
            // .add(self.lines.clone());
            .add(square)
            .add(group);

        svg::save(file_path, &document).expect("Unable to save SVG file");
    }
}

fn run(commands: impl Iterator<Item = Command>, image_path: &str) {
    let mut turtle = Turtle::new();
    let mut image = Image::new();

    for command in commands {
        turtle.execute(&command, &mut image);
    }

    image.save(image_path);
}

fn main() {
    let input = "repeat 2 [ forward 50 turn 90 ] forward 30";
    let commands = Command::parse(input);
    let image_path = "output.svg";

    run(commands.into_iter(), image_path);

    println!("SVG file saved to {}", image_path);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star() {
        let input = "repeat 5 [ forward 100 turn 144 ]";
        let commands = Command::parse(input);
        let image_path = "img/star.svg";

        run(commands.into_iter(), image_path);
    }
}
