use crate::tokens::{Token, TokenType};

pub struct Lexer {
    code: Vec<char>,
    code_raw: String,
    line: usize,
    col: usize,
    pos: usize,
    ch: Option<char>,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code_raw: String) -> Self {
        let code: Vec<char> = code_raw.chars().collect();
        Self {
            code,
            code_raw,
            line: 1,
            col: 0,
            pos: 0,
            ch: None,
            tokens: vec![],
        }
    }

    fn error(&self, e: String) {
        eprintln!("Lexer Error: {}", e);
        eprintln!("    {}", self.code_raw.lines().nth(self.line - 1).unwrap());
        let mut cursor = String::from("    ");
        for _ in 0..self.col - 1 {
            cursor.push(' ');
        }
        cursor.push('^');
        eprintln!("{}", cursor);
        std::process::exit(1);
    }

    pub fn tokenize(&mut self) {
        self.advance();
        self.add_token(TokenType::Program);
        while !(self.ch == None) {
            match self.next_token() {
                Ok(_) => {}
                Err(e) => {
                    self.error(e);
                }
            }
        }

        self.add_token(TokenType::Eof);
    }
    fn next_token(&mut self) -> Result<(), String> {
        self.skip_whitespace();

        if self.ch == None {
            return Ok(());
        }

        if self.ch == Some('+') {
            self.add_token_advance(TokenType::Plus);
            Ok(())
        } else if self.ch == Some('-') {
            self.add_token_advance(TokenType::Minus);
            Ok(())
        } else if self.ch == Some('*') {
            self.add_token_advance(TokenType::Asterisk);
            Ok(())
        } else if self.ch == Some('/') {
            self.add_token_advance(TokenType::Slash);
            Ok(())
        } else if self.ch == Some(';') {
            self.add_token_advance(TokenType::Semicolon);
            Ok(())
        } else if self.ch == Some('(') {
            self.add_token_advance(TokenType::Lparen);
            Ok(())
        } else if self.ch == Some(')') {
            self.add_token_advance(TokenType::Rparen);
            Ok(())
        } else if self.ch == Some('{') {
            self.add_token_advance(TokenType::Lbrace);
            Ok(())
        } else if self.ch == Some('}') {
            self.add_token_advance(TokenType::Rbrace);
            Ok(())
        } else if self.ch == Some(',') {
            self.add_token_advance(TokenType::Comma);
            Ok(())
        } else if self.ch == Some('=') {
            self.add_token_advance(TokenType::Equal);
            Ok(())
        } else if self.is_alpha() {
            self.parse_ident_or_key();
            Ok(())
        } else if self.is_int(self.ch) {
            self.parse_number();
            Ok(())
        } else {
            let mut c = String::new();
            if self.ch.is_none() {
                c.push_str("something");
            } else {
                c.push(self.ch.unwrap())
            }
            Err(format!("unexpected character '{}' at {}:{}", c, self.line, self.col).to_string())
        }
    }

    fn advance(&mut self) {
        if self.ch == Some('\n') {
            self.line += 1;
            self.col = 1;
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
        while self.ch == Some(' ')
            || self.ch == Some('\n')
            || self.ch == Some('\t')
            || self.ch == Some('\r')
        {
            self.advance()
        }
    }
    fn add_token(&mut self, ty: TokenType) {
        self.tokens.push(Token::new(ty, self.line, self.col))
    }
    fn add_token_advance(&mut self, ty: TokenType) {
        self.add_token(ty);
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

        self.add_token(TokenType::Number(num.parse().unwrap()));
    }

    fn peek(&self) -> Option<char> {
        if self.pos >= self.code.len() {
            return None;
        }
        Some(self.code[self.pos])
    }

    fn is_alpha(&self) -> bool {
        self.ch >= Some('a') && self.ch <= Some('z') || self.ch >= Some('A') && self.ch <= Some('Z')
    }

    fn is_alnum(&self) -> bool {
        self.is_alpha() || self.is_int(self.ch)
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
            _ => TokenType::Identifier(ident.clone()),
        };
        self.add_token(ty)
    }
}
