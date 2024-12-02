mod parser;
mod turtle;
mod image;
mod parser_types;

use turtle::*;
use parser::*;
use image::*;
use parser_types::*;

use std::collections::HashMap;

fn substitute_token(token: &Token, param_evaluator: &HashMap<String, f64>) -> Token {
    match token {
        Token::Stop => {Token::Stop},
        Token::Forward(expr) => {
            Token::Forward(substitute_expr(expr, param_evaluator))
        }
        Token::Back(expr) => {
            Token::Back(substitute_expr(expr, param_evaluator))
        }
        Token::TurnRight(expr) => {
            Token::TurnRight(substitute_expr(expr, param_evaluator))
        }
        Token::TurnLeft(expr) => {
            Token::TurnLeft(substitute_expr(expr, param_evaluator))
        }
        Token::Repeat(expr, body) => {
            let substituted_expr = substitute_expr(expr, param_evaluator);
            let substituted_body = substitute_token(body, param_evaluator);
            Token::Repeat(substituted_expr, Box::new(substituted_body))
        }
        Token::Bracket(tokens) => {
            let substituted_tokens = tokens
                .iter()
                .map(|t| substitute_token(t, param_evaluator))
                .collect();
            Token::Bracket(substituted_tokens)
        }
        Token::Expression(expr) => {
            Token::Expression(substitute_expr(expr, param_evaluator))
        }
        Token::FnCall(label, args) => {
            Token::FnCall(label.clone(), args.iter().map(|arg| *substitute_expr(arg, param_evaluator)).collect())
        }
        Token::FnLabel(label) =>
        {token.clone()}
        Token::If(log_expr, block) => {
            Token::If(log_expr.substitute(param_evaluator), Box::new(substitute_token(block, param_evaluator)))
        }
    }
}

fn substitute_expr(expr: &Expr, param_evaluator: &HashMap<String, f64>) -> Box<Expr> {
    match expr {
        Expr::Arg(Arg::Param(param)) => {
            if let Some(&value) = param_evaluator.get(param) {
                Box::new(Expr::Arg(Arg::Val(value)))
            } else {
                panic!("Parameter '{}' not found in evaluator", param);
            }
        }
        Expr::Arg(Arg::Val(_)) => {Box::new(expr.clone())},
        Expr::Mul(lhs, rhs) => Box::new(Expr::Mul(
            substitute_expr(lhs, param_evaluator),
            substitute_expr(rhs, param_evaluator),
        )),
        Expr::Div(lhs, rhs) => Box::new(Expr::Div(
            substitute_expr(lhs, param_evaluator),
            substitute_expr(rhs, param_evaluator),
        )),
        Expr::Add(lhs, rhs) => Box::new(Expr::Add(
            substitute_expr(lhs, param_evaluator),
            substitute_expr(rhs, param_evaluator),
        )),
        Expr::Sub(lhs, rhs) => Box::new(Expr::Sub(
            substitute_expr(lhs, param_evaluator),
            substitute_expr(rhs, param_evaluator),
        )),
    }
}


pub fn run(input: &str, image_path: &str) {
    let (commands, fns) = parse(input);

    let mut turtle = Turtle::new();
    let mut image = Image::new();

    //println!("Begin executing commands {:?}", commands);
    for command in commands {
        turtle.execute(&command, &mut image, &fns);
    }

    image.save(image_path);
}