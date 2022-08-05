use std::collections::HashMap;

use crate::lox::Lox;
use crate::token::{self, Token};
use crate::token_type::TokenType;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                (String::from("and"), TokenType::AND),
                (String::from("class"), TokenType::CLASS),
                (String::from("else"), TokenType::ELSE),
                (String::from("false"), TokenType::FALSE),
                (String::from("for"), TokenType::FOR),
                (String::from("fun"), TokenType::FUN),
                (String::from("if"), TokenType::IF),
                (String::from("nil"), TokenType::NIL),
                (String::from("or"), TokenType::OR),
                (String::from("print"), TokenType::PRINT),
                (String::from("return"), TokenType::RETURN),
                (String::from("super"), TokenType::SUPER),
                (String::from("this"), TokenType::THIS),
                (String::from("true"), TokenType::TRUE),
                (String::from("var"), TokenType::VAR),
                (String::from("while"), TokenType::WHILE),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            //scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            "Null".to_string(),
            self.line,
        ));

        // Return reference or add clone to Token type struct.
        return &self.tokens;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as u32;
    }

    fn scan_token(&mut self, mut lox: Lox) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFTPAREN, "Null".to_string()),
            ')' => self.add_token(TokenType::RIGHTPAREN, "Null".to_string()),
            '{' => self.add_token(TokenType::LEFTBRACE, "Null".to_string()),
            '}' => self.add_token(TokenType::RIGHTBRACE, "Null".to_string()),
            ',' => self.add_token(TokenType::COMMA, "Null".to_string()),
            '.' => self.add_token(TokenType::DOT, "Null".to_string()),
            '-' => self.add_token(TokenType::MINUS, "Null".to_string()),
            '+' => self.add_token(TokenType::PLUS, "Null".to_string()),
            ';' => self.add_token(TokenType::SEMICOLON, "Null".to_string()),
            '*' => self.add_token(TokenType::STAR, "Null".to_string()),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BANGEQUAL, "Null".to_string())
                } else {
                    self.add_token(TokenType::BANG, "Null".to_string())
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EQUALEQUAL, "Null".to_string())
                } else {
                    self.add_token(TokenType::EQUAL, "Null".to_string())
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LESSEQUAL, "Null".to_string())
                } else {
                    self.add_token(TokenType::LESS, "Null".to_string())
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GREATEREQUAL, "Null".to_string())
                } else {
                    self.add_token(TokenType::GREATER, "Null".to_string())
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, "Null".to_string());
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(lox),
            'o' => {
                if self.matches('r') {
                    self.add_token(TokenType::OR, "Null".to_string());
                }
            }
            _ => {
                if self.is_digit(c) {
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    lox.error(self.line, "Unexpected character")
                }
            }
        };
    }

    fn advance(&mut self) -> char {
        let curr_char: char = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        return curr_char;
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let text: &str = &self.source[self.start as usize..self.current as usize];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line))
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\n';
        }
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn string(&mut self, mut lox: Lox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            lox.error(self.line, "Unterminated string");
        }

        self.advance();

        let start_inc: u32 = self.start + 1;
        let current_dec: u32 = self.current - 1;

        let value: &str = &self.source[start_inc as usize..current_dec as usize];
        self.add_token(TokenType::STRING, value.to_string())
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let num_to_str: &str = &self.source[self.start as usize..self.current as usize];
        self.add_token(TokenType::NUMBER, num_to_str.to_string())
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }
        return self.source.chars().nth(self.current as usize + 1).unwrap();
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: &str = &self.source[self.start as usize..self.current as usize];
        let token_type: Option<&TokenType> = self.keywords.get(&text.to_string());
        match token_type {
            Some(TokenType::AND) => self.add_token(TokenType::AND, "Null".to_string()),
            Some(TokenType::CLASS) => self.add_token(TokenType::CLASS, "Null".to_string()),
            Some(TokenType::ELSE) => self.add_token(TokenType::ELSE, "Null".to_string()),
            Some(TokenType::FALSE) => self.add_token(TokenType::FALSE, "Null".to_string()),
            Some(TokenType::FOR) => self.add_token(TokenType::FOR, "Null".to_string()),
            Some(TokenType::FUN) => self.add_token(TokenType::FUN, "Null".to_string()),
            Some(TokenType::IF) => self.add_token(TokenType::IF, "Null".to_string()),
            Some(TokenType::NIL) => self.add_token(TokenType::NIL, "Null".to_string()),
            Some(TokenType::OR) => self.add_token(TokenType::OR, "Null".to_string()),
            Some(TokenType::PRINT) => self.add_token(TokenType::PRINT, "Null".to_string()),
            Some(TokenType::RETURN) => self.add_token(TokenType::RETURN, "Null".to_string()),
            Some(TokenType::SUPER) => self.add_token(TokenType::SUPER, "Null".to_string()),
            Some(TokenType::THIS) => self.add_token(TokenType::THIS, "Null".to_string()),
            Some(TokenType::TRUE) => self.add_token(TokenType::TRUE, "Null".to_string()),
            Some(TokenType::VAR) => self.add_token(TokenType::VAR, "Null".to_string()),
            Some(TokenType::WHILE) => self.add_token(TokenType::WHILE, "Null".to_string()),
            _ => self.add_token(TokenType::IDENTIFIER, "Null".to_string()),
        };
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }
}
