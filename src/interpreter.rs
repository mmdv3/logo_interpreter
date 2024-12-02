mod parser;
mod turtle;
mod image;
mod parser_types;

use turtle::*;
use parser::*;
use image::*;
use parser_types::*;
// !!!!


use std::collections::HashMap;
use svg::node::element::Rectangle;
use svg::node::element::{Group, Line};
use svg::Document;

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





// pub fn run(commands: impl Iterator<Item = Command>, cmd_env: Env, image_path: &str) {

// pub fn run(commands: impl Iterator<Item = Token> + std::fmt::Debug, cmd_env: Env, image_path: &str) {

pub fn run(input: &str, image_path: &str) {
    let (commands, cmd_env) = parse(input); //Env rename to Functions?
// let image_path = "img/fern.svg";

// run(commands.into_iter(), cmd_env, image_path);

    let mut turtle = Turtle::new();
    let mut image = Image::new();

    //println!("Begin executing commands {:?}", commands);
    for command in commands {
        turtle.execute(&command, &mut image, &cmd_env);
    }

    image.save(image_path);
}