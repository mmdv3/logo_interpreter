use std::collections::HashMap;

use svg::node::element::Rectangle;
use svg::node::element::{Group, Line};
use svg::Document;


const param_prefix: &str = ":";

// #[derive(Debug, Clone)]
// enum Arg<T> {
//     Val(T),
//     Param(String),
// }

#[derive(Debug,Clone)]
pub enum Arg { // generic type?
    Val(f64),
    // Val_bool(bool), //temporary
    Param(String),
}

#[derive(Debug, Clone)]
pub enum LogExpr {
    Greater(Box<Expr>, Box<Expr>), // Represents expr > expr
    Less(Box<Expr>, Box<Expr>),    // Represents expr < expr
    Val(bool), //represents computed value
}


impl LogExpr {
    // pub fn evaluate(&self, param_evaluator: &HashMap<String, f64>) -> LogExpr {
    //     match self {
    //         LogExpr::Greater(lhs, rhs) => LogExpr::Val(substitute_expr(lhs, param_evaluator).evaluate() > substitute_expr(rhs, param_evaluator).evaluate()),
    //         LogExpr::Less(lhs, rhs) => LogExpr::Val(substitute_expr(lhs, param_evaluator).evaluate() < substitute_expr(rhs, param_evaluator).evaluate()),
    //     }
    // }
    pub fn evaluate(&self) -> LogExpr { // może zjadać self?
        match self {
            LogExpr::Greater(lhs, rhs) => LogExpr::Val(lhs.evaluate() > rhs.evaluate()),
            LogExpr::Less(lhs, rhs) => LogExpr::Val(lhs.evaluate() < rhs.evaluate()),
            LogExpr::Val(_) => {self.clone()},
        }
    }

    pub fn substitute(&self, param_evaluator: &HashMap<String, f64>) -> LogExpr { //może zjadać self?
        match self {
            LogExpr::Greater(lhs, rhs) => LogExpr::Greater(
                Box::new(substitute_expr(lhs, param_evaluator)) , 
                Box::new(substitute_expr(rhs, param_evaluator))),
            LogExpr::Less(lhs, rhs) => LogExpr::Less(
                Box::new(substitute_expr(lhs, param_evaluator)) , 
                Box::new(substitute_expr(rhs, param_evaluator))),
            LogExpr::Val(_) => {self.clone()},
        }
    }
}

#[derive(Debug,Clone)]
pub enum Token { //concrete type for arg, no boxes. Add bigger enum for generic Arg + Token. Rename arg
    Forward(Box<Expr>),
    Back(Box<Expr>), //temporary?
    Turn(Box<Expr>),
    Left(Box<Expr>), //temporary?
    Repeat(Box<Expr>, Box<Token>), // do poprawy bracket sam z siebie nie powinien istnieć
    FnCall(String), // FnCallPartial - interpreter should panic
    FnCallComplete(String, Vec<Expr>), // FnCall
    Bracket(Vec<Token>), // bracket jednak ma swoją rolę. Naprawdę nazywa się Scope
    If(LogExpr, Box<Token>), 
    Expression(Box<Expr>),
    Stop,
    // Mul(Box<Expr>, Box<Expr>),
    // Div(Box<Expr>, Box<Expr>),
    // Add(Box<Expr>, Box<Expr>),
    // Sub(Box<Expr>, Box<Expr>),
}

