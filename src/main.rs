use std::fs::File;
use std::io::Write;

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
    // fn parse_tokens(tokens: Vec<&str>) -> Vec<Command> {
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
                
                        let start = i+3;
                
                        let end = tokens
                            .iter()
                            .rposition(|&w| w == "]")
                            .expect("No matching ']' found for '[' after 'repeat' command");


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
    path: Vec<String>, // Stores SVG path commands
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
                self.path.push(format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" />",
                    self.x, self.y, new_x, new_y
                ));
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

/// Save the turtle's path to an SVG file
fn save_to_file(path: &[String], file_path: &str) {
    let mut file = File::create(file_path).expect("Unable to create file");
    writeln!(
        file,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-100 -100 200 200\">"
    )
    .unwrap();
    for command in path {
        writeln!(file, "{}", command).unwrap();
    }
    writeln!(file, "</svg>").unwrap();
}

fn run(commands: impl Iterator<Item = Command>, image_path: &str) {
    let mut turtle = Turtle::new();

    for command in commands {
        turtle.execute(&command);
    }

    save_to_file(&turtle.path, image_path);
}

fn main() {
    let commands = Command::parse("repeat 2 [ forward 15 turn 90 ]");
    let image_path = "output.svg";

    run(commands.into_iter(), image_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_angle() {
        let commands = Command::parse("repeat 2 [ forward 15 turn 90 ]");
    let image_path = "output.svg";

    run(commands.into_iter(), image_path);
    }
}