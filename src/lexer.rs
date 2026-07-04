use crate::tokens::{Token, TokenType};

pub struct Lexer {
    code: Vec<char>,
    line: usize,
    col: usize,
    pos: usize,
    ch: Option<char>,
    pub tokens: Vec<Token>
}

impl Lexer {
    pub fn new(code: String) -> Self {
        let code: Vec<char> = code.chars().collect();
        Self {
            code, line: 1, col: 0, pos: 0, ch: None, tokens: vec![]
        }
    }

    pub fn tokenize(&mut self) {
        self.advance();
        self.add_token(TokenType::Program, "program".to_string());
        while !(self.ch == None) {
            self.next_token();
        }

        self.add_token(TokenType::Eof, "eof".to_string());
    }
    fn next_token(&mut self) {
        self.skip_whitespace();

        if self.ch == None {
            return;
        }

        if self.ch == Some('+') {
            self.add_token_advance(TokenType::Plus, "+".to_string());
        } else if self.ch == Some('-') {
            self.add_token_advance(TokenType::Minus, "-".to_string());
        } else if self.ch == Some('*') {
            self.add_token_advance(TokenType::Asterisk, "*".to_string())
        } else if self.ch == Some('/') {
            self.add_token_advance(TokenType::Slash, "/".to_string());
        } else if self.ch == Some(';') {
            self.add_token_advance(TokenType::Semicolon, ";".to_string());
        } else if self.ch == Some('(') {
            self.add_token_advance(TokenType::Lparen, "(".to_string());
        } else if self.ch == Some(')') {
            self.add_token_advance(TokenType::Rparen, ")".to_string());
        } else if self.ch == Some('{') {
            self.add_token_advance(TokenType::Lbrace, "{".to_string());
        } else if self.ch == Some('}') {
            self.add_token_advance(TokenType::Rbrace, "}".to_string());
        } else if self.ch == Some(',') {
            self.add_token_advance(TokenType::Comma, ",".to_string());
        } else if self.ch == Some('=') {
            self.add_token_advance(TokenType::Equal, "=".to_string());
        } else if self.is_alpha() {
            self.parse_ident_or_key();
        } 
        else if self.is_int(self.ch) {
            self.parse_number();
        } 
    }

    fn advance(&mut self) {
        if self.ch == Some('\n') {
            self.line += 1;
            self.col   = 1;
        } else {
            self.col += 1
        }

        if self.pos >= self.code.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.code[self.pos])
        }

        self.pos += 1
    }
    fn skip_whitespace(&mut self) {
        while self.ch == Some(' ') || self.ch == Some('\n') || self.ch == Some('\t') || self.ch == Some('\r') {
            self.advance()
        }
    }
    fn add_token(&mut self, ty: TokenType, val: String) {
        self.tokens.push(Token::new(ty, val, self.line, self.col))
    }
    fn add_token_advance(&mut self, ty: TokenType, val: String) {
        self.add_token(ty, val);
        self.advance()
    }

    fn is_int(&self, ch: Option<char>) -> bool {
        ch >= Some('0') && ch <= Some('9')
    }

    fn parse_number(&mut self) {
        let mut num = String::new();

        while self.is_int(self.ch) {
            num.push(self.ch.unwrap());
            self.advance();
        }

        if matches!(self.ch, Some('.')) && self.is_int(self.peek()) {
            num.push(self.ch.unwrap());
            self.advance();
            while self.is_int(self.ch) {
                num.push(self.ch.unwrap());
                self.advance();
            }
        }

        self.add_token(TokenType::Number(num.parse().unwrap()), num);
    }

    fn peek(&self) -> Option<char> {
        if self.pos >= self.code.len() {
            return None
        }
        Some(self.code[self.pos])
    }
    
    fn is_alpha(&self) -> bool {
        self.ch >= Some('a') && self.ch <= Some('z') || self.ch >= Some('A') && self.ch <= Some('Z')
    }

    fn is_alnum(&self) -> bool {
        self.is_alpha() && self.is_alnum()
    }

    fn get_ident(&mut self) -> String {
        let mut ident = String::new();
        while self.is_alnum() {
            ident.push(self.ch.unwrap());
            self.advance()
        }

        ident
    }

    fn parse_ident_or_key(&mut self) {
        let ident = self.get_ident();
        let ty = match ident.as_str() {
            "let" => TokenType::Let,
            "fun" => TokenType::Fun,
            "return" => TokenType::Return,
            _ => TokenType::Identifier(ident.clone())
        };
        self.add_token(ty, ident)
    }
}