use core::panic;
use std::num::ParseFloatError;

use crate::{
    expr::{Expr, LiteralRepresentations},
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

    fn expression(&mut self) -> Box<Expr> {
        return self.equality();
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

        return expr;
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
        return expr;
    }

    fn term(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.factor();

        while self.matches(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.previous().clone();
            let right: Box<Expr> = self.factor();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: operator,
                right: right,
            })
        }
        return expr;
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
        return expr;
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
        return self.primary();
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.matches(vec![TokenType::FALSE]) {
            return Box::new(Expr::Literal(LiteralRepresentations::Boolean(false)));
        }
        if self.matches(vec![TokenType::TRUE]) {
            return Box::new(Expr::Literal(LiteralRepresentations::Boolean(true)));
        }
        if self.matches(vec![TokenType::NIL]) {
            return Box::new(Expr::Literal(LiteralRepresentations::Nil));
        }
        if self.matches(vec![TokenType::NUMBER, TokenType::STRING]) {
            let tt: Token = self.previous().clone();
            if tt.token_type == TokenType::NUMBER {
                let tt_val: Result<f64, ParseFloatError> = tt.literal.parse::<f64>();
                match tt_val {
                    Ok(val) => {
                        return Box::new(Expr::Literal(LiteralRepresentations::Number(
                            tt_val.unwrap(),
                        )))
                    }
                    Err(e) => panic!("Failed to parse float"),
                };
            } else {
                return Box::new(Expr::Literal(LiteralRepresentations::String(tt.literal)));
            }
        }

        if self.matches(vec![TokenType::LEFTPAREN]) {
            let expr: Box<Expr> = self.expression();
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression.");
            return Box::new(Expr::Grouping(expr));
        }

        return Box::new(Expr::FailScenario {
            reason: "Reached end, expecting an expression".to_string(),
        });
    }

    fn consume(&self, token_type: TokenType, msg: &str) {}

    fn matches(&mut self, token_type_vec: Vec<TokenType>) -> bool {
        for token in token_type_vec {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        let tt: Token = self.peek().clone();
        return tt.token_type == token_type;
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            self.current += 1;
        }
        return self.previous().clone();
    }

    fn is_at_end(&self) -> bool {
        let tt: Token = self.peek().clone();
        return tt.token_type == TokenType::EOF;
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current as usize];
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current as usize - 1];
    }
}
