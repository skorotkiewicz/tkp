use crate::ast::*;
use crate::lexer::{Token, TokenWithPos};
use std::fmt;

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, line: usize) -> Self {
        Self {
            message: message.into(),
            line,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at line {}: {}", self.line, self.message)
    }
}

// ── Parser state ──────────────────────────────────────────────────────────────

struct Parser {
    tokens: Vec<TokenWithPos>,
    pos: usize,
    no_struct_literal: bool,
}

impl Parser {
    fn new(tokens: Vec<TokenWithPos>) -> Self {
        Self {
            tokens,
            pos: 0,
            no_struct_literal: false,
        }
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .map(|t| &t.token)
            .unwrap_or(&Token::Eof)
    }

    fn peek_pos(&self) -> (usize, usize) {
        self.tokens
            .get(self.pos)
            .map(|t| (t.line, t.col))
            .unwrap_or((0, 0))
    }

    fn advance(&mut self) -> &Token {
        while self.pos < self.tokens.len() {
            let tok = &self.tokens[self.pos];
            if matches!(tok.token, Token::Newline) {
                self.pos += 1;
            } else {
                break;
            }
        }
        let t = self
            .tokens
            .get(self.pos)
            .map(|t| &t.token)
            .unwrap_or(&Token::Eof);
        self.pos += 1;
        t
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
        let tok = self.advance().clone();
        if std::mem::discriminant(&tok) != std::mem::discriminant(expected) {
            let (line, _) = self.peek_pos();
            Err(ParseError::new(
                format!("Expected {:?}, found {:?}", expected, tok),
                line,
            ))
        } else {
            Ok(())
        }
    }

    fn span_here(&self) -> Span {
        let (line, col) = self.peek_pos();
        Span::new(line, col)
    }

    // ── Top-level ─────────────────────────────────────────────────────────────