#[derive(Debug,Clone)]
pub enum Expr {
    Arg(Arg),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Evaluate the expression, substituting parameters if necessary.
    pub fn evaluate(&self) -> f64 {
        match self {
            Expr::Arg(Arg::Val(value)) => *value,
            Expr::Arg(Arg::Param(param)) => {
                panic!(
                    "Parameter '{}' substitution should have been handled before execution",
                    param
                );
            }
            Expr::Mul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            Expr::Div(lhs, rhs) => lhs.evaluate() / rhs.evaluate(),
            Expr::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expr::Sub(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
            // Expr::Arg(Arg::Val_bool(_)) => {panic!("Tried to evaluate boolean variable");},
        }
    }
}




// #[derive(Debug, Clone)]
// pub enum Command {
//     Forward(Arg<f64>),
//     Turn(Arg<f64>),
//     Repeat(Arg<u32>, Vec<Command>),
//     Fn_call(String, Vec<String>),
// }

struct Fun {
    params: Vec<String>,
    // body: Vec<Command>,
    body: Vec<Token>
}

// fn substitute(cmd: &Command, param_evaluator: &HashMap<String, String>) -> Command {
//     match cmd {
//         Command::Forward(Arg::Param(param)) => {
//             let value = param_evaluator
//                 .get(param)
//                 .expect(&format!("Value for parameter '{}' not provided", param));
//             Command::Forward(Arg::Val(value.parse::<f64>().expect("Invalid value for f64")))
//         }

//         Command::Turn(Arg::Param(param)) => {
//             let value = param_evaluator
//                 .get(param)
//                 .expect(&format!("Value for parameter '{}' not provided", param));
//             Command::Turn(Arg::Val(value.parse::<f64>().expect("Invalid value for f64")))
//         }

//         Command::Repeat(Arg::Param(param), body) => {
//             let value = param_evaluator
//                 .get(param)
//                 .expect(&format!("Value for parameter '{}' not provided", param));
//             let parsed_value = value.parse::<u32>().expect("Invalid value for u32");
//             Command::Repeat(Arg::Val(parsed_value), body.iter().map(|c| substitute(c, param_evaluator)).collect())
//         }
//         Command::Repeat(Arg::Val(val), body) => {
//             Command::Repeat(Arg::Val(*val), body.iter().map(|c| substitute(c, param_evaluator)).collect())
//         }

//         Command::Fn_call(name, args) => {
//             let substituted_args = args
//                 .iter()
//                 .map(|arg| {
//                     param_evaluator
//                         .get(arg)
//                         .unwrap_or(arg) // Leave the argument as is if no substitution is found.
//                         .to_string()
//                 })
//                 .collect();
//             Command::Fn_call(name.clone(), substituted_args)
//         }

//         _ => {cmd.clone()}
//     }
// }

// fn eval_params(commands: &Vec<Command>, param_evaluator: HashMap<String, String>) -> Vec<Command> {
//     commands.iter().map(|cmd| substitute(cmd, &param_evaluator)).collect()
// }

impl Fun{
    fn new(commands: Vec<Token>, params: Vec<String>) -> Fun {
        Fun {params, body: commands}
    }
    // fn fun_body(&self, param_vals: &Vec<String>) -> Vec<Token> {
    //     let mut param_evaluator: HashMap<String, String> = HashMap::new();
    //     for (param_name, param_val) in self.params.iter().zip(param_vals.iter()) {
    //         param_evaluator.insert(param_name.clone(), param_val.clone());
    //     }

