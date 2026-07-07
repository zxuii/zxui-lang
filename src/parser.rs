use crate::ast::{BinOp, CompOp, Expr, LogicalOp, Stmt, UnaryOp};
use crate::tokens::{Token, TokenType};

pub struct Parser {
    code: String,
    filename: String,
    tokens: Vec<Token>,
    pos: usize,
    ct: Option<Token>,

    fun_counter: usize,
    loop_counter: usize,
}

impl Parser {
    pub fn new(filename: String, code: String, tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            code,
            filename,
            tokens,
            pos: 0,
            ct: None,
            fun_counter: 0,
            loop_counter: 0,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.ct = Some(self.tokens[self.pos].clone());
            self.pos += 1;
        }
    }

    // fn peek(&self) -> Option<Token> {
    //     if !self.is_at_end() {
    //         Some(self.tokens[self.pos].clone())
    //     } else {
    //         None
    //     }
    // }

    fn is_at_end(&self) -> bool {
        matches!(&self.ct, Some(tok) if tok.ty == TokenType::Eof)
    }

    fn consume(&mut self, ty: TokenType) -> Result<(), String> {
        if let Some(tok) = &self.ct {
            if tok.ty == ty {
                self.advance();
                return Ok(());
            }
        }
        self.error::<()>(None, Some(vec![ty]))
    }

    fn error<T>(&self, msg: Option<&str>, expect: Option<Vec<TokenType>>) -> Result<T, String> {
        let expect_str = match expect {
            Some(list) => list
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<_>>()
                .join("' or '"),
            None => "unknown".to_string(),
        };

        let snippet = format!(
            "    {}",
            self.code
                .lines()
                .nth(self.ct.as_ref().unwrap().line - 1)
                .unwrap()
        );
        let mut cursor = String::from("    ");
        for _ in 0..self.ct.as_ref().unwrap().col - 1 {
            cursor.push(' ');
        }
        cursor.push('^');

        match &self.ct {
            None => Err(format!("unexpected End of File. expected '{}'", expect_str)),
            Some(tok) if tok.ty == TokenType::Eof => Err(format!(
                "unexpected End of File. expected '{}' at {}:{}:{}\n{}\n{}",
                expect_str, self.filename, tok.line, tok.col, snippet, cursor
            )),
            Some(tok) => {
                if let Some(m) = msg {
                    Err(format!(
                        "{} at {}:{}:{}\n{}\n{}",
                        m, self.filename, tok.line, tok.col, snippet, cursor
                    ))
                } else {
                    Err(format!(
                        "unexpected token '{}'. expected '{}' at {}:{}:{}\n{}\n{}",
                        tok.ty, expect_str, self.filename, tok.line, tok.col, snippet, cursor
                    ))
                }
            }
        }
    }

    pub fn parse(&mut self) -> Result<Stmt, String> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Program)?;
        if self.ct.as_ref().unwrap().ty == TokenType::Eof {
            return Ok(Stmt::Program(vec![]));
        }
        Ok(Stmt::Program(self.parse_block()?))
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        while !matches!(
            self.ct.as_ref().unwrap().ty,
            TokenType::Eof | TokenType::Rbrace
        ) {
            stmts.push(self.parse_stmt()?);
            if self.ct.as_ref().unwrap().ty == TokenType::Semicolon {
                self.consume(TokenType::Semicolon)?;
            }
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.ct.as_ref().unwrap().ty.clone() {
            TokenType::Fun => self.parse_fun_decl(),
            TokenType::Let => self.parse_var_decl(),
            TokenType::Lbrace => {
                self.consume(TokenType::Lbrace)?;
                let stmts = self.parse_block()?;
                self.consume(TokenType::Rbrace)?;
                Ok(Stmt::Block(stmts))
            }
            TokenType::Identifier(_) => {
                let expr = self.parse_factor()?;
                if matches!(
                    self.ct.as_ref().unwrap().ty,
                    TokenType::Equal
                        | TokenType::PlusEq
                        | TokenType::MinusEq
                        | TokenType::AsteriskEq
                        | TokenType::SlashEq
                ) {
                    self.parse_var_assign(expr)
                } else {
                    Ok(Stmt::ExprStmt(expr))
                }
            }
            TokenType::Return => {
                if self.fun_counter > 0 {
                    self.parse_return()
                } else {
                    self.error(Some("return statement must be inside some function."), None)
                }
            }
            TokenType::If => self.parse_if_decl(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::Break => {
                if self.loop_counter > 0 {
                    self.parse_break()
                } else {
                    self.error(Some("break statement must be inside some loop."), None)
                }
            }
            TokenType::Continue => {
                if self.loop_counter > 0 {
                    self.parse_continue()
                } else {
                    self.error(Some("continue statement must be inside some loop."), None)
                }
            }
            _ => Ok(Stmt::ExprStmt(self.parse_expr()?)),
        }
    }

    fn parse_fun_decl(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Fun)?;
        let name = match &self.ct.as_ref().unwrap().ty {
            TokenType::Identifier(n) => n.clone(),
            _ => return self.error(None, Some(vec![TokenType::Identifier(String::new())])),
        };
        self.consume(TokenType::Identifier(name.clone()))?;
        self.consume(TokenType::Lparen)?;
        let params = self.parse_params()?;
        self.consume(TokenType::Rparen)?;
        self.consume(TokenType::Lbrace)?;
        self.fun_counter += 1;
        let body = self.parse_block()?;
        self.fun_counter -= 1;
        self.consume(TokenType::Rbrace)?;
        Ok(Stmt::FunDecl { name, params, body })
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = vec![];
        if self.ct.as_ref().unwrap().ty != TokenType::Rparen {
            args.push(self.parse_expr()?);
            while self.ct.as_ref().unwrap().ty == TokenType::Comma {
                self.consume(TokenType::Comma)?;
                if self.ct.as_ref().unwrap().ty != TokenType::Rparen {
                    args.push(self.parse_expr()?);
                }
            }
        }
        Ok(args)
    }

    fn parse_array_literal(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = vec![];
        if self.ct.as_ref().unwrap().ty != TokenType::Rbracket {
            args.push(self.parse_expr()?);
            while self.ct.as_ref().unwrap().ty == TokenType::Comma {
                self.consume(TokenType::Comma)?;
                if self.ct.as_ref().unwrap().ty != TokenType::Rbracket {
                    args.push(self.parse_expr()?);
                }
            }
        }
        Ok(args)
    }

    fn parse_params(&mut self) -> Result<Vec<String>, String> {
        let mut params = vec![];
        if let TokenType::Identifier(n) = &self.ct.as_ref().unwrap().ty {
            params.push(n.clone());
            self.advance();
        }
        while self.ct.as_ref().unwrap().ty == TokenType::Comma {
            self.consume(TokenType::Comma)?;
            if let TokenType::Identifier(n) = &self.ct.as_ref().unwrap().ty {
                params.push(n.clone());
                self.advance();
            }
        }
        Ok(params)
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Return)?;
        if matches!(
            self.ct.as_ref().unwrap().ty,
            TokenType::Semicolon | TokenType::Rbrace | TokenType::Eof
        ) {
            return Ok(Stmt::Return(Expr::NoOp));
        }
        Ok(Stmt::Return(self.parse_expr()?))
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Let)?;
        let name = match &self.ct.as_ref().unwrap().ty {
            TokenType::Identifier(n) => n.clone(),
            _ => return self.error(None, Some(vec![TokenType::Identifier(String::new())])),
        };
        self.consume(TokenType::Identifier(name.clone()))?;
        self.consume(TokenType::Equal)?;
        let expr = self.parse_expr()?;
        Ok(Stmt::Let { name, expr })
    }

    fn parse_var_assign(&mut self, target: Expr) -> Result<Stmt, String> {
        match self.ct.as_ref().unwrap().ty {
            TokenType::Equal => {
                self.consume(TokenType::Equal)?;
                let expr = self.parse_expr()?;
                Ok(Stmt::Assign { target, expr })
            }
            TokenType::PlusEq => {
                self.consume(TokenType::PlusEq)?;
                let expr = self.parse_expr()?;
                Ok(Stmt::CompAssign { target, op: BinOp::Plus, expr })
            }
            TokenType::MinusEq => {
                self.consume(TokenType::MinusEq)?;
                let expr = self.parse_expr()?;
                Ok(Stmt::CompAssign { target, op: BinOp::Minus, expr })
            }
            TokenType::AsteriskEq => {
                self.consume(TokenType::AsteriskEq)?;
                let expr = self.parse_expr()?;
                Ok(Stmt::CompAssign { target, op: BinOp::Multiply, expr })
            }
            TokenType::SlashEq => {
                self.consume(TokenType::SlashEq)?;
                let expr = self.parse_expr()?;
                Ok(Stmt::CompAssign { target, op: BinOp::Divide, expr })
            }
            _ => unreachable!("harusnya ini ga akan pernah tercapai sama sekali...")
        }
    }

    fn parse_if_decl(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::If)?;
        // self.consume(TokenType::Lparen)?;
        let expr = self.parse_expr()?;
        // self.consume(TokenType::Rparen)?;
        self.consume(TokenType::Lbrace)?;
        let block = self.parse_block()?;
        self.consume(TokenType::Rbrace)?;

        let else_block = if self.ct.as_ref().unwrap().ty == TokenType::Else {
            self.consume(TokenType::Else)?;

            if self.ct.as_ref().unwrap().ty == TokenType::If {
                Some(vec![self.parse_if_decl()?])
            } else {
                self.consume(TokenType::Lbrace)?;
                let stmts = self.parse_block()?;
                self.consume(TokenType::Rbrace)?;
                Some(stmts)
            }
        } else {
            None
        };
        Ok(Stmt::If {
            expr,
            block,
            else_block,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::While)?;
        // self.consume(TokenType::Lparen)?;
        let expr = self.parse_expr()?;
        // self.consume(TokenType::Rparen)?;
        self.consume(TokenType::Lbrace)?;
        self.loop_counter += 1;
        let block = self.parse_block()?;
        self.loop_counter -= 1;
        self.consume(TokenType::Rbrace)?;
        Ok(Stmt::While { expr, block })
    }

    fn parse_break(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Break)?;
        Ok(Stmt::Break)
    }

    fn parse_continue(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Continue)?;
        Ok(Stmt::Continue)
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::For)?;
        let name = match &self.ct.as_ref().unwrap().ty {
            TokenType::Identifier(n) => n.clone(),
            _ => return self.error(None, Some(vec![TokenType::Identifier(String::new())])),
        };
        self.consume(TokenType::Identifier(name.clone()))?;
        self.consume(TokenType::In)?;
        let expr = self.parse_expr()?;
        self.consume(TokenType::Lbrace)?;
        self.loop_counter += 1;
        let block = self.parse_block()?;
        self.loop_counter -= 1;
        self.consume(TokenType::Rbrace)?;
        Ok(Stmt::For { name, expr, block })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_logical_and()?;

        while matches!(self.ct.as_ref().unwrap().ty, TokenType::Or) {
            self.consume(TokenType::Or)?;
            node = Expr::LogicalOp {
                left: Box::new(node),
                op: LogicalOp::Or,
                right: Box::new(self.parse_logical_and()?),
            };
        }
        Ok(node)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_comparison()?;

        while matches!(self.ct.as_ref().unwrap().ty, TokenType::And) {
            self.consume(TokenType::And)?;
            node = Expr::LogicalOp {
                left: Box::new(node),
                op: LogicalOp::And,
                right: Box::new(self.parse_comparison()?),
            };
        }
        Ok(node)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_additive()?;

        while matches!(
            self.ct.as_ref().unwrap().ty,
            TokenType::Lt
                | TokenType::Gt
                | TokenType::LtEq
                | TokenType::GtEq
                | TokenType::EqEq
                | TokenType::BangEq
        ) {
            let op = self.ct.as_ref().unwrap().ty.clone();
            self.consume(op.clone())?;

            let comp_op = match op {
                TokenType::Lt => CompOp::Lt,
                TokenType::Gt => CompOp::Gt,
                TokenType::LtEq => CompOp::LtEq,
                TokenType::GtEq => CompOp::GtEq,
                TokenType::EqEq => CompOp::EqEq,
                TokenType::BangEq => CompOp::NotEq,
                _ => unreachable!(),
            };

            node = Expr::CompOp {
                left: Box::new(node),
                op: comp_op,
                right: Box::new(self.parse_additive()?),
            };
        }
        Ok(node)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_term()?;

        while matches!(
            self.ct.as_ref().unwrap().ty,
            TokenType::Plus | TokenType::Minus
        ) {
            let op = self.ct.as_ref().unwrap().ty.clone();
            self.consume(op.clone())?;

            let bin_op = match op {
                TokenType::Plus => BinOp::Plus,
                TokenType::Minus => BinOp::Minus,
                _ => unreachable!(),
            };

            node = Expr::BinOp {
                left: Box::new(node),
                op: bin_op,
                right: Box::new(self.parse_term()?),
            };
        }
        Ok(node)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_unary()?;
        while matches!(
            self.ct.as_ref().unwrap().ty,
            TokenType::Asterisk | TokenType::Slash
        ) {
            let op = self.ct.as_ref().unwrap().ty.clone();
            if op == TokenType::Asterisk {
                self.consume(TokenType::Asterisk)?;
                node = Expr::BinOp {
                    left: Box::new(node),
                    op: BinOp::Multiply,
                    right: Box::new(self.parse_unary()?),
                };
            } else {
                self.consume(TokenType::Slash)?;
                node = Expr::BinOp {
                    left: Box::new(node),
                    op: BinOp::Divide,
                    right: Box::new(self.parse_unary()?),
                };
            }
        }
        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.ct.as_ref().unwrap().ty.clone() {
            TokenType::Plus => {
                self.consume(TokenType::Plus)?;
                Ok(Expr::Unary {
                    op: UnaryOp::Plus,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            TokenType::Minus => {
                self.consume(TokenType::Minus)?;
                Ok(Expr::Unary {
                    op: UnaryOp::Minus,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            TokenType::Bang => {
                self.consume(TokenType::Bang)?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            _ => self.parse_factor(),
        }
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.ct.as_ref().unwrap().ty {
                TokenType::Lparen => {
                    self.consume(TokenType::Lparen)?;
                    let args = self.parse_args()?;
                    self.consume(TokenType::Rparen)?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                        line: self.ct.as_ref().unwrap().line,
                    };
                }

                TokenType::Lbracket => {
                    self.consume(TokenType::Lbracket)?;
                    let index = self.parse_expr()?;
                    self.consume(TokenType::Rbracket)?;
                    expr = Expr::Index {
                        target: Box::new(expr),
                        index: Box::new(index),
                    };
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.ct.as_ref().unwrap().ty.clone() {
            TokenType::Number(n) => {
                self.consume(TokenType::Number(n))?;
                Ok(Expr::Number(n))
            }
            TokenType::String(str) => {
                self.consume(TokenType::String(str.clone()))?;
                Ok(Expr::String(str))
            }
            TokenType::Null => {
                self.consume(TokenType::Null)?;
                Ok(Expr::Null)
            }
            TokenType::True => {
                self.consume(TokenType::True)?;
                Ok(Expr::Boolean(true))
            }
            TokenType::False => {
                self.consume(TokenType::False)?;
                Ok(Expr::Boolean(false))
            }
            TokenType::Lparen => {
                self.consume(TokenType::Lparen)?;
                let node = self.parse_expr()?;
                self.consume(TokenType::Rparen)?;
                Ok(node)
            }
            TokenType::Lbracket => {
                self.consume(TokenType::Lbracket)?;
                let node = Expr::Array(self.parse_array_literal()?);
                self.consume(TokenType::Rbracket)?;
                Ok(node)
            }
            TokenType::Identifier(name) => {
                self.consume(TokenType::Identifier(name.clone()))?;
                Ok(Expr::Identifier(name))
            }
            _ => self.error(
                None,
                Some(vec![
                    TokenType::Number(0.0),
                    TokenType::String("".to_string()),
                    TokenType::Lparen,
                    TokenType::Lbracket,
                    TokenType::Lbrace,
                ]),
            ),
        }
    }
}
