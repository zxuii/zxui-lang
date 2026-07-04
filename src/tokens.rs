pub enum TokenType {
    Identifier(String),
    Number(f64),
    Let,
    Fun,
    Return,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Equal,
    Eof,
    Program
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Identifier(ident) => { write!(f, "identifier({ident})") }
            TokenType::Number(num) => { write!(f, "number({num})")}
            TokenType::Let => { write!(f, "let") }
            TokenType::Fun => { write!(f, "fun") }
            TokenType::Return => { write!(f, "return") }
            TokenType::Plus => { write!(f, "+") }
            TokenType::Minus => { write!(f, "-") }
            TokenType::Asterisk => { write!(f, "*") }
            TokenType::Slash => { write!(f, "/") }
            TokenType::Semicolon => { write!(f, ";") }
            TokenType::Lparen => { write!(f, "(") }
            TokenType::Rparen => { write!(f, ")") }
            TokenType::Lbrace => { write!(f, "{{") }
            TokenType::Rbrace => { write!(f, "}}") }
            TokenType::Comma => { write!(f, ",") }
            TokenType::Equal => { write!(f, "=") }
            TokenType::Eof => { write!(f, "eof") }
            TokenType::Program => { write!(f, "program") }
        }
    }
}

pub struct Token {
    pub ty: TokenType,
    pub val: String,
    pub line: usize,
    pub col: usize
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}:{}", self.ty, self.line, self.col)
    }
}

impl Token {
    pub fn new(ty: TokenType, val: String, line: usize, col: usize) -> Self {
        Self {
            ty, val, line, col
        }
    }
}

