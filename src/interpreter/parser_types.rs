use std::collections::HashMap;

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
    // fn fun_body(&self, param_vals: &Vec<String>) -> Vec<Token> {
    //     let mut param_evaluator: HashMap<String, String> = HashMap::new();
    //     for (param_name, param_val) in self.params.iter().zip(param_vals.iter()) {
    //         param_evaluator.insert(param_name.clone(), param_val.clone());
    //     }

    //     eval_params(&self.body, param_evaluator)
    // }
    pub fn fun_body(&self) -> &Vec<Token> {
        // chciałbym, żeby to wołało interpreter(parser?), który zrobi podstawienia
        // wersja tymczasowa, aż naprawię exec
        // let mut param_evaluator: HashMap<String, String> = HashMap::new();
        // for (param_name, param_val) in self.params.iter().zip(param_vals.iter()) {
        //     param_evaluator.insert(param_name.clone(), param_val.clone());
        // }

        // eval_params(&self.body, param_evaluator)
        &self.body
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
}
