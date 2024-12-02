use std::collections::HashMap;

use crate::interpreter::image::*;
use crate::interpreter::parser::*;
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
            Token::TurnLeft(expr) => {
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
                if fns.contains(label) {
                    for command in fns.get_commands(label, args) {
                        if !self.execute(&command, image, fns) {
                            return true; // exit the scope
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
            }
            Token::Stop => {
                return false;
            }
            _ => panic!("Unsupported token in execute: {:?}", token),
        }

        true
    }
}