    //     eval_params(&self.body, param_evaluator)
    // }
    fn fun_body(&self) -> &Vec<Token> { // wersja tymczasowa, aż naprawię exec
        // let mut param_evaluator: HashMap<String, String> = HashMap::new();
        // for (param_name, param_val) in self.params.iter().zip(param_vals.iter()) {
        //     param_evaluator.insert(param_name.clone(), param_val.clone());
        // }

        // eval_params(&self.body, param_evaluator)
        &self.body
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
    // fn get(&self, label: &String, param_vals: &Vec<String>) -> Vec<Command> {
    //     self.functions.get(label).unwrap().fun_body(param_vals)
    // }
    fn get(&self, label: &String) -> Vec<Token> { // wersja tymczasowa
        self.functions.get(label).unwrap().fun_body().clone() // paskudny klon
    }

    fn push(&mut self, (label, commands, params): (String, Vec<Token>, Vec<String>)) {
        // fn parse_fn(tokens: &[&str], labels: &mut Vec<String>, label_arity: &mut HashMap<String, usize>) -> (String, Vec<Token>, Vec<String>) {
        self.functions.insert(label, Fun::new(commands, params));
    }
}

pub fn wrap_fn_call(tokens: Vec<Token>, cmd_env: &Env) -> Vec<Token> {
    let mut wrapped_tokens = Vec::new();
    let mut iter = tokens.into_iter();

    while let Some(token) = iter.next() {
        println!("Wrap function call checking token {:?}", token);
        match token {
            Token::FnCall(label) => {
                println!("Wrapping function call to {}", label);
                if let Some(fun) = cmd_env.functions.get(&label) {
                    let arity = fun.params.len();
                    let mut args = Vec::new();

                    for _ in 0..arity {
                        if let Some(Token::Expression(expr)) = iter.next() {
                            args.push(*expr); // Unwrap the boxed expression and add it to args
                        } else {
                            panic!(
                                "Function '{}' expected {} arguments but fewer were provided",
                                label, arity
                            );
                        }
                    }

                    wrapped_tokens.push(Token::FnCallComplete(label, args));
                } else {
                    panic!("Function '{}' not found in the environment", label);
                }
            }
            Token::Repeat(iterations,block ) => {
                if let Token::Bracket(bracket) = *block {
                    let wrapped_block = wrap_fn_call(bracket, cmd_env);
                    wrapped_tokens.push(Token::Repeat(iterations, Box::new(Token::Bracket(wrapped_block)))); 
                }
                else {
                    panic!("Repeating token that is not a bracket");
                }
            }
            Token::If(log_expr, body) => { // modify - box is always a bracket
                // Recursively wrap function calls inside the conditional body
                let wrapped_body = wrap_fn_call(vec![*body], cmd_env);
                wrapped_tokens.push(Token::If(log_expr, Box::new(wrapped_body[0].clone())));
            }
            // Token::Bracket(inner_tokens) => {
            //     // Recursively wrap function calls inside brackets
            //     let wrapped_inner = wrap_fn_call(inner_tokens, cmd_env);
            //     wrapped_tokens.push(Token::Bracket(wrapped_inner));
            // }
            other => wrapped_tokens.push(other), // Add all other tokens unchanged
        }
    }

    wrapped_tokens
}

// pub fn parse(input: &str) -> (Vec<Command>, Env) {
    pub fn parse(input: &str) -> (Vec<
        Token>, Env) {
    let mut commands = vec![];
    let mut cmd_env = Env::new();
    let mut labels: Vec<String> = vec![];
    let mut label_arity: HashMap<String, usize> = HashMap::new(); // function arity?

    let input = input.replace("/", " / ")
    .replace("*", " * ")
    .replace("[", " [ ")
    .replace("]", " ] ")
    .replace("\n", " "); // simplify output, remove all fd, rt etc.
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
            //commands.append(&mut parse_tokens(&tokens[..], &mut labels, &mut label_arity)); // move function call logic to
            //interpreter

            let (mut tokens, _) = parse_tokens(&tokens[..], &labels);
            // commands.append(&mut parse_tokens(&tokens[..], &labels));
            commands.append(&mut tokens);
        }
    }

    let wrapped_commands = wrap_fn_call(commands,&cmd_env); //debug
    // (wrap_fn_call(commands,&cmd_env), cmd_env)
    println!("Finished wrapping commands. Result is {:?}", wrapped_commands);
    (wrapped_commands, cmd_env)
}


fn parse_fn(tokens: &[&str], labels: &mut Vec<String>, label_arity: &mut HashMap<String, usize>) -> (String, Vec<Token>, Vec<String>) {
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
    // (String::from(label), parse_tokens(&tokens[fn_body_start..], &mut labels.clone(), label_arity), params)

    let (fn_body, _) =  parse_tokens(&tokens[fn_body_start..], &labels);
    println!("Function body is {:#?}", fn_body);
    // (String::from(label), parse_tokens(&tokens[fn_body_start..], &labels), params)
    (String::from(label), fn_body, params)
}

// fn parse_tokens(tokens: &[&str], labels: &mut Vec<String>, label_arity: &HashMap<String, usize>) -> Vec<Command> { //label zupełnie nie nie mówi
//     let mut commands = Vec::new();
//     let mut i = 0;

