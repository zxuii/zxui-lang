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

fn run_file(path: &str) {
    let file = read_to_string(path);
    match file {
        Ok(f) => match Lexer::new(path.to_string(), f.clone()).tokenize() {
            Ok(tokens) => match Parser::new(path.to_string(), f.clone(), tokens).parse() {
                Ok(stmt) => match Interpreter::new(path.to_string(), f).exec_stmt(&stmt) {
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
}

fn print_usage() {
    eprintln!("usage: zxui <command or file>");
    eprintln!("Commands:");
    eprintln!("run  [path] - run project. if path is empty,");
    eprintln!("              it will run project in current dir.");
    eprintln!("init [path] - init project. if path is empty,");
    eprintln!("              it will init on current dir.");
    eprintln!();
    eprintln!("Args:");
    eprintln!("<file>      - run single file and cannot be using");
    eprintln!("              import feature.");
}

fn run() {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        match path.as_str() {
            "run" => {}
            "init" => {}
            "help" => print_usage(),
            _ => run_file(path),
        }
    } else {
        print_usage();
        exit(1);
    }
}
