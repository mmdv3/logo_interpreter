use std::collections::HashMap;

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






// #[derive(Debug, Clone)]
// pub enum Command {
//     Forward(Arg<f64>),
//     Turn(Arg<f64>),
//     Repeat(Arg<u32>, Vec<Command>),
//     Fn_call(String, Vec<String>),
// }

pub struct Fun {
    pub params: Vec<String>,
    // body: Vec<Command>,
    pub body: Vec<Token>
}

impl Fun{
    pub fn new(commands: Vec<Token>, params: Vec<String>) -> Fun {
        Fun {params, body: commands}
    }
    // fn fun_body(&self, param_vals: &Vec<String>) -> Vec<Token> {
    //     let mut param_evaluator: HashMap<String, String> = HashMap::new();
    //     for (param_name, param_val) in self.params.iter().zip(param_vals.iter()) {
    //         param_evaluator.insert(param_name.clone(), param_val.clone());
    //     }

    //     eval_params(&self.body, param_evaluator)
    // }
    pub fn fun_body(&self) -> &Vec<Token> { // wersja tymczasowa, aż naprawię exec
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

pub struct Env {
    pub functions: HashMap<String, Fun>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            functions: HashMap::new(),
        }
    }
    // fn get(&self, label: &String, param_vals: &Vec<String>) -> Vec<Command> {
    //     self.functions.get(label).unwrap().fun_body(param_vals)
    // }
    // fn get(&self, label: &String) -> Vec<Token> { // wersja tymczasowa
    pub fn get(&self, label: &String) -> Option<&Fun> { // wersja tymczasowa
        // self.functions.get(label).unwrap().fun_body().clone() // paskudny klon
        self.functions.get(label)
    }

    pub fn push(&mut self, (label, commands, params): (String, Vec<Token>, Vec<String>)) {
        // fn parse_fn(tokens: &[&str], labels: &mut Vec<String>, label_arity: &mut HashMap<String, usize>) -> (String, Vec<Token>, Vec<String>) {
        self.functions.insert(label, Fun::new(commands, params));
    }
}