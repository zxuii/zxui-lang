#[derive(PartialEq, Clone)]
pub enum TokenType {
    // Literals
    Identifier(String),
    Number(f64),
    String(String),
    True,
    False,
    Null,

    // Keywords
    Let,
    Fun,
    Return,
    If,
    Else,

    // Symbols
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
    EqEq, // ==
    Lt,   // <
    Gt,   // >
    LtEq, // <=
    GtEq, // >=
    Bang, // !
    BangEq, // !=
    And, // and
    Or, // or

    // Specials
    Eof,
    Program
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Identifier(_) => { write!(f, "identifier") }
            TokenType::Number(_) => { write!(f, "number")}
            TokenType::String(_) => { write!(f, "string") }
            TokenType::True => { write!(f, "true") }
            TokenType::False => { write!(f, "false") }
            TokenType::Null => { write!(f, "null")}
            TokenType::Let => { write!(f, "let") }
            TokenType::Fun => { write!(f, "fun") }
            TokenType::Return => { write!(f, "return") }
            TokenType::If => { write!(f, "if") }
            TokenType::Else => { write!(f, "else") }
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
            TokenType::EqEq => { write!(f, "==") }
            TokenType::Lt => { write!(f, "<") }
            TokenType::Gt => { write!(f, ">") }
            TokenType::LtEq => { write!(f, "<=") }
            TokenType::GtEq => { write!(f, ">=") }
            TokenType::Bang => { write!(f, "!") }
            TokenType::BangEq => { write!(f, "!=") }
            TokenType::And => { write!(f, "and") }
            TokenType::Or => { write!(f, "or") }
            TokenType::Eof => { write!(f, "eof") }
            TokenType::Program => { write!(f, "program") }
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub ty: TokenType,
    pub line: usize,
    pub col: usize
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}:{}", self.ty, self.line, self.col)
    }
}

impl Token {
    pub fn new(ty: TokenType, line: usize, col: usize) -> Self {
        Self {
            ty, line, col
        }
    }
}

