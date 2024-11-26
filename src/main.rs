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
                        if tokens[i + 2] != "[" {
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
                            panic!("repeat: found unclosed '['");
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
    path: Vec<Line>, // Stores SVG lines
}

impl Turtle {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            path: vec![],
        }
    }

    fn execute(&mut self, command: &Command) {
        match command {
            Command::Forward(distance) => {
                let radians = self.angle.to_radians();
                let new_x = self.x + distance * radians.cos();
                let new_y = self.y + distance * radians.sin();
                self.path.push(
                    Line::new()
                        .set("x1", self.x)
                        .set("y1", self.y)
                        .set("x2", new_x)
                        .set("y2", new_y)
                        .set("stroke", "black"),
                );
                self.x = new_x;
                self.y = new_y;
            }
            Command::Turn(angle) => {
                self.angle = (self.angle + angle) % 360.0;
            }
            Command::Repeat(times, commands) => {
                for _ in 0..*times {
                    for command in commands.iter() {
                        self.execute(command);
                    }
                }
            }
        }
    }
}


fn save_to_file(path: &[Line], file_path: &str) {
    let mut group = Group::new();
    for line in path {
        group = group.add(line.clone());
    }

    let document = Document::new()
        .set("viewBox", (-100, -100, 200, 200))
        .add(group);

    svg::save(file_path, &document).expect("Unable to save SVG file");
}

fn run(commands: impl Iterator<Item = Command>, image_path: &str) {
    let mut turtle = Turtle::new();

    for command in commands {
        turtle.execute(&command);
    }

    save_to_file(&turtle.path, image_path);
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
    fn right_angle() {
        let input = "repeat 2 [ forward 50 turn 90 ] forward 30";
    let commands = Command::parse(input);
    let image_path = "output.svg";

    run(commands.into_iter(), image_path);
    }
}