//     while i < tokens.len() {
//         match tokens[i] {
//             "forward" => {
//                 if tokens[i+1].starts_with(param_prefix) {
//                     commands.push(Command::Forward(Arg::Param(String::from(tokens[i+1]))));
//                     i+=2;
//                 }
//                 else if let Ok(value) = tokens[i + 1].parse::<f64>() {
//                     commands.push(Command::Forward(Arg::Val(value)));
//                     i += 2;
//                 } else {
//                     panic!("Invalid forward command: expected a number or parameter");
//                 }
//             }
//             "turn" => {
//                 if let Ok(value) = tokens[i + 1].parse::<f64>() {
//                     commands.push(Command::Turn(Arg::Val(value)));
//                     i += 2;
//                 } else {
//                     panic!("Invalid turn command: expected a number");
//                 }
//             }
//             "repeat" => {
//                 if let Ok(repeats) = tokens[i + 1].parse::<u32>() {
//                     if tokens[i + 2] != "[" {
//                         panic!(
//                             "Expected '[' after 'repeat {}', but found {:?} or nothing.",
//                             repeats,
//                             tokens.get(i + 2)
//                         );
//                     }

//                     let start = i + 3;
//                     let end = tokens
//                         .iter()
//                         .rposition(|&w| w == "]")
//                         .expect("No matching ']' found for '[' after 'repeat' command");

//                     if start >= end {
//                         panic!("Malformed repeat block: '[' found but no commands before ']'");
//                     }

//                     let nested_commands = &tokens[start..end];
//                     commands.push(Command::Repeat(
//                         Arg::Val(repeats),
//                         parse_tokens(nested_commands, &mut labels.clone(), label_arity),
//                     ));

//                     i = end + 1;
//                 } else {
//                     panic!("Invalid repeat command");
//                 }
//             }
//             label if labels.contains(&String::from(label)) => {
//                 let params_num = *label_arity.get(label).unwrap();
//                 let param_vals = tokens[i+1..i+params_num+1].iter()
//                 .map(|&param| String::from(param)).collect();
//                 commands.push(Command::Fn_call(String::from(label), param_vals));
//                 i += params_num+1;
//             }
//             _ => panic!("Unknown command {}", tokens[i]),
//         }
//     }

//     commands
// }




