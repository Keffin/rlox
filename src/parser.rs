use core::panic;
use std::num::ParseFloatError;

use crate::{
    expr::{Expr, LiteralRepresentations},
    lox::{self, Lox},
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

    pub fn parse(&mut self) -> Box<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.comparison();

        while self.matches(vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator: Token = self.previous().clone();
            let right: Box<Expr> = self.comparison();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }

        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.term();

        while self.matches(vec![
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator: Token = self.previous().clone();
            let right: Box<Expr> = self.term();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }
        expr
    }

    fn term(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.factor();
        while self.matches(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.previous().clone();
            print!("{:#?}", operator);
            let right: Box<Expr> = self.factor();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }
        expr
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.unary();

        while self.matches(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = self.previous().clone();
            let right: Box<Expr> = self.unary();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }
        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.matches(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.previous().clone();
            let right: Box<Expr> = self.unary();
            return Box::new(Expr::Unary {
                operator: operator,
                right: right,
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.matches(vec![TokenType::FALSE]) {
            return Box::new(Expr::Literal {
                literal: LiteralRepresentations::CustomBoolean { val: false },
            });
        }
        if self.matches(vec![TokenType::TRUE]) {
            return Box::new(Expr::Literal {
                literal: LiteralRepresentations::CustomBoolean { val: true },
            });
        }
        if self.matches(vec![TokenType::NIL]) {
            return Box::new(Expr::Literal {
                literal: LiteralRepresentations::CustomNil {
                    val: "Null".to_string(),
                },
            });
        }
        if self.matches(vec![TokenType::NUMBER, TokenType::STRING]) {
            let tt: Token = self.previous().clone();
            if tt.token_type == TokenType::NUMBER {
                let tt_val: Result<f64, ParseFloatError> = tt.literal.parse::<f64>();
                match tt_val {
                    Ok(val) => {
                        return Box::new(Expr::Literal {
                            literal: LiteralRepresentations::CustomNumber { val },
                        })
                    }
                    Err(e) => panic!("Failed to parse float {}", e),
                };
            } else {
                return Box::new(Expr::Literal {
                    literal: LiteralRepresentations::CustomString { val: tt.literal },
                });
            }
        }

        if self.matches(vec![TokenType::LEFTPAREN]) {
            let expr: Box<Expr> = self.expression();
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression.");
            return Box::new(Expr::Grouping { expr });
        }

        Box::new(Expr::FailScenario {
            reason: "Reached end, expecting an expression".to_string(),
        })
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
        if self.is_at_end() {
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
