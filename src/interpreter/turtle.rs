use std::collections::HashMap;
use svg::node::element::Rectangle;
use svg::node::element::{Group, Line};
use svg::Document;

use crate::interpreter::parser::*;

use crate::interpreter::image::*;

use crate::interpreter::*;

#[derive(Debug)]
pub struct Turtle {
    x: f64,
    y: f64,
    angle: f64,
}

impl Turtle {
    pub fn new() -> Self {
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

    pub fn execute(&mut self, token: &Token, image: &mut Image, fns: &Functions) -> bool {
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
            Token::TurnRight(expr) => {
                let angle = expr.evaluate();
                self.angle = (self.angle + angle) % 360.0;

            }
            Token::TurnLeft(expr) => { //temporary
                let angle = expr.evaluate();
                self.angle = (self.angle - angle) % 360.0;

            
            } 
            Token::Repeat(expr, body) => {
                let times = expr.evaluate() as u32;
                for _ in 0..times {
                    match body.as_ref() {
                        Token::Bracket(tokens) => {
                            for token in tokens {
                                if !self.execute(token, image, fns) {
                                    return false;
                                };
                            }
                        }
                        _ => panic!("Repeat body must be a Bracket token"),
                    }
                }

            }
            Token::FnCall(label, args) => {
                //println!("Begin function call");
                if let Some(fun) = fns.functions.get(label) {
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

                    println!("Calling function with parameters: {:?}", param_evaluator);

                    let commands = fun //kiedyś robiłem to jakoś na zewnątrz, sprawdź jak jest lepiej
                        .body
                        .iter()
                        .map(|token| substitute_token(token, &param_evaluator))
                        .collect::<Vec<Token>>();

                    //println!("Commands before wrapping {:?}", commands);
                    let wrapped_commands = wrap_fn_call(commands, fns); // debug
                    //println!("Commands after wrapping {:#?}", wrapped_commands);
                    // TODO Execute the substituted tokens
                    // for command in commands {
                    // for command in wrap_fn_call(commands, cmd_env) { // using parser inside interpreter. Weird but ok
                    for command in wrapped_commands { 
                        if !self.execute(&command, image, fns) {
                            return true; // TODO PROBLEM - ten false miał zabić aktualny scope, który tu się właśnie kończy
                        };
                    }
                } else {
                    panic!("Function '{}' not found in the environment", label);
                }
            }
            Token::Bracket(tokens) => {
                for token in tokens {
                    if !self.execute(token, image, fns) {
                        return false;
                    };
                }
            }
            Token::If(log_expr, body) => {
                //println!("Evaluating logical expression {:?}", log_expr);
                if let LogExpr::Val(true) = log_expr.evaluate() {
                    //println!("Evaluated true");
                    match body.as_ref() {
                        Token::Bracket(tokens) => {
                            for token in tokens {
                                if !self.execute(token, image, fns) {
                                    return false;
                                };
                            }
                        }
                        _ => panic!("If body must be a Bracket token"),
                    }
                }
                else {
                    //println!("Evaluated false");
                }  //debug
            }
            Token::Stop => { return false;},
            _ => panic!("Unsupported token in execute: {:?}", token),
        }

        true
    }
}