pub fn parse_tokens(input: &[&str], labels: & Vec<String>) -> (Vec<Token>, usize) { // usize is artifact
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < input.len() { // mało funkcyjnie
        let debug_val = (input[i], i);
        println!("parsing text-token {} at index {} of {:?}", input[i], i, input);
        match input[i] {
            "stop" => {
                i += 1;
                tokens.push(Token::Stop);
            }
            "forward" => {
                i += 1;
                let expr = parse_expr(input, &mut i);
                println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Forward(Box::new(expr)));
            }
            "back" => {
                i += 1;
                let expr = parse_expr(input, &mut i);
                println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Back(Box::new(expr)));
            }
            "turn" => {
                i += 1;
                println!("Start parsing turn");
                let expr = parse_expr(input, &mut i);
                println!("End parsing turn");
                println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Turn(Box::new(expr)));
            }
            "right" => { // same as turn
                i += 1;
                println!("Start parsing turn");
                let expr = parse_expr(input, &mut i);
                println!("End parsing turn");
                println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Turn(Box::new(expr)));
            }
            "left" => {
                i += 1;
                println!("Start parsing turn");
                let expr = parse_expr(input, &mut i);
                println!("End parsing turn");
                println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Left(Box::new(expr)));
            }
            "repeat" => {
                println!("Parsing repeat starting at {:?}", input);
                i +=1; // move to parse_expr?
                let expr = parse_expr(input, &mut i);
                println!("Bracket parsing - start");
                i +=1; // TODO
                let bracket = parse_bracket(input, &mut i, labels);
                println!("Bracket parsing - end");
                println!("Parsed expr is {:?}", expr);
                println!("Parsed bracket is {:?}", bracket);
                tokens.push(Token::Repeat(Box::new(expr), Box::new(bracket)));
            }
            "if" => {
                i += 1;
                let expr1 = parse_expr(input, &mut i);
                let log_op = input[i];
                i +=1;
                let expr2 = parse_expr(input, &mut i);
                i+=1;
                let body = parse_bracket(input, &mut i, labels);
                println!("If statement body is {:#?}", body);
                // TODO PROBLEM (body is too large)

                match log_op {
                    ">" => {
                        tokens.push(Token::If(LogExpr::Greater(Box::new(expr1), Box::new(expr2)), Box::new(body)));}
                        "<"=>{
                        tokens.push(Token::If(LogExpr::Less(Box::new(expr1), Box::new(expr2)), Box::new(body)));}
                        other => {panic!("Logical expression with invalid operator {}", other);}
                }
                // tokens.push(Token::If(log_expr, Box::new(body)));
            }
            "]" => {println!("Finished parsing as bracket is closed, returning"); // TODO PROBLEM
        return (tokens, i-1);},
            // "[" => { // tutaj chyba nigdy nie wchodzimy!
            //     panic!("Open bracket without context");
            //     i += 1;
            //     println!("Bracket parsing - start");
            //     let bracket = parse_bracket(input, &mut i, labels);
            //     println!("Bracket parsing - end");
            //     tokens.push(bracket);
            // }
            // token if token.starts_with(':') => {
            //     tokens.push(Token::FnCall(token.to_string()));
            // }
            token if labels.contains(&String::from(token)) => {
                i +=1;
                tokens.push(Token::FnCall(token.to_string()));
            }
            token => {
                println!("Start expr wrapping at text token {}", token);
                let expr = parse_expr(input, &mut i); //debug
                println!("Parsed expr is {:?}", expr);
                // tokens.push(Token::Expression(Box::new(parse_expr(input, &mut i))));
                tokens.push(Token::Expression(Box::new(expr)));
                // panic!("Processing illegal text-token {}", token);
            }
        }
        // i += 1; // TODO
        println!("Finished parsing token at index {:?} next index is {}", debug_val, i)
    }
    (tokens, i)
}

