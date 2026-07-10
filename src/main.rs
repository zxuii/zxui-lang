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
    let child = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB untuk call stack, gede sih awokawok
        .spawn(run) // jalanin run function
        .unwrap();

    child.join().unwrap();
}

fn run() {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let file = read_to_string(path);
        match file {
            Ok(f) => match Lexer::new(path.clone(), f.clone()).tokenize() {
                Ok(tokens) => match Parser::new(path.clone(), f, tokens).parse() {
                    Ok(stmt) => match Interpreter::new().exec_stmt(&stmt) {
                        Ok(_) => {}

                        Err(e) => {
                            eprintln!("Runtime Error: {e}");
                            exit(1)
                        }
                    },
                    Err(e) => {
                        eprintln!("Parse Error: {e}");
                        exit(1)
                    }
                },
                Err(e) => {
                    eprintln!("Lexing Error: {e}");
                    exit(1)
                }
            },

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
