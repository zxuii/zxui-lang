mod tokens;
mod lexer;

use lexer::Lexer;

fn main() {
    let mut lex = Lexer::new("let x = 5".to_string());
    lex.tokenize();
    for t in lex.tokens {
        println!("{}", t);
    }
}
