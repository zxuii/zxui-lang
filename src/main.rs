mod ast;
mod builtins;
mod environment;
mod interpreter;
mod lexer;
mod object;
mod parser;
mod tokens;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::{env, fs::read_to_string, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let file = read_to_string(path);
        match file {
            Ok(f) => {
                let mut lex = Lexer::new(f.clone());
                lex.tokenize();
                // for t in &lex.tokens {
                //     println!("{}", t);
                // }

                match Parser::new(f, lex.tokens).parse() {
                    Ok(stmt) => {
                        // println!("{:#?}", stmt);
                        match Interpreter::new().exec_stmt(&stmt) {
                            Ok(_) => {
                                // println!("{:?}", result.unwrap_or(object::Value::Null) );
                            }

                            Err(e) => eprintln!("Runtime Error: {e}"),
                        }
                    }
                    Err(e) => eprintln!("Parse Error: {e}"),
                }
                // println!("{:#?}", parse.parse().expect("Parse Error"));
            }

            Err(e) => {
                eprintln!("Error when opening file '{}': {}", path, e);
                exit(1);
            }
        }
    } else {
        eprintln!("usage: zxui <file.zxui>");
        exit(1);
    }
}
