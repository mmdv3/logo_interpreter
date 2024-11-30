use std::collections::HashMap;

use svg::node::element::Rectangle;
use svg::node::element::{Group, Line};
use svg::Document;


const param_prefix: &str = ":";

#[derive(Debug, Clone)]
enum Arg<T> {
    Val(T),
    Param(String),
}


#[derive(Debug, Clone)]
pub enum Command {
    Forward(Arg<f64>),
    Turn(Arg<f64>),
    Repeat(Arg<u32>, Vec<Command>),
    Fn_call(String, Vec<String>),
}

struct Fun {
    params: Vec<String>,
    body: Vec<Command>,
}

fn substitute(cmd: &Command, param_evaluator: &HashMap<String, String>) -> Command {
    match cmd {
        Command::Forward(Arg::Param(param)) => {
            let value = param_evaluator
                .get(param)
                .expect(&format!("Value for parameter '{}' not provided", param));
            Command::Forward(Arg::Val(value.parse::<f64>().expect("Invalid value for f64")))
        }

        Command::Turn(Arg::Param(param)) => {
            let value = param_evaluator
                .get(param)
                .expect(&format!("Value for parameter '{}' not provided", param));
            Command::Turn(Arg::Val(value.parse::<f64>().expect("Invalid value for f64")))
        }

        Command::Repeat(Arg::Param(param), body) => {
            let value = param_evaluator
                .get(param)
                .expect(&format!("Value for parameter '{}' not provided", param));
            let parsed_value = value.parse::<u32>().expect("Invalid value for u32");
            Command::Repeat(Arg::Val(parsed_value), body.iter().map(|c| substitute(c, param_evaluator)).collect())
        }
        Command::Repeat(Arg::Val(val), body) => {
            Command::Repeat(Arg::Val(*val), body.iter().map(|c| substitute(c, param_evaluator)).collect())
        }

        Command::Fn_call(name, args) => {
            let substituted_args = args
                .iter()
                .map(|arg| {
                    param_evaluator
                        .get(arg)
                        .unwrap_or(arg) // Leave the argument as is if no substitution is found.
                        .to_string()
                })
                .collect();
            Command::Fn_call(name.clone(), substituted_args)
        }

        _ => {cmd.clone()}
    }
}

fn eval_params(commands: &Vec<Command>, param_evaluator: HashMap<String, String>) -> Vec<Command> {
    commands.iter().map(|cmd| substitute(cmd, &param_evaluator)).collect()
}

impl Fun{
    fn new(commands: Vec<Command>, params: Vec<String>) -> Fun {
        Fun {params, body: commands}
    }
    fn fun_body(&self, param_vals: &Vec<String>) -> Vec<Command> {
        let mut param_evaluator: HashMap<String, String> = HashMap::new();
        for (param_name, param_val) in self.params.iter().zip(param_vals.iter()) {
            param_evaluator.insert(param_name.clone(), param_val.clone());
        }

        eval_params(&self.body, param_evaluator)
    }
}

pub struct Env {
    functions: HashMap<String, Fun>,
}

impl Env {
    fn new() -> Env {
        Env {
            functions: HashMap::new(),
        }
    }
    fn get(&self, label: &String, param_vals: &Vec<String>) -> Vec<Command> {
        self.functions.get(label).unwrap().fun_body(param_vals)
    }

    fn push(&mut self, (label, commands, params): (String, Vec<Command>, Vec<String>)) {
        self.functions.insert(label, Fun::new(commands, params));
    }
}

pub fn parse(input: &str) -> (Vec<Command>, Env) {
    let mut commands = vec![];
    let mut cmd_env = Env::new();
    let mut labels: Vec<String> = vec![];
    let mut label_arity: HashMap<String, usize> = HashMap::new(); // function arity?

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
            cmd_env.push(parse_fn(&tokens[..], &mut labels, &mut label_arity));
        } else {
            commands.append(&mut parse_tokens(&tokens[..], &mut labels, &mut label_arity));
        }
    }

    (commands, cmd_env)
}

fn parse_fn(tokens: &[&str], labels: &mut Vec<String>, label_arity: &mut HashMap<String, usize>) -> (String, Vec<Command>, Vec<String>) {
    println!("Parsing function body {:?}", tokens);
    if tokens[0] != "to" {
        panic!();
    }

    let mut params: Vec<String>;
    let mut fn_body_start = 2;
    if tokens[2].starts_with(param_prefix) {
        print!("Parsing function, found parameters");
        let param_start: usize = 2;
        fn_body_start = tokens[param_start..].iter().position(|token| !token.starts_with(param_prefix)).unwrap() + param_start;
       
        params = tokens[param_start..fn_body_start].iter().map(|&token| String::from(token)).collect();
    }
    else {
        println!("Parsing function, no parameters found");
        params = vec![];
    }

    let label = tokens[1];
    labels.push(String::from(label));
    label_arity.insert(String::from(label), params.len());

    println!("Parsing function body from token {}, found {} params", fn_body_start, params.len());
    (String::from(label), parse_tokens(&tokens[fn_body_start..], &mut labels.clone(), label_arity), params)
}

fn parse_tokens(tokens: &[&str], labels: &mut Vec<String>, label_arity: &HashMap<String, usize>) -> Vec<Command> { //label zupełnie nie nie mówi
    let mut commands = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i] {
            "forward" => {
                if tokens[i+1].starts_with(param_prefix) {
                    commands.push(Command::Forward(Arg::Param(String::from(tokens[i+1]))));
                    i+=2;
                }
                else if let Ok(value) = tokens[i + 1].parse::<f64>() {
                    commands.push(Command::Forward(Arg::Val(value)));
                    i += 2;
                } else {
                    panic!("Invalid forward command: expected a number or parameter");
                }
            }
            "turn" => {
                if let Ok(value) = tokens[i + 1].parse::<f64>() {
                    commands.push(Command::Turn(Arg::Val(value)));
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
                        Arg::Val(repeats),
                        parse_tokens(nested_commands, &mut labels.clone(), label_arity),
                    ));

                    i = end + 1;
                } else {
                    panic!("Invalid repeat command");
                }
            }
            label if labels.contains(&String::from(label)) => {
                let params_num = *label_arity.get(label).unwrap();
                let param_vals = tokens[i+1..i+params_num+1].iter()
                .map(|&param| String::from(param)).collect();
                commands.push(Command::Fn_call(String::from(label), param_vals));
                i += params_num+1;
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
            Command::Forward(Arg::Val(distance)) => {
                let radians = self.angle.to_radians();
                let new_x = self.x + distance * radians.cos();
                let new_y = self.y + distance * radians.sin();

                image.add_line(self.x, self.y, new_x, new_y);

                self.x = new_x;
                self.y = new_y;
            }
            Command::Turn(Arg::Val(angle)) => {
                self.angle = (self.angle + angle) % 360.0;
            }
            Command::Repeat(Arg::Val(times), commands) => {
                for _ in 0..*times {
                    for command in commands.iter() {
                        self.execute(command, image, cmd_env);
                    }
                }
            }
            Command::Fn_call(label, param_vals) => {
                let commands = cmd_env.get(label, param_vals);

                for command in commands {
                    self.execute(&command, image, cmd_env);
                }
            }
            _ => {
                panic!("Tried to call {:?} command possibly with unspecified parameter value", command);
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

pub fn run(commands: impl Iterator<Item = Command>, cmd_env: Env, image_path: &str) {
    let mut turtle = Turtle::new();
    let mut image = Image::new();

    for command in commands {
        turtle.execute(&command, &mut image, &cmd_env);
    }

    image.save(image_path);
}