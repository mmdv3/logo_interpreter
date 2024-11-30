use std::collections::HashMap;

use svg::node::element::Rectangle;
use svg::node::element::{Group, Line};
use svg::Document;

#[derive(Debug)]
enum Command {
    Forward(f64),
    Turn(f64),
    Repeat(u32, Vec<Command>),
    Fn_call(String),
}

struct Env {
    functions: HashMap<String, Vec<Command>>,
}

impl Env {
    fn new() -> Env {
        Env {
            functions: HashMap::new(),
        }
    }

    fn get(&self, label: &String) -> &Vec<Command> {
        self.functions.get(label).unwrap()
    }

    fn push(&mut self, (label, commands): (String, Vec<Command>)) {
        self.functions.insert(label, commands);
    }
}

fn parse(input: &str) -> (Vec<Command>, Env) {
    let mut commands = vec![];
    let mut cmd_env = Env::new();
    let mut labels: Vec<String> = vec![];

    let blocks: Vec<(&str, &str)> = input
        .split("end")
        .into_iter()
        .map(|block| {
            if block.starts_with("to") {
                ("fn", block)
            } else {
                ("exec", block)
            }
        })
        .collect();

    for (bl_type, block) in blocks {
        let tokens: Vec<&str> = block.split_whitespace().collect();

        if bl_type == "fn" {
            cmd_env.push(parse_fn(&tokens[..], &mut labels));
        } else {
            commands.append(&mut parse_tokens(&tokens[..], &mut labels));
        }
    }

    (commands, cmd_env)
}

fn parse_fn(tokens: &[&str], labels: &mut Vec<String>) -> (String, Vec<Command>) {
    if tokens[0] != "to" {
        panic!();
    }

    let label = tokens[1];
    labels.push(String::from(label));

    (String::from(label), parse_tokens(&tokens[2..], &mut labels.clone()))
}

fn parse_tokens(tokens: &[&str], labels: &mut Vec<String>) -> Vec<Command> {
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
                        panic!("Malformed repeat block: '[' found but no commands before ']'");
                    }

                    let nested_commands = &tokens[start..end];
                    commands.push(Command::Repeat(
                        repeats,
                        parse_tokens(nested_commands, &mut labels.clone()),
                    ));

                    i = end + 1;
                } else {
                    panic!("Invalid repeat command");
                }
            }
            label if labels.contains(&String::from(label)) => {
                commands.push(Command::Fn_call(String::from(label)));
                i += 1;
            }
            _ => panic!("Unknown command {}", tokens[i]),
        }
    }

    commands
}

#[derive(Debug)]
struct Turtle {
    x: f64,
    y: f64,
    angle: f64,
}

impl Turtle {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 270.0,
        }
    }

    fn execute(&mut self, command: &Command, image: &mut Image, cmd_env: &Env) {
        match command {
            Command::Forward(distance) => {
                let radians = self.angle.to_radians();
                let new_x = self.x + distance * radians.cos();
                let new_y = self.y + distance * radians.sin();

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
                        self.execute(command, image, cmd_env);
                    }
                }
            }
            Command::Fn_call(label) => {
                let commands = cmd_env.get(label);

                for command in commands {
                    self.execute(command, image, cmd_env);
                }
            }
        }
    }
}

struct Image {
    lines: Vec<Line>,
}

impl Image {
    fn new() -> Self {
        Self { lines: vec![] }
    }

    fn add_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.lines.push(
            Line::new()
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2)
                .set("stroke", "black"),
        );
    }

    fn save(&self, file_path: &str) {
        let mut group = Group::new();
        for line in &self.lines {
            group = group.add(line.clone());
        }

        let square = Rectangle::new()
            .set("x", -100)
            .set("y", -100)
            .set("width", 300)
            .set("height", 300)
            .set("fill", "white");

        let document = Document::new()
            .set("viewBox", (-100, -100, 200, 200))
            .add(square)
            .add(group);

        svg::save(file_path, &document).expect("Unable to save SVG file");
    }
}

fn run(commands: impl Iterator<Item = Command>, cmd_env: Env, image_path: &str) {
    let mut turtle = Turtle::new();
    let mut image = Image::new();

    for command in commands {
        turtle.execute(&command, &mut image, &cmd_env);
    }

    image.save(image_path);
}

fn main() {
    let input = "repeat 2 [ forward 50 turn 90 ] forward 30";
    let (commands, cmd_env) = parse(input);
    let image_path = "img/output.svg";

    run(commands.into_iter(), cmd_env, image_path);

    println!("SVG file saved to {}", image_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star() {
        let input = "to star repeat 5 [ forward 100 turn 144 ] end star";

        let (commands, cmd_env) = parse(input);
        let image_path = "img/star.svg";

        run(commands.into_iter(), cmd_env, image_path);
    }
}