    fn parse(&mut self) -> Result<Program, ParseError> {
        let mut stmts = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            if matches!(self.peek(), Token::Newline) {
                self.pos += 1;
                continue;
            }
            stmts.push(self.parse_stmt()?);
        }
        Ok(Program::new(stmts))
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().clone() {
            Token::Ijo => self.parse_var_decl(true),
            Token::Awen => self.parse_var_decl(false),
            Token::Pali => {
                // Check if it's a lambda (pali followed by '(')
                if self
                    .tokens
                    .get(self.pos + 1)
                    .is_some_and(|t| matches!(t.token, Token::Identifier(_)))
                    && self
                        .tokens
                        .get(self.pos + 2)
                        .is_some_and(|t| matches!(t.token, Token::LParen))
                {
                    return self.parse_func_def();
                }
                // Otherwise treat as expression
                let span = self.span_here();
                let expr = self.parse_expr()?;
                Ok(Stmt::new(StmtKind::ExprStmt(expr), span))
            }
            Token::Pana => self.parse_return(),
            Token::La => self.parse_if(),
            Token::Sin => self.parse_for(),
            Token::Lon => self.parse_while(),
            Token::Pini => {
                let span = self.span_here();
                self.advance();
                Ok(Stmt::new(StmtKind::Break, span))
            }
            Token::Tawa => {
                let span = self.span_here();
                self.advance();
                Ok(Stmt::new(StmtKind::Continue, span))
            }
            Token::Kulupu => self.parse_struct_def(),
            Token::Lukin => self.parse_try_catch(),
            Token::Jo => {
                let span = self.span_here();
                self.advance();
                let path = match self.advance().clone() {
                    Token::StringLiteral(s) => s,
                    tok => {
                        return Err(ParseError::new(
                            format!("Expected path string, found {:?}", tok),
                            span.line,
                        ));
                    }
                };
                Ok(Stmt::new(StmtKind::Import(path), span))
            }
            Token::Sama => self.parse_match(),
            Token::Nanpa => self.parse_enum_def(),
            Token::Ken => self.parse_impl_block(),
            _ => {
                let span = self.span_here();
                let expr = self.parse_expr()?;
                Ok(Stmt::new(StmtKind::ExprStmt(expr), span))
            }
        }
    }

    // ── Declarations ──────────────────────────────────────────────────────────

    fn parse_var_decl(&mut self, mutable: bool) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance(); // consume ijo / awen

        let name = match self.advance().clone() {
            Token::Identifier(n) => n,
            tok => {
                return Err(ParseError::new(
                    format!("Expected identifier, found {:?}", tok),
                    span.line,
                ));
            }
        };

        let ty = if matches!(self.peek(), Token::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(&Token::Eq)?;
        let value = self.parse_expr()?;
        Ok(Stmt::new(
            StmtKind::VarDecl {
                name,
                ty,
                value,
                mutable,
            },
            span,
        ))
    }

    fn parse_func_def(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance(); // consume pali

        let name = match self.advance().clone() {
            Token::Identifier(n) => n,
            tok => {
                return Err(ParseError::new(
                    format!("Expected function name, found {:?}", tok),
                    span.line,
                ));
            }
        };

        self.expect(&Token::LParen)?;
        let mut params = Vec::new();

        while !matches!(self.peek(), Token::RParen | Token::Eof) {
            let pname = match self.advance().clone() {
                Token::Identifier(n) => n,
                tok => {
                    return Err(ParseError::new(
                        format!("Expected param name, found {:?}", tok),
                        span.line,
                    ));
                }
            };
            self.expect(&Token::Colon)?;
            let pty = self.parse_type()?;
            params.push((pname, pty));
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }
        self.expect(&Token::RParen)?;

        let return_type = if matches!(self.peek(), Token::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;
        Ok(Stmt::new(
            StmtKind::FuncDef {
                name,
                params,
                return_type,
                body,
            },
            span,
        ))
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance(); // consume pana

        if matches!(self.peek(), Token::RBrace | Token::Newline | Token::Eof) {
            return Ok(Stmt::new(StmtKind::Return(None), span));
        }
        let expr = self.parse_expr()?;
        Ok(Stmt::new(StmtKind::Return(Some(expr)), span))
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance(); // consume la
        self.no_struct_literal = true;
        let cond = self.parse_expr()?;
        self.no_struct_literal = false;
        let then_block = self.parse_block()?;
        let else_block = if matches!(self.peek(), Token::Ante) {
            self.advance();
            if matches!(self.peek(), Token::La) {
                let nested = self.parse_if()?;
                Some(vec![nested])
            } else {
                Some(self.parse_block()?)
            }
        } else {
            None
        };
        Ok(Stmt::new(
            StmtKind::If {
                cond,
                then_block,
                else_block,
            },
            span,
        ))
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance(); // consume sin

        // for-in: sin x insa iterable { ... }
        if matches!(self.peek(), Token::Identifier(_)) {
            let next_pos = self.pos + 1;
            let after_name = self.tokens.get(next_pos).map(|t| &t.token);
            if matches!(after_name, Some(Token::Insa)) {
                let var_name = match self.advance().clone() {
                    Token::Identifier(n) => n,
                    _ => unreachable!(),
                };
                self.advance(); // consume insa
                let iterable = self.parse_expr()?;
                let body = self.parse_block()?;
                return Ok(Stmt::new(
                    StmtKind::ForIn {
                        var_name,
                        iterable,
                        body,
                    },
                    span,
                ));
            }
        }

        // traditional for: sin ijo i = 0; i < 10; i += 1 { ... }
        let init = if matches!(self.peek(), Token::Ijo) {
            self.parse_var_decl(true)?
        } else {
            let s = self.span_here();
            let e = self.parse_expr()?;
            Stmt::new(StmtKind::ExprStmt(e), s)
        };

        self.expect(&Token::Semicolon)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::Semicolon)?;

        let step_span = self.span_here();
        let step_expr = self.parse_expr()?;
        let step = Stmt::new(StmtKind::ExprStmt(step_expr), step_span);

        let body = self.parse_block()?;

        Ok(Stmt::new(
            StmtKind::ForLoop {
                init: Box::new(init),
                cond,
                step: Box::new(step),
                body,
            },
            span,
        ))
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance(); // consume lon
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::new(StmtKind::WhileLoop { cond, body }, span))
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.expect(&Token::LBrace)?;
        let mut stmts = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::Eof) {
            if matches!(self.peek(), Token::Newline) {
                self.pos += 1;
                continue;
            }
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&Token::RBrace)?;
        Ok(stmts)
    }

    // ── Expressions ───────────────────────────────────────────────────────────

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_or()?;
        match self.peek().clone() {
            Token::Eq => {
                self.advance();
                let value = self.parse_assignment()?;
                match left {
                    Expr::Identifier(name) => Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    }),
                    Expr::FieldAccess { object, field } => Ok(Expr::FieldAssign {
                        object,
                        field,
                        value: Box::new(value),
                    }),
                    Expr::Index { object, index } => Ok(Expr::IndexAssign {
                        object,
                        index,
                        value: Box::new(value),
                    }),
                    _ => {
                        let (line, _) = self.peek_pos();
                        Err(ParseError::new("Invalid assign target", line))
                    }
                }
            }
            Token::PlusEq | Token::MinusEq | Token::StarEq | Token::SlashEq => {
                let op_tok = self.advance().clone();
                let rhs = self.parse_assignment()?;
                let op = match op_tok {
                    Token::PlusEq => BinaryOpKind::Add,
                    Token::MinusEq => BinaryOpKind::Sub,
                    Token::StarEq => BinaryOpKind::Mul,
                    Token::SlashEq => BinaryOpKind::Div,
                    _ => unreachable!(),
                };
                if let Expr::Identifier(name) = left {
                    Ok(Expr::Assign {
                        name: name.clone(),
                        value: Box::new(Expr::BinaryOp {
                            op,
                            left: Box::new(Expr::Identifier(name)),
                            right: Box::new(rhs),
                        }),
                    })
                } else {
                    let (line, _) = self.peek_pos();
                    Err(ParseError::new("Invalid compound assignment target", line))
                }
            }
            _ => Ok(left),
        }
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Token::PipePipe) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinaryOp {
                op: BinaryOpKind::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_equality()?;
        while matches!(self.peek(), Token::AmpAmp) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::BinaryOp {
                op: BinaryOpKind::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;
        while matches!(self.peek(), Token::EqEq | Token::BangEq) {
            let op = if matches!(self.advance(), Token::EqEq) {
                BinaryOpKind::Eq
            } else {
                BinaryOpKind::NotEq
            };
            let right = self.parse_comparison()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_addition()?;
        while matches!(
            self.peek(),
            Token::Lt | Token::Gt | Token::LtEq | Token::GtEq
        ) {
            let op = match self.advance() {
                Token::Lt => BinaryOpKind::Lt,
                Token::Gt => BinaryOpKind::Gt,
                Token::LtEq => BinaryOpKind::LtEq,
                Token::GtEq => BinaryOpKind::GtEq,
                _ => unreachable!(),
            };
            let right = self.parse_addition()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplication()?;
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = if matches!(self.advance(), Token::Plus) {
                BinaryOpKind::Add
            } else {
                BinaryOpKind::Sub
            };
            let right = self.parse_multiplication()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        while matches!(self.peek(), Token::Star | Token::Slash | Token::Percent) {
            let op = match self.advance() {
                Token::Star => BinaryOpKind::Mul,
                Token::Slash => BinaryOpKind::Div,
                Token::Percent => BinaryOpKind::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().clone() {
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOpKind::Neg,
                    expr: Box::new(expr),
                })
            }
            Token::Bang => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOpKind::Not,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek().clone() {
                Token::LParen => {
                    if let Expr::Identifier(name) = expr {
                        self.advance();
                        let mut args = Vec::new();
                        while !matches!(self.peek(), Token::RParen | Token::Eof) {
                            args.push(self.parse_expr()?);
                            if matches!(self.peek(), Token::Comma) {
                                self.advance();
                            }
                        }
                        self.expect(&Token::RParen)?;
                        expr = Expr::Call { name, args };
                    } else {
                        break;
                    }
                }
                Token::LBracket => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(&Token::RBracket)?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Token::Dot => {
                    self.advance();
                    let member = match self.advance().clone() {
                        Token::Identifier(n) => n,
                        Token::IntLiteral(n) => {
                            expr = Expr::TupleIndex {
                                object: Box::new(expr),
                                index: n as usize,
                            };
                            continue;
                        }
                        tok => {
                            let (line, _) = self.peek_pos();
                            return Err(ParseError::new(
                                format!("Expected field name, found {:?}", tok),
                                line,
                            ));
                        }
                    };
                    if matches!(self.peek(), Token::LParen) {
                        self.advance();
                        let mut args = Vec::new();
                        while !matches!(self.peek(), Token::RParen | Token::Eof) {
                            args.push(self.parse_expr()?);
                            if matches!(self.peek(), Token::Comma) {
                                self.advance();
                            }
                        }
                        self.expect(&Token::RParen)?;
                        expr = Expr::MethodCall {
                            object: Box::new(expr),
                            method: member,
                            args,
                        };
                    } else {
                        expr = Expr::FieldAccess {
                            object: Box::new(expr),
                            field: member,
                        };
                    }
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let (line, _) = self.peek_pos();
        match self.peek().clone() {
            Token::IntLiteral(n) => {
                let val = n;
                self.advance();
                // range check
                if matches!(self.peek(), Token::DotDot) {
                    self.advance();
                    let end = self.parse_expr()?;
                    return Ok(Expr::Range {
                        start: Box::new(Expr::IntLiteral(val)),
                        end: Box::new(end),
                    });
                }
                Ok(Expr::IntLiteral(val))
            }
            Token::FloatLiteral(f) => {
                self.advance();
                Ok(Expr::FloatLiteral(f))
            }
            Token::StringLiteral(_) => {
                if let Token::StringLiteral(s) = self.advance().clone() {
                    Ok(Expr::StringLiteral(s))
                } else {
                    unreachable!()
                }
            }
            Token::Kin => {
                self.advance();
                Ok(Expr::BoolLiteral(true))
            }
            Token::Ala => {
                self.advance();
                Ok(Expr::BoolLiteral(false))
            }
            Token::Weka => {
                self.advance();
                Ok(Expr::NullLiteral)
            }
            Token::Pali => {
                // Lambda: pali(params) { body }
                self.advance();
                self.expect(&Token::LParen)?;
                let mut params = Vec::new();
                while !matches!(self.peek(), Token::RParen | Token::Eof) {
                    let pname = match self.advance().clone() {
                        Token::Identifier(n) => n,
                        tok => {
                            return Err(ParseError::new(
                                format!("Expected param, found {:?}", tok),
                                line,
                            ));
                        }
                    };
                    let pty = if matches!(self.peek(), Token::Colon) {
                        self.advance();
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    params.push((pname, pty));
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(&Token::RParen)?;
                let body = self.parse_block()?;
                Ok(Expr::Lambda { params, body })
            }
            Token::Identifier(_) => {
                if let Token::Identifier(name) = self.advance().clone() {
                    // Check for :: (enum variant / path)
                    if matches!(self.peek(), Token::ColonColon) {
                        self.advance();
                        if let Token::Identifier(variant) = self.advance().clone() {
                            return Ok(Expr::Identifier(format!("{}::{}", name, variant)));
                        }
                    }
                    // Check for struct literal: Name { ... }
                    if matches!(self.peek(), Token::LBrace) && !self.no_struct_literal {
                        let save = self.pos;
                        self.advance();
                        if matches!(self.peek(), Token::Identifier(_)) {
                            let next2 = self.tokens.get(self.pos + 1).map(|t| &t.token);
                            if matches!(next2, Some(Token::Colon)) {
                                // struct literal
                                let mut fields = Vec::new();
                                while !matches!(self.peek(), Token::RBrace | Token::Eof) {
                                    let fname = match self.advance().clone() {
                                        Token::Identifier(n) => n,
                                        tok => {
                                            return Err(ParseError::new(
                                                format!("Expected field name, found {:?}", tok),
                                                line,
                                            ));
                                        }
                                    };
                                    self.expect(&Token::Colon)?;
                                    let fval = self.parse_expr()?;
                                    fields.push((fname, fval));
                                    if matches!(self.peek(), Token::Comma) {
                                        self.advance();
                                    }
                                }
                                self.expect(&Token::RBrace)?;
                                return Ok(Expr::StructLiteral { name, fields });
                            }
                        }
                        // not a struct literal, backtrack
                        self.pos = save;
                    }
                    // range: identifier..end
                    if matches!(self.peek(), Token::DotDot) {
                        self.advance();
                        let end = self.parse_expr()?;
                        return Ok(Expr::Range {
                            start: Box::new(Expr::Identifier(name)),
                            end: Box::new(end),
                        });
                    }
                    Ok(Expr::Identifier(name))
                } else {
                    unreachable!()
                }
            }
            Token::LParen => {
                self.advance();
                // Tuple or grouped expr
                let first = self.parse_expr()?;
                if matches!(self.peek(), Token::Comma) {
                    let mut items = vec![first];
                    while matches!(self.peek(), Token::Comma) {
                        self.advance();
                        if matches!(self.peek(), Token::RParen) {
                            break;
                        }
                        items.push(self.parse_expr()?);
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Expr::TupleLiteral(items))
                } else {
                    self.expect(&Token::RParen)?;
                    Ok(first)
                }
            }
            Token::LBracket => {
                self.advance();
                let mut items = Vec::new();
                while !matches!(self.peek(), Token::RBracket | Token::Eof) {
                    items.push(self.parse_expr()?);
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(&Token::RBracket)?;
                Ok(Expr::ArrayLiteral(items))
            }
            Token::Toki => {
                self.advance();
                self.expect(&Token::LParen)?;
                let mut args = Vec::new();
                while !matches!(self.peek(), Token::RParen | Token::Eof) {
                    args.push(self.parse_expr()?);
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(&Token::RParen)?;
                Ok(Expr::Call {
                    name: "toki".to_string(),
                    args,
                })
            }
            Token::Kute => {
                self.advance();
                self.expect(&Token::LParen)?;
                let mut args = Vec::new();
                while !matches!(self.peek(), Token::RParen | Token::Eof) {
                    args.push(self.parse_expr()?);
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(&Token::RParen)?;
                Ok(Expr::Call {
                    name: "kute".to_string(),
                    args,
                })
            }
            tok => Err(ParseError::new(
                format!("Expected expression, found {:?}", tok),
                line,
            )),
        }
    }

    // ── Type parsing ──────────────────────────────────────────────────────────

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let (line, _) = self.peek_pos();
        if matches!(self.peek(), Token::LParen) {
            self.advance();
            let mut types = Vec::new();
            while !matches!(self.peek(), Token::RParen | Token::Eof) {
                types.push(self.parse_type()?);
                if matches!(self.peek(), Token::Comma) {
                    self.advance();
                }
            }
            self.expect(&Token::RParen)?;
            return Ok(Type::Wan(types));
        }
        if matches!(self.peek(), Token::LBracket) {
            self.advance();
            let inner = self.parse_type()?;
            self.expect(&Token::RBracket)?;
            return Ok(Type::Kulupu(Box::new(inner)));
        }
        match self.advance().clone() {
            Token::NanpaTy => Ok(Type::Nanpa),
            Token::KipisiTy => Ok(Type::Kipisi),
            Token::SitelenTy => Ok(Type::Sitelen),
            Token::LawaTy => Ok(Type::Lawa),
            Token::Weka => Ok(Type::Weka),
            Token::Pali => Ok(Type::PaliNasin),
            Token::Identifier(name) => Ok(Type::KulupuIjo(name)),
            tok => Err(ParseError::new(
                format!(
                    "Expected type (nanpa_kind/kipisi/sitelen/lawa/weka), found {:?}",
                    tok
                ),
                line,
            )),
        }
    }

    fn parse_struct_def(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance();
        let name = match self.advance().clone() {
            Token::Identifier(n) => n,
            tok => {
                return Err(ParseError::new(
                    format!("Expected struct name, found {:?}", tok),
                    span.line,
                ));
            }
        };
        self.expect(&Token::LBrace)?;
        let mut fields = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::Eof) {
            let fname = match self.advance().clone() {
                Token::Identifier(n) => n,
                tok => {
                    return Err(ParseError::new(
                        format!("Expected field name, found {:?}", tok),
                        span.line,
                    ));
                }
            };
            self.expect(&Token::Colon)?;
            let ftype = self.parse_type()?;
            fields.push((fname, ftype));
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }
        self.expect(&Token::RBrace)?;
        Ok(Stmt::new(StmtKind::StructDef { name, fields }, span))
    }

    fn parse_try_catch(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance();
        let try_block = self.parse_block()?;
        let (line, _) = self.peek_pos();
        match self.peek().clone() {
            Token::Alasa => {
                self.advance();
                self.expect(&Token::LParen)?;
                let error_name = match self.advance().clone() {
                    Token::Identifier(n) => n,
                    tok => {
                        return Err(ParseError::new(
                            format!("Expected error var name, found {:?}", tok),
                            line,
                        ));
                    }
                };
                self.expect(&Token::RParen)?;
                let catch_block = self.parse_block()?;
                Ok(Stmt::new(
                    StmtKind::TryCatch {
                        try_block,
                        error_name,
                        catch_block,
                    },
                    span,
                ))
            }
            tok => Err(ParseError::new(
                format!("Expected 'alasa', found {:?}", tok),
                line,
            )),
        }
    }

    fn parse_match(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance();
        self.no_struct_literal = true;
        let expr = self.parse_or()?;
        self.no_struct_literal = false;
        self.expect(&Token::LBrace)?;
        let mut arms = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::Eof) {
            let pattern = self.parse_pattern()?;
            self.expect(&Token::FatArrow)?;
            let body = if matches!(self.peek(), Token::LBrace) {
                self.parse_block()?
            } else {
                let s = self.parse_stmt()?;
                if matches!(self.peek(), Token::Comma) {
                    self.advance();
                }
                vec![s]
            };
            arms.push(MatchArm { pattern, body });
        }
        self.expect(&Token::RBrace)?;
        Ok(Stmt::new(StmtKind::Match { expr, arms }, span))
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        let (line, _) = self.peek_pos();
        match self.peek().clone() {
            Token::IntLiteral(n) => {
                self.advance();
                Ok(Pattern::IntLiteral(n))
            }
            Token::FloatLiteral(f) => {
                self.advance();
                Ok(Pattern::FloatLiteral(f))
            }
            Token::StringLiteral(_) => {
                if let Token::StringLiteral(s) = self.advance().clone() {
                    Ok(Pattern::StringLiteral(s))
                } else {
                    unreachable!()
                }
            }
            Token::Kin => {
                self.advance();
                Ok(Pattern::BoolLiteral(true))
            }
            Token::Ala => {
                self.advance();
                Ok(Pattern::BoolLiteral(false))
            }
            Token::Identifier(name) if name == "_" => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            Token::Identifier(_) => {
                if let Token::Identifier(name) = self.advance().clone() {
                    Ok(Pattern::Identifier(name))
                } else {
                    unreachable!()
                }
            }
            Token::LBracket => {
                self.advance();
                let mut pats = Vec::new();
                while !matches!(self.peek(), Token::RBracket | Token::Eof) {
                    pats.push(self.parse_pattern()?);
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(&Token::RBracket)?;
                Ok(Pattern::ArrayPattern(pats))
            }
            tok => Err(ParseError::new(
                format!("Expected pattern, found {:?}", tok),
                line,
            )),
        }
    }

    fn parse_impl_block(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance();
        let struct_name = match self.advance().clone() {
            Token::Identifier(n) => n,
            tok => {
                return Err(ParseError::new(
                    format!("Expected struct name, found {:?}", tok),
                    span.line,
                ));
            }
        };
        self.expect(&Token::LBrace)?;
        let mut methods = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::Eof) {
            if matches!(self.peek(), Token::Pali) {
                methods.push(self.parse_func_def()?);
            } else {
                break;
            }
        }
        self.expect(&Token::RBrace)?;
        Ok(Stmt::new(
            StmtKind::ImplBlock {
                struct_name,
                methods,
            },
            span,
        ))
    }

    fn parse_enum_def(&mut self) -> Result<Stmt, ParseError> {
        let span = self.span_here();
        self.advance();
        let name = match self.advance().clone() {
            Token::Identifier(n) => n,
            tok => {
                return Err(ParseError::new(
                    format!("Expected enum name, found {:?}", tok),
                    span.line,
                ));
            }
        };
        self.expect(&Token::LBrace)?;
        let mut variants = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::Eof) {
            match self.advance().clone() {
                Token::Identifier(v) => variants.push(v),
                tok => {
                    return Err(ParseError::new(
                        format!("Expected variant name, found {:?}", tok),
                        span.line,
                    ));
                }
            }
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }
        self.expect(&Token::RBrace)?;
        Ok(Stmt::new(StmtKind::EnumDef { name, variants }, span))
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

pub fn parse(tokens: Vec<TokenWithPos>) -> Result<Program, ParseError> {
    Parser::new(tokens).parse()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    fn parse_src(src: &str) -> Program {
        let tokens = tokenize(src);
        parse(tokens).expect("parse failed")
    }

    #[test]
    fn test_parse_func_def() {
        let src = "pali add(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind { pana a + b }";
        let prog = parse_src(src);
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0].kind {
            StmtKind::FuncDef {
                name,
                params,
                return_type,
                body,
            } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0], ("a".to_string(), Type::Nanpa));
                assert_eq!(params[1], ("b".to_string(), Type::Nanpa));
                assert!(matches!(return_type, Some(Type::Nanpa)));
                assert_eq!(body.len(), 1);
                assert!(matches!(body[0].kind, StmtKind::Return(Some(_))));
            }
            _ => panic!("Expected FuncDef"),
        }
    }

    #[test]
    fn test_parse_var_decl() {
        let src = "ijo age = 20";
        let prog = parse_src(src);
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0].kind {
            StmtKind::VarDecl {
                name,
                ty,
                value,
                mutable,
            } => {
                assert_eq!(name, "age");
                assert!(ty.is_none());
                assert!(matches!(value, Expr::IntLiteral(20)));
                assert!(*mutable);
            }
            _ => panic!("Expected VarDecl"),
        }
    }

    #[test]
    fn test_parse_if_else() {
        let src = "la x > 0 { pana x } ante { pana -x }";
        let prog = parse_src(src);
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0].kind {
            StmtKind::If {
                cond,
                then_block,
                else_block,
            } => {
                assert!(matches!(
                    cond,
                    Expr::BinaryOp {
                        op: BinaryOpKind::Gt,
                        ..
                    }
                ));
                assert_eq!(then_block.len(), 1);
                assert!(else_block.is_some());
                assert_eq!(else_block.as_ref().unwrap().len(), 1);
            }
            _ => panic!("Expected If"),
        }
    }

    #[test]
    fn test_parse_for_loop() {
        let src = "sin ijo i = 0; i < 10; i += 1 { toki(i) }";
        let prog = parse_src(src);
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0].kind {
            StmtKind::ForLoop {
                init,
                cond,
                step: _,
                body,
            } => {
                assert!(matches!(&init.kind, StmtKind::VarDecl { name, .. } if name == "i"));
                assert!(matches!(
                    cond,
                    Expr::BinaryOp {
                        op: BinaryOpKind::Lt,
                        ..
                    }
                ));
                assert_eq!(body.len(), 1);
            }
            _ => panic!("Expected ForLoop"),
        }
    }

    #[test]
    fn test_expr_precedence() {
        let src = "3 + 5 * 2";
        let prog = parse_src(src);
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0].kind {
            StmtKind::ExprStmt(Expr::BinaryOp { op, left, right }) => {
                assert_eq!(*op, BinaryOpKind::Add);
                assert!(matches!(left.as_ref(), Expr::IntLiteral(3)));
                match right.as_ref() {
                    Expr::BinaryOp { op, left, right } => {
                        assert_eq!(*op, BinaryOpKind::Mul);
                        assert!(matches!(left.as_ref(), Expr::IntLiteral(5)));
                        assert!(matches!(right.as_ref(), Expr::IntLiteral(2)));
                    }
                    _ => panic!("Expected inner BinaryOp(Mul)"),
                }
            }
            _ => panic!("Expected ExprStmt(BinaryOp(Add, ...))"),
        }
    }

    #[test]
    fn test_parse_while_loop() {
        let src = "lon kin { pini }";
        let prog = parse_src(src);
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0].kind {
            StmtKind::WhileLoop { cond, body } => {
                assert!(matches!(cond, Expr::BoolLiteral(true)));
                assert_eq!(body.len(), 1);
                assert!(matches!(body[0].kind, StmtKind::Break));
            }
            _ => panic!("Expected WhileLoop"),
        }
    }

    #[test]
    fn test_parse_const_decl_with_type() {
        let src = "awen max_val: nanpa_kind = 100";
        let prog = parse_src(src);
        match &prog.stmts[0].kind {
            StmtKind::VarDecl {
                name,
                ty,
                value,
                mutable,
            } => {
                assert_eq!(name, "max_val");
                assert!(matches!(ty, Some(Type::Nanpa)));
                assert!(matches!(value, Expr::IntLiteral(100)));
                assert!(!mutable);
            }
            _ => panic!("Expected VarDecl"),
        }
    }

    #[test]
    fn test_parse_func_call() {
        let src = "add(1, 2)";
        let prog = parse_src(src);
        match &prog.stmts[0].kind {
            StmtKind::ExprStmt(Expr::Call { name, args }) => {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected ExprStmt(Call)"),
        }
    }

    #[test]
    fn test_parse_else_if_chain() {
        let src = "la x > 10 { toki(1) } ante la x > 5 { toki(2) } ante { toki(3) }";
        let prog = parse_src(src);
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0].kind {
            StmtKind::If {
                else_block: Some(else_stmts),
                ..
            } => {
                assert_eq!(else_stmts.len(), 1);
                assert!(matches!(else_stmts[0].kind, StmtKind::If { .. }));
                if let StmtKind::If {
                    else_block: Some(inner_else),
                    ..
                } = &else_stmts[0].kind
                {
                    assert_eq!(inner_else.len(), 1);
                } else {
                    panic!("Expected inner else block");
                }
            }
            _ => panic!("Expected If with else-if chain"),
        }
    }

    #[test]
    fn test_span_tracking() {
        let src = "ijo age = 20";
        let prog = parse_src(src);
        assert_eq!(prog.stmts[0].span.line, 1);
    }
}
