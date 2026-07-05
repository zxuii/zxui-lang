mod lexer;
mod tokens;
mod ast;
mod parser;

use lexer::Lexer;
use std::{env, fs::read_to_string, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let file = read_to_string(path);
        match file {
            Ok(f) => {
                let mut lex = Lexer::new(f);
                lex.tokenize();
                for t in &lex.tokens {
                    println!("{}", t);
                }

                let mut parse = parser::Parser::new(lex.tokens);
                println!("{:#?}", parse.parse().expect("Parse Error"));
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
