use crate::lox::Lox;
use crate::token::Token;
use crate::token_type::TokenType;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while (!self.is_at_end()) {
            self.start = self.current;
            //scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            "Null".to_string(),
            self.line,
        ));

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
            _ => lox.error(self.line, "Unexpected character"),
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
}
