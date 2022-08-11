use core::panic;
use std::num::ParseFloatError;

use crate::{
    expr::{Expr, LiteralRepresentations},
    lox::{self, Lox},
    stmt::{Expression, Print, Stmt},
    token::{self, Token},
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse_stmts(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement());
        }

        statements
    }

    fn statement(&mut self) -> Stmt {
        if self.matches(vec![TokenType::PRINT]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Stmt {
        let print_expr: Expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        return Stmt::Print(Print {
            print_expression: print_expr,
        });
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr: Expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.");
        return Stmt::Expression(Expression { expression: expr });
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.matches(vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.matches(vec![
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();
        while self.matches(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.previous().clone();
            print!("{:#?}", operator);
            let right: Expr = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.matches(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary();
            return Expr::Unary {
                operator: operator,
                right: Box::new(right),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.matches(vec![TokenType::FALSE]) {
            return Expr::Literal {
                literal: LiteralRepresentations::CustomBoolean { val: false },
            };
        }
        if self.matches(vec![TokenType::TRUE]) {
            return Expr::Literal {
                literal: LiteralRepresentations::CustomBoolean { val: true },
            };
        }
        if self.matches(vec![TokenType::NIL]) {
            return Expr::Literal {
                literal: LiteralRepresentations::CustomNil {
                    val: "Null".to_string(),
                },
            };
        }
        if self.matches(vec![TokenType::NUMBER, TokenType::STRING]) {
            let tt: Token = self.previous().clone();
            if tt.token_type == TokenType::NUMBER {
                let tt_val: Result<f64, ParseFloatError> = tt.literal.parse::<f64>();
                match tt_val {
                    Ok(val) => {
                        return Expr::Literal {
                            literal: LiteralRepresentations::CustomNumber { val },
                        }
                    }
                    Err(e) => panic!("Failed to parse float {}", e),
                };
            } else {
                return Expr::Literal {
                    literal: LiteralRepresentations::CustomString { val: tt.literal },
                };
            }
        }

        if self.matches(vec![TokenType::LEFTPAREN]) {
            let expr: Expr = self.expression();
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression.");
            return Expr::Grouping {
                expr: Box::new(expr),
            };
        }

        Expr::FailScenario {
            reason: "Reached end, expecting an expression".to_string(),
        }
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        msg: &'static str,
    ) -> Result<Token, (&Token, &'static str)> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        return Err((self.peek(), msg));
    }

    fn matches(&mut self, token_type_vec: Vec<TokenType>) -> bool {
        for token in token_type_vec {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        let tt: Token = self.peek().clone();
        tt.token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        let tt: Token = self.peek().clone();
        tt.token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current as usize]
    }

    fn previous(&self) -> &Token {
        if self.current == 0 {
            return &self.tokens[0];
        }
        &self.tokens[self.current as usize - 1]
    }

    fn error(&self, token: Token, message: &str, mut lox: Lox) {
        lox.parser_error(token, message);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => (),
            }

            self.advance();
        }
    }
}
