use std::collections::HashMap;

use crate::interpreter::parser::wrap_fn_call;
use crate::interpreter::{substitute_token, substitute_expr};

#[derive(Debug, Clone)]
pub enum Arg {
    Val(f64),
    Param(String),
}

#[derive(Debug, Clone)]
pub enum LogExpr {
    Greater(Box<Expr>, Box<Expr>), // Represents expr > expr
    Less(Box<Expr>, Box<Expr>),    // Represents expr < expr
    Val(bool),                     //represents computed value
}

impl LogExpr {
    pub fn evaluate(&self) -> LogExpr {
        match self {
            LogExpr::Greater(lhs, rhs) => LogExpr::Val(lhs.evaluate() > rhs.evaluate()),
            LogExpr::Less(lhs, rhs) => LogExpr::Val(lhs.evaluate() < rhs.evaluate()),
            LogExpr::Val(_) => {self.clone()},
        }
    }

    pub fn substitute(&self, param_evaluator: &HashMap<String, f64>) -> LogExpr {
        match self {
            LogExpr::Greater(lhs, rhs) => LogExpr::Greater(
                substitute_expr(lhs, param_evaluator) , 
                substitute_expr(rhs, param_evaluator)),
            LogExpr::Less(lhs, rhs) => LogExpr::Less(
                substitute_expr(lhs, param_evaluator) , 
                substitute_expr(rhs, param_evaluator)),
            LogExpr::Val(_) => {self.clone()},
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Forward(Box<Expr>),
    Back(Box<Expr>),
    TurnRight(Box<Expr>),
    TurnLeft(Box<Expr>),
    Repeat(Box<Expr>, Box<Token>),
    FnLabel(String), // execution should panic
    FnCall(String, Vec<Expr>),
    Bracket(Vec<Token>),
    If(LogExpr, Box<Token>),
    Expression(Box<Expr>),
    Stop,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Arg(Arg),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(&self) -> f64 {
        match self {
            Expr::Arg(Arg::Val(value)) => *value,
            Expr::Arg(Arg::Param(param)) => {
                panic!(
                    "Parameter '{}' found during evaluation",
                    param
                );
            }
            Expr::Mul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            Expr::Div(lhs, rhs) => lhs.evaluate() / rhs.evaluate(),
            Expr::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expr::Sub(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
        }
    }
}

pub struct Fun {
    pub params: Vec<String>,
    pub body: Vec<Token>,
}

impl Fun {
    pub fn new(commands: Vec<Token>, params: Vec<String>) -> Fun {
        Fun {
            params,
            body: commands,
        }
    }
    pub fn arity(&self) -> usize {
        self.params.len()
    }
}

pub struct Functions {
    pub functions: HashMap<String, Fun>,
}

impl Functions {
    pub fn new() -> Functions {
        Functions {
            functions: HashMap::new(),
        }
    }

    pub fn get(&self, label: &String) -> Option<&Fun> {
        self.functions.get(label)
    }

    pub fn push(&mut self, (label, commands, params): (String, Vec<Token>, Vec<String>)) {
        self.functions.insert(label, Fun::new(commands, params));
    }

    pub fn contains(&self, label: &String) -> bool {
        self.functions.contains_key(label)
    }

    pub fn labels(&self) -> Vec<&String> {
        self.functions.keys().collect()
    }

    pub fn get_commands(&self, label: &String, args: &Vec<Expr>) -> Vec<Token> {
        let fun = self.get(label).unwrap();
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

        let commands = fun
                        .body
                        .iter()
                        .map(|token| substitute_token(token, &param_evaluator))
                        .collect::<Vec<Token>>();

                    //println!("Commands before wrapping {:?}", commands);
        wrap_fn_call(commands, self)
    }
}