fn parse_expr(input: &[&str], i: &mut usize) -> Expr { // rewrite functionally
    println!("Parsing {:?} at index {}", input, *i);
    let mut expr_stack = Vec::new();
    let mut op_stack = Vec::new();

    let mut expr_finished = false;
    // while *i < input.len() { //tODO
    // *i += 1;
    while !expr_finished && *i < input.len() {
        match input[*i] {
            "*" | "/" | "+" | "-" => {
                while let Some(op) = op_stack.pop() {
                    let rhs = expr_stack.pop().unwrap();
                    let lhs = expr_stack.pop().unwrap();
                    expr_stack.push(match op {
                        "*" => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                        "/" => Expr::Div(Box::new(lhs), Box::new(rhs)),
                        "+" => Expr::Add(Box::new(lhs), Box::new(rhs)),
                        "-" => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                        _ => unreachable!(),
                    });
                }
                op_stack.push(input[*i]);
                *i += 1;
            }
            // "[" | "]" => break, // remove case?
            token if token.starts_with(':') => {
                expr_stack.push(Expr::Arg(Arg::Param(token.to_string())));
                *i += 1;
            }
            token => {
                if let Ok(num) = token.parse::<f64>() {

                expr_stack.push(Expr::Arg(Arg::Val(num)));
                *i += 1;
                println!("Pushed value {} to stack", num);
                }
                else {println!("Finished parsing expression at {}", token); expr_finished = true; break}
            }
            // token => { // TODO
            //     if let Ok(num) = token.parse::<f64>() {
            //         expr_stack.push(Expr::Arg(Arg::Val(num)));
            //     } else {
            //         panic!("Unexpected token: {}", token);
            //     }
            // }
        }
        // *i += 1;
    }

    println!("Popping operators from stack");
    while let Some(op) = op_stack.pop() {
        let rhs = expr_stack.pop().unwrap();
        let lhs = expr_stack.pop().unwrap();
        expr_stack.push(match op {
            "*" => Expr::Mul(Box::new(lhs), Box::new(rhs)),
            "/" => Expr::Div(Box::new(lhs), Box::new(rhs)),
            "+" => Expr::Add(Box::new(lhs), Box::new(rhs)),
            "-" => Expr::Sub(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        });
    }

    expr_stack.pop().unwrap()
}

fn parse_bracket(input: &[&str], i: &mut usize, labels: & Vec<String>) -> Token {
    // *i += 1; // Skip the opening "["
    let mut contents = Vec::new();

    // while *i < input.len() && input[*i] != "]" { // pewnie też by można było zapisać funkcyjnie + stos.
    while *i < input.len() {
        match input[*i] {
            "]" => { *i += 1; break;},
            "[" => { *i += 1; contents.push(parse_bracket(input, i, labels));},
            // token => contents.extend(parse(&[token])),

            // token => contents.extend(parse_tokens(&[token])),
            
            _token => {
                let (tokens, num_processed_text_tokens) = parse_tokens(&input[*i..], labels);
                // contents.extend(parse_tokens(&input[*i..], labels));
                contents.extend(tokens);
                *i += num_processed_text_tokens;
            }
        }
        *i += 1; // ?
    }

    // bracket is actually at the position *i-1
    if *i < input.len() && input[*i-1] != "]" {
        panic!("Unmatched '[' in input {:?}, position {}, instead {}",input, *i, input[*i]);
    }
    //TODO PROBLEM

    Token::Bracket(contents)
}

#[derive(Debug)]
struct Turtle {
    x: f64,
    y: f64,
    angle: f64,
}

fn substitute_token(token: &Token, param_evaluator: &HashMap<String, f64>) -> Token {
    match token {
        Token::Stop => {Token::Stop},
        Token::Forward(expr) => {
            Token::Forward(Box::new(substitute_expr(expr, param_evaluator)))
        }
        Token::Back(expr) => {
            Token::Back(Box::new(substitute_expr(expr, param_evaluator)))
        }
        Token::Turn(expr) => {
            Token::Turn(Box::new(substitute_expr(expr, param_evaluator)))
        }
        Token::Left(expr) => {
            Token::Left(Box::new(substitute_expr(expr, param_evaluator)))
        }
        Token::Repeat(expr, body) => {
            let substituted_expr = substitute_expr(expr, param_evaluator);
            let substituted_body = substitute_token(body, param_evaluator);
            Token::Repeat(Box::new(substituted_expr), Box::new(substituted_body))
        }
        Token::Bracket(tokens) => {
            let substituted_tokens = tokens
                .iter()
                .map(|t| substitute_token(t, param_evaluator))
                .collect();
            Token::Bracket(substituted_tokens)
        }
        Token::Expression(expr) => {
            Token::Expression(Box::new(substitute_expr(expr, param_evaluator)))
        }
        Token::FnCallComplete(label, args) => {
            Token::FnCallComplete(label.clone(), args.iter().map(|arg| substitute_expr(arg, param_evaluator)).collect())
        }
        Token::FnCall(label) => //{panic!("Unprocessed function label {}", label)} // now it is normal to encounter unprocessed function calls
        {token.clone()}
        Token::If(log_expr, block) => {
            Token::If(log_expr.substitute(param_evaluator), Box::new(substitute_token(block, param_evaluator)))
        }
        // other => other.clone(), // Return unmodified for other token types
        //handle all cases explicitly!!
    }
}

fn substitute_expr(expr: &Expr, param_evaluator: &HashMap<String, f64>) -> Expr {
    match expr {
        Expr::Arg(Arg::Param(param)) => {
            if let Some(&value) = param_evaluator.get(param) {
                Expr::Arg(Arg::Val(value))
            } else {
                panic!("Parameter '{}' not found in evaluator", param);
            }
        }
        Expr::Arg(Arg::Val(_)) => {expr.clone()},
        Expr::Mul(lhs, rhs) => Expr::Mul(
            Box::new(substitute_expr(lhs, param_evaluator)),
            Box::new(substitute_expr(rhs, param_evaluator)),
        ),
        Expr::Div(lhs, rhs) => Expr::Div(
            Box::new(substitute_expr(lhs, param_evaluator)),
            Box::new(substitute_expr(rhs, param_evaluator)),
        ),
        Expr::Add(lhs, rhs) => Expr::Add(
            Box::new(substitute_expr(lhs, param_evaluator)),
            Box::new(substitute_expr(rhs, param_evaluator)),
        ),
        Expr::Sub(lhs, rhs) => Expr::Sub(
            Box::new(substitute_expr(lhs, param_evaluator)),
            Box::new(substitute_expr(rhs, param_evaluator)),
        ),
        // other => other.clone(), // handle all cases explicitly
    }
}

impl Turtle {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 270.0,
        }
    }

    // fn execute(&mut self, command: &Command, image: &mut Image, cmd_env: &Env) {
    //     match command {
    //         Command::Forward(Arg::Val(distance)) => {
    //             let radians = self.angle.to_radians();
    //             let new_x = self.x + distance * radians.cos();
    //             let new_y = self.y + distance * radians.sin();

    //             image.add_line(self.x, self.y, new_x, new_y);

    //             self.x = new_x;
    //             self.y = new_y;
    //         }
    //         Command::Turn(Arg::Val(angle)) => {
    //             self.angle = (self.angle + angle) % 360.0;
    //         }
    //         Command::Repeat(Arg::Val(times), commands) => {
    //             for _ in 0..*times {
    //                 for command in commands.iter() {
    //                     self.execute(command, image, cmd_env);
    //                 }
    //             }
    //         }
    //         Command::Fn_call(label, param_vals) => {
    //             let commands = cmd_env.get(label, param_vals);

    //             for command in commands {
    //                 self.execute(&command, image, cmd_env);
    //             }
    //         }
    //         _ => {
    //             panic!("Tried to call {:?} command possibly with unspecified parameter value", command);
    //         }
    //     }
    // }

    // pub fn execute(&mut self, token: &Token, image: &mut Image, cmd_env: &Env) {

    //     pub fn execute(&mut self, token: &Token, image: &mut Image, cmd_env: &Env) { //execute one?
    //     match token {
    //         Token::Forward(expr) => {
    //             let distance = expr.evaluate(cmd_env);
    //             let radians = self.angle.to_radians();
    //             let new_x = self.x + distance * radians.cos();
    //             let new_y = self.y + distance * radians.sin();

    //             image.add_line(self.x, self.y, new_x, new_y);
    //             self.x = new_x;
    //             self.y = new_y;
    //         }
    //         Token::Turn(expr) => {
    //             let angle = expr.evaluate(cmd_env);
    //             self.angle = (self.angle + angle) % 360.0;
    //         }
    //         Token::Repeat(expr, body) => {
    //             let times = expr.evaluate(cmd_env) as u32;
    //             for _ in 0..times {
    //                 match body.as_ref() {
    //                     Token::Bracket(tokens) => {
    //                         for token in tokens {
    //                             self.execute(token, image, cmd_env);
    //                         }
    //                     }
    //                     _ => panic!("Repeat body must be a Bracket token"),
    //                 }
    //             }
    //         }
    //         Token::FnCall(label) => {
    //             if let Some(fun) = cmd_env.functions.get(label) {

    //                 let mut param_vals = Vec::new();
    //                 for param in &fun.params {
    //                     let value = cmd_env.functions.get(param).map_or_else(
    //                         || panic!("Missing parameter value for {}", param),
    //                         |arg| arg.params.join(""), // Add parameters logic here.
    //                     );
    //                     param_vals.push(value);
    //                 }

    //                 let commands = fun.body.clone();
    //                 for token in commands {
    //                     self.execute(&token, image, cmd_env);
    //                 }
    //             } else {
    //                 panic!("Function '{}' not found in the environment", label);
    //             }
    //         }
    //         Token::Bracket(tokens) => {
    //             for token in tokens {
    //                 self.execute(token, image, cmd_env);
    //             }
    //         }
    //         _ => panic!("Unsupported token in execute: {:?}", token),
    //     }
    // }

    pub fn execute(&mut self, token: &Token, image: &mut Image, cmd_env: &Env) {
        match token {
            Token::Forward(expr) => {
                let distance = expr.evaluate();
                let radians = self.angle.to_radians();
                let new_x = self.x + distance * radians.cos();
                let new_y = self.y + distance * radians.sin();

                image.add_line(self.x, self.y, new_x, new_y);
                self.x = new_x;
                self.y = new_y;
            }
            Token::Back(expr) => {
                let distance = expr.evaluate();
                let radians = self.angle.to_radians();
                let new_x = self.x - distance * radians.cos();
                let new_y = self.y - distance * radians.sin();

                image.add_line(self.x, self.y, new_x, new_y);
                self.x = new_x;
                self.y = new_y;
            }
            Token::Turn(expr) => {
                let angle = expr.evaluate();
                self.angle = (self.angle + angle) % 360.0;
            }
            Token::Left(expr) => { //temporary
                let angle = expr.evaluate();
                self.angle = (self.angle - angle) % 360.0;
            } 
            Token::Repeat(expr, body) => {
                let times = expr.evaluate() as u32;
                for _ in 0..times {
                    match body.as_ref() {
                        Token::Bracket(tokens) => {
                            for token in tokens {
                                self.execute(token, image, cmd_env);
                            }
                        }
                        _ => panic!("Repeat body must be a Bracket token"),
                    }
                }
            }
            Token::FnCallComplete(label, args) => {
                println!("Begin function call");
                if let Some(fun) = cmd_env.functions.get(label) {
                    // Substitute the arguments into the function body
                    let param_evaluator: HashMap<String, f64> = fun
                        .params
                        .iter()
                        .zip(args.iter())
                        .map(|(param, expr)| {
                            (
                                param.clone(),
                                expr.evaluate(), // Evaluate the expression argument
                                // All of the substitutions were done by the parser
                            )
                        })
                        .collect();

                    let commands = fun //kiedyś robiłem to jakoś na zewnątrz, sprawdź jak jest lepiej
                        .body
                        .iter()
                        .map(|token| substitute_token(token, &param_evaluator))
                        .collect::<Vec<Token>>();

                    println!("Commands before wrapping {:?}", commands);
                    let wrapped_commands = wrap_fn_call(commands, cmd_env); // debug
                    println!("Commands after wrapping {:#?}", wrapped_commands);
                    // TODO Execute the substituted tokens
                    // for command in commands {
                    // for command in wrap_fn_call(commands, cmd_env) { // using parser inside interpreter. Weird but ok
                    for command in wrapped_commands { 
                        self.execute(&command, image, cmd_env);
                    }
                } else {
                    panic!("Function '{}' not found in the environment", label);
                }
            }
            Token::Bracket(tokens) => {
                for token in tokens {
                    self.execute(token, image, cmd_env);
                }
            }
            Token::If(log_expr, body) => {
                println!("Evaluating logical expression {:?}", log_expr);
                if let LogExpr::Val(true) = log_expr.evaluate() {
                    println!("Evaluated true");
                    match body.as_ref() {
                        Token::Bracket(tokens) => {
                            for token in tokens {
                                self.execute(token, image, cmd_env);
                            }
                        }
                        _ => panic!("If body must be a Bracket token"),
                    }
                }
                else {println!("Evaluated false");}  //debug
            }
            Token::Stop => { return;},
            _ => panic!("Unsupported token in execute: {:?}", token),
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

// pub fn run(commands: impl Iterator<Item = Command>, cmd_env: Env, image_path: &str) {

pub fn run(commands: impl Iterator<Item = Token> + std::fmt::Debug, cmd_env: Env, image_path: &str) {
    let mut turtle = Turtle::new();
    let mut image = Image::new();

    println!("Begin executing commands {:?}", commands);
    for command in commands {
        turtle.execute(&command, &mut image, &cmd_env);
    }

    image.save(image_path);
}