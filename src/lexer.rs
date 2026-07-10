use crate::tokens::{Token, TokenType};

pub struct Lexer {
    code: Vec<char>,
    code_raw: String,
    filename: String,
    line: usize,
    col: usize,
    pos: usize,
    ch: Option<char>,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(filename: String, code_raw: String) -> Self {
        let code: Vec<char> = code_raw.chars().collect();
        Self {
            code,
            code_raw,
            filename,
            line: 1,
            col: 0,
            pos: 0,
            ch: None,
            tokens: vec![],
        }
    }

    fn error<T>(&self, e: String) -> Result<T, String> {
        let mut msg = format!(
            "{} at {}:{}:{}\n    {}",
            e,
            self.filename,
            self.line,
            self.col,
            self.code_raw.lines().nth(self.line.saturating_sub(1)).unwrap_or("")
        );
        let mut cursor = String::from("    ");
        for _ in 0..self.col.saturating_sub(1) {
            cursor.push(' ');
        }
        cursor.push('^');
        msg.push_str(&cursor);
        Err(msg)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        self.advance();
        self.add_token(TokenType::Program, self.line, self.col);
        while self.ch != None {
            self.next_token()?;
        }

        self.add_token(TokenType::Eof, self.line, self.col);
        Ok(self.tokens.clone())
    }

    fn next_token(&mut self) -> Result<(), String> {
        self.skip_whitespace();

        let ch = match self.ch {
            Some(c) => c,
            None => return Ok(()),
        };

        match ch {
            '+' => {
                if self.peek() == Some('=') {
                    self.add_token(TokenType::PlusEq, self.line, self.col);
                    self.advance();
                    self.advance();
                } else {
                    self.add_token_advance(TokenType::Plus);
                }
            }
            '-' => {
                if self.peek() == Some('=') {
                    self.add_token(TokenType::MinusEq, self.line, self.col);
                    self.advance();
                    self.advance();
                } else {
                    self.add_token_advance(TokenType::Minus);
                }
            }
            '*' => {
                if self.peek() == Some('=') {
                    self.add_token(TokenType::AsteriskEq, self.line, self.col);
                    self.advance();
                    self.advance();
                } else {
                    self.add_token_advance(TokenType::Asterisk);
                }
            }
            '/' => {
                if self.peek() == Some('/') {
                    self.advance(); // advance '/'
                    self.advance(); // advance '/'
                    while self.ch != Some('\n') && self.ch != None {
                        self.advance(); // skip semua sampe newline
                    }
                } else if self.peek() == Some('*') {
                    self.advance(); // advance '/'
                    self.advance(); // advance '*'
                    let mut depth: usize = 1;
                    loop {
                        if self.ch.is_none() {
                            return self.error("unterminated block comment".to_string());
                        }
                        if self.ch == Some('/') && self.peek() == Some('*') {
                            self.advance();
                            self.advance();
                            depth += 1;
                        } else if self.ch == Some('*') && self.peek() == Some('/') {
                            self.advance();
                            self.advance();
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        } else {
                            self.advance(); // skip semua sampe penutup
                        }
                    }
                } else if self.peek() == Some('=') {
                    self.add_token(TokenType::SlashEq, self.line, self.col);
                    self.advance();
                    self.advance();
                } else {
                    self.add_token_advance(TokenType::Slash);
                }
            }
            ';' => self.add_token_advance(TokenType::Semicolon),
            '(' => self.add_token_advance(TokenType::Lparen),
            ')' => self.add_token_advance(TokenType::Rparen),
            '{' => self.add_token_advance(TokenType::Lbrace),
            '}' => self.add_token_advance(TokenType::Rbrace),
            '[' => self.add_token_advance(TokenType::Lbracket),
            ']' => self.add_token_advance(TokenType::Rbracket),
            ',' => self.add_token_advance(TokenType::Comma),
            '.' => self.add_token_advance(TokenType::Dot),
            '=' => {
                if self.peek() == Some('=') {
                    self.add_token(TokenType::EqEq, self.line, self.col);
                    self.advance();
                    self.advance();
                } else {
                    self.add_token_advance(TokenType::Equal);
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.add_token(TokenType::LtEq, self.line, self.col);
                    self.advance();
                    self.advance();
                } else {
                    self.add_token_advance(TokenType::Lt);
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.add_token(TokenType::GtEq, self.line, self.col);
                    self.advance();
                    self.advance();
                } else {
                    self.add_token_advance(TokenType::Gt);
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.add_token(TokenType::BangEq, self.line, self.col);
                    self.advance();
                    self.advance();
                } else {
                    self.add_token_advance(TokenType::Bang);
                }
            }
            '"' => self.parse_string()?,
            _ => {
                if self.is_alpha() {
                    self.parse_ident_or_key();
                } else if self.is_int(self.ch) {
                    self.parse_number();
                } else {
                    let mut c = String::new();
                    if self.ch.is_none() {
                        c.push_str("unknown");
                    } else {
                        c.push(self.ch.unwrap())
                    }
                    return self.error(format!("unexpected character '{}'", c));
                }
            }
        }

        Ok(())
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

    fn add_token(&mut self, ty: TokenType, line: usize, col: usize) {
        self.tokens.push(Token::new(ty, line, col))
    }

    fn add_token_advance(&mut self, ty: TokenType) {
        self.add_token(ty, self.line, self.col);
        self.advance()
    }

    fn is_int(&self, ch: Option<char>) -> bool {
        ch >= Some('0') && ch <= Some('9')
    }

    fn parse_number(&mut self) {
        let start_line = self.line;
        let start_col = self.col;
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

        self.add_token(
            TokenType::Number(num.parse().unwrap()),
            start_line,
            start_col,
        );
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

    fn parse_string(&mut self) -> Result<(), String> {
        let start_line = self.line;
        let start_col = self.col;

        self.advance();
        let mut str = String::new();

        while self.ch != Some('"') && self.ch != None {
            if self.ch == Some('\\') {
                self.advance();
                match self.ch {
                    Some('n') => str.push('\n'),
                    Some('t') => str.push('\t'),
                    Some('r') => str.push('\r'),
                    Some('\\') => str.push('\\'),
                    Some('"') => str.push('"'),
                    Some('0') => str.push('\0'),
                    Some(c) => {
                        return self.error(format!("unknown escape sequence '\\{}'", c));
                    }
                    None => {
                        return self.error("unterminated string".to_string());
                    }
                }
                self.advance();
            } else {
                str.push(self.ch.unwrap());
                self.advance();
            }
        }

        if self.ch.is_none() {
            return self.error("unterminated string".to_string());
        }

        self.advance();

        self.add_token(TokenType::String(str), start_line, start_col);

        Ok(())
    }

    fn parse_ident_or_key(&mut self) {
        let start_line = self.line;
        let start_col = self.col;
        let ident = self.get_ident();
        let ty = match ident.as_str() {
            "let" => TokenType::Let,
            "fun" => TokenType::Fun,
            "return" => TokenType::Return,
            "null" => TokenType::Null,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "or" => TokenType::Or,
            "and" => TokenType::And,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "import" => TokenType::Import,
            _ => TokenType::Identifier(ident.clone()),
        };
        self.add_token(ty, start_line, start_col)
    }
}
