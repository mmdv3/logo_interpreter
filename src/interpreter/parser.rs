use crate::interpreter::parser_types::*;

use std::collections::HashMap;

// use std::collections::HashMap;

// mod parser_types;

// use parser_types::*; // !!!!


/// end parser types

// use crate::code::substitute_expr;

const param_prefix: &str = ":";





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



pub fn wrap_fn_call(tokens: Vec<Token>, cmd_env: &Env) -> Vec<Token> {
    let mut wrapped_tokens = Vec::new();
    let mut iter = tokens.into_iter();

    while let Some(token) = iter.next() {
        //println!("Wrap function call checking token {:?}", token);
        match token {
            Token::FnCall(label) => {
                //println!("Wrapping function call to {}", label);
                // if let Some(fun) = cmd_env.functions.get(&label) {
                if let Some(fun) = cmd_env.get(&label) {
                    // let arity = fun.params.len();
                    let arity = fun.arity();
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
    //println!("Finished wrapping commands. Result is {:?}", wrapped_commands);
    (wrapped_commands, cmd_env)
}


fn parse_fn(tokens: &[&str], labels: &mut Vec<String>, label_arity: &mut HashMap<String, usize>) -> (String, Vec<Token>, Vec<String>) {
    //println!("Parsing function body {:?}", tokens);
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
        //println!("Parsing function, no parameters found");
        params = vec![];
    }

    let label = tokens[1];
    labels.push(String::from(label));
    label_arity.insert(String::from(label), params.len());

    //println!("Parsing function body from token {}, found {} params", fn_body_start, params.len());
    // (String::from(label), parse_tokens(&tokens[fn_body_start..], &mut labels.clone(), label_arity), params)

    let (fn_body, _) =  parse_tokens(&tokens[fn_body_start..], &labels);
    //println!("Function body is {:#?}", fn_body);
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
        //println!("parsing text-token {} at index {} of {:?}", input[i], i, input);
        match input[i] {
            "stop" => {
                i += 1;
                tokens.push(Token::Stop);
            }
            "forward" => {
                i += 1;
                let expr = parse_expr(input, &mut i);
                //println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Forward(Box::new(expr)));
            }
            "back" => {
                i += 1;
                let expr = parse_expr(input, &mut i);
                //println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Back(Box::new(expr)));
            }
            "turn" => {
                i += 1;
                //println!("Start parsing turn");
                let expr = parse_expr(input, &mut i);
                //println!("End parsing turn");
                //println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Turn(Box::new(expr)));
            }
            "right" => { // same as turn
                i += 1;
                //println!("Start parsing turn");
                let expr = parse_expr(input, &mut i);
                //println!("End parsing turn");
                //println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Turn(Box::new(expr)));
            }
            "left" => {
                i += 1;
                //println!("Start parsing turn");
                let expr = parse_expr(input, &mut i);
                //println!("End parsing turn");
                //println!("Parsed expr is {:?}", expr);
                tokens.push(Token::Left(Box::new(expr)));
            }
            "repeat" => {
                //println!("Parsing repeat starting at {:?}", input);
                i +=1; // move to parse_expr?
                let expr = parse_expr(input, &mut i);
                //println!("Bracket parsing - start");
                i +=1; // TODO
                let bracket = parse_bracket(input, &mut i, labels);
                //println!("Bracket parsing - end");
                //println!("Parsed expr is {:?}", expr);
                //println!("Parsed bracket is {:?}", bracket);
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
                //println!("If statement body is {:#?}", body);
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
            "]" => {//println!("Finished parsing as bracket is closed, returning"); // TODO PROBLEM
        return (tokens, i-1);},
            // "[" => { // tutaj chyba nigdy nie wchodzimy!
            //     panic!("Open bracket without context");
            //     i += 1;
            //     //println!("Bracket parsing - start");
            //     let bracket = parse_bracket(input, &mut i, labels);
            //     //println!("Bracket parsing - end");
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
                //println!("Start expr wrapping at text token {}", token);
                let expr = parse_expr(input, &mut i); //debug
                //println!("Parsed expr is {:?}", expr);
                // tokens.push(Token::Expression(Box::new(parse_expr(input, &mut i))));
                tokens.push(Token::Expression(Box::new(expr)));
                // panic!("Processing illegal text-token {}", token);
            }
        }
        // i += 1; // TODO
        //println!("Finished parsing token at index {:?} next index is {}", debug_val, i)
    }
    (tokens, i)
}

fn parse_expr(input: &[&str], i: &mut usize) -> Expr { // rewrite functionally
    println!("Parsing {:?} at index {}", input, *i);
    let mut expr_stack = Vec::new();
    let mut op_stack = Vec::new();

    let mut read_next = "variable";

    let mut expr_finished = false;
    // while *i < input.len() { //tODO
    // *i += 1;
    while !expr_finished && *i < input.len() {
        println!("Parse expr matching token {}", input[*i]);

        match read_next {
            "operator" => {
        
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
                read_next = "variable";
            }
            other => {
                println!("Parser found {} instead of operator, finishing expression", other);
                expr_finished = true; 
                break;
            }
        }
    }
        "variable" => {
            match input[*i] {
            // "[" | "]" => break, // remove case?
            token if token.starts_with(':') => {
                expr_stack.push(Expr::Arg(Arg::Param(token.to_string())));
                *i += 1;
                read_next = "operator";
            }
            token => {
                if let Ok(num) = token.parse::<f64>() {

                expr_stack.push(Expr::Arg(Arg::Val(num)));
                *i += 1;
                read_next = "operator"
                //println!("Pushed value {} to stack", num);
                }
            else {
                panic!("Parser found {} instead of variable", token);
            }
                // else {
                //     // println!("Finished parsing expression at {}", token);
                //      expr_finished = true; break}
            }

            // token => { // TODO
            //     if let Ok(num) = token.parse::<f64>() {
            //         expr_stack.push(Expr::Arg(Arg::Val(num)));
            //     } else {
            //         panic!("Unexpected token: {}", token);
            //     }
            // }
        }
    }
        other => {
            panic!{"Illegal value of read_next {}", other};
        }
    }
}
        // *i += 1;
    

    //println!("Popping operators from stack");
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

    println!("Finished parsing single expression");